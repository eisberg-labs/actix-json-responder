#[macro_use]
extern crate actix_json_responder;

use actix_web::body::to_bytes;
use actix_web::{test, Responder};
use serde::Serialize;

#[actix_rt::test]
async fn responder_is_implemented() {
    #[derive(Serialize, PartialEq, JsonResponder)]
    struct TestStruct {
        name: String,
    }

    let req = test::TestRequest::default().to_http_request();
    let test_value = TestStruct {
        name: "Test".to_string(),
    };

    let res = test_value.respond_to(&req).await;

    assert!(res.is_ok());

    let res = to_bytes(res.expect("Response should be present").into_body())
        .await
        .unwrap();
    assert_eq!(res, &b"{\"name\":\"Test\"}"[..]);
}
