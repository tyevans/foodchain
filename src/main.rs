extern crate core;

use actix_web::{App, HttpServer};

mod services;
mod settings;
mod views;
mod persist;
mod models;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    settings::print_config_location();

    match settings::get_app_settings() {
        Ok(settings) => {
            HttpServer::new(|| App::new().service(views::upc_lookup))
                .bind((settings.server_host, settings.server_port))?
                .run()
                .await
        }
        Err(_e) => {
            std::process::exit(1);
        }
    }
}
