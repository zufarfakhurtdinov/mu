pub struct UserMessage {
    pub content: String,
}

pub struct AssistantMessage {
    pub content: String,
}

pub enum Message {
    User(UserMessage),
    Assistant(AssistantMessage),
}
