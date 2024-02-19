mod error;
mod ipfs;
mod model;
mod utils;
mod web3client;

use axum::extract::Path;
use axum::http::header::{ACCEPT, AUTHORIZATION};
use axum::http::StatusCode;
use axum::routing::post;
use axum::routing::{get, get_service};
use axum::Json;
use axum::{Extension, Router};
use reqwest::header::CONTENT_TYPE;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::{env, io};
use tower_http::services::ServeDir;
use web3::contract::{Contract, Options};
use web3::types::{Address, H160, U256};
use web3::{Transport, Web3};

use crate::error::AppError;
use crate::model::{MintNftRequest, NftMetadata};
use crate::web3client::Web3Client;

use crate::utils::mock_sign_data;
use utoipa::OpenApi;

#[derive(utoipa::OpenApi)]
#[openapi(
    handlers(process_mint_nft, get_nft_metadata, list_tokens),
    components(MintNftRequest, NftMetadata)
)]
struct ApiDoc;

// Return JSON version of the OpenAPI schema
#[utoipa::path(
get,
    path = "/api/openapi.json",
responses(
    (status = 200, description = "JSON file", body = Json )
)
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[utoipa::path(
post,
    path = "/mint",
    request_body = MintNftRequest,
responses(
    (status = 200, description = "NFT minted successfully", body = NftMetadata),
    (status = 400, description = "Bad Request"),
    (status = 500, description = "Internal Server Error")
)
)]
async fn process_mint_nft(
    Extension(web3_client): Extension<Arc<Web3Client>>,
    Json(payload): Json<MintNftRequest>,
) -> Result<Json<NftMetadata>, AppError> {
    let owner_address = payload
        .owner_address
        .parse::<Address>()
        .map_err(|_| AppError::BadRequest("Invalid owner address".into()))?;

    // Retrieve the mock private key from environment variables
    let mock_private_key = env::var("MOCK_PRIVATE_KEY").expect("MOCK_PRIVATE_KEY must be set");

    // Simulate data to be signed
    let data_to_sign = format!("{}:{}", payload.owner_address, payload.token_name).into_bytes();

    // Perform mock signature
    let _mock_signature = mock_sign_data(&data_to_sign, &mock_private_key)?;

    let upload_response = match ipfs::file_upload(payload.file_path.clone()).await {
        Ok(response) => response,
        Err(_) => unreachable!(), // Since Err is Infallible, this branch will never be executed
    };

    let uploaded_token_uri = upload_response.token_uri.clone().unwrap();

    // Call mint_nft using the file_url as the token_uri
    let token_id = mint_nft(
        &web3_client.web3,
        &web3_client.contract,
        owner_address,
        uploaded_token_uri.clone(),
        payload.token_name.clone(),
    )
    .await
    .map_err(|e| AppError::InternalServerError(format!("Failed to mint NFT: {}", e)))?;

    Ok(Json(NftMetadata {
        token_id: token_id.to_string(),
        owner_address: payload.owner_address,
        token_name: payload.token_name,
        token_uri: uploaded_token_uri.clone(),
    }))
}

async fn get_nft_details<T: Transport>(
    contract: &Contract<T>,
    token_id: String,
) -> Result<(U256, String, H160, String), AppError> {
    let parsed_token_id = token_id
        .parse::<U256>()
        .map_err(|_| AppError::BadRequest("Invalid token ID".into()))?;

    let options = Options::with(|opt| {
        opt.gas = Some(1_000_000.into());
    });

    let result = contract
        .query("getTokenDetails", parsed_token_id, None, options, None)
        .await;

    match result {
        Ok(details) => Ok(details),
        Err(e) => {
            // Log the error for debugging purposes
            eprintln!("Smart contract call failed: {:?}", e);

            // Check if the error message contains the specific revert message from the smart contract
            if e.to_string()
                .contains("ERC721: Query for nonexistent token")
            {
                Err(AppError::NotFound("Token does not exist".into()))
            } else {
                Err(AppError::InternalServerError(
                    "Failed to retrieve NFT details".into(),
                ))
            }
        }
    }
}

#[utoipa::path(
get,
    path = "/tokens/{owner_address}",
params(
    ("owner_address" = Option<String>, description = "Owner address to filter tokens by. Type 0 to list all tokens.")
),
responses(
    (status = 200, description = "Token list retrieved successfully", body = [NftMetadata]),
    (status = 400, description = "Bad Request"),
    (status = 500, description = "Internal Server Error")
)
)]

async fn list_tokens(
    Extension(web3_client): Extension<Arc<Web3Client>>,
    token_owner: Option<Path<String>>,
) -> Result<Json<Vec<NftMetadata>>, StatusCode> {
    let owner_address = match token_owner {
        Some(ref owner) if owner.0 != "0" => match owner.0.parse::<Address>() {
            // Check if owner is not "0"
            Ok(addr) => addr,
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        },
        _ => Address::default(), // Treat "0" or None as an indication to list all tokens
    };

    let token_ids =
        match get_all_owned_tokens(&web3_client.web3, &web3_client.contract, owner_address).await {
            Ok(ids) => ids,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

    let mut nft_metadata_list = Vec::new();
    for token_id in token_ids {
        match get_nft_details(&web3_client.contract, token_id.to_string()).await {
            Ok((_, token_name, _onwer, token_uri)) => {
                let nft_metadata = NftMetadata {
                    token_id: token_id.to_string(),
                    owner_address: _onwer.to_string(),
                    token_name,
                    token_uri,
                };
                nft_metadata_list.push(nft_metadata);
            }
            Err(e) => eprintln!("Failed to get metadata for token {}: {:?}", token_id, e), // Log or handle errors as needed
        }
    }

    Ok(Json(nft_metadata_list))
}

#[utoipa::path(
get,
    path = "/nft/{token_id}",
params(
    ("token_id" = String, )),
responses(
    (status = 200, description = "NFT metadata retrieved successfully", body = NftMetadata),
    (status = 400, description = "Bad Request"),
    (status = 500, description = "Internal Server Error")
)
)]
async fn get_nft_metadata(
    Extension(web3_client): Extension<Arc<Web3Client>>,
    Path(token_id): Path<String>,
) -> Result<Json<NftMetadata>, AppError> {
    let parsed_token_id = token_id
        .parse::<U256>()
        .map_err(|_| AppError::BadRequest("Invalid token ID".into()))?;

    match get_nft_details(&web3_client.contract, parsed_token_id.to_string()).await {
        Ok((_, token_name, token_owner, token_uri)) => {
            // Construct NftMetadata for the token
            let nft_metadata = NftMetadata {
                token_id: parsed_token_id.to_string(),
                owner_address: format!("{:?}", token_owner),
                token_name,
                token_uri,
            };

            Ok(Json(nft_metadata))
        }
        Err(AppError::NotFound(msg)) => Err(AppError::NotFound(msg)),
        Err(_) => Err(AppError::InternalServerError(
            "Failed to retrieve NFT details".into(),
        )),
    }
}

async fn mint_nft<T: Transport>(
    _web3: &Web3<T>,
    contract: &Contract<T>,
    owner: Address,
    token_uri: String,
    token_name: String,
) -> Result<U256, Box<dyn Error>> {
    let options = Options::with(|opt| {
        opt.gas = Some(1_000_000.into());
    });

    let token_id = contract
        .call("mintNFT", (owner, token_name, token_uri), owner, options)
        .await?;

    Ok(U256::from(token_id.to_low_u64_be()))
}

async fn get_all_owned_tokens<T: Transport>(
    _web3: &Web3<T>,
    contract: &Contract<T>,
    owner: Address,
) -> Result<Vec<u64>, Box<dyn Error>> {
    let options = Options::with(|opt| {
        opt.gas = Some(1_000_000.into());
    });

    let result: Vec<u64> = contract
        .query("getAllTokensByOwner", owner, owner, options, None)
        .await?;

    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <smart_contract_address>", args[0]);
        std::process::exit(1);
    }

    let contract_address = &args[1];

    let web3_client = Arc::new(Web3Client::new(contract_address).unwrap());

    let app = Router::new()
        .route("/mint", post(process_mint_nft))
        .route("/nft/:token_id", get(get_nft_metadata))
        .route("/tokens/:owner_address?", get(list_tokens))
        .route("/api/openapi.json", get(openapi))
        .nest(
            "/swagger-ui",
            get_service(ServeDir::new("./static/swagger-ui/")).handle_error(handle_serve_dir_error),
        )
        .layer(Extension(web3_client))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION, ACCEPT])
                .allow_methods(vec![axum::http::Method::GET, axum::http::Method::POST]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3010));

    println!("Listening on https://{}", addr);

    if let Err(e) = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        eprintln!("Server failed to start: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

async fn handle_serve_dir_error(error: io::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to serve static file: {}", error),
    )
}

// Unit Tests module
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use dotenvy::dotenv;
    use std::env;
    use tower::ServiceExt; // Provides `oneshot` for services

    async fn setup() -> Router {
        dotenv().ok(); // Load environment variables from .env file

        // Access environment variables
        let contract_address = env::var("TEST_CONTRACT_ADDRESS").expect("TEST_CONTRACT_ADDRESS must be set");

        // Mock Web3Client initialization with environment variable
        let web3_client = Arc::new(Web3Client::new(&contract_address).unwrap());

        // Setup the Router with necessary routes and middleware
        Router::new()
            .route("/mint", post(process_mint_nft))
            .route("/nft/:token_id", get(get_nft_metadata))
            .route("/tokens/:owner_address?", get(list_tokens))
            .layer(Extension(web3_client))
    }

    #[tokio::test]
    async fn test_mint_nft_success() {
        let app = setup().await;

        // Access environment variables for test data
        let owner_address = env::var("TEST_OWNER_ADDRESS").expect("TEST_OWNER_ADDRESS must be set");

        let request_payload = serde_json::to_string(&MintNftRequest {
            owner_address,
            token_name: "TestToken".to_string(),
            token_uri: "https://example.com/token".to_string(),
            file_path: "token.jpg".to_string(),
        }).unwrap();

        let request = Request::builder()
            .uri("/mint")
            .method(axum::http::Method::POST)
            .header("content-type", "application/json")
            .body(Body::from(request_payload))
            .unwrap();

        let response = app.oneshot(request).await.expect("Failed to execute request");

        assert_eq!(response.status(), StatusCode::OK);
    }
}