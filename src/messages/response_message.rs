use actix::Handler;
use serde_json::Value;

use super::MyWebSocket;

pub struct ResponseMessage {
    pub code: String,
    pub message: String,
    pub data: Value
}

impl actix::Message for ResponseMessage {
    type Result = ();
}

impl Handler<ResponseMessage> for MyWebSocket {
    type Result = ();

    #[allow(unused_variables)]
    fn handle(&mut self, msg: ResponseMessage, ctx: &mut Self::Context) -> Self::Result {
        // 在这里处理 ResponseMessage
        // let s = format!("code: {}, message: {}, data: {}", msg.code, msg.message, msg.data);
        // ctx.text(s);

        
    }
}