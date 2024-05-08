use std::env;

pub struct Config {
    pub ipaddr: String,
    pub port: String,
}

impl Config {
    pub fn from_env() -> Self {
        if let Ok(env_file_name) = env::var("CARGO_DOTENV_FILE") {
            dotenv::from_filename(env_file_name).ok();
        }

        let ipaddr = env::var("APP_IPADDR")
            .expect("cannot find APP_IPADDR env");
        let port = env::var("APP_PORT")
            .expect("cannot find APP_PORT env");

        Config {
            ipaddr,
            port,
        }
    }
}