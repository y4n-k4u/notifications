pub enum MessageType {
    Info,
    Error,
    Success,
}

pub struct ConsoleMessage {
    pub text: String,
    pub message_type: MessageType,
}
