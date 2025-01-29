mod webserver;
mod yandex_api;

use crate::webserver::search_artists_web;
use actix_web::{web, App, HttpServer};
use anyhow::{Ok, Result};
use reqwest::Client;
use webserver::get_song;

#[actix_web::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN").ok();

    let client = Client::new();

    let shared_state = SharedState { token, client };

    HttpServer::new(move || {
        App::new()
            .service(get_song)
            .service(search_artists_web)
            .app_data(web::Data::new(shared_state.clone()))
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await?;
    Ok(())
}

#[derive(Clone)]
pub struct SharedState {
    pub token: Option<String>,
    pub client: Client,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            token: None,
            client: Client::new(),
        }
    }
}
