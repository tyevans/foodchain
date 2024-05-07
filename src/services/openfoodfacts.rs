use bson::doc;
use mongodb::Cursor;
use mongodb::options::FindOptions;

use crate::models::product::Product;
use crate::{persist, settings};

pub(crate) async fn get_products_by_code(code: String) -> mongodb::error::Result<Cursor<Product>> {
    let settings = settings::get_app_settings().unwrap();
    let mongo_client = persist::get_mongo_client().await.unwrap();

    let db = mongo_client.database(&*settings.openfoodfacts_mongodb_name);
    let typed_collection = db.collection::<Product>("products");

    let filter = doc! { "code": code };
    let find_options = FindOptions::builder().sort(doc! { "code": 1 }).build();
    typed_collection.find(filter, find_options).await
}
