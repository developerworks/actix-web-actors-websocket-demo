use actix_web::{App, HttpServer};
use std::{collections::HashMap, sync::Arc};

use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

type HandlerType = HashMap<String, Arc<dyn Fn(String, &mut ws::WebsocketContext<MyWebSocket>) + Send + Sync>>;

pub struct MyWebSocket {
    pub id: usize,
    handlers: HandlerType,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // 注册消息处理函数
        message_handler::register_handlers(self.handlers.clone());
    }
}

// 定义一个新的消息类型
// pub struct TextMessage(String);

// impl actix::Message for TextMessage {
//     type Result = ();
// }

// // 为MyWebSocket处理TextMessage消息
// impl Handler<TextMessage> for MyWebSocket {
//     type Result = ();

//     fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) {
//         ctx.text(msg.0);
//     }
// }

async fn websocket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        MyWebSocket {
            id: 1,
            handlers: HashMap::new(),
        },
        &req,
        stream,
    );
    println!("{:?}", resp);
    resp
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                // 发送TextMessage消息给自己
                // ctx.address().do_send(TextMessage(text.to_string()));

                let data: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&text);

                match data {
                    Ok(v) => {
                        if let Some(message_type) = v.get("type") {
                            let message_type = message_type.as_str();
                            if let Some(message_type) = message_type {
                                let handler = self.handlers.get(message_type);
                                if let Some(handler) = handler {
                                    let message = v["message"].clone();
                                    // 这里处理消息
                                    handler(message.to_string(), ctx);
                                } else {
                                    println!("No message handler")
                                }
                            } else {
                                println!("JSON 'type'字段值无效");
                            }
                        } else {
                            println!("JSON没有'type'字段.");
                        }
                    }
                    Err(_) => {
                        println!("接收到的消息不是有效的JSON.");
                    }
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

// pub struct LoginMessage {
//     pub user_id: usize,
//     pub msg: ws::Message,
// }

// impl actix::Message for LoginMessage {
//     type Result = ();
// }

// impl Handler<LoginMessage> for MyWebSocket {
//     type Result = ();

//     fn handle(&mut self, msg: LoginMessage, _ctx: &mut Self::Context) {
//         println!("User {} logged in", msg.user_id);
//         self.id = msg.user_id;
//     }
// }

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| App::new().route("/ws/", web::get().to(websocket_route)))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();

    let _ = server.await;
}

mod message_handler {
    use std::sync::Arc;

    use crate::HandlerType;

    pub fn register_handlers(mut handlers: HandlerType) {
        handlers.insert(
            "message_type1".to_string(),
            Arc::new(|text, ctx| {
                // 处理message_type1
            }),
        );
    }
}

// 解析文本消息到消息类型和消息内容
#[allow(unused)]
fn parse_text(text: String) -> (String, String) {
    // 这里需要你自己实现解析逻辑
    (
        "login".to_string(),
        r#"{"type":"login", "message": {"user_id":"1", "pass":"password"}}"#.to_string(),
    )
}
