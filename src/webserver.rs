use crate::yandex_api::{get_track_download_info, get_track_info, search_artists, ArtistsJson, TrackData};
use actix_web::{
    error, get,
    web::{Json, Path},
    Error,
};
use std::result::Result;

#[get("/get_song?track_id={track_id}")]
pub async fn get_song(track_id: Path<String>) -> Result<Json<String>, Error> {
    let track_id: usize = track_id.trim().parse().map_err(error::ErrorBadRequest)?;
    let track_url = get_track_download_info(track_id)
        .await
        .map_err(error::ErrorInternalServerError)?;
    let track_info = get_track_info(track_id)
        .await
        .map_err(error::ErrorInternalServerError)?;

    let track_data = TrackData {
        name: track_info.name,
        artists: track_info.artists,
        cover_url: track_info.cover_url,
        download_info: track_url,
    };
    

    let track_data = serde_json::to_string(&track_data).map_err(error::ErrorInternalServerError)?;
    
    Ok(Json(track_data))
}

#[get("/search?artist_name={query}")]
pub async fn search_artists_web(query: Path<String>) -> Result<Json<ArtistsJson>, Error> {
    let artists_json = search_artists(query.to_string())
        .await
        .map_err(error::ErrorInternalServerError)?;
    
    Ok(Json(artists_json))
}
