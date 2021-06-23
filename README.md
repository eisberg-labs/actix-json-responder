# Actix Json Responder [![travis-badge][]][travis] [![cargo-badge][]][cargo] [![license-badge][]][license] [![rust-version-badge][]][rust-version]

A procedural macro to reduce json response boilerplate on actix projects.

# Usage
Prerequisite for this to work is to define actix_web, serde, serde_json dependencies.
`Error` has to be in context where `JsonResponder` is implemented. Either use `actix_web::Error`
or implement your own `Error` enum. Simple example shown below:
```rust
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
```
Working example is in [example](./example) directory.

# License

Distributed under the terms of [MIT license](./LICENSE-MIT) and [Apache license](./LICENSE-APACHE).


[travis-badge]: https://img.shields.io/travis/eisberg-labs/actix-json-responder/master.svg?style=flat-square
[travis]: https://travis-ci.org/eisberg-labs/actix-json-responder
[cargo-badge]: https://img.shields.io/crates/v/actix-json-responder.svg?style=flat-square
[cargo]: https://crates.io/crates/actix-json-responder
[license-badge]: https://img.shields.io/badge/license-MIT/Apache--2.0-lightgray.svg?style=flat-square
[license]: #license
[rust-version-badge]: https://img.shields.io/badge/rust-1.15+-blue.svg?style=flat-square
[rust-version]: .travis.yml#L5
