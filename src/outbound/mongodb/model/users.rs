use mongodb::bson::{DateTime, doc}; // ✅ Use `bson::DateTime`
use serde::{Deserialize, Serialize};
use uuid::Uuid; // ✅ Correct `Uuid` import from `uuid` crate

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: String, // ✅ Correct `Uuid`
    pub nik: Option<i64>,
    pub passport_number: Option<String>,
    pub full_name: String,
    pub born_date: String,
    pub mobile_number: String,
    pub email: String,
    pub created_at: DateTime, // ✅ Use `bson::DateTime`
    pub updated_at: DateTime,
}
