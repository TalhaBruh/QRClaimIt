use std::env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
        }
    }
}
