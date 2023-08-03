use std::io::Error as IoError;
use crate::event::Event;

pub mod console_channel;

pub trait Channel {
    fn send(&self, message: &dyn Event) -> Result<(), IoError>;
}
