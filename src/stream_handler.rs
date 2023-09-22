use actix::StreamHandler;
use actix_web_actors::ws;

use crate::messages::MyWebSocket;

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

#[cfg(not)]
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let data: serde_json::Value = serde_json::from_str(&text).unwrap();
                let message_type = data["type"].as_str().unwrap().to_string();
                let message = data["message"].clone();
                self.dispatcher.dispatch(message_type, message);
            },
            _ => (),
        }
    }
}