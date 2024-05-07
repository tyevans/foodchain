use mongodb::options::ClientOptions;
use crate::settings;

pub(crate) async fn get_mongo_client() -> mongodb::error::Result<mongodb::Client> {
    let settings = settings::get_app_settings().unwrap();
    let client_options = ClientOptions::parse(settings.openfoodfacts_mongodb_uri).await?;
    mongodb::Client::with_options(client_options)
}
