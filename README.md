# Actix Json Responder [![Continuous Integration](https://github.com/eisberg-labs/actix-json-responder/actions/workflows/ci.yml/badge.svg)](https://github.com/eisberg-labs/actix-json-responder/actions/workflows/ci.yml) [![license-badge][]][license] [![rust-version-badge][]][rust-version]

A procedural macro to reduce json response boilerplate on actix projects.

# Usage
Implementing struct has to be serializable. Example shown in tests and below:
```rust
#[macro_use]
extern crate actix_json_responder;

use serde::Serialize;
use actix_web::{web, App, HttpServer};

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


[cargo-badge]: https://img.shields.io/crates/v/actix-json-responder.svg?style=flat-square
[cargo]: https://crates.io/crates/actix-json-responder
[license-badge]: https://img.shields.io/badge/license-MIT/Apache--2.0-lightgray.svg?style=flat-square
[license]: #license
[rust-version-badge]: https://img.shields.io/badge/rust-1.15+-blue.svg?style=flat-square
[rust-version]: .travis.yml#L5
