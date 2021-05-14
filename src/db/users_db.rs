use super::get_connection;
use crate::controllers::users_controller::{User, UserData};
use crate::utils::time::timestamp_ms;

pub fn get_user_with_password(username: &str) -> Option<User> {
    let conn = get_connection();
    for row in &conn
        .query(
            "SELECT username, password, active FROM users WHERE username=$1 LIMIT 1",
            &[&username],
        )
        .expect("Error getting user data")
    {
        // todo: improve this implementation
        let active: bool = row.get(2);
        if active == false {
            debug!("user inactive, returning junk password");
            return Some(User {
                username: String::from("inactive"),
                password: String::from("x"),
            });
        }

        return Some(User {
            username: row.get(0),
            password: row.get(1),
        });
    }
    return None;
}

pub fn get_user_all_data(username: &str) -> Option<UserData> {
    let conn = get_connection();
    for row in &conn
        .query(
            "SELECT username, created_at, updated_at FROM users WHERE username=$1 LIMIT 1",
            &[&username],
        )
        .expect("Error getting user data")
    {
        return Some(UserData {
            username: row.get(0),
            created_at: row.get(1),
            updated_at: row.get(2),
        });
    }
    None
}

pub fn insert_user(username: &str, hashed_password: &str) -> bool {
    let conn = get_connection();

    let now = timestamp_ms();
    let now = now as i64;

    match conn.execute(
        "INSERT INTO users (username, password, created_at, updated_at, active) VALUES ($1, $2, $3, $4, $5)",
        &[&username, &hashed_password, &now, &now, &true],
    ) {
        Ok(_result) => {
            info!("user inserted");
            return true;
        }
        _ => {
            warn!("insert_user failed");
            return false;
        }
    }
}

pub fn update_user_password(username: &str, hashed_password: &str) -> bool {
    let conn = get_connection();

    let now = timestamp_ms();
    let now = now as i64;

    match conn.execute(
        "UPDATE users SET password = $1, updated_at = $2 WHERE username = $3",
        &[&hashed_password, &now, &username],
    ) {
        Ok(_result) => {
            info!("user password updated");
            return true;
        }
        _ => {
            warn!("failed to update user password");
            return false;
        }
    }
}
