/// 消息分发器（Message Dispatcher）。消息分发器的作用是根据消息类型将消息分发到对应的处理函数。
/// 这种方式的优点是可以将处理逻辑分散到各个处理函数中，使得代码更加清晰。
use serde_json::Value;
use std::collections::HashMap;

#[allow(unused)]
type Handler = fn(Value) -> ();

#[allow(unused)]
pub struct MessageDispatcher {
    handlers: HashMap<String, Handler>,
}

#[allow(unused)]
impl MessageDispatcher {
    pub fn new() -> Self {
        Self { handlers: HashMap::new() }
    }

    pub fn register_handler(&mut self, message_type: String, handler: Handler) {
        self.handlers.insert(message_type, handler);
    }

    pub fn dispatch(&self, message_type: String, message: Value) {
        if let Some(handler) = self.handlers.get(&message_type) {
            handler(message);
        } else {
            println!("No handler for message type {}", message_type);
        }
    }
}
