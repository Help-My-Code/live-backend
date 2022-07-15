use actix::{Actor, Context, Handler, Supervised, SystemService};
use log::info;
use rand::Rng;

use crate::models::event::{WsMessage, CodeUpdate, CompileCode, CodeUpdateOutput, Connect, Disconnect};

use super::code_server::CodeServer;

impl Actor for CodeServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for CodeServer {
    type Result = usize;

    fn handle(&mut self, event: Connect, _ctx: &mut Self::Context) -> Self::Result {
        let id = self.rng.gen::<usize>();
        let mut room = self.take_room(event.room_name.as_str()).unwrap();
        room.insert(id, event.addr);
        self.rooms.insert(event.room_name.clone(), room);
        info!(
            "Websocket Client {} connected to room {}",
            id, event.room_name
        );
        id
    }
}

impl Handler<Disconnect> for CodeServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) {
        info!("Websocket Client {} disconnected", msg.id);
    }
}

impl Handler<CodeUpdate> for CodeServer {
    type Result = ();

    fn handle(&mut self, msg: CodeUpdate, _ctx: &mut Self::Context) {
        let message = WsMessage::CodeUpdate(CodeUpdateOutput { user: msg.user, content: msg.code });
            let code_updates: String = match serde_json::to_string(&message) {
            Ok(code_updates) => code_updates,
            Err(err) => panic!("failed to serialize code updates: {}", err),
        };

        self.send_update_code(&code_updates, msg.id, &msg.room_name);
    }
}

impl Handler<CompileCode> for CodeServer {
    type Result = ();

    fn handle(&mut self, msg: CompileCode, _ctx: &mut Self::Context) {
        let code = msg.code.clone();
        info!("Compiling code");
        // TODO add start event
        self.execute_code(code, &msg.room_name);
    }
}

impl SystemService for CodeServer {}
impl Supervised for CodeServer {}
