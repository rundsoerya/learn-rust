use crate::outbound::directus::place_category::ref_place_category;
use rocket::serde::{Serialize, json::Json};
use rocket::{get, routes};

#[derive(Debug, Serialize)]
pub struct PlaceCategoryResponse {
    pub key: String,
    pub place_name: String,
}

#[derive(Debug, Serialize)]
pub struct CustomApiResponse {
    pub success: bool,
    pub code: u16,
    pub data: Vec<PlaceCategoryResponse>,
    pub message: String,
    pub api_version: String,
}

#[get("/place-categories")]
pub async fn get_place_categories() -> Json<CustomApiResponse> {
    let response = ref_place_category().await; // This returns `Json<ApiResponse>`

    if response.data.is_empty() {
        return Json(CustomApiResponse {
            success: false,
            code: 404,
            data: vec![],
            message: "No categories found".to_string(),
            api_version: "v1".to_string(),
        });
    }

    // ✅ Map the response
    let mapped_categories: Vec<PlaceCategoryResponse> = response
        .data
        .iter()
        .map(|category| PlaceCategoryResponse {
            key: category.key.clone(),
            place_name: category.name.clone(),
        })
        .collect();

    Json(CustomApiResponse {
        success: true,
        code: 200,
        data: mapped_categories,
        message: "Successfully retrieved place categories".to_string(),
        api_version: "v1".to_string(),
    })
}

// Register routes
pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Place Category Module", |rocket| async {
        rocket.mount("/api", routes![get_place_categories])
    })
}
