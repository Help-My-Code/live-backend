use actix::prelude::*;
use serde::Serialize;

use super::delta::Delta;

#[derive(Message, Debug)]
#[rtype(usize)]
pub struct Message(pub String);

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct CodeUpdate {
    pub id: usize,
    pub code: Vec<Delta>,
    pub room_name: String,
}

impl CodeUpdate {
    pub fn new(id: usize, code: Vec<Delta>, room_name: String) -> Self {
        CodeUpdate {
            id,
            code,
            room_name,
        }
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct CompileCode {
    pub id: usize,
    pub code: String,
    pub room_name: String,
}

impl CompileCode {
    pub fn new(id: usize, code: String, room_name: String) -> Self {
        CompileCode {
            id,
            code,
            room_name,
        }
    }
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub room_name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message, Serialize, Debug)]
#[rtype(result = "()")]
pub struct ExecutionResponse {
    pub stdout: String,
}
