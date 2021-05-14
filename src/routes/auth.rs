use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::controllers::users_controller::UsersControllerResponse;
use crate::controllers::users_controller::{create_user, login_user, User};

use super::{HttpResponse, Token};

#[post("/auth/signup", data = "<new_user>")]
pub fn signup(new_user: Json<User>) -> Result<Json<HttpResponse>, Status> {
    debug!("POST /auth/signup called");

    let response: UsersControllerResponse = create_user(&new_user);

    if response.success == true {
        match response.token {
            Some(token) => {
                return Ok(Json(HttpResponse {
                    data: String::from(token),
                }));
            }
            None => {
                return Err(Status::InternalServerError);
            }
        }
    } else {
        Err(Status::BadRequest)
    }
}

#[post("/auth/login", data = "<user>")]
pub fn login(user: Json<User>) -> Result<Json<HttpResponse>, Status> {
    debug!("POST /auth/login called");

    let response: UsersControllerResponse = login_user(&user);

    if response.success == true {
        match response.token {
            Some(token) => {
                return Ok(Json(HttpResponse {
                    data: String::from(token),
                }));
            }
            None => {
                return Err(Status::InternalServerError);
            }
        }
    } else {
        Err(Status::BadRequest)
    }
}

#[post("/auth/validate")]
pub fn validate(_token: Token) -> Result<Json<HttpResponse>, Status> {
    debug!("POST /auth/validate called");

    Ok(Json(HttpResponse {
        data: String::from("valid token"),
    }))
}
