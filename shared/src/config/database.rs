use std::env;

pub struct DataBaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl DataBaseConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Self {
            url: env::var("DATABASE_URL").expect("Database url could not found !"),
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("DB_MAX_CONNECTIONS must be a number"),
        }
    }
}
