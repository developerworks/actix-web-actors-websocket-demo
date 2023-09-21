use std::collections::HashMap;

use actix::Actor;
use actix_web_actors::ws;

use self::{get_tasks::GetTasksHandler, login::LoginHandler, text_message::TextMessageHandler};

pub mod error_message;
pub mod get_tasks;
pub mod login;
pub mod response_message;
pub mod text_message;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // 注册消息处理函数
        self.handlers
            .insert("text".to_string(), Box::new(TextMessageHandler));
        self.handlers
            .insert("login".to_string(), Box::new(LoginHandler));
        self.handlers
            .insert("get_tasks".to_string(), Box::new(GetTasksHandler));
    }
}

// 定义处理器 trait
// pub trait MessageHandler: Send + Sync {
//     fn handle(&self, msg: String, ctx: &mut ws::WebsocketContext<MyWebSocket>);
// }
pub trait MessageHandler: Send + Sync {
    fn handle(&self, msg: serde_json::Value, ctx: &mut ws::WebsocketContext<MyWebSocket>);
}

pub struct MyWebSocket {
    pub id: usize,
    pub handlers: HashMap<String, Box<dyn MessageHandler>>,
}
