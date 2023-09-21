// // write a websocket server with actix_web_actors
// use actix_web::{web, App, HttpServer, Responder, HttpRequest};
// use actix_web_actors::ws;
// use actix::prelude::*;
// use serde_json::json;

// async fn index(req: HttpRequest, stream: web::Payload) -> impl Responder {
//     ws::start(MyWebSocket::new(), &req, stream)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().route("/ws/", web::get().to(index))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

// pub struct MyWebSocket {
//     pub id: usize,
// }

// impl Actor for MyWebSocket {
//     type Context = ws::WebsocketContext<Self>;
// }

// impl MyWebSocket {
//     fn new() -> Self {
//         Self {
//             // 初始化你的字段
//             id: 0
//         }
//     }
// }

// // translate the following node.js code to rust, use actix web actor
// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(ws::Message::Text(text)) => {
//                 let data: serde_json::Value = serde_json::from_str(&text).unwrap();
//                 let userid = data["userid"].as_str();
//                 if userid.is_none() {
//                     return;
//                 }
//                 let req_data = json!({
//                     "userid": userid.unwrap()
//                 });
//                 // Call the common handler
//                 handler_base_info(req_data, |result, back_data| {
//                     back_data["errcode"] = json!(result);
//                     let back_data_str = serde_json::to_string(&back_data).unwrap();
//                     ctx.text(back_data_str);
//                 });
//             },
//             _ => (),
//         }
//     }
// }

// fn handler_base_info<F>(req_data: serde_json::Value, mut callback: F)
// where
//     F: FnMut(i32, &mut serde_json::Value),
// {
//     // 这里是你的处理逻辑
//     let result = 0; // 假设这是你的结果
//     let mut back_data = req_data.clone(); // 假设这是你的返回数据
//     callback(result, &mut back_data);
// }

// main.rs

// mod crate::messages::text_message;
// mod login;
// mod get_tasks;

mod messages;

use std::collections::HashMap;

use actix::StreamHandler;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use messages::MyWebSocket;

// ...



#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| App::new().route("/ws/", web::get().to(websocket_route)))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();

    let _ = server.await;
}

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

#[cfg(not)]
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                // 发送TextMessage消息给自己
                // ctx.address().do_send(TextMessage(text.to_string()));

                // 根据文本内容判断消息类型
                if text.starts_with("login:") {
                    // let username = text[6..].to_string();
                    #[allow(unused_variables)]
                    if let Some(username) = text.strip_prefix("login:") {
                        // 发送 Login 消息给自己
                        ctx.address().do_send(Login {
                            username: username.to_string(),
                            password: "1231231".to_string(),
                        });
                    } else {
                        ctx.address().do_send(ErrorMessage {
                            code: "0001".to_string(),
                            message: "user name can not empty.".to_string(),
                        });
                    }
                } else if text == "get_tasks" {
                    // 发送 GetTasks 消息给自己
                    ctx.address().do_send(GetTasks);
                } else {
                    // 发送 TextMessage 消息给自己
                    ctx.address().do_send(TextMessage(text.to_string()));
                }

                // let data: Result<serde_json::Value, serde_json::Error> =
                //     serde_json::from_str(&text);

                // match data {
                //     Ok(v) => {
                //         if let Some(message_type) = v.get("type") {
                //             let message_type = message_type.as_str();
                //             if let Some(message_type) = message_type {
                //                 let handler = self.handlers.get(message_type);
                //                 if let Some(handler) = handler {
                //                     let message = v["message"].clone();
                //                     // 这里处理消息
                //                     handler(message.to_string(), ctx);
                //                 } else {
                //                     println!("No message handler")
                //                 }
                //             } else {
                //                 println!("JSON 'type'字段值无效");
                //             }
                //         } else {
                //             println!("JSON没有'type'字段.");
                //         }
                //     }
                //     Err(_) => {
                //         println!("接收到的消息不是有效的JSON.");
                //     }
                // }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let data: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&text);
                match data {
                    Ok(v) => {
                        if let Some(message_type) = v.get("type").and_then(|v| v.as_str()) {
                            let handler = self.handlers.get(message_type);
                            if let Some(handler) = handler {
                                let message = v["message"].clone();
                                // 这里处理消息
                                handler.handle(message, ctx);
                            } else {
                                println!("No message handler")
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

#[cfg(not)]
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                // 解析消息类型
                let data: Result<serde_json::Value, serde_json::Error> =
                    serde_json::from_str(&text);
                match data {
                    Ok(v) => {
                        if let Some(message_type) = v.get("type").and_then(|v| v.as_str()) {
                            match message_type {
                                "login" => {
                                    let username = v
                                        .get("username")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string();
                                    let password = v
                                        .get("password")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string();
                                    // 发送 Login 消息给自己
                                    ctx.address().do_send(Login { username, password });
                                }
                                "get_tasks" => {
                                    // 发送 GetTasks 消息给自己
                                    ctx.address().do_send(GetTasks);
                                }
                                _ => {
                                    // 发送 TextMessage 消息给自己
                                    ctx.address().do_send(TextMessage(text.to_string()));
                                }
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
