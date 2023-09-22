

use std::collections::HashMap;

use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

#[rustfmt::skip]
use crate::{
    dispatcher::MessageDispatcher, 
    messages::MyWebSocket
};

pub async fn websocket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        MyWebSocket {
            id: 1,
            handlers: HashMap::new(),
            dispatcher: MessageDispatcher::new(),
        },
        &req,
        stream,
    );
    println!("{:?}", resp);
    resp
}

pub fn register_handlers(cfg: &mut web::ServiceConfig) {
    cfg.route("/ws", web::get().to(websocket_route));
}
