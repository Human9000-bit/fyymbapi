use crate::yandex_api::{
    get_track_download_info, get_track_info, search_artists, ArtistsJson, TrackData,
};
use crate::SharedState;
use actix_web::{
    error, get, web,
    web::{Json, Path},
    Error,
};
use std::result::Result;

#[get("/get_song/{track_id}")]
pub async fn get_song(
    track_id: Path<usize>,
    state: web::Data<SharedState>,
) -> Result<Json<TrackData>, Error> {
    let track_id = track_id.into_inner();
    let track_url = get_track_download_info(track_id, state.clone().into_inner())
        .await
        .map_err({
            dbg!("failed to get track download info");
            error::ErrorInternalServerError
        })?;

    let track_info = get_track_info(track_id, state.get_ref())
        .await
        .map_err(error::ErrorInternalServerError)?;

    let track_data = TrackData {
        name: track_info.name,
        artists: track_info.artists,
        cover_url: track_info.cover_url,
        download_info: track_url,
    };

    Ok(Json(track_data))
}

#[get("/search/{query}")]
pub async fn search_artists_web(query: Path<String>) -> Result<Json<ArtistsJson>, Error> {
    let artists_json = search_artists(query.to_string())
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(Json(artists_json))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_song_success() {
        let app = test::init_service(App::new().service(get_song)).await;
        let req = test::TestRequest::get()
            .uri("/get_song/127986642")
            .to_request();
        let resp = test::call_service(&app, req).await;
        dbg!(resp.status());
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_search_artists_success() {
        let app = test::init_service(App::new().service(search_artists_web)).await;
        let req = test::TestRequest::get()
            .uri("/search?artist_name=127986642")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
