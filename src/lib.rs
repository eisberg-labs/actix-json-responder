extern crate proc_macro;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Reduces boilerplate for actix view models
/// Note: type Error has to be in context ( define enum Error or import Error)
#[proc_macro_derive(JsonResponder)]
pub fn responder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    (quote! {
        impl actix_web::Responder for #ident {
            type Body = actix_web::body::BoxBody;

            fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
                match serde_json::to_string(&self) {
                    Err(err) => actix_web::HttpResponse::from_error(err),
                    Ok(value) => actix_web::HttpResponse::Ok().content_type("application/json").body(value)
                }
            }
        }
    }).into()
}
