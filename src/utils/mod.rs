pub mod jwt;
pub mod time;

pub fn get_env_var(key: &str) -> String {
    match std::env::var(key) {
        Ok(val) => return val,
        Err(e) => {
            println!("couldn't interpret {}: {}", key, e);
            return String::from("");
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_env_var() {
        let key = "key12345";
        let value = "xyz999";
        std::env::set_var(key, value);

        assert_eq!(value, super::get_env_var(key));
    }
}
