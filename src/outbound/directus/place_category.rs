use crate::config::env;
use reqwest::{Client, header};
use rocket::serde::{Deserialize, Serialize, json::Json}; // ✅ Load environment variables

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceCategory {
    pub id: String,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub data: Vec<PlaceCategory>,
}

// ✅ Make `ref_place_category` a reusable async function (remove Rocket route annotation)
pub async fn ref_place_category() -> Json<ApiResponse> {
    let env_vars = env::load_env(); // ✅ Load env variables
    let url = format!("{}/items/ref_place_category", env_vars.directus_base_url);
    let token = env_vars.directus_token;

    let client = Client::new();
    let request = client
        .get(&url)
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .header(header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    match request {
        Ok(response) => match response.json::<ApiResponse>().await {
            Ok(parsed) => Json(parsed),
            Err(_) => Json(ApiResponse { data: vec![] }), // Handle JSON parsing error
        },
        Err(_) => Json(ApiResponse { data: vec![] }), // Handle request error
    }
}
