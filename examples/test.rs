use actix::prelude::*;
use actix_web::{HttpRequest, HttpResponse, Error, web};
use actix_web_actors::ws;

pub struct MyWebSocket {
    pub id: usize,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

async fn websocket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWebSocket { id: 1}, &req, stream);
    println!("{:?}", resp);
    resp
}


impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub struct LoginMessage {
    pub user_id: usize,
    pub msg: ws::Message,
}

impl actix::Message for LoginMessage {
    type Result = ();
}
// login message handler
impl Handler<LoginMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: LoginMessage, _ctx: &mut Self::Context) {
        println!("User {} logged in", msg.user_id);
        self.id = msg.user_id;
    }
}
fn main() {}
