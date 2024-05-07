use actix_web::{get, Responder, Result, web};
use futures::stream::TryStreamExt;
use serde::Deserialize;

use crate::models::product::ProductList;
use crate::services::openfoodfacts;

#[derive(Deserialize)]
struct UpcLookup {
    code: String,
}

#[get("/upc")]
async fn upc_lookup(lookup: web::Query<UpcLookup>) -> Result<impl Responder> {
    let code: String = <std::string::String as Clone>::clone(&lookup.code);
    let product_list = match openfoodfacts::get_products_by_code(code.into()).await {
        Ok(iter) => {
            match iter.try_collect().await {
                Ok(products) => products,
                Err(_) => vec![],
            }
        }
        Err(_) => vec![],
    };

    Ok(web::Json(ProductList {
        objects: product_list.into()
    }))
}
