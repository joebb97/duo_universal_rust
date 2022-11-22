use std::time::Duration;

pub const DEFAULT_STATE_LENGTH: u8 = 36;
pub const MINIMUM_STATE_LENGTH: u8 = 22;
pub const MAXIMUM_STATE_LENGTH: u16 = 1024;
pub const DEFAULT_JTI_LENGTH: u8 = 36;
pub const CLIENT_ID_LENGTH: u8 = 20;
pub const CLIENT_SECRET_LENGTH: u8 = 40;
pub const EXPIRATION_TIME: u16 = 300;
pub const ALLOWED_SKEW: Duration = Duration::from_secs(60);
pub const HEALTH_CHECK_ENDPOINTH: &str = "https://{}/oauth/v1/health_check";
pub const OAUTH_V1_AUTHORIZE_ENDPOINT: &str = "https://{}/oauth/v1/authorize";
pub const API_HOST_URI_FORMAT: &str = "https://{}";
pub const TOKEN_ENDPOINT: &str = "https://{}/oauth/v1/token";
pub const CLIENT_ASSERTION_TYPE: &str =
    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";

pub const CLIENT_ID_LENGTH_ERROR: &str = "Incorrect client_id length";
pub const CLIENT_SECRET_LENGTH_ERROR: &str = "Incorrect client_secret length";

pub const USERNAME_EMPTY_ERROR: &str = "Username can't be empty";
pub const PARAMATER_ERROR: &str = "Did not recieve expected parameters.";
pub const DUO_CODE_ERROR: &str = "Missing authorization code";
pub const HTTP_USE_ERROR: &str = "This client does not allow use of http, please use https";
pub const DUO_VERSION: &str = "1.0.3";

pub const DUO_PINNED_CERT: &str = include_str!("duo_pinned_cert.crt");
