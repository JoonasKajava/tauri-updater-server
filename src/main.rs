use actix_web::{get, App, HttpServer, Responder, Result, web::Data};
use reqwest::Client;
use serde::Serialize;

use crate::{models::release::Release, settings::Configuration};

pub mod models;
mod settings;

#[derive(Serialize)]
struct Version {
    pub version: String,
    pub pub_date: String,
    pub url: String,
    pub notes: String,
    pub signature: String,
}

#[get("/latest")]
async fn latest(config: Data<Configuration>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = client
    .get(format!("https://api.github.com/repos/{}/{}/releases/latest", config.owner, config.repo))
    .header("User-Agent", "tauri-updater-server")
    .bearer_auth(&config.token)
    .build()?;
// 
    let response = client.execute(request).await?.json::<Release>().await?;

    println!("{:#?}", &response);
    
    let version = Version {
        version: response.tag_name,
        pub_date: response.published_at,
        url: response.assets[0].browser_download_url.to_string(),
        notes: response.body,
        signature: response.id.to_string(),
    };

    Ok(actix_web::web::Json(version))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = settings::get();

    println!("Server running on port {}", config.port);

    let config_data = config.clone();

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(config_data.clone()))
        .service(latest)
    })
    .bind((config.address, config.port))?
    .run()
    .await
}