// text_message.rs

use actix::{AsyncContext, Handler};
use actix_web_actors::ws;

use crate::messages::response_message::ResponseMessage;

use super::{MessageHandler, MyWebSocket};

// 定义 TextMessage 消息类型

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TextMessage {
    pub text: String,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TextMessageResponse {
    pub text_message: TextMessage,
}

impl actix::Message for TextMessage {
    type Result = ();
}

// 实现 TextMessageHandler
#[allow(unused)]
pub struct TextMessageHandler;

#[allow(unused)]
impl MessageHandler for TextMessageHandler {
    fn handle(&self, msg: serde_json::Value, ctx: &mut ws::WebsocketContext<MyWebSocket>) {
        // ctx.text(msg);

        match serde_json::from_value::<TextMessage>(msg) {
            Ok(text_message) => {
                let text_message_response = TextMessageResponse {
                    text_message: TextMessage {
                        text: "OK".to_string(),
                    },
                };
                let response = ResponseMessage {
                    code: "0".to_string(),
                    message: "Login successful".to_string(),
                    data: serde_json::json!(text_message_response),
                };
                ctx.address().do_send(response);
            }
            Err(_) => {

                let response = ResponseMessage {
                    code: "1001".to_string(),
                    message: "Login successful".to_string(),
                    data: serde_json::json!({}),
                };
                ctx.address().do_send(response);
            }
        }
    }
}

impl Handler<TextMessage> for MyWebSocket {
    type Result = ();

    #[allow(unused_variables)]
    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
