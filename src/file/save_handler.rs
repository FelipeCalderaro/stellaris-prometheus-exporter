use futures::future::ok;
use jomini::TextTape;
use log::{debug, error, trace};

use crate::{file_io::load_save_content, models::gamestate_model::Gamestate, parser::parse_save};
use std::{error::Error, fs::File, io::Write, process::exit, string::FromUtf8Error};

pub struct GameContent {
    pub filename: String,
    pub game_id: String,
    pub meta: Box<String>,
    pub gamestate: Box<String>,
}

pub fn parse_save_file_2(save_path: &str) -> Result<GameContent, String> {
    let save_file = match load_save_content(save_path) {
        Ok(sf) => sf,
        Err(_) => exit(1),
    };

    let meta = match parse_content(save_file.meta.as_str()) {
        Ok(c) => c,
        Err(e) => {
            error!("Error while parsing contents of Meta: {:#?}", e);
            return Err("Parsing Meta was not possible".to_string());
        }
    };
    let gamestate = match parse_content(save_file.gamestate.as_str()) {
        Ok(c) => c,
        Err(e) => {
            error!("Error while parsing contents of Meta: {:#?}", e);
            return Err("Parsing Meta was not possible".to_string());
        }
    };

    Ok(GameContent {
        filename: save_file.filename,
        game_id: save_file.game_id,
        meta,
        gamestate,
    })
}

fn parse_content(content: &str) -> Result<Box<String>, Box<dyn Error>> {
    let tape = TextTape::from_slice(content.as_bytes())?;
    let reader = tape.utf8_reader();
    let actual = reader.json().to_string();
    Ok(Box::new(actual.to_string()))
}

pub fn parse_save_file(save_path: &str) -> Result<GameContent, String> {
    let save_file = match load_save_content(save_path) {
        Ok(sf) => sf,
        Err(_) => exit(1),
    };
    match parse_save(&save_file) {
        Ok(parsed_save) => {
            let obj = parsed_save.gamestate;
            match serde_json::to_string(&obj) {
                Ok(js) => {
                    return Ok(GameContent {
                        filename: save_file.filename.clone(),
                        game_id: parsed_save.game_id,
                        meta: Box::new(save_file.meta),
                        gamestate: Box::new(js),
                    })
                }
                Err(e) => return Err(format!("Error while converting to json: {}", e)),
            };
        }
        Err(msg) => {
            error!("Failed to parse {}: {}", save_file.filename, msg);
            return Err(format!("Failed to parse {}: {}", save_file.filename, msg));
        }
    };
}

pub fn string_to_json(string: &str) -> serde_json::Result<Box<serde_json::Value>> {
    let result: Box<serde_json::Value> = Box::new(serde_json::from_str(string)?);
    return Ok(result);
}

pub fn map_to_model(json: Box<String>) -> serde_json::Result<Box<Gamestate>> {
    let gm: Box<Gamestate> = Box::new(serde_json::from_str(&*json)?);
    return Ok(gm);
}

pub fn save_json_to_file(json: &Box<String>) -> std::io::Result<()> {
    trace!("Saving Save information to json file");
    let mut file = File::create("gamestate.json")?;
    let _ = file.write_all(json.as_bytes())?;
    trace!("Save done");
    Ok(())
}

pub fn read_from_json_file() -> Result<Box<String>, FromUtf8Error> {
    let file = std::fs::read("gamestate.json").expect("File found");
    match String::from_utf8(file) {
        Ok(s) => Ok(Box::new(s)),
        Err(e) => Err(e),
    }
}

pub fn convert_to_pretty_str(content: String) -> serde_json::Result<String> {
    let json: serde_json::Value = serde_json::from_str(&content.as_str())?;
    let pretty = serde_json::to_string_pretty(&json).unwrap();
    Ok(pretty)
}
