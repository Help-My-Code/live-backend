use std::time::Instant;

use actix::{StreamHandler, ActorContext};
use actix_web_actors::ws;

use crate::{code_session::CodeSession, models::{event::{CompileCode, CodeUpdate}, delta::Delta}};

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for CodeSession {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
      log::debug!("WEBSOCKET MESSAGE: {:?}", msg);
      match msg {
          Ok(ws::Message::Ping(msg)) => {
              self.hb = Instant::now();
              ctx.pong(&msg);
          }
          Ok(ws::Message::Pong(_)) => {
              self.hb = Instant::now();
          }
          Ok(ws::Message::Text(text)) => {
              println!("Websocket Server received {:?}", text);
              let msg = text.trim();

              if msg.starts_with('/') {
                  let mut command = msg.splitn(2, ' ');

                  match command.next() {
                      Some("/name") => {
                          if let Some(name) = command.next() {
                              self.name = Some(name.to_owned());
                              ctx.text(format!("name changed to: {name}"));
                          } else {
                              ctx.text("!!! name is required");
                          }
                      }
                      Some("/compile") => {
                          let code = command.next();
                          if code.is_none() {
                              ctx.text("!!! code is required");
                              return;
                          }
                          let code = code.unwrap();
                          self.addr.do_send(CompileCode::new(self.id, code.to_owned(), self.room.clone()));
                      }
                      Some("/code_updates") => {
                          let code = command.next();
                          if code.is_none() {
                              ctx.text("!!! code is required");
                              return;
                          }
                          let code = code.unwrap();
                          let change: Result<Vec<Delta>, serde_json::Error> = serde_json::from_str(code);
                          let change = match change {
                              Ok(change) => change,
                              Err(err) => panic!("failed to parse changes: {}", err),
                          };
                          println!("change: {:?}", change);
                          self.addr.do_send(CodeUpdate::new(self.id, code.to_owned(),self.room.clone()));
                      }
                      _ => ctx.text(format!("!!! unknown command: {msg:?}")),
                  }

                  return;
              }
          }
          Ok(ws::Message::Binary(_)) => (),
          Ok(ws::Message::Close(_)) => {
              ctx.stop();
          }
          _ => (),
      }
  }
}
