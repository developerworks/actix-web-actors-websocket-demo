use crate::dispatcher::MessageDispatcher;

use actix::Actor;
use actix_web_actors::ws;
use std::collections::HashMap;

#[rustfmt::skip]
use self::{
    examples::{
        text_message::TextMessageHandler, 
        get_tasks::GetTasksHandler
    }, 
    hall::{
        login::LoginHandler, 
        user_mail::UserMailHandler
    }
};

pub mod examples;
pub mod hall;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // 注册消息处理函数
        self.handlers.insert("text".to_string(), Box::new(TextMessageHandler));
        self.handlers.insert("login".to_string(), Box::new(LoginHandler));
        self.handlers.insert("get_tasks".to_string(), Box::new(GetTasksHandler));
        self.handlers.insert("user_email".to_string(), Box::new(UserMailHandler));
    }
}

// 定义处理器 trait
pub trait MessageHandler: Send + Sync {
    fn handle(&self, msg: serde_json::Value, ctx: &mut ws::WebsocketContext<MyWebSocket>);
}

pub struct MyWebSocket {
    pub id: usize,
    pub handlers: HashMap<String, Box<dyn MessageHandler>>,
    pub dispatcher: MessageDispatcher,
}
