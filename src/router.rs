use std::collections::HashMap;

use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::messages::hall::login::Login;
#[rustfmt::skip]
use crate::{
    dispatcher::MessageDispatcher, 
    messages::MyWebSocket
};

pub async fn websocket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    // 握手之前进行身份认证
    let login = Login::new("account".to_string(), "sign".to_string());
    if !login.authenticate() {
        // 认证失败时，关闭 WebSocket 连接
        return Ok(HttpResponse::Unauthorized().append_header(("Connection", "close")).finish());
    }
    // 握手
    let resp = ws::start(
        MyWebSocket {
            id: 1,
            handlers: HashMap::new(),
            dispatcher: MessageDispatcher::new(),
            login: Login::default(),
        },
        &req,
        stream,
    );
    println!("{:?}", resp);
    resp
}

pub fn register_handlers(cfg: &mut web::ServiceConfig) {
    cfg.route("/ws/", web::get().to(websocket_route));
}
