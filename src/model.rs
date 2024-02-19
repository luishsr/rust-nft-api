use serde::{Deserialize, Serialize};
use utoipa::Component;

// Define the request body for minting an NFT
#[derive(Serialize, Deserialize, Component)]
pub struct MintNftRequest {
    pub(crate) owner_address: String,
    pub(crate) token_name: String,
    pub(crate) token_uri: String,
    pub(crate) file_path: String,
}

#[derive(Serialize, Deserialize, Component)]
pub struct TokenFileForm {
    file: Vec<u8>,
}

#[derive(Serialize, Deserialize, Component)]
pub struct ApiResponse {
    pub(crate) success: bool,
    pub(crate) message: String,
    pub(crate) token_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Component)]
pub struct NftMetadata {
    pub(crate) token_id: String,
    pub(crate) owner_address: String,
    pub(crate) token_name: String,
    pub(crate) token_uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct UploadResponse {
    token_uri: String,
}
