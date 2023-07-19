use log::{debug, trace};

use crate::{file_io::load_save_content, models::gamestate_model::Gamestate, parser::parse_save};
use std::{fs::File, io::Write, process::exit, string::FromUtf8Error};

pub struct GameContent {
    pub filename: String,
    pub game_id: String,
    pub meta: String,
    pub gamestate: Box<String>,
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
                        meta: save_file.meta,
                        gamestate: Box::new(js),
                    })
                }
                Err(e) => return Err(format!("Error while converting to json: {}", e)),
            };
        }
        Err(msg) => {
            println!("Failed to parse {}: {}", save_file.filename, msg);
            return Err(format!("Failed to parse {}: {}", save_file.filename, msg));
        }
    };
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
