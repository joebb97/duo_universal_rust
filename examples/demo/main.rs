use duo_universal_rust::ClientBuilder;
use duo_universal_rust::DuoConfig;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct IntegrationConfig<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    api_hostname: &'a str,
    redirect_uri: &'a str,
    failmode: &'a str,
}

struct Session {
    duo_state: String,
    duo_username: String,
    failmode: String,
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = fs::read("examples/demo/duo_config.toml")?;
    let duo_config: IntegrationConfig = toml::from_slice(config.as_slice())?;
    let duo_config = DuoConfig {
        client_id: duo_config.client_id,
        client_secret: duo_config.client_secret,
        api_hostname: duo_config.api_hostname,
        redirect_uri: duo_config.redirect_uri,
    };
    let client = ClientBuilder::new(duo_config)?.build();
    println!("{} {:?}", client.do_stuff(), client);
    Ok(())
}
