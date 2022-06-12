use actix::prelude::*;
use serde::Serialize;

#[derive(Message, Debug)]
#[rtype(usize)]
pub struct Message(pub String);

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct CodeUpdate {
    pub id: usize,
    pub code: String,
    pub room_name: String,
}

impl CodeUpdate {
    pub fn new(id: usize, code: String, room_name: String) -> Self {
        CodeUpdate { id, code, room_name }
    }
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Clone, Message)]
#[rtype(result = "usize")]
pub struct JoinRoom(pub String, pub Option<String>, pub Recipient<Message>);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveRoom(pub String, pub usize);

#[derive(Clone, Message)]
#[rtype(result = "Vec<String>")]
pub struct ListRooms;

#[derive(Message, Serialize, Debug)]
#[rtype(result = "()")]
pub struct ExecutionResponse {
    pub stdout: String,
}
