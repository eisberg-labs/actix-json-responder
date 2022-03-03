#[macro_use]
extern crate actix_json_responder;

use actix_web::body::to_bytes;
use actix_web::{test, HttpResponse, Responder};
use serde::Serialize;

#[actix_web::test]
async fn responder_is_implemented() {
    #[derive(Serialize, PartialEq, JsonResponder)]
    struct TestStruct {
        name: String,
    }
    let req = test::TestRequest::default().to_http_request();
    let test_value = TestStruct {
        name: "Test".to_string(),
    };

    let res: HttpResponse = test_value.respond_to(&req);
    let res = to_bytes(res.into_body()).await.unwrap();

    assert_eq!(res, &b"{\"name\":\"Test\"}"[..]);
}
