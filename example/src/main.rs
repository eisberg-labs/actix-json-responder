#[macro_use]
extern crate actix_json_responder;

use serde::Serialize;
use actix_web::{web, App, HttpServer, Error};

#[derive(Serialize, JsonResponder, PartialEq)]
struct HelloStruct {
    title: String,
}

async fn index() -> Result<HelloStruct, Error> {
    Ok(HelloStruct { title: "Hello json!".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(web::resource("/index.html").to(index)))
        .bind("127.0.0.1:8888")?
        .run()
        .await
}
