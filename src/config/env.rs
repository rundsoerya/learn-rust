use dotenv::dotenv;
use std::env;

pub struct Config {
    pub directus_base_url: String,
    pub directus_token: String,
    pub mongodb_host: String,
}

pub fn load_env() -> Config {
    dotenv().ok(); // ✅ Load .env file

    Config {
        directus_base_url: env::var("DIRECTUS_BASE_URL").expect("DIRECTUS_BASE_URL not set"),
        directus_token: env::var("DIRECTUS_TOKEN").expect("DIRECTUS_TOKEN not set"),
        mongodb_host: env::var("MONGODB_HOST").expect("MONGODB_HOST not set"),
    }
}
