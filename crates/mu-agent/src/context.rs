use crate::{Message, Tool};

pub struct Context {
    pub system_prompt: Option<String>,
    pub messages: Vec<Message>,
    pub tools: Option<Vec<Tool>>,
}
