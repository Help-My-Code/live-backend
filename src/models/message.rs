use crate::models::delta::Delta;
use crate::models::user::User;
use uuid::Uuid;

pub enum CompilationEvent {
    START,
    END,
}

pub enum WsMessage {
    Message {
        user_id: User,
        content: String,
        room_id: Uuid,
    },
    CodeUpdate {
        user_id: String,
        content: Delta,
    },
    CompilationEvent {
        state: CompilationEvent,
        stdout: Option<String>,
    },

}
