mod websrv;
mod yapi;

use actix_web::{App, HttpServer};
use anyhow::{Ok, Result};
use websrv::get_song;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(get_song))
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await?;
    Ok(())
}
