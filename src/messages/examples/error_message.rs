use actix::Handler;

use crate::messages::MyWebSocket;

pub struct ErrorMessage {
    pub code: String,
    pub message: String,
}

impl actix::Message for ErrorMessage {
    type Result = ();
}

impl Handler<ErrorMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: ErrorMessage, ctx: &mut Self::Context) -> Self::Result {
        // 在这里处理 ErrorMessage
        let s = format!("{}:{}", msg.code, msg.message);
        ctx.text(s);
    }
}
