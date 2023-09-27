// login.rs

use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use serde::Deserialize;

use crate::messages::{examples::resp::ResponseMessage, MessageHandler, MyWebSocket};

// 定义 Login 消息类型
#[derive(Debug, Deserialize, Default)]
pub struct Login {
    pub account: String,
    pub sign: String,
}

#[allow(unused)]
impl Login {
    pub fn new(account: String, sign: String) -> Self {
        Self { account, sign }
    }

    /// 用户身份认证
    pub fn authenticate(&self) -> bool {
        // 在实际应用中，你应该根据用户名和密码验证用户身份
        // 这里只是一个简单的示例，始终返回true来模拟认证成功
        self.account == "admin" && self.sign == "admin"
    }
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

impl Handler<Login> for MyWebSocket {
    type Result = ();

    #[allow(unused_variables)]
    fn handle(&mut self, msg: Login, ctx: &mut Self::Context) -> Self::Result {
        // 在这里处理 Login 消息
        let response = ResponseMessage {
            code: "200".to_string(),
            message: "Login successful".to_string(),
            data: serde_json::json!({ "username": msg.account }),
        };
        ctx.address().do_send(response);
    }
}
