use lazy_static::lazy_static;
use log::{debug, trace};
use regex::Regex;
use serde_json::Value;
use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::Path,
    sync::{Arc, Mutex},
};
use walkdir::WalkDir;

use crate::exporter::configs::CONFIGS;

lazy_static! {
    // r#"F:\SteamLibrary\steamapps\common\Stellaris\localisation\english\"#.to_string();
    static ref LOCALIZATION_PATH: String = CONFIGS.lock().unwrap().paths.localisation_path.clone();
    static ref GLOBAL_RENDERER: Arc<Mutex<NameRenderer>> = Arc::new(Mutex::new(NameRenderer::new(
        get_localization_files(Path::new(&*LOCALIZATION_PATH))
    )));
}

#[derive(Debug)]
pub struct NameRenderer {
    localization_files: Vec<String>,
    name_mapping: HashMap<String, String>,
}

impl NameRenderer {
    pub fn new(localization_files: Vec<String>) -> NameRenderer {
        NameRenderer {
            localization_files,
            name_mapping: HashMap::new(),
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

    fn apply_grammar_rules(&mut self, mut rule: String, variables: Vec<Value>) -> String {
        trace!(
            "Rule is:{:?}, Variables len: {:?}, Variables: {:#?}",
            rule,
            variables.len(),
            variables,
        );
        for obj in variables {
            let key = obj.get("key").map(|f| match f {
                Value::Number(n) => n.to_string(),
                Value::String(s) => s.to_owned(),
                _ => "".to_string(),
            });
            let value = obj.get("value");
            trace!("(k,v): {:?},{:?}", key, value);
            if let (Some(ks), Some(v)) = (key, value) {
                let k = ks.as_str();
                trace!("Analysing {:?}", k);
                if rule.contains(k) {
                    trace!("{:?} contains {:?}", rule, k);
                    if let Some(content) = v.get("key").and_then(|x| x.as_str()) {
                        let mapping = self.name_mapping.clone();
                        let from_localization = &mapping.get(content);
                        if content == "%ADJ%" || from_localization.is_none() {
                            let mut result = String::new();
                            // // Check if the input object has a "variables" field
                            if let Some(variables) = v.get("variables").and_then(|v| v.as_array()) {
                                // If it does, iterate over each item in the list
                                result.push_str(&self.apply_grammar_rules(
                                    "$1$ ".repeat(variables.len()).trim_end().to_string(),
                                    variables.to_owned(),
                                ));
                            }
                            rule = rule.replace(k, &result);
                        } else if let Some(localized) = from_localization {
                            trace!("Located localization for {:?}:{:?}", content, localized);
                            if let Some(vars) = v.get("variables").and_then(|v| v.as_array()) {
                                trace!("Localization contains another variables");
                                let mut replaced = String::new();
                                replaced.push_str(rule.replace(k, localized).as_str());
                                replaced.push(' ');
                                replaced.push_str("$1$ ".repeat(vars.len()).trim_end());
                                let applied = self.apply_grammar_rules(replaced, vars.to_owned());
                                rule = rule.replace(k, &applied);
                            }
                            // } else {
                            trace!("Localization already is final, returning");
                            rule = rule.replace(k, localized);
                            // }
                        }
                    }
                } else {
                    trace!("{:?} not contains {:?}", rule, k);
                }
            }
        }

        rule.replace("$", "")
            .replace("]", "")
            .replace("[", "")
            .trim()
            .to_string()
    }

    fn transform_input_to_readable(&mut self, input_object: &Value) -> String {
        trace!("Transforming Object: {:?}", input_object);
        // Initialize an empty string to hold the final result
        let mut result = String::new();

        // Check if the input object has a "key" field
        if let Some(key) = input_object["key"].as_str() {
            if input_object["literal"].is_null() && input_object["variables"].is_null() {
                // If "literal" and "variables" are null, use the key as the result
                if let Some(k) = self.name_mapping.get(key) {
                    result.push_str(k);
                    result.push(' ');
                } else {
                    result.push_str(key);
                    result.push(' ');
                }
            } else if let (Some(key), Some(values)) =
                (self.name_mapping.get(key), input_object.get("variables"))
            {
                // If it does have key, add the corresponding value from the map to the result string
                let grammar_aplied =
                    self.apply_grammar_rules(key.to_owned(), values.as_array().unwrap().to_owned());
                result.push_str(&grammar_aplied);
                result.push(' ');
            } else if let Some(literal) = input_object["literal"].as_str() {
                if literal == "yes" && input_object["variables"].is_null() {
                    // If "literal" is "yes" and "variables" is null, use the key as the result
                    result.push_str(key);
                    result.push(' ');
                }
            }
        }

        // // Check if the input object has a "variables" field
        // if let Some(variables) = input_object["variables"].as_array() {
        //     // If it does, iterate over each item in the list
        //     for variable in variables {
        //         // Each item is another object that needs to be transformed, so call the function recursively
        //         result.push_str(&self.transform_input_to_readable(&variable["value"]));
        //     }
        // }

        // Return the final result string
        result.trim().to_string()
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

pub fn render_name(key: String) -> Result<String, Box<dyn Error>> {
    let mut renderer = GLOBAL_RENDERER.lock()?;
    if renderer.name_mapping.is_empty() {
        renderer.load_name_mapping();
    }
    Ok(renderer.from_key(key.clone()).unwrap_or(key))
}

pub fn transform_input_name(input: &Value) -> Result<String, Box<dyn Error>> {
    let mut renderer = GLOBAL_RENDERER.lock()?;
    if renderer.name_mapping.is_empty() {
        renderer.load_name_mapping();
    }
    Ok(renderer.transform_input_to_readable(input))
}
