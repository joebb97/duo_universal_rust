use duo_universal_rust::ClientBuilder;
use duo_universal_rust::DuoConfig;
use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::{get, post, routes, FromForm};
use rocket_dyn_templates::{context, Template};
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

#[get("/")]
fn login_get() -> Template {
    Template::render(
        "login",
        context! {
            message: "This is a demo"
        },
    )
}

#[derive(FromForm)]
struct LoginForm<'a> {
    username: &'a str,
    password: &'a str,
}

#[post("/", data = "<form>")]
fn login_post(form: Form<LoginForm<'_>>) -> Template {
    let message = if form.username == "" {
        "Username required".to_string()
    } else if form.password == "" {
        "Password required".to_string()
    } else {
        format!("Username = {}, Password = {}", form.username, form.password)
    };
    Template::render(
        "login",
        context! {
            message: message
        },
    )
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = include_bytes!("duo_config.toml");
    let duo_config: IntegrationConfig = toml::from_slice(config.as_slice())?;
    let duo_config = DuoConfig {
        client_id: duo_config.client_id,
        client_secret: duo_config.client_secret,
        api_hostname: duo_config.api_hostname,
        redirect_uri: duo_config.redirect_uri,
    };
    let duo_client = ClientBuilder::new(duo_config)?.build();
    println!("{} {:?}", duo_client.do_stuff(), duo_client);

    let _rocket = rocket::build()
        .mount("/", routes![login_get, login_post])
        .mount("/index.html", routes![login_get])
        .mount("/static/", FileServer::from("static/"))
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}
