use actix_web::{get, http::header::ContentType, HttpRequest, HttpResponse};
use log::{debug, error};
use prometheus::Encoder;
use regex::Regex;
use std::io::Write;
use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
    process::exit,
};
use walkdir::WalkDir;

use crate::singletons::singletons::get_game_data;
use crate::{
    exporter::{
        exporter::{REGISTRY, STELLARIS_INCOMING_REQUESTS},
        extractor::{get_battles, get_country_infos, get_megastructures},
    },
    file::save_handler::{self, read_from_json_file},
};

fn get_localization_files(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    WalkDir::new(dir).into_iter().for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            match path.extension() {
                Some(extension) => {
                    if extension == "yaml" || extension == "yml" {
                        files.push(path.to_string_lossy().into_owned());
                    }
                }
                None => {}
            }
        }
    });
    debug!("Loaded {:?} localization files", files.len());
    return files;
}

fn parse_data(data: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    let loc_re = Regex::new(r#"\s(?P<key>\S+?):\d+\s\"(?P<value>.*)\""#).unwrap();
    for line in data.lines() {
        // Ignore comments and empty lines
        if line.trim().starts_with('#') || line.trim().is_empty() {
            continue;
        }

        // Use the regex to capture the key-value pairs
        if let Some(captures) = loc_re.captures(line) {
            let key = captures.name("key").unwrap().as_str();
            let value = captures.name("value").unwrap().as_str();

            // Insert into the HashMap
            map.insert(key.to_string(), value.to_string());
        }
    }

    map
}

#[get("/teste")]
pub async fn test(_req: HttpRequest) -> HttpResponse {
    STELLARIS_INCOMING_REQUESTS.inc();
    let files = get_localization_files(Path::new(
        r#"F:\SteamLibrary\steamapps\common\Stellaris\localisation\english\"#,
    ));
    let mut hash: HashMap<String, String> = HashMap::new();
    for p in &files {
        let s = fs::read(p).unwrap();
        let a = String::from_utf8(s).unwrap();
        let res = parse_data(a.as_str());
        hash.extend(res);
    }
    let mut buffer = File::create("map.txt").unwrap();
    let res = serde_json::to_string(&hash).unwrap();
    let _ = buffer.write(res.as_bytes());
    return HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(format!("Done: {:#?}", hash));
}

#[get("/metrics")]
pub async fn metrics(_req: HttpRequest) -> HttpResponse {
    STELLARIS_INCOMING_REQUESTS.inc();
    // let name = match std::env::var("STELLARIS_FILENAME") {
    //     Ok(n) => n,
    //     Err(_) => "NO_NAME".to_owned(),
    // };
    let _game_id = match std::env::var("STELLARIS_GAMEID") {
        Ok(n) => n,
        Err(_) => "NO_ID".to_owned(),
    };

    // let result = match read_from_json_file() {
    //     Ok(c) => c,
    //     Err(_) => {
    //         return HttpResponse::NotFound()
    //             .content_type(ContentType::plaintext())
    //             .body("")
    //     }
    // };

    let result = get_game_data();

    if let Err(_) = result {
        error!("Data could no be read from memory or not ready yet");
        return HttpResponse::NotFound()
            .content_type(ContentType::plaintext())
            .body("Data could not be read or not ready yet");
    }

    let model = match save_handler::map_to_model(result.unwrap()) {
        Ok(m) => m,
        Err(e) => {
            error!("map_to_model:: {:?}", e);
            return HttpResponse::NotFound()
                .content_type(ContentType::plaintext())
                .body("Data could not be read or not ready yet");
        }
    };
    get_country_infos(*model.clone(), _game_id.as_ref());
    get_battles(*model.clone(), _game_id.as_ref());
    get_megastructures(*model.clone(), _game_id.as_ref());

    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&REGISTRY.gather(), &mut buffer) {
        eprintln!("could not encode custom metrics: {}", e);
    };
    let mut res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("custom metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        eprintln!("could not encode prometheus metrics: {}", e);
    };
    let res_custom = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("prometheus metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    res.push_str(&res_custom);
    return HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(res);
}

#[get("/parse")]
pub async fn parse(_req: HttpRequest) -> HttpResponse {
    STELLARIS_INCOMING_REQUESTS.inc();

    let save_path = r#"C:/Users/kella/OneDrive/Documentos/Paradox Interactive/Stellaris/save games/mentemasquaica3_-1946963056/autosave_2207.04.01.sav"#;
    let content = match save_handler::parse_save_file(save_path) {
        Ok(m) => m,
        Err(e) => {
            println!("Error parsing file: {}", e);
            exit(1)
        }
    };

    std::env::set_var("STELLARIS_FILENAME", content.filename);
    std::env::set_var("STELLARIS_GAMEID", content.game_id);

    let v: serde_json::Value = serde_json::from_str(&content.gamestate).unwrap();
    let s = serde_json::to_string_pretty(&v).unwrap();

    let _ = save_handler::save_json_to_file(&Box::new(s));
    return HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Done!");
}

#[get("/")]
pub async fn index(_req: HttpRequest) -> HttpResponse {
    STELLARIS_INCOMING_REQUESTS.inc();

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("Welcome");
}
