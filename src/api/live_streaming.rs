use std::path::PathBuf;

use axum::{extract::Path, response::Response};
use axum::http::{ StatusCode};
use axum::body::Body;

pub async fn http_live_streaming(Path(filename): Path<String>) ->Result<Response<Body>,StatusCode>{
    let path=PathBuf::from("media").join(&filename);
    println!("Requested file: {}\n", path.display());

    // Check if the file exists
    if !path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }   
let content_type=if filename.ends_with(".m3u8") {
        "application/vnd.apple.mpegurl"
    } else if filename.ends_with(".ts") {
        "video/MP2T"
    } else {
        "application/octet-stream"
    };
    let file_content=tokio::fs::read(&path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create a response with the file content and appropriate headers
    let mut response= Response::new(Body::from(file_content));
    response.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        content_type.parse().unwrap(),
    );  

    Ok(response)
}