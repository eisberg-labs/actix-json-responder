#[macro_use]
extern crate actix_json_responder;

use actix_web::{test, Responder};
use serde::Serialize;
use std::io::Error;
use actix_web::body::{ResponseBody, Body};

#[actix_rt::test]
async fn responder_is_implemented() {
    #[derive(Serialize, JsonResponder, PartialEq)]
    struct TestStruct{ name: String}

    let req = test::TestRequest::default().to_http_request();
    let test_value = TestStruct {name: "Test".to_string()};

    let res = test_value.respond_to(&req).await;

    assert!(res.is_ok());

    let res = body_as_text(res.unwrap().body());
    assert_eq!(res, "{\"name\":\"Test\"}");
}

fn body_as_text(body: &ResponseBody<Body>) -> String {
    match body {
        ResponseBody::Body(actix_web::dev::Body::Bytes(bytes)) => {
            String::from_utf8(bytes.to_vec()).expect("Cannot convert bytes to string")
        }
        _ => panic!("Invalid result"),
    }
}
