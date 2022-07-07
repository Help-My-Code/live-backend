use std::time::Instant;

use actix::{ActorContext, StreamHandler};
use actix_web_actors::ws;

use crate::code_session::CodeSession;
use bytestring::ByteString;

impl CodeSession {
    fn handle_text_message(
        &mut self,
        text: ByteString,
        ctx: &mut ws::WebsocketContext<CodeSession>,
    ) {
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
                Some("/compile") => self.compile_code(&mut command, ctx),
                Some("/code_updates") => self.code_updates(command, ctx),
                _ => ctx.text(format!("!!! unknown command: {msg:?}")),
            }
            return;
        }
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
            Ok(ws::Message::Text(text)) => self.handle_text_message(text, ctx),
            Ok(ws::Message::Binary(_)) => (),
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}
