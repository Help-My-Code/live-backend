use std::{time::{Duration, Instant}};
use actix::prelude::*;
use actix_web_actors::ws;

use crate::{event::{self, CodeUpdate, Disconnect, Connect, CompileCode}, code_server::CodeServer};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodeSession {
    pub id: usize,
    pub hb: Instant,
    pub addr: Addr<CodeServer>,
    pub room: String,
    pub name: Option<String>,
}

impl CodeSession {
    pub fn new(id: usize, addr: Addr<CodeServer>, room: String, name: Option<String>) -> Self {
        CodeSession {
            id,
            hb: Instant::now(),
            addr,
            room: room.clone(),
            name: name.clone(),
        }
    }
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");

                act.addr.do_send(Disconnect { id: act.id.clone() });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }

}

impl Handler<event::Message> for CodeSession {
    type Result = usize;

    fn handle(&mut self, msg: event::Message, ctx: &mut Self::Context) -> usize {
        println!("Websocket Client {} received: {:?}", self.id, msg);
        ctx.text(msg.0);
        self.id
    }
}

impl Actor for CodeSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
                room_name: self.room.clone(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        println!("Websocket Client stopping");
        self.addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

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
                        Some("/code_update") => {
                            let code = command.next();
                            if code.is_none() {
                                ctx.text("!!! code is required");
                                return;
                            }
                            let code = code.unwrap();
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
