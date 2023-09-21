// login.rs

use actix::{Handler, AsyncContext};
use actix_web_actors::ws;
use serde::Deserialize;

use super::{MyWebSocket, MessageHandler, response_message::ResponseMessage};

// 定义 Login 消息类型
#[derive(Debug,Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl actix::Message for Login {
    type Result = ();
}

// 实现 LoginHandler
#[allow(unused)]
pub struct LoginHandler;

#[allow(unused)]

impl MessageHandler for LoginHandler {
    fn handle(&self, msg: serde_json::Value, ctx: &mut ws::WebsocketContext<MyWebSocket>) {
        println!("User {} logged in", msg);


        match serde_json::from_value::<Login>(msg) {
            Ok(login) => {
                println!("User {} logged in", login.username);
                let response = ResponseMessage {
                    code: "200".to_string(),
                    message: "Login successful".to_string(),
                    data: serde_json::json!({ "username": login.username }),
                };
                ctx.address().do_send(response);
            },
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

impl Handler<Login> for MyWebSocket {
    type Result = ();

    #[allow(unused_variables)]
    fn handle(&mut self, msg: Login, ctx: &mut Self::Context) -> Self::Result {
        // 在这里处理 Login 消息
        let response = ResponseMessage {
            code: "200".to_string(),
            message: "Login successful".to_string(),
            data: serde_json::json!({ "username": msg.username }),
        };
        ctx.address().do_send(response);
    }
}