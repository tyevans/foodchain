use confy::ConfyError;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FoodchainSettings {
    pub server_host: String,
    pub server_port: u16,
    pub openfoodfacts_mongodb_uri: String,
    pub openfoodfacts_mongodb_name: String,
}

impl ::std::default::Default for FoodchainSettings {
    fn default() -> Self {
        Self {
            server_host: "127.0.0.1".into(),
            server_port: 8080,
            openfoodfacts_mongodb_uri: "mongodb://root:example@localhost:27017/".into(),
            openfoodfacts_mongodb_name: "off".into()
        }
    }
}



pub fn print_config_location() {
    match confy::get_configuration_file_path("foodchain", "settings") {
        Ok(path) => {
            println!("Loading Config From: {:?}", path);
        }
        Err(e) => {
            println!("Failed to read configuration: {}", e);
            std::process::exit(1);
        }
    }
}


pub fn get_app_settings() -> Result<FoodchainSettings, ConfyError> {
    confy::load("foodchain", "settings")
}
