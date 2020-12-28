use config::{Config, File, ConfigError};

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub token: String,
    pub folder: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::default();
        match settings.merge(File::with_name("config")) {
            Err(why) => Err(why),
            Ok(_) => settings.try_into(),
        }
    }
}
