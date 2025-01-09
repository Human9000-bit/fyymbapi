use anyhow::Result;
use serde_json::Value;

pub async fn get_track_info(track_id: usize) -> Result<TrackInfo> {
    let track_info = reqwest::get(format!("https://api.music.yandex.ru/tracks/{track_id}"))
        .await?
        .text()
        .await?;

    let data: Value = serde_json::from_str(&track_info)?;
    let data = data["result"]["0"].to_owned();

    let cover_url = data["coverUri"].to_string();

    let name = data["title"].to_string();

    let artists = data["artists"].as_array().unwrap().to_owned();
    
    let artists: Vec<String> = artists
        .into_iter()
        .map(|val| val["name"].as_str().unwrap().to_owned())
        .collect();

    Ok(TrackInfo {
        name,
        artists,
        cover_url,
    })
}

pub async fn get_track_download_info(track_id: usize) -> Result<String> {
    let download_info = reqwest::get(format!(
        "https://api.music.yandex.ru/tracks/{track_id}/download-info"
    ))
    .await?
    .text()
    .await?;
    
    let data: Value = serde_json::from_str(&download_info)?;
    let track_url = data["result"][0]["downloadInfoUrl"].as_str().unwrap();

    Ok(track_url.to_owned())
}

#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq)]
pub struct TrackData {
    pub name: String,
    pub artists: Vec<String>,
    pub cover_url: String,
    pub download_info: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TrackInfo {
    pub name: String,
    pub artists: Vec<String>,
    pub cover_url: String,
}

#[cfg(test)]
mod tests {
    use super::{get_track_info, TrackInfo};

    #[actix_web::test]
    async fn test_track_info() {
        let track_info = get_track_info(127986642).await.unwrap();

        let needed = TrackInfo {
            name: "nerves".to_string(),
            artists: vec!["ptasinski".to_string(), "RJ Pasin".to_string()],
            cover_url: "avatars.yandex.net/get-music-content/9868087/7c2773ac.a.32037431-1/%%".to_string()
        };
        
        assert_eq!(needed, track_info)
    }
}
