use lazy_static::lazy_static;
use log::{debug, error, trace, warn};
use regex::Regex;
use serde_json::Value;
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    error::Error,
    fs::{self, read, File},
    io::{self, BufRead, BufReader},
    path::Path,
    sync::{Arc, Mutex},
};
use walkdir::WalkDir;

use crate::models::gamestate_model::PurpleName;

lazy_static! {
    static ref LOCALIZATION_PATH: String =
        r#"F:\SteamLibrary\steamapps\common\Stellaris\localisation\english\"#.to_string();
    static ref GLOBAL_RENDERER: Arc<Mutex<NameRenderer>> = Arc::new(Mutex::new(NameRenderer::new(
        get_localization_files(Path::new(&*LOCALIZATION_PATH))
    )));
}

#[derive(Debug)]
pub struct NameRenderer {
    localization_files: Vec<String>,
    name_mapping: HashMap<String, String>,
    default_name: String,
}

impl NameRenderer {
    pub fn new(localization_files: Vec<String>) -> NameRenderer {
        NameRenderer {
            localization_files,
            name_mapping: HashMap::new(),
            default_name: "Unknown name".to_string(),
        }
    }

    pub fn load_name_mapping(&mut self) {
        trace!("Loading name mapping");

        for p in &self.localization_files {
            let file_content = fs::read(p).unwrap();
            let as_string = String::from_utf8(file_content).unwrap();
            let result = parse_yaml(&as_string);
            trace!("Adding more {} to the localizations map", result.len());
            self.name_mapping.extend(result);
        }

        // Add missing format that is similar to but not the same as adj_format in practice
        self.name_mapping
            .entry("%ADJECTIVE%".to_string())
            .or_insert("$adjective$ $1$".to_string());
        // Alternate format with no template (meant to be concatenated?). Incomplete solution.
        // self.name_mapping.entry("%ADJ%".to_string()).or_insert("$1$".to_string());
        self.name_mapping
            .entry("%LEADER_1%".to_string())
            .or_insert("$1$ $2$".to_string());
        self.name_mapping
            .entry("%LEADER_2%".to_string())
            .or_insert("$1$ $2$".to_string());
        trace!("{} Localization Entrys loaded", self.name_mapping.len());
    }

    fn from_key(&mut self, key: String) -> Option<String> {
        trace!("Name rendering from Key Started.");
        trace!("Searching for: {}", key);
        let res = self.name_mapping.get(&key);
        trace!("Localizations extraction resulted: {:?}", res);
        if let Some(r) = res {
            return Some(r.to_owned());
        }
        return None;
    }

    fn get_name(&mut self, purple_name: PurpleName) -> Option<String> {
        let mut name_parts: Vec<String> = Vec::new();

        // Check if key is %ADJECTIVE% or %ADJ%
        if *purple_name.key != Some("%ADJECTIVE%".to_string())
            && *purple_name.key != Some("%ADJ%".to_string())
        {
            // Check if literal field contains "yes"
            if *purple_name.literal == Some("yes".to_string()) {
                name_parts.push(purple_name.key.unwrap());
            } else if *purple_name.literal == None && *purple_name.variables == None {
                // Check if the key exists in the hashmap and append its value
                if let Some(value) = self.name_mapping.get(&purple_name.key.unwrap()) {
                    name_parts.push(value.to_string());
                }
            }
        }

        // Handle the variables field
        if let Some(variables) = &*purple_name.variables {
            for variable in variables {
                // Check if the key is "adjective" or "1"
                if *variable.key == Some("adjective".to_string())
                    || *variable.key == Some("1".to_string())
                {
                    if let Some(v) = &*variable.value {
                        if let Some(k) = *v.key.clone() {
                            // Check if the value.key exists in the hashmap and append its value
                            if let Some(value) = self.name_mapping.get(&k) {
                                name_parts.push(value.to_string());
                            }
                        }
                    }
                } else {
                    if let Some(v) = &*variable.value {
                        if let Some(k) = *v.key.clone() {
                            // Check if the value.key exists in the hashmap and append its value
                            if let Some(value) = self.name_mapping.get(&k) {
                                name_parts.push(value.to_string());
                            }
                        }
                    }
                }
            }
        }

        // Join the name parts into a single string and return it
        Some(name_parts.join(" "))
    }

    pub fn format_name_old(&mut self, name_json: &str) -> String {
        // Parse the JSON string into a serde_json::Value
        let name: Value = serde_json::from_str(name_json).unwrap();

        // Initialize an empty vector to hold the parts of the name
        let mut name_parts: Vec<String> = vec![];

        // Check the 'key' field of the dictionary
        let key = name["key"].as_str().unwrap_or("");

        if key == "%ADJECTIVE%" || key == "%ADJ%" {
            // Process the variables
            let s = vec![];
            let variables = name["variables"].as_array().unwrap_or(&s);
            for variable in variables {
                let var_key = variable["key"].as_str().unwrap_or("");
                if var_key == "adjective" || var_key == "1" {
                    let value_key = variable["value"]["key"].as_str().unwrap_or("");
                    if let Some(name_part) = self.name_mapping.get(value_key) {
                        name_parts.push(name_part.to_string());
                    }
                }
            }
        } else {
            // Process the 'literal' field
            let literal = name["literal"].as_str().unwrap_or("");
            if literal == "yes" {
                name_parts.push(key.to_string());
            }

            let s = vec![];
            // Process the variables
            let variables = name["variables"].as_array().unwrap_or(&s);
            for variable in variables {
                let value_key = variable["value"]["key"].as_str().unwrap_or("");
                if let Some(name_part) = self.name_mapping.get(value_key) {
                    name_parts.push(name_part.to_string());
                }
            }

            // If 'literal' and 'variables' are both null, look up the key in the location map
            if literal == "" && variables.is_empty() {
                if let Some(name_part) = self.name_mapping.get(key) {
                    name_parts.push(name_part.to_string());
                }
            }
        }

        // Join all parts of the name into a single string
        let formatted_name = name_parts.join(" ");

        formatted_name
    }

    fn recursive_search(
        &mut self,
        key: &str,
        location_map: &HashMap<String, String>,
        variables: Option<&Vec<Value>>,
    ) -> String {
        let mut key = key.to_string();
        if let Some(variables) = variables {
            for variable in variables {
                let value_key = variable["value"]["key"].as_str().unwrap_or("");
                if let Some(name_part) = location_map.get(value_key) {
                    key = name_part.to_string();
                }
                if variable["value"]["variables"].is_array() {
                    key.push_str(&self.recursive_search(
                        "",
                        location_map,
                        variable["value"]["variables"].as_array(),
                    ));
                }
            }
        }
        key
    }

    pub fn format_name(&mut self, name_json: &str) -> String {
        // Parse the JSON string into a serde_json::Value
        let name: Value = serde_json::from_str(name_json).unwrap();

        // Initialize an empty vector to hold the parts of the name
        let mut name_parts: Vec<String> = Vec::new();

        // Check the 'key' field of the dictionary
        let key = name["key"].as_str().unwrap_or("");

        if key == "%ADJECTIVE%" || key == "%ADJ%" {
            // Process the variables
            if name["variables"].is_array() {
                for variable in name["variables"].as_array().unwrap() {
                    let var_key = variable["key"].as_str().unwrap_or("");
                    if var_key == "adjective" || var_key == "1" {
                        let value_key = variable["value"]["key"].as_str().unwrap_or("");
                        if let Some(name_part) = self.name_mapping.get(value_key) {
                            name_parts.push(name_part.to_string());
                        }
                        if variable["value"]["variables"].is_array() {
                            let res = self.recursive_search(
                                "",
                                &self.name_mapping.clone(),
                                variable["value"]["variables"].as_array(),
                            );
                            name_parts.push(res);
                        }
                    }
                }
            }
        } else {
            // Process the 'literal' field
            let literal = name["literal"].as_str().unwrap_or("");
            if literal == "yes" {
                name_parts.push(key.to_string());
            }

            // Process the variables
            if name["variables"].is_array() {
                for variable in name["variables"].as_array().unwrap() {
                    let value_key = variable["value"]["key"].as_str().unwrap_or("");
                    if let Some(name_part) = self.name_mapping.get(value_key) {
                        name_parts.push(name_part.to_string());
                    }
                    if variable["value"]["variables"].is_array() {
                        name_parts.push(
                            self.recursive_search(
                                "",
                                &self.name_mapping.clone(),
                                variable["value"]["variables"].as_array(),
                            )
                            .to_owned(),
                        );
                    }
                }
            }

            // If 'literal' and 'variables' are both null, look up the key in the location map
            if literal == "" && !name["variables"].is_array() {
                if let Some(name_part) = self.name_mapping.get(key) {
                    name_parts.push(name_part.to_string());
                }
            }
        }

        // Join all parts of the name into a single string
        let formatted_name = name_parts.join(" ");

        formatted_name
    }
}

fn parse_yaml(data: &str) -> HashMap<String, String> {
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

// pub fn render_name(json: &str) -> Result<String, Box<dyn Error>> {
//     let renderer = GLOBAL_RENDERER.clone();
//     let mut renderer = renderer.lock()?;
//     if renderer.name_mapping.len() == 0 {
//         renderer.load_name_mapping();
//     }
//     return renderer.render_from_json(json);
// }
pub fn render_name(key: String) -> Result<String, Box<dyn Error>> {
    let mut renderer = GLOBAL_RENDERER.lock()?;
    if renderer.name_mapping.is_empty() {
        renderer.load_name_mapping();
    }
    Ok(renderer.from_key(key).unwrap_or("".to_string()))
}

pub fn format_name(name: String) -> Result<String, Box<dyn Error>> {
    let mut renderer = GLOBAL_RENDERER.lock()?;
    if renderer.name_mapping.is_empty() {
        renderer.load_name_mapping();
    }
    let res = renderer.format_name(name.as_str());
    Ok(res)
}
