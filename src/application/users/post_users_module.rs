use crate::outbound::mongodb::model::users::User;
use crate::outbound::mongodb::repository::users::UserRepository;
use chrono::Utc;
use mongodb::bson::DateTime as BsonDateTime;
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::{post, routes};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqRegisterUser {
    pub full_name: String,
    pub nik: Option<i64>,
    pub passport_number: Option<String>,
    pub mobile_number: String,
    pub email: String,
    pub born_date: String,
}

#[derive(Debug, Serialize)]
pub struct CustomApiResponse {
    pub success: bool,
    pub code: u16,
    pub user_id: Option<String>,
    pub message: String,
    pub api_version: String,
}

#[post("/create-users", data = "<user>")]
pub async fn create_user(user: Json<ReqRegisterUser>) -> Json<CustomApiResponse> {
    // Check if `nik` is either `None` or `0`
    let is_nik_invalid = user.nik.unwrap_or(0) == 0;
    let generator_uuid = Uuid::new_v4();
    let uuid_string = generator_uuid.to_string();

    // Check if `passport_number` is `None` or an empty string
    let is_passport_invalid = user.passport_number.as_deref().unwrap_or("").is_empty();

    if is_nik_invalid && is_passport_invalid {
        return Json(CustomApiResponse {
            success: false,
            code: 400, // HTTP 400 Bad Request
            user_id: None,
            message: "❌ Error: Either 'nik' or 'passport_number' must be provided.".to_string(),
            api_version: "v1".to_string(),
        });
    }

    // ✅ Validation passed, process user registration
    let new_user = User {
        user_id: uuid_string,
        nik: user.nik,
        passport_number: user.passport_number.clone(),
        full_name: user.full_name.clone(),
        born_date: user.born_date.clone(),
        mobile_number: user.mobile_number.clone(),
        email: user.email.clone(),
        created_at: BsonDateTime::from(SystemTime::from(Utc::now())),
        updated_at: BsonDateTime::from(SystemTime::from(Utc::now())),
    };

    match UserRepository::insert_user(&new_user).await {
        Ok(_) => Json(CustomApiResponse {
            success: true,
            code: 201,
            user_id: Some(new_user.user_id.to_string()),
            message: "✅ User inserted successfully!".to_string(),
            api_version: "v1".to_string(),
        }),
        Err(err) => Json(CustomApiResponse {
            success: false,
            code: 500,
            user_id: None,
            message: format!("❌ Error inserting user: {:?}", err),
            api_version: "v1".to_string(),
        }),
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Post Users Module", |rocket| async {
        rocket.mount("/api", routes![create_user])
    })
}
