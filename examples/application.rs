use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

#[get("/state")]
async fn state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name

    format!("Hello {}!", app_name) // <- response with app_name
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/count")]
async fn count(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .service(state)
            .app_data(counter.clone())
            .service(count)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let counter = web::Data::new(AppStateWithCounter {
//         counter: Mutex::new(0),
//     });

//     HttpServer::new(move || {
//         // move counter into the closure
//         App::new()
//             // Note: using app_data instead of data
//             .app_data(counter.clone()) // <- register the created data
//             .route("/count", web::get().to(count))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
