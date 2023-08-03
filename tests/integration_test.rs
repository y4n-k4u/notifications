use std::io::Error as IoError;
use notifications::channel::Channel;
use notifications::event::Event;
use notifications::Notification;

struct TestChannel;

impl Channel for TestChannel {
    fn send(&self, _message: &dyn Event) -> Result<(), IoError> {
        Ok(())
    }
}

struct UserRegisteredMessage;

impl Event for UserRegisteredMessage {}

#[test]
fn test_send_message() {
    let channel = TestChannel;
    let message = UserRegisteredMessage;
    let result = Notification::via(vec![Box::new(channel)]).send(&message);
    assert!(result.is_ok());
}
