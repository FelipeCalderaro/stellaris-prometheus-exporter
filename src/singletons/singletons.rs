use lazy_static::lazy_static;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

pub struct GamestateData {
    pub parsed: Box<String>,
}

impl GamestateData {
    pub fn new() -> GamestateData {
        GamestateData {
            parsed: Box::new(String::new()),
        }
    }
}
// -----------

// INSTANTIATE
lazy_static! {
    static ref GAME_DATA: Arc<Mutex<GamestateData>> = Arc::new(Mutex::new(GamestateData::new()));
}
// GETTERS & SETTERS

pub fn get_game_data() -> Result<Box<String>, Box<dyn Error>> {
    let data = GAME_DATA.lock()?;
    let content = data.parsed.clone();
    Ok(content)
}

pub fn set_game_data(content: String) -> Result<(), Box<dyn Error>> {
    let mut data = GAME_DATA.lock()?;
    data.parsed = Box::new(content);

    Ok(())
}
