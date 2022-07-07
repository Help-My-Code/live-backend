use actix::Recipient;
use rand::prelude::ThreadRng;
use std::collections::HashMap;
use std::env;

use crate::config;
use crate::models::event;
use crate::models::event::ExecutionResponse;
use crate::models::program_dto::{Language, ProgramRequest, ProgramResponse};

type Client = Recipient<event::Message>;
type CodeRoom = HashMap<usize, Client>;

#[derive(Default)]
pub struct CodeServer {
    pub rooms: HashMap<String, CodeRoom>,
    pub rng: ThreadRng,
}

impl CodeServer {
    pub fn new() -> CodeServer {
        CodeServer {
            rooms: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    fn insert_if_not_exist(&mut self, room_name: &str) {
        let room = self.rooms.get(room_name);
        if room.is_some() {
            println!("Room {} {:?} already exists", room_name, room);
            return;
        }
        self.rooms.insert(room_name.to_string(), HashMap::new());
    }

    pub fn take_room(&mut self, room_name: &str) -> Option<CodeRoom> {
        self.insert_if_not_exist(room_name);
        let room = self.rooms.get_mut(room_name).unwrap();
        let room = std::mem::take(room);
        Some(room)
    }

    pub fn send_update_code(&mut self, message: &str, skip_id: usize, room_name: &str) {
        let room = self.take_room(room_name).unwrap();
        self.rooms.insert(room_name.to_owned(), room.clone());
        for (id, client) in room {
            println!("Sending update code to client {} {:?}", id, client);
            if id != skip_id {
                client.do_send(event::Message(message.to_owned()));
            }
        }
    }

    pub fn execute_code(&mut self, code: String, room_name: &str) {
        let program_dto = ProgramRequest {
            stdin: code,
            language: Language::DART,
        };
        let client = reqwest::Client::new();
        let room = self.take_room(room_name).unwrap();
        let room_copy = room.clone();
        self.rooms.insert(room_name.to_owned(), room);

        actix_rt::spawn(async move {
            let res = client
                .post(env::var("COMPILER_URL").expect("doesn't find COMPILER_URL"))
                .json(&serde_json::to_value(&program_dto).unwrap())
                .send()
                .await
                .unwrap()
                .json::<ProgramResponse>()
                .await;
            match res {
                Ok(program_response) => {
                    let execution = ExecutionResponse {
                        stdout: program_response.stdout,
                    };
                    println!("execution response {:?}", execution);
                    for (_id, addr) in room_copy {
                        let _ = addr
                            .do_send(event::Message(serde_json::to_string(&execution).unwrap()));
                    }
                }
                Err(_) => todo!(),
            };
        });
    }
}