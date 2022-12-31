use erased_serde::Serialize;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha512;
use std::collections::HashMap;
use std::error::Error;

const DUO_SIGNATURE_ALGORITHM: &str = "HS512";
const AUD_LENGTH_ERR: &str = "didn't receive exactly 1 aud";

pub type MapClaims<'a> = HashMap<&'a str, Box<dyn Serialize + 'a>>;

pub fn jwt_create_signed_token(
    claims: MapClaims,
    secret: &str,
) -> Result<String, Box<dyn Error>> {
    let key: Hmac<Sha512> = Hmac::new_from_slice(secret.as_bytes())?;
    let token_str = claims.sign_with_key(&key)?;
    Ok(token_str)
}
