use std::{time::{Duration, Instant}, collections::HashMap};
use actix::prelude::*;
use actix_web_actors::ws;
use rand::{prelude::ThreadRng, Rng};

use crate::event::{CodeUpdate, Disconnect, Connect};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodeSession {
    pub id: usize,
    pub hb: Instant,
    pub addr: Addr<CodeServer>,
}

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

impl CodeSession {
    pub fn new(id: usize, addr: Addr<CodeServer>) -> Self {
        CodeSession {
            id,
            hb: Instant::now(),
            addr,
        }
    }
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(Disconnect { id: act.id.clone() });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
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

impl Handler<CodeUpdate> for CodeSession {
    type Result = ();

    fn handle(&mut self, msg: CodeUpdate, ctx: &mut Self::Context) {
//        ctx.text(msg.code);
    }
}

impl Actor for CodeServer {
    type Context = Context<Self>;
}

impl Actor for CodeSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
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
                self.addr.do_send(CodeUpdate {
                    id: self.id,
                    code: text.parse().unwrap(),
                });
            }
            Ok(ws::Message::Binary(_)) => (),
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}
