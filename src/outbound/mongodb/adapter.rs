use crate::config::env;
use mongodb::{Client, Database, options::ClientOptions};
use rocket::tokio::sync::OnceCell;
use rocket::{
    Build, Rocket, error,
    fairing::{Fairing, Info, Kind},
    info,
};
use std::sync::Arc;

static DB: OnceCell<Arc<Database>> = OnceCell::const_new();

pub struct MongoDB;

#[rocket::async_trait]
impl Fairing for MongoDB {
    fn info(&self) -> Info {
        Info {
            name: "MongoDB Connection",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        let env_vars = env::load_env(); // ✅ Load environment variables
        let mongodb_conn = env_vars.mongodb_host;
        let dbname = "rustdb";

        match ClientOptions::parse(mongodb_conn.clone()).await {
            Ok(client_options) => match Client::with_options(client_options) {
                Ok(client) => {
                    let database = Arc::new(client.database(dbname));
                    if DB.set(database).is_err() {
                        error!("❌ Failed to set MongoDB instance.");
                        return Err(rocket);
                    }

                    info!("✅ Successfully connected to MongoDB at {}", mongodb_conn);
                    Ok(rocket)
                }
                Err(err) => {
                    error!("❌ MongoDB Client Initialization Error: {:?}", err);
                    Err(rocket)
                }
            },
            Err(err) => {
                error!("❌ MongoDB Connection String Parsing Error: {:?}", err);
                Err(rocket)
            }
        }
    }
}

// Function to get the database instance
pub fn get_db() -> Option<Arc<Database>> {
    DB.get().cloned()
}
