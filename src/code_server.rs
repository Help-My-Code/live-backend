use std::collections::HashMap;
use rand::Rng;
use rand::prelude::ThreadRng;
use actix::prelude::*;
use actix::Recipient;

use crate::config;
use crate::event;
use crate::event::CodeUpdate;
use crate::event::CompileCode;
use crate::event::Connect;
use crate::event::Disconnect;
use crate::program_dto::{Language, ProgramRequest, ProgramResponse};
use event::ExecutionResponse;
use common_macros::hash_map;


type Client = Recipient<event::Message>;
type Room = HashMap<usize, Client>;


#[derive(Default)]
pub struct CodeServer {
  rooms: HashMap<String, Room>,
  rng: ThreadRng,
}

impl CodeServer {
  pub fn new() -> CodeServer {
      CodeServer {
          rooms: hash_map! { "room_id".to_string() => HashMap::new() },
          rng: rand::thread_rng(),
      }
  }

  fn take_room(&mut self, room_name: &str) -> Option<Room> {
    let room = self.rooms.get_mut(room_name)?;
    let room = std::mem::take(room);
    Some(room)
  }

  fn send_update_code(&mut self, message: &str, skip_id: usize, room_name: &str) {
    let mut room = self.take_room(room_name).unwrap();
    println!("room: {:?}", room);

    for (id, client) in room.drain() {
      println!("Sending update code to client {} {:?}", id, client);
      if id != skip_id {
        client.do_send(event::Message(message.to_owned()));
      }
    }
  }

  fn execute_code(&mut self, code: String, room_name: &str) {
    let program_dto = ProgramRequest {
      stdin: code,
      language: Language::DART,
    };
    let client = reqwest::Client::new();
    let room = self.take_room(room_name).unwrap();
    let room_copy = room.clone();

    actix_rt::spawn(async move {
      let res = client.post(config::COMPILER_URL)
      .json(&serde_json::to_value(&program_dto).unwrap())
      .send()
      .await
      .unwrap()
      .json::<ProgramResponse>()
      .await;
      match res {
        Ok(program_response) => {
          let execution = ExecutionResponse { stdout: program_response.stdout };
          for (_id, addr) in room_copy {
            let _ = addr.do_send(event::Message(serde_json::to_string(&execution).unwrap()));
          }
        },
        Err(_) => todo!(),
      };
    });
  }
}

impl Actor for CodeServer {
  type Context = Context<Self>;
}

impl Handler<Connect> for CodeServer {
  type Result = usize;

  fn handle(&mut self, _msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
      let id = self.rng.gen::<usize>();
      println!("Websocket Client {} connected", id);
      id
  }
}

impl Handler<Disconnect> for CodeServer {
  type Result = ();

  fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) {
      println!("Websocket Client {} disconnected", msg.id);
  }
}

impl Handler<CodeUpdate> for CodeServer {
  type Result = ();

  fn handle(&mut self, msg: CodeUpdate, _ctx: &mut Self::Context) {
    self.send_update_code( &msg.code, msg.id, &msg.room_name );
  }

}

impl Handler<CompileCode> for CodeServer {
  type Result = ();

  fn handle(&mut self, msg: CompileCode, _ctx: &mut Self::Context) {
    let code = msg.code.clone();
    println!("Compiling code");
    self.execute_code(code, &msg.room_name);
  }

}

impl SystemService for CodeServer {}
impl Supervised for CodeServer {}
