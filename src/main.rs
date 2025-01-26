mod webserver;
mod yandex_api;

use crate::webserver::search_artists_web;
use actix_web::{App, HttpServer};
use anyhow::{Ok, Result};
use webserver::get_song;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(get_song).service(search_artists_web))
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await?;
    Ok(())
}
