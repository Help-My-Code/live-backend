use actix::prelude::*;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct CodeUpdate {
    pub id: usize,
    pub code: String,
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<CodeUpdate>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}
