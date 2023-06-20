use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::LevelFilter;
use simple_logger::SimpleLogger;
// use std::collections::HashMap;
// use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// #[get("/getenv")]
// async fn getenv(_: String) -> impl Responder {
//     let envvals: HashMap<_, _> = env::vars().collect();
//     let serialized_data = serde_json::to_string(&envvals).unwrap();

//     HttpResponse::Ok().body(serialized_data)
// }

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(Env::default().default_filter_or("info"));

    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(healthcheck)
            .service(echo)
            // .service(getenv)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
