use duo_universal_rust::{Client, ClientBuilder, DuoConfig};
use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket::{get, post, routes, FromForm, State};
use rocket_dyn_templates::{context, Template};
use serde::Deserialize;
use tokio::sync::Mutex;

const DUO_UNAVAILABLE: &str = "Duo unavailable";

#[derive(Deserialize)]
struct IntegrationConfig<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    api_hostname: &'a str,
    redirect_uri: &'a str,
    duo_certs: Option<&'a str>,
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
async fn login_post<'a>(
    duo_client: &State<Client<'a>>,
    session: &State<Mutex<Session>>,
    form: Form<LoginForm<'_>>,
) -> Result<Redirect, Template> {
    let mut session = session.lock().await;
    if form.username.is_empty() {
        Err(Template::render(
            "login",
            context! {
                message: "Username required".to_string()
            },
        ))
    } else if form.password.is_empty() {
        Err(Template::render(
            "login",
            context! {
                message: "Password required".to_string()
            },
        ))
    } else {
        session.duo_username = form.username.to_string();
        let health_check = duo_client.health_check().await;
        if health_check.is_err() {
            let upper = session.failmode.to_uppercase();
            let template_to_render = if upper == "CLOSED" {
                "login"
            } else {
                "success"
            };
            return Err(Template::render(
                template_to_render,
                context! {
                    message: DUO_UNAVAILABLE
                },
            ));
        }
        if let Ok(duo_state) = duo_client.generate_state() {
            session.duo_state = duo_state;
            let redirect_to_duo_url = duo_client
                .create_auth_url(form.username, &session.duo_state)
                .unwrap();
            let redirect_to_duo_url = String::from(redirect_to_duo_url.as_str());

            return Ok(Redirect::to(redirect_to_duo_url));
        }
        Err(Template::render(
            "login",
            context! {
                message: format!("Username = {}, Password = {}", form.username, form.password)
            },
        ))
    }
}

#[get("/?<state>&<code>")]
async fn duo_callback(
    duo_client: &State<Client<'_>>,
    session: &State<Mutex<Session>>,
    state: Option<&str>,
    code: Option<&str>,
) -> Template {
    let session = session.lock().await;
    println!("Hey there i'm in the callback {:?} {:?}", state, code);
    match (state, code) {
        (Some(state), Some(code)) => {
            if state != session.duo_state {
                return Template::render(
                    "login",
                    context! {message: "Duo state does not match saved state" },
                );
            }
            Template::render("success", context! {message: "Soon implement" })
        }
        (_, _) => Template::render(
            "login",
            context! {message: "Expected both url_state and code"},
        ),
    }
    // if let Some(url_state) = url_state {
    //     if url_state != session.duo_state {
    //     }
    // }
    // if code.is_none() {
    //     return Template::render("login.html", "Expected a code");
    // }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = include_bytes!("duo_config.toml");
    let duo_config: IntegrationConfig = toml::from_slice(config.as_slice())?;
    let session = Mutex::new(Session {
        duo_state: String::new(),
        failmode: String::from(duo_config.failmode),
        duo_username: String::new(),
    });
    let duo_config = DuoConfig {
        client_id: duo_config.client_id,
        client_secret: duo_config.client_secret,
        api_hostname: duo_config.api_hostname,
        redirect_uri: duo_config.redirect_uri,
        duo_certs: duo_config.duo_certs,
    };

    let duo_client = ClientBuilder::new(duo_config)?.build();
    let _rocket = rocket::build()
        .mount("/", routes![login_get, login_post])
        .mount("/index.html", routes![login_get])
        .mount("/static/", FileServer::from("static/"))
        .mount("/duo-callback", routes![duo_callback])
        .manage(duo_client)
        .manage(session)
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}
