use crate::outbound::mongodb::adapter::get_db;
use crate::outbound::mongodb::model::users::User;
use mongodb::{Collection, error::Result};

pub struct UserRepository;

impl UserRepository {
    pub async fn insert_user(user: &User) -> Result<()> {
        if let Some(db) = get_db() {
            let collection: Collection<User> = db.collection("users");
            collection.insert_one(user, None).await?;
            println!("✅ User inserted successfully!");
            Ok(())
        } else {
            Err(mongodb::error::Error::custom("Database not initialized"))
        }
    }

    // pub async fn find_user_by_email(email: &str) -> Result<Option<User>> {
    //     if let Some(db) = get_db() {
    //         let collection: Collection<User> = db.collection("users");
    //         let user = collection.find_one(doc! { "email": email }, None).await?;
    //         Ok(user)
    //     } else {
    //         Err(mongodb::error::Error::custom("Database not initialized"))
    //     }
    // }
}
