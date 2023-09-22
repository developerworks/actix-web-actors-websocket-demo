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
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
    let _ = server.await;
}
