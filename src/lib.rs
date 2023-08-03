use std::io::Error as IoError;
use event::Event;
use channel::Channel;

pub mod channel;
pub mod event;

pub struct Notification {
    channels: Vec<Box<dyn Channel>>,
}

impl Notification {
    pub fn via(channels: Vec<Box<dyn Channel>>) -> Self {
        Notification { channels }
    }

    pub fn send(&self, message: &dyn Event) -> Result<(), IoError> {
        for channel in &self.channels {
            channel.send(message)?;
        }
        Ok(())
    }
}

