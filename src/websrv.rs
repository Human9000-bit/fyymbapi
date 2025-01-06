use crate::yapi::{TrackData, get_track_info, get_track_download_info};
use actix_web::{get, web::{self, Json, Path}};

#[get("/get_song?track_id={track_id}")]
pub async fn get_song(
    track_id: Path<String>,
) -> std::result::Result<Json<String>, actix_web::Error> {
    let track_id: usize = track_id.trim().parse().unwrap();
    let track_url = get_track_download_info(track_id).await.unwrap();
    let track_info = get_track_info(track_id).await.unwrap();

    let track_data = TrackData {
        name: track_info.name,
        artists: track_info.artists,
        cover_url: track_info.cover_url,
        download_info: track_url,
    };

    let json = serde_json::to_string(&track_data).unwrap();

    std::result::Result::Ok(web::Json(json))
}
