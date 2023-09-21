pub struct Message1 {}
pub struct Message2 {}
pub struct Message1000 {}
pub enum MessageType {
    Type1(Message1),
    Type2(Message2),
    // ...
    Type1000(Message1000),
}

mod message1 {
    pub struct Message1 {
        // ...
    }

    impl Message1 {
        pub fn handle(&self) {
            // ...
        }
    }
}

pub trait MessageHandler {
    fn handle(&self);
}

fn create_handler(message_type: MessageType) -> Box<dyn MessageHandler> {
    match message_type {
        MessageType::Type1(msg) => Box::new(message1::Message1::new(msg)),
        MessageType::Type2(msg) => Box::new(message2::Message2::new(msg)),
        // ...
        MessageType::Type1000(msg) => Box::new(message1000::Message1000::new(msg)),
    }
}

impl MessageHandler for Message1 {
    fn handle(&self) {
        // ...
    }
}

fn main() {
    let message_type = get_message_type();
    let handler = create_handler(message_type);
    handler.handle();
}