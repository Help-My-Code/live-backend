use std::collections::HashMap;
use rand::Rng;
use rand::prelude::ThreadRng;
use actix::prelude::*;
use actix::Recipient;

use crate::event::CodeUpdate;
use crate::event::Connect;
use crate::event::Disconnect;


pub struct CodeServer {
  sessions: HashMap<usize, Recipient<CodeUpdate>>,
  rng: ThreadRng,
}

impl CodeServer {
  pub fn new() -> CodeServer {
      CodeServer {
          sessions: HashMap::new(),
          rng: rand::thread_rng(),
      }
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

  fn handle(&mut self, msg: CodeUpdate, ctx: &mut Self::Context) {
//        ctx.text(msg.code);
  }
}