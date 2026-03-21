use std::env;

pub struct Config {
    host: String,
    port: u32,
    pub db_dsn: String,
}

pub fn load_config() -> Config {
    Config {
        host: env::var("HOST").unwrap_or("localhost".to_string()),
        port: env::var("PORT")
            .unwrap_or("8888".to_string())
            .parse()
            .unwrap_or(8888),
        db_dsn: env::var("DB_DSN").unwrap(),
    }
}

impl Config {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
