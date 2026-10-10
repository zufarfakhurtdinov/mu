use crate::Tool;

pub struct Context {
    pub system_prompt: Option<String>,
    pub messages: Vec<String>,
    pub tools: Option<Vec<Tool>>,
}
