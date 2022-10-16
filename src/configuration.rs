#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // initialize configuration reader
    let mut settings = config::Config::default();

    // Add configuration values from a file named "configuration".
    // It will look for any top-level file with an extension
    // that `config` knows how to parse: yaml, json, etc.
    let _ = settings.merge(config::File::with_name("configuration"));

    // try to convert the configuration values it read into Settings
    settings.try_into()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}