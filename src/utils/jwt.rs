use crate::utils::get_env_var;

use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, Algorithm, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::time::timestamp_ms;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: u128,
    pub iat: u128,
    pub iss: String,
    pub sub: String,
}

pub fn encode_sub(sub: String) -> String {
    let mut header = Header::default();
    header.alg = Algorithm::HS512;

    let iat = timestamp_ms();
    let iss = get_env_var("ISSUER");

    let valid_for = 1000 * 60 * 60 * 3; // 3 hours
    let exp = iat + valid_for;

    let claims = Claims { exp, iat, iss, sub };

    let secret = get_env_var("SECRET");

    encode(&header, &claims, secret.as_ref()).unwrap()
}

pub fn decode_token(token: &str) -> Result<TokenData<Claims>, Error> {
    let secret = get_env_var("SECRET");

    decode::<Claims>(
        &token,
        secret.as_bytes(),
        &Validation::new(Algorithm::HS512),
    )
}

pub fn validate_token(token: &str) -> bool {
    match decode_token(token) {
        Ok(decoded_token) => {
            if decoded_token.claims.iss != get_env_var("ISSUER") {
                return false;
            }

            if timestamp_ms() < decoded_token.claims.exp {
                return true;
            }
        }
        _ => return false,
    }
    return false;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode_sub() {
        let sub = "xxx999ggg";
        let encoded = super::encode_sub(sub.to_string());
        let slice = &encoded[..3];
        assert_eq!(slice, "eyJ".to_string());
    }

    #[test]
    fn test_validate_token() {
        let sub = "xxx999ggg";
        let encoded = super::encode_sub(sub.to_string());
        let is_valid = super::validate_token(&encoded);
        assert_eq!(is_valid, true);
    }
}
