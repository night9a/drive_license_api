use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
struct AgeRequest {
    age: u8,
}

async fn check_license(data: web::Json<AgeRequest>) -> impl Responder {
    let age = data.age;
    if age >= 18 {
        HttpResponse::Ok().json({"message": "You are eligible for a driver's license."})
    } else {
        HttpResponse::Ok().json({"message": "You are not eligible for a driver's license yet."})
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Serve the static files located in the "static" folder
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            // API endpoint
            .route("/check-license", web::post().to(check_license))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
