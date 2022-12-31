mod constants;
mod jwt;
mod models;

use rand::distributions::{Alphanumeric, DistString};
use reqwest::{Method, Url};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use url::form_urlencoded;

// Allow users to use this for generate_state_with_length
use crate::jwt::MapClaims;
use constants::*;
pub use constants::{DEFAULT_STATE_LENGTH, MAXIMUM_STATE_LENGTH, MINIMUM_STATE_LENGTH};
use models::HealthCheckResponse;
type AnyError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct DuoConfig<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub api_hostname: &'a str,
    pub redirect_uri: &'a str,
    pub duo_certs: Option<&'a str>,
}

pub struct ClientBuilder<'a> {
    client: Client<'a>,
}

impl<'a> ClientBuilder<'a> {
    pub fn new(duo_config: DuoConfig<'a>) -> Result<ClientBuilder<'a>, AnyError> {
        let parsed_config = parse_duo_config(duo_config)?;
        Ok(ClientBuilder {
            client: Client {
                duo_config: parsed_config,
                use_duo_code_attribute: false,
                duo_http_client: ClientBuilder::default_http_client()?,
            },
        })
    }

    fn default_http_client() -> Result<reqwest::Client, AnyError> {
        let mut http_client = reqwest::ClientBuilder::new()
            .tls_built_in_root_certs(false)
            .https_only(true);
        for cert in DUO_PINNED_CERT.split("\n\n") {
            let cert = reqwest::Certificate::from_pem(cert.as_bytes())?;
            http_client = http_client.add_root_certificate(cert);
        }
        Ok(http_client.build()?)
    }

    pub fn use_duo_code_attribute(mut self) -> ClientBuilder<'a> {
        self.client.use_duo_code_attribute = true;
        self
    }

    pub fn http_client(mut self, new_client: reqwest::Client) -> ClientBuilder<'a> {
        self.client.duo_http_client = new_client;
        self
    }

    pub fn build(self) -> Client<'a> {
        self.client
    }
}

#[derive(Debug)]
pub struct Client<'a> {
    duo_config: ParsedDuoConfig<'a>,
    use_duo_code_attribute: bool,
    duo_http_client: reqwest::Client,
}

#[derive(Debug)]
struct ParsedDuoConfig<'a>(DuoConfig<'a>);

fn parse_duo_config(duo_config: DuoConfig) -> Result<ParsedDuoConfig, Box<dyn std::error::Error>> {
    if duo_config.client_id.len() != CLIENT_ID_LENGTH as usize {
        Err(CLIENT_ID_LENGTH_ERROR)?
    }
    if duo_config.client_secret.len() != CLIENT_SECRET_LENGTH as usize {
        Err(CLIENT_SECRET_LENGTH_ERROR)?
    }
    Ok(ParsedDuoConfig(duo_config))
}

impl Client<'_> {
    pub fn do_stuff(&self) -> &str {
        "I did stuff"
    }

    pub async fn health_check(&self) -> Result<HealthCheckResponse, AnyError> {
        let mut health_check_url = Url::parse(HEALTH_CHECK_ENDPOINT).unwrap();
        let ParsedDuoConfig(duo_config) = &self.duo_config;
        health_check_url.set_host(Some(duo_config.api_hostname))?;
        let token = self.create_jwt_args(health_check_url.as_str())?;
        let encoded: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("client_assertion", &token)
            .append_pair("client_id", duo_config.client_id)
            .finish();

        let request = self
            .duo_http_client
            .request(Method::POST, health_check_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(encoded);

        let health_check_response = request.send().await?;
        let health_check_response = health_check_response.json::<HealthCheckResponse>().await?;
        if health_check_response.stat != "OK" {
            Err(format!(
                "{:?}: {:?}",
                health_check_response.message, health_check_response.message_detail
            ))?
        }
        Ok(health_check_response)
    }

    fn create_jwt_args(&self, audience: &str) -> Result<String, AnyError> {
        let jti = self.generate_state_with_length(DEFAULT_JTI_LENGTH as u16)?;
        let ParsedDuoConfig(cfg) = &self.duo_config;
        let mut claims = MapClaims::new();
        let dur = Duration::from_secs(EXPIRATION_TIME as u64);
        let exp = SystemTime::now()
            .checked_add(dur)
            .ok_or("Couldn't make expiration")?
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .to_string();
        claims.insert("iss", Box::new(cfg.client_id));
        claims.insert("sub", Box::new(cfg.client_id));
        claims.insert("aud", Box::new(audience));
        claims.insert("jti", Box::new(jti));
        claims.insert("exp", Box::new(exp));

        let token = jwt::jwt_create_signed_token(claims, cfg.client_secret)?;
        Ok(token)
    }

    pub fn generate_state(&self) -> Result<String, AnyError> {
        self.generate_state_with_length(DEFAULT_STATE_LENGTH)
    }

    pub fn generate_state_with_length(&self, length: u16) -> Result<String, AnyError> {
        let acceptable_range = MINIMUM_STATE_LENGTH..=MAXIMUM_STATE_LENGTH;
        if !acceptable_range.contains(&length) {
            let err_msg = format!(
                "Provided state length {} not in range [{}, {}]",
                length, MINIMUM_STATE_LENGTH, MAXIMUM_STATE_LENGTH
            );
            Err(err_msg)?
        }
        let state = Alphanumeric.sample_string(&mut rand::thread_rng(), length as usize);
        Ok(state)
    }

    pub fn create_auth_url(&self, username: &str, state: &str) -> Result<reqwest::Url, AnyError> {
        let state_length = state.len() as u16;
        let acceptable_range = MINIMUM_STATE_LENGTH..=MAXIMUM_STATE_LENGTH;
        if !acceptable_range.contains(&state_length) {
            let err_msg = format!(
                "Provided state length {} not in range [{}, {}]",
                state_length, MINIMUM_STATE_LENGTH, MAXIMUM_STATE_LENGTH
            );
            Err(err_msg)?
        }
        if username.is_empty() {
            Err(USERNAME_EMPTY_ERROR)?
        }

        let mut authorize_endpoint_url = Url::parse(OAUTH_V1_AUTHORIZE_ENDPOINT).unwrap();
        let ParsedDuoConfig(cfg) = &self.duo_config;
        authorize_endpoint_url.set_host(Some(cfg.api_hostname))?;

        let dur = Duration::from_secs(EXPIRATION_TIME as u64);
        let exp = SystemTime::now()
            .checked_add(dur)
            .ok_or("Couldn't make expiration")?
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .to_string();

        let mut claims = MapClaims::new();
        claims.insert("scope", Box::new("openid"));
        claims.insert("redirect_uri", Box::new(cfg.redirect_uri));
        claims.insert("client_id", Box::new(cfg.client_id));
        claims.insert("iss", Box::new(cfg.client_id));
        claims.insert("aud", Box::new(format!("https://{}", cfg.api_hostname)));
        claims.insert("exp", Box::new(exp));
        claims.insert("state", Box::new(state));
        claims.insert("response_type", Box::new("code"));
        claims.insert("duo_uname", Box::new(username));
        claims.insert("use_duo_code_attribute", Box::new(self.use_duo_code_attribute));

        let token = jwt::jwt_create_signed_token(claims, cfg.client_secret)?;

        let encoded: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("response_type", "code")
            .append_pair("request", &token)
            .append_pair("client_id", cfg.client_id)
            .finish();

        authorize_endpoint_url.set_query(Some(&encoded));
        Ok(authorize_endpoint_url)
    }

    fn exchange_authorization_code_for_2fa_result(&self, duo_code: &str, username: &str) {
    }
}
