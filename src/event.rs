use actix::prelude::*;

#[derive(Message, Debug)]
#[rtype(usize)]
pub struct Message(pub String);

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct CodeUpdate {
    pub id: usize,
    pub code: String,
}

impl CodeUpdate {
    pub fn new(id: usize, code: String) -> Self {
        CodeUpdate { id, code }
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
