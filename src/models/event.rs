use actix::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::user::User;
use super::delta::Delta;

#[derive(Debug, Serialize)]
pub enum Language {
  DART,
  PYTHON,
  C,
}

impl From<String> for Language {
    fn from(lang: String) -> Self {
        match lang.as_str() {
            "DART" => Language::DART,
            "PYTHON" => Language::PYTHON,
            "C" => Language::C,
            _ => unreachable!(),
        }
    }
}

#[derive(Message, Debug)]
#[rtype(usize)]
pub struct Message(pub String);

#[derive(Debug, Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct CodeUpdate {
    pub id: usize,
    pub code: Vec<Delta>,
    pub user: User,
    pub room_id: String,
}

impl CodeUpdate {
    pub fn new(id: usize, code: Vec<Delta>, room_id: String, user: User) -> Self {
        CodeUpdate {
            id,
            user,
            code,
            room_id,
        }
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct CompileCode {
    pub id: usize,
    pub code: String,
    pub language: Language,
    pub room_id: String,
}

impl CompileCode {
    pub fn new(id: usize, language: Language, code: String, room_id: String) -> Self {
        CompileCode {
            id,
            code,
            language,
            room_id,
        }
    }
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub room_id: String,
    pub user_id: User,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum CompilationState {
    START,
    END,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WsMessage {
    Message(ChatMessage),
    CodeUpdate(CodeUpdateOutput),
    CompilationEvent(CompilationEvent),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub user: User,
    pub content: String,
    pub room_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeUpdateOutput {
    pub user: User,
    pub content: Vec<Delta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilationEvent {
    pub state: CompilationState,
    pub stdout: Option<String>,
}

