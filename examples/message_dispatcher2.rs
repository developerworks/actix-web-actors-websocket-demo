use std::{collections::HashMap, any::Any};
use actix::{Actor, Addr, System, StreamHandler, Handler, Message};
use actix_web::{web, HttpServer};
use actix_web_actors::ws;
use serde_json::Value;

pub struct MyWebSocket;
impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let data: serde_json::Value = serde_json::from_str(&text).unwrap();
                let message_type = data["type"].as_str().unwrap().to_string();
                let message = data["message"].clone();
            },
            _ => (),
        }
    }
}


fn main() {
    System::new().block_on(async {

        HttpServer::new(move || {
            actix_web::App::new()
                .route("/websocket/", web::get().to(move |req, stream| ws::start(addr.clone(), req, stream)))
        })
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
        .unwrap();
    });
}
