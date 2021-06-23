extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Reduces boilerplate for actix view models
/// Note: type Error has to be in context ( define enum Error or import Error)
#[proc_macro_derive(JsonResponder)]
pub fn responder_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident.to_string();
    let generic_impl = "
    impl actix_web::Responder for {} {
        type Error = Error;
        type Future = std::future::Ready<Result<actix_web::HttpResponse, Error>>;

        fn respond_to(self, _: &actix_web::HttpRequest) -> Self::Future {
            let body = serde_json::to_string(&self).unwrap();
            std::future::ready(Ok(actix_web::HttpResponse::Ok()
                .content_type(\"application/json\")
                .body(body)))
        }
    }
    ";
    generic_impl
        .replace("{}", struct_name.as_str())
        .parse()
        .unwrap()
}
