mod constants;
mod models;

use crate::constants::*;
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

// #[derive(Serialize, Deserialize, Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

impl Client<'_> {
    pub fn do_stuff(&self) -> &str {
        "I did stuff"
    }

    // pub fn health_check() -> Result<Point, AnyError> {
    //     let p = Point{x: 8, y: 99};
    //     let serialized = serde_json::to_string(&point).unwrap();
    //     println!("serialized = {}", serialized);
    // }
}
