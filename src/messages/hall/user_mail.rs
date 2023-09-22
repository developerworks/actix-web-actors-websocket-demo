use actix::AsyncContext;
use serde::Deserialize;

use crate::messages::{examples::resp::ResponseMessage, MessageHandler, MyWebSocket};

#[derive(Debug, Deserialize)]
pub struct UserMail {
    pub account: String,
    pub page: u32,
}

impl actix::Message for UserMail {
    type Result = ();
}

impl actix::Handler<UserMail> for MyWebSocket {
    type Result = ();

    #[allow(unused_variables)]
    fn handle(&mut self, msg: UserMail, ctx: &mut Self::Context) -> Self::Result {
        let response = ResponseMessage {
            code: "0".to_string(),
            message: "".to_string(),
            data: serde_json::json!({}),
        };

        ctx.address().do_send(response);
    }
}

pub struct UserMailHandler;

impl MessageHandler for UserMailHandler {
    fn handle(&self, msg: serde_json::Value, ctx: &mut actix_web_actors::ws::WebsocketContext<MyWebSocket>) {
        match serde_json::from_value::<UserMail>(msg) {
            Ok(login) => {
                println!("User {} logged in", login.account);
                let response = ResponseMessage {
                    code: "200".to_string(),
                    message: "Login successful".to_string(),
                    data: serde_json::json!({ "username": login.account }),
                };
                ctx.address().do_send(response);
            }
            Err(_) => {
                let response = ResponseMessage {
                    code: "200".to_string(),
                    message: "Login successful".to_string(),
                    data: serde_json::Value::Null,
                };
                ctx.address().do_send(response);
            }
        }
    }
}
