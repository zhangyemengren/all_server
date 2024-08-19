use axum::http::HeaderMap;

pub fn get_env_var(key: &str) -> String {
    dotenvy::dotenv().ok();
    if let Ok(v) = std::env::var(key) {
        return v;
    }

    "".to_string()
}

pub fn get_author_header() -> HeaderMap {
    use axum::http::HeaderMap;

    let author = format!("Bearer {}", get_env_var("TMDB_API_KEY"));
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", author.parse().unwrap());
    headers
}