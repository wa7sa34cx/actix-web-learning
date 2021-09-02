use actix_web::{get, App, HttpResponse, HttpServer, Responder};
// use std::time::Duration;
use tokio::time::{sleep, Duration};

#[get("/duration")]
async fn duration() -> impl Responder {
    // std::thread::sleep(Duration::from_secs(5)); // <-- Bad practice! Will cause the current worker thread to hang!
    sleep(Duration::from_secs(5)).await;
    HttpResponse::Ok().body("Delay for 5 seconds...")
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Server!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(duration)
            .service(hello)
    })
    .workers(1)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
