use crate::event::console_message::ConsoleMessage;

pub mod console_message;

pub trait Event {
    fn to_console(&self) -> ConsoleMessage{
        todo!()
    }
}
