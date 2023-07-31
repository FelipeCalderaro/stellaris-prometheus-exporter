mod api;
mod exporter;
mod file;
mod file_io;
mod models;
mod parser;
mod singletons;

use std::{fmt::Display, fs, process::exit};

use crate::{api::exp_api, exporter::renderers::render_name};
use actix_web::{middleware::Logger, App, HttpServer};
use exporter::{
    configs::{read_configs, CONFIGS},
    exporter::register_metrics,
    renderers::transform_input_name,
};
use file::{save_handler, watcher::spawn_file_watcher};
use jomini::TextTape;
use log::{error, trace};
use once_cell::sync::Lazy;

// ------
// fn tests() {
//     std::env::set_var("RUST_LOG", "trace");
//     std::env::set_var("RUST_BACKTRACE", "1");
//     env_logger::init();

//     // ----- //
//     let rs = save_handler::parse_save_file_2(
//         r#"C:\Users\kella\OneDrive\Documentos\Paradox Interactive\Stellaris\save games\thebanished_1801220590\autosave_2229.12.01.sav"#,
//     ).expect("Err 2");

//     fs::write("gamestate.json", rs.gamestate.as_bytes());
//     // fs::write("meta.json", rs.meta.as_bytes());

//     let model = match save_handler::string_to_json(rs.gamestate.as_str()) {
//         Ok(m) => m,
//         Err(e) => {
//             error!("map_to_model:: {:?}", e);
//             std::process::exit(1)
//         }
//     };

//     // let c = model
//     //     .get("megastructures")
//     //     .and_then(|megastructures| megastructures.get("138"))
//     //     .and_then(|megastructure| megastructure.get("type"))
//     //     .and_then(|v| v.as_str())
//     //     .map(|c| render_name(c.to_string()));

//     // let country = model
//     //     .get("country")
//     //     .and_then(|v| v.get("2"))
//     //     .and_then(|v| v.as_object());

//     // let fleet = model.get("fleet").and_then(|v| v.as_object());
//     // let designs = model.get("ship_design").and_then(|v| v.as_object());
//     // let ships = model.get("ships").and_then(|v| v.as_object());

//     // if let (Some(c), Some(f), Some(d), Some(s)) = (country, fleet, designs, ships) {
//     //     get_country_ships_types(c, f, s, d, "name", "save");
//     // }

//     // trace!("Rendered Name: {:?}", c);
// }

// fn main() {
//     tests();
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    register_metrics();
    let mut config = CONFIGS.lock().unwrap();
    *config = match read_configs() {
        Ok(conf) => conf,
        Err(err) => {
            error!("Error while trying to read the config.toml: {:?}", err);
            exit(1);
        }
    };

    // r#"C:\Users\kella\OneDrive\Documentos\Paradox Interactive\Stellaris\save games"#.to_owned(),
    spawn_file_watcher(config.paths.save_location.clone());

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(exp_api::index)
            .service(exp_api::parse)
            .service(exp_api::metrics)
            .service(exp_api::test)
    })
    .workers(4)
    .bind((config.api.ip.clone(), config.api.port.clone()))?
    .run()
    .await
}
