mod constants;
use crate::constants::*;

#[derive(Debug)]
pub struct DuoConfig<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub api_hostname: &'a str,
    pub redirect_uri: &'a str,
}

pub struct ClientBuilder<'a> {
    client: Client<'a>,
}

impl<'a> ClientBuilder<'a> {
    pub fn new(duo_config: DuoConfig<'a>) -> Result<ClientBuilder<'a>, Box<dyn std::error::Error>> {
        let http_client = reqwest::Client::new();
        let parsed_config = parse_duo_config(duo_config)?;
        Ok(ClientBuilder {
            client: Client {
                duo_config: parsed_config,
                use_duo_code_attribute: false,
                duo_http_client: http_client,
            },
        })
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
}
