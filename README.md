# Yandex Music Proxy API Server

A high-performance proxy server for accessing Yandex Music API endpoints with Rust. This server provides simplified access to track information and artist search functionality.

## Features

- 🔍 Artist search with auto-complete suggestions
- 🎵 Track metadata retrieval (title, artists, cover art)
- 📥 Direct download URLs for tracks
- 🔒 Optional Yandex OAuth token support
- ⚡ Actix-web powered asynchronous processing

## Installation

### Requirements
- Rust 1.70+ (recommended: use [rustup](https://rustup.rs))
- Cargo (included with Rust installation)

### Build from Source
```bash
git clone https://github.com/your-repo/fyymbapi.git
cd fyymbapi
cargo build --release
```

## Configuration

Set optional environment variable:
```bash
export TOKEN="your_yandex_oauth_token"
```

## API Endpoints

### 1. Search Artists
```http
GET /search/{query}
```

**Example:**
```bash
curl http://localhost:8080/search/radiohead
```

**Response:**
```json
{
  "artists": [
    {
      "name": "Radiohead",
      "cover_url": "avatars.yandex.net/.../radiohead.jpg"
    },
    {
      "name": "Radiohead Tribute Band",
      "cover_url": "avatars.yandex.net/.../tribute.jpg"
    }
  ]
}
```

### 2. Get Track Information
```http
GET /get_song/{track_id}
```

**Example:**
```bash
curl http://localhost:8080/get_song/127986642
```

**Response:**
```json
{
  "name": "nerves",
  "artists": ["ptasinski", "RJ Pasin"],
  "cover_url": "avatars.yandex.net/.../32037431-1/300x300",
  "download_info": "https://storage.mds.yandex.net/.../track.mp3"
}
```

## Running the Server

```bash
# Dev mode with hot reloading
cargo watch -x 'run'

# Production mode
cargo run --release
```