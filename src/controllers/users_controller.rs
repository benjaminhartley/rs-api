use serde::{Deserialize, Serialize};

use crate::db::users_db::{
    get_user_all_data, get_user_with_password, insert_user, update_user_password,
};
use crate::utils::jwt::encode_sub;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub username: String,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct UsersControllerResponse {
    pub message: String,
    pub success: bool,
    pub token: Option<String>,
    pub user: Option<UserData>,
}

pub fn create_user(u: &User) -> UsersControllerResponse {
    match get_user_with_password(&u.username) {
        Some(_user) => {
            debug!("user already exists");
            return UsersControllerResponse {
                success: false,
                message: String::from("user already exists"),
                token: None,
                user: None,
            };
        }
        None => match bcrypt::hash(&u.password, 8) {
            Ok(hashed_pwd) => match insert_user(&u.username, &hashed_pwd) {
                true => {
                    debug!("user created");
                    return UsersControllerResponse {
                        success: true,
                        message: String::from("user created"),
                        token: Some(encode_sub(u.username.clone())),
                        user: None,
                    };
                }
                false => {
                    debug!("user not created");
                    return UsersControllerResponse {
                        success: false,
                        message: String::from("user not created"),
                        token: None,
                        user: None,
                    };
                }
            },
            Err(e) => {
                error!("bcrypt::hash error: {}", e);
                return UsersControllerResponse {
                    success: false,
                    message: String::from("server error"),
                    token: None,
                    user: None,
                };
            }
        },
    }
}

pub fn login_user(u: &User) -> UsersControllerResponse {
    match get_user_with_password(&u.username) {
        Some(user) => match bcrypt::verify(&u.password, &user.password) {
            Ok(result) => {
                debug!("bcrypt::verify result: {}", result);

                if result == true {
                    debug!("logged in");
                    return UsersControllerResponse {
                        success: true,
                        message: String::from("logged in"),
                        token: Some(encode_sub(u.username.clone())),
                        user: None,
                    };
                } else {
                    debug!("invalid password");
                    return UsersControllerResponse {
                        success: false,
                        message: String::from("invalid password"),
                        token: None,
                        user: None,
                    };
                }
            }
            Err(e) => {
                error!("bcrypt::verify error: {}", e);
                return UsersControllerResponse {
                    success: false,
                    message: String::from("server error"),
                    token: None,
                    user: None,
                };
            }
        },
        None => {
            debug!("user not found");
            return UsersControllerResponse {
                success: false,
                message: String::from("user not found"),
                token: None,
                user: None,
            };
        }
    }
}

pub fn get_user_full_data(username: &str) -> UsersControllerResponse {
    match get_user_all_data(&username) {
        Some(user) => {
            return UsersControllerResponse {
                success: true,
                message: String::from("got user data"),
                token: None,
                user: Some(user),
            }
        }
        None => {
            debug!("user not found");
            return UsersControllerResponse {
                success: false,
                message: String::from("user not found"),
                token: None,
                user: None,
            };
        }
    }
}

/* currently only updated password */
pub fn update_user(u: &User) -> UsersControllerResponse {
    match bcrypt::hash(&u.password, 8) {
        Ok(hashed_pwd) => match update_user_password(&u.username, &hashed_pwd) {
            true => {
                debug!("user updated");
                return UsersControllerResponse {
                    success: true,
                    message: String::from("user updated"),
                    token: Some(encode_sub(u.username.clone())),
                    user: None,
                };
            }
            false => {
                debug!("user not updated");
                return UsersControllerResponse {
                    success: false,
                    message: String::from("user not updated"),
                    token: None,
                    user: None,
                };
            }
        },
        Err(e) => {
            error!("bcrypt::hash error: {}", e);
            return UsersControllerResponse {
                success: false,
                message: String::from("server error"),
                token: None,
                user: None,
            };
        }
    }
}
