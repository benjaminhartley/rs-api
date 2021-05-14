use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde::Serialize;

pub mod auth;
pub mod user;

use crate::utils::jwt::validate_token;

#[derive(Serialize)]
pub struct HttpResponse {
    data: String,
}

pub struct Token(String);

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Token, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let token = keys[0];
        if !validate_token(token) {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        return Outcome::Success(Token(token.to_string()));
    }
}
