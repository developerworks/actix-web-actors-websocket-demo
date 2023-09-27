use actix_web::{App, HttpServer};
use std::{collections::HashMap, sync::Arc};

use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

// 定义消息类型
pub struct TextMessage(String);

impl actix::Message for TextMessage {
    type Result = ();
}

// 定义处理器 trait
pub trait MessageHandler: Send + Sync {
    fn handle(&self, msg: String, ctx: &mut ws::WebsocketContext<MyWebSocket>);
}

// 实现 MessageHandler trait 的具体处理器
pub struct TextMessageHandler;

impl MessageHandler for TextMessageHandler {
    fn handle(&self, msg: String, ctx: &mut ws::WebsocketContext<MyWebSocket>) {
        ctx.text(msg);
    }
}

pub struct MyWebSocket {
    pub id: usize,
    handlers: HashMap<String, Box<dyn MessageHandler>>,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // 注册消息处理函数
        self.handlers.insert("text".to_string(), Box::new(TextMessageHandler));
        self.handlers.insert("login".to_string(), Box::new(LoginHandler));
        self.handlers.insert("get_tasks".to_string(), Box::new(GetTasksHandler));
    }
}

impl Handler<TextMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) {
        if let Some(handler) = self.handlers.get("text") {
            handler.handle(msg.0, ctx);
        }
    }
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

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                // 发送TextMessage消息给自己
                ctx.address().do_send(TextMessage(text.to_string()));
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| App::new().route("/ws/", web::get().to(websocket_route)))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();

    let _ = server.await;
}

// 定义 Login 消息类型
pub struct Login(String);

impl actix::Message for Login {
    type Result = ();
}

// 定义 GetTasks 消息类型
pub struct GetTasks;

impl actix::Message for GetTasks {
    type Result = ();
}

// 实现 Login 消息的处理器
pub struct LoginHandler;

impl MessageHandler for LoginHandler {
    fn handle(&self, msg: String, _ctx: &mut ws::WebsocketContext<MyWebSocket>) {
        println!("User {} logged in", msg);
    }
}

// 实现 GetTasks 消息的处理器
pub struct GetTasksHandler;

impl MessageHandler for GetTasksHandler {
    fn handle(&self, _msg: String, _ctx: &mut ws::WebsocketContext<MyWebSocket>) {
        println!("Getting tasks");
    }
}

impl Handler<Login> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: Login, ctx: &mut Self::Context) {
        if let Some(handler) = self.handlers.get("login") {
            handler.handle(msg.0, ctx);
        }
    }
}

impl Handler<GetTasks> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, _msg: GetTasks, ctx: &mut Self::Context) {
        if let Some(handler) = self.handlers.get("get_tasks") {
            handler.handle("".to_string(), ctx);
        }
    }
}
