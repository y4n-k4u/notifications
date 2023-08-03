use std::io::Error as IoError;

use colored::Colorize;

use crate::channel::Channel;
use crate::event::console_message::MessageType;
use crate::event::Event;

pub struct ConsoleChannel;

impl Channel for ConsoleChannel {
    fn send(&self, event: &dyn Event) -> Result<(), IoError> {
        let message = event.to_console();
        let format_message = match message.message_type {
            MessageType::Error => format!("{}{}", "error: ".bold().red(), message.text),
            MessageType::Info => format!("{}{}", "info: ".bold().blue(), message.text),
            MessageType::Success => format!("{}{}", "success: ".bold().green(), message.text),
        };
        println!("{}", format_message);
        Ok(())
    }
}
