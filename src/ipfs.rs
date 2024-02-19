use crate::model::ApiResponse;
use axum::Json;
use reqwest::Client;
use serde_json::Value;
use std::convert::Infallible;
use std::env;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn file_upload(file_name: String) -> Result<Json<ApiResponse>, Infallible> {
    let client = Client::new();
    let ipfs_api_endpoint = "http://127.0.0.1:5001/api/v0/add";

    // Get the current directory
    let mut path = env::current_dir().expect("Failed to get current directory");
    // Append the 'nft-images' subdirectory to the path
    path.push("nft-images");
    // Append the file name to the path
    path.push(file_name);

    //println!("Full path: {}", path.display());

    // Open the file asynchronously
    let mut file = File::open(path.clone()).await.expect("Failed to open file");

    // Read file bytes
    let mut file_bytes = Vec::new();
    file.read_to_end(&mut file_bytes)
        .await
        .expect("Failed to read file bytes");

    // Extract the file name from the path
    let file_name = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap_or_default()
        .to_string();

    let form = reqwest::multipart::Form::new().part(
        "file",
        reqwest::multipart::Part::stream(file_bytes).file_name(file_name),
    );

    let response = client
        .post(ipfs_api_endpoint)
        .multipart(form)
        .send()
        .await
        .expect("Failed to send file to IPFS");

    if response.status().is_success() {
        let response_body = response
            .text()
            .await
            .expect("Failed to read response body as text");

        let ipfs_response: Value =
            serde_json::from_str(&response_body).expect("Failed to parse IPFS response");
        let ipfs_hash = format!(
            "https://ipfs.io/ipfs/{}",
            ipfs_response["Hash"].as_str().unwrap_or_default()
        );

        Ok(Json(ApiResponse {
            success: true,
            message: "File uploaded to IPFS successfully.".to_string(),
            token_uri: Some(ipfs_hash),
        }))
    } else {
        Ok(Json(ApiResponse {
            success: false,
            message: "IPFS upload failed.".to_string(),
            token_uri: None,
        }))
    }
}
