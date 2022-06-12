use std::collections::HashMap;
use rand::Rng;
use rand::prelude::ThreadRng;
use actix::prelude::*;
use actix::Recipient;

use crate::config;
use crate::event;
use crate::event::CodeUpdate;
use crate::event::Connect;
use crate::event::Disconnect;
use crate::program_dto::{Language, ProgramRequest, ProgramResponse};
use event::ExecutionResponse;

pub struct CodeServer {
  sessions: HashMap<usize, Recipient<event::Message>>,
  rng: ThreadRng,
}

impl CodeServer {
  pub fn new() -> CodeServer {
      CodeServer {
          sessions: HashMap::new(),
          rng: rand::thread_rng(),
      }
  }

  fn send_update_code(&self, message: &str, skip_id: usize) {
    for (id, _addr) in &self.sessions {
      if *id != skip_id {
        if let Some(addr) = self.sessions.get(&id) {
          let _ = addr.do_send(event::Message(message.to_string()));
        }
      }
    }
  }
  fn execute_code(&self, code: String) {
    let program_dto = ProgramRequest {
      stdin: code,
      language: Language::DART,
    };
    let client = reqwest::Client::new();
    let session_copy = self.sessions.clone();
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
          for (_id, addr) in session_copy {
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

  fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
      println!("Websocket Client");
      let id = self.rng.gen::<usize>();
      self.sessions.insert(id, msg.addr);
      println!("Websocket Client {} connected", id);
      id
  }
}

impl Handler<Disconnect> for CodeServer {
  type Result = ();

  fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) {
      println!("Websocket Client {} disconnected", msg.id);
      self.sessions.remove(&msg.id);
  }
}

impl Handler<CodeUpdate> for CodeServer {
  type Result = ();

  fn handle(&mut self, msg: CodeUpdate, _ctx: &mut Self::Context) {
    let code = msg.code.clone();

    self.send_update_code( &msg.code, msg.id );
    self.execute_code(code);
  }

}