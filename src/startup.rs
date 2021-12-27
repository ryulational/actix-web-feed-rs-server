use crate::routes::{feed_entries, feed_info, health_check};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/feed", web::post().to(feed_info))
            .route("/entries", web::post().to(feed_entries))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
