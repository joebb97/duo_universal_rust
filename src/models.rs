use serde::Deserialize;

pub type FlagStatus = i32;

#[derive(Debug, Deserialize)]
pub struct HealthCheckTime {
    pub timestamp: i32,
}

#[derive(Debug, Deserialize)]
pub struct HealthCheckResponse {
    pub stat: String,
    pub message: Option<String>,
    pub message_detail: Option<String>,
    // This will be populated if the request succeeded
    pub response: Option<HealthCheckTime>,
    // This will be populated if the request failed
    // See https://duo.com/docs/oauthapi#health-check
    pub timestamp: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct BodyToken {
    pub id_token: String,
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthResultInfo {
    pub result: String,
    pub status: String,
    pub status_msg: String,
}

#[derive(Debug, Deserialize)]
pub struct LocationInfo {
    pub city: String,
    pub country: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct AccessDeviceInfo {
    pub browser: String,
    pub browser_version: String,
    pub flash_version: String,
    pub host_name: String,
    pub ip: String,
    pub is_encryption_enabled: FlagStatus,
    pub is_firewall_enabled: FlagStatus,
    pub is_password_set: FlagStatus,
    pub java_version: String,
    pub location: LocationInfo,
    pub os: String,
    pub os_version: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthDeviceInfo {
    pub ip: String,
    pub location: LocationInfo,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationInfo {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub groups: Vec<String>,
    pub key: String,
    pub name: String,
}
pub struct AuthContextInfo {
    pub access_device: AccessDeviceInfo,
    pub alias: String,
    pub application: ApplicationInfo,
    pub auth_device: AuthDeviceInfo,
    pub email: String,
    pub event_type: String,
    pub factor: String,
    pub isotimestamp: String,
    pub ood_software: String,
    pub reason: String,
    pub result: String,
    pub timestamp: i64,
    pub txid: String,
    pub user: UserInfo,
}

pub struct TokenResponse {
    pub preferred_username: String,
    pub auth_time: i64,
    pub nonce: String,
    pub auth_result: AuthResultInfo,
    pub auth_context: AuthContextInfo,
    pub aud: String, // Audience
    pub exp: i64,    // ExpiresAt
    pub jti: String, // Id
    pub iat: i64,    // IssuedAt
    pub iss: String, // Issuer
    pub sub: String, // Subject
}
