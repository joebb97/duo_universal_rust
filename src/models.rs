use serde::Deserialize;

#[derive(Deserialize)]
pub struct HealthCheckTime {
    pub timestamp: i64
}

#[derive(Deserialize)]
pub struct HealthCheckResponse<'a> {
    pub stat: &'a str,
    pub message: &'a str,
    pub message_detail: &'a str,
    pub response: HealthCheckTime,
}
