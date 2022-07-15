use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};

use crate::{
    code_server::code_server::CodeServer,
    models::{
        delta::Delta,
        user::User,
        event::{self, CodeUpdate, CompileCode, Connect, Disconnect},
    },
};

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

    pub fn compile_code(
        &mut self,
        command: &mut std::str::SplitN<char>,
        ctx: &mut ws::WebsocketContext<CodeSession>,
    ) {
        let code = command.next();
        if code.is_none() {
            ctx.text("!!! code is required");
            return;
        }
        let code = code.unwrap();
        self.addr.do_send(CompileCode::new(
            self.id,
            code.to_owned(),
            self.room.clone(),
        ));
    }

    pub fn code_updates(
        &mut self,
        mut command: std::str::SplitN<char>,
        ctx: &mut ws::WebsocketContext<CodeSession>,
    ) {
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
        self.addr
            .do_send(CodeUpdate::new(self.id, change, self.room.clone()));
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
                user_id: User::new(uuid::Uuid::new_v4()),
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
