mod api;
mod exporter;
mod file;
mod file_io;
mod models;
mod parser;
mod singletons;

use std::{fmt::Display, fs};

use crate::api::exp_api;
use actix_web::{middleware::Logger, App, HttpServer};
use exporter::exporter::register_metrics;
use file::watcher::spawn_file_watcher;
use log::error;
// ------

fn tests() {
    std::env::set_var("RUST_LOG", "trace");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let res = file::save_handler::parse_save_file(
        r#"C:\Users\kella\OneDrive\Documentos\Paradox Interactive\Stellaris\save games\unitednationsspacecommand_1564293158\autosave_2338.08.01.sav"#,
    );
    match res {
        Ok(o) => {}
        Err(err) => error!("{}", err),
    }
    // let content = fs::read(
    //     r#"C:\Users\kella\OneDrive\Documentos\Paradox Interactive\Stellaris\save games\diemachine2_-1986694869\gamestate"#,
    // ).unwrap();
    // let content_string = String::from_utf8(content).unwrap();
    // let rs = parser::parse_file(content_string.as_str()).unwrap();

    // fs::write("content", format!("{}", rs));
}

fn main() {
    tests();
}

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     register_metrics();
//     spawn_file_watcher(
//         r#"C:\Users\kella\OneDrive\Documentos\Paradox Interactive\Stellaris\save games"#.to_owned(),
//     );
//     std::env::set_var("RUST_LOG", "debug");
//     std::env::set_var("RUST_BACKTRACE", "1");
//     env_logger::init();

//     HttpServer::new(move || {
//         let logger = Logger::default();
//         App::new()
//             .wrap(logger)
//             .service(exp_api::index)
//             .service(exp_api::parse)
//             .service(exp_api::metrics)
//             .service(exp_api::test)
//     })
//     .workers(4)
//     .bind(("localhost", 8882))?
//     .bind(("0.0.0.0", 8881))?
//     .run()
//     .await
// }
