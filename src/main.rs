mod dispatcher;
mod messages;
mod router;
mod stream_handler;

use actix_web::{App, HttpServer};

#[rustfmt::skip]
#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new().configure(router::register_handlers)
    })
    .bind("127.0.0.1:9091")
    .unwrap()
    .run();
    println!("Websocket server listening at: 127.0.0.1:9091");
    let _ = server.await;
}
