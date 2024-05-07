use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductNutriments {
    pub energy_value: Option<u32>,
    pub energy_unit: Option<String>,
    pub fat_100g: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub code: String,
    pub url: Option<String>,
    pub quantity: Option<String>,
    pub product_name: Option<String>,
    pub product_quantity: Option<f64>,
    pub generic_name: Option<String>,
    pub brands: Option<String>,
    pub nutriments: Option<ProductNutriments>
}


#[derive(Serialize)]
pub struct ProductList {
    pub(crate) objects: Vec<Product>,
}
