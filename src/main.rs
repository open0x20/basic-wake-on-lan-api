use actix_web::{web, App, HttpServer};

mod handlers;
use handlers::WakeOnLanHandler;

#[path ="./magic_packet_sender.rs"] mod magic_packet_sender;
use magic_packet_sender::MagicPacketSender;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/wake").route(web::post().to(WakeOnLanHandler::wake)))
    })
    .bind("127.0.0.1:8080").unwrap()
    .run()
    .await
    
}