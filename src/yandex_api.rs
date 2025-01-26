use anyhow::{Context, Ok, Result};
use serde_json::Value;

pub async fn get_track_info(track_id: usize) -> Result<TrackInfo> {
    let track_info = reqwest::get(format!("https://api.music.yandex.ru/tracks/{track_id}"))
        .await?
        .text()
        .await?;

    dbg!(&track_info);

    let data: Value =
        serde_json::from_str(&track_info).context("failed to parse the whole json")?;

    let data = data["result"][0].to_owned();

    let cover_url = data["coverUri"]
        .to_string()
        .trim_matches(|c| c == '\\' || c == '"')
        .to_string();

    let cover_url = format!("{}300x300", cover_url.trim_matches(|c| c == '%'));

    let name = data["title"]
        .to_string()
        .trim_matches(|c| c == '\\' || c == '"')
        .to_string();

    let artists = data["artists"].as_array().unwrap();

    let artists: Vec<String> = artists
        .iter()
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

pub async fn search_artists(name: String) -> Result<ArtistsJson> {
    let data = reqwest::get(format!("https://api.music.yandex.ru/search/instant/mixed?text={name}&type=artist&page=0&filter=artist&pageSize=3"))
        .await?.text().await?;

    let data: Value = serde_json::from_str(data.as_str())?;
    let artists = data["result"]["results"].to_owned();

    let artists = ArtistsJson::from_serde(artists)?;
    Ok(artists)
}

#[derive(Debug, serde::Serialize, Clone, Eq, PartialEq)]
pub struct Artist {
    pub name: String,
    pub cover_url: String,
}

impl Artist {
    pub fn new(name: String, cover_url: String) -> Self {
        Self { name, cover_url }
    }

    pub fn parse(artist: Value) -> Result<Self> {
        let name = artist["artist"]["name"].as_str().unwrap().to_owned();
        let cover_url = artist["artist"]["cover"]["uri"]
            .as_str()
            .unwrap()
            .to_owned();

        Ok(Self::new(name, cover_url))
    }
}

#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq)]
pub struct ArtistsJson {
    pub artists: Vec<Artist>,
}

impl ArtistsJson {
    pub fn new(artists: Vec<Artist>) -> Self {
        Self { artists }
    }

    pub fn from_serde(artists: Value) -> Result<Self> {
        let artists = artists.as_array().unwrap().to_owned();

        let artists = Self::new(
            artists
                .into_iter()
                .map(|art| Artist::parse(art).unwrap())
                .collect(),
        );
        Ok(artists)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::{get_track_info, TrackInfo};

    #[actix_web::test]
    async fn test_track_info() -> Result<(), anyhow::Error> {
        let track_info = get_track_info(127986642).await?;

        let needed = TrackInfo {
            name: "nerves".to_string(),
            artists: vec!["ptasinski".to_string(), "RJ Pasin".to_string()],
            cover_url: "avatars.yandex.net/get-music-content/9868087/7c2773ac.a.32037431-1/300x300"
                .to_string(),
        };

        assert_eq!(needed, track_info);
        Ok(())
    }
}
