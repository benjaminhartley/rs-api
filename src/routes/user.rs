use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::controllers::users_controller::UsersControllerResponse;
use crate::controllers::users_controller::{get_user_full_data, update_user, User};
use crate::utils::jwt::decode_token;

use super::{HttpResponse, Token};

#[get("/user")]
pub fn get_user(token: Token) -> Result<Json<HttpResponse>, Status> {
    debug!("GET /user called");

    match decode_token(&token.0) {
        Ok(decoded_token) => {
            debug!("getting user data for {}", decoded_token.claims.sub);

            let response: UsersControllerResponse = get_user_full_data(&decoded_token.claims.sub);
            if response.success == true {
                match response.user {
                    Some(user) => {
                        return Ok(Json(HttpResponse {
                            data: serde_json::to_string(&user).unwrap(),
                        }));
                    }
                    None => {
                        return Err(Status::InternalServerError);
                    }
                }
            } else {
                return Err(Status::NotFound);
            }
        }
        _ => {
            return Err(Status::InternalServerError);
        }
    }
}

#[put("/user", data = "<user>")]
pub fn update_password(token: Token, user: Json<User>) -> Result<Json<HttpResponse>, Status> {
    debug!("PUT /user called");

    match decode_token(&token.0) {
        Ok(decoded_token) => {
            /* only has permission to update password for username in token */
            if decoded_token.claims.sub != user.username {
                return Err(Status::Unauthorized);
            }

            let response: UsersControllerResponse = update_user(&user);

            if response.success == true {
                return Ok(Json(HttpResponse {
                    data: String::from("password updated"),
                }));
            } else {
                return Err(Status::InternalServerError);
            }
        }
        _ => {
            return Err(Status::InternalServerError);
        }
    }
}
