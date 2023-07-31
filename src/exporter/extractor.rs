use lazy_static::__Deref;
use log::{debug, error, info, trace, warn};
use serde_json::{json, Map, Number, Value};
use std::{collections::HashMap, error::Error};

use super::exporter::{
    STELLARIS_COUNTRY_BALANCE, STELLARIS_COUNTRY_BATTLE_LOSSES, STELLARIS_COUNTRY_FLEETS,
    STELLARIS_COUNTRY_POWER, STELLARIS_COUNTRY_SHIP_SIZES, STELLARIS_COUNTRY_VICTORY_STATUS,
    STELLARIS_MEGASTRUCTURES,
};
use crate::{
    exporter::{
        exporter::{
            STELLARIS_COUNTRY_COLONIZED_PLANETS, STELLARIS_COUNTRY_CONTROLLED_CELESTIAL_BODIES,
            STELLARIS_COUNTRY_SURVEYED_SYSTEMS, STELLARIS_COUNTRY_WAR_ALLIES,
            STELLARIS_COUNTRY_WAR_BATLLES,
        },
        renderers::{render_name, transform_input_name},
    },
    models::gamestate_model::Gamestate,
};

pub fn get_country_infos(gm: Gamestate, save: &str) {
    info!("Collecting Country Infos");
    if let None = *gm.country {
        error!("Gamestate has no Countries: {:?}", gm.country);
        return;
    }

    if let Value::Object(countries) = gm.country.clone().unwrap() {
        debug!(
            "Detected {} Countries to extract info from",
            countries.len()
        );
        for (key, value) in countries {
            trace!("Analysing current country: {}", key);
            match value {
                Value::Object(country) => {
                    // let name_temp = get_country_name(&country).unwrap_or(format!("{:?}", key));
                    // let name = name_temp.as_str();
                    let rendered_name = match country.get("name").map(|v| transform_input_name(v)) {
                        Some(res) => match res {
                            Ok(s) => s,
                            Err(_) => key.clone().to_string(),
                        },
                        None => key.to_string(),
                    };
                    let name = rendered_name.as_str();
                    get_country_powers(&country, name, save);
                    get_country_fleets(&country, name, save);
                    get_country_balance(&country, name, save);
                    get_country_victory_score_n_rank(&country, name, save);
                    get_country_war_allies(&country, name, save);
                    get_country_controlled_celestial_bodies(&country, name, save);
                    get_country_colonized_planets(&country, name, save);
                    get_country_surveyed_systems(&country, name, save);
                    if let (
                        Some(Value::Object(fleet)),
                        Some(Value::Object(designs)),
                        Some(Value::Object(ships)),
                    ) = (
                        *gm.fleet.clone(),
                        *gm.ship_design.clone(),
                        *gm.ships.clone(),
                    ) {
                        get_country_ships_types(&country, &fleet, &ships, &designs, name, save);
                    }
                }
                _ => {}
            }
        }
    }
}

fn get_country_powers(country: &Map<String, Value>, name: &str, save: &str) {
    trace!("\tSeparating country powers");
    match country["military_power"].clone() {
        Value::Number(n) => {
            STELLARIS_COUNTRY_POWER
                .with_label_values(&[save, "military".as_ref(), name])
                .set(n.as_f64().unwrap_or(0.0));
        }
        _ => {}
    }
    match country["tech_power"].clone() {
        Value::Number(n) => {
            STELLARIS_COUNTRY_POWER
                .with_label_values(&[save, "tech".as_ref(), name])
                .set(n.as_f64().unwrap_or(0.0));
        }
        _ => {}
    }
    match country["economy_power"].clone() {
        Value::Number(n) => {
            STELLARIS_COUNTRY_POWER
                .with_label_values(&[save, "economic".as_ref(), name])
                .set(n.as_f64().unwrap_or(0.0));
        }
        _ => {}
    }
}

fn get_country_fleets(country: &Map<String, Value>, name: &str, save: &str) {
    trace!("\tSeparating country fleets");

    country
        .get("fleets_manager")
        .and_then(|f| f.as_object())
        .and_then(|fleets_manager| fleets_manager.get("owned_fleets"))
        .and_then(|v| v.as_array())
        .map(|fleets| {
            STELLARIS_COUNTRY_FLEETS
                .with_label_values(&[save, name])
                .set(fleets.len() as i64)
        });
}

fn get_country_balance(country: &Map<String, Value>, name: &str, save: &str) {
    trace!("\tSeparating country balance");

    country
        .get("budget")
        .and_then(|v| v.as_object())
        .and_then(|budget| budget.get("income_high_water_mark"))
        .and_then(|v| v.as_object())
        .and_then(|income| income.get("current"))
        .and_then(|v| v.as_object())
        .map(|current| {
            for (key, value) in current {
                if let Value::Number(n) = value {
                    STELLARIS_COUNTRY_BALANCE
                        .with_label_values(&[save, name, key.as_str()])
                        .set(n.as_f64().unwrap_or(0.0));
                }
            }
        });
}

fn get_country_victory_score_n_rank(country: &Map<String, Value>, name: &str, save: &str) {
    trace!("\tSeparating country victory rank and score");
    country
        .get("victory_rank")
        .and_then(|rank| rank.as_i64())
        .map(|value| {
            STELLARIS_COUNTRY_VICTORY_STATUS
                .with_label_values(&[save, name, "rank"])
                .set(value as f64)
        });

    country
        .get("victory_score")
        .and_then(|rank| rank.as_i64())
        .map(|value| {
            STELLARIS_COUNTRY_VICTORY_STATUS
                .with_label_values(&[save, name, "score"])
                .set(value as f64)
        });
}

fn get_country_war_allies(country: &Map<String, Value>, name: &str, save: &str) {
    trace!("\tSeparating country War Allies");
    country
        .get("war_allies")
        .and_then(|v| v.as_array())
        .map(|v| {
            STELLARIS_COUNTRY_WAR_ALLIES
                .with_label_values(&[save, name])
                .set(v.len() as i64);
        });
}

fn get_country_controlled_celestial_bodies(country: &Map<String, Value>, name: &str, save: &str) {
    trace!("\tSeparating country Controlled Celestial Bodies");

    country
        .get("controlled_planets")
        .and_then(|planets| planets.as_array())
        .map(|planets| planets.len())
        .map(|length| {
            STELLARIS_COUNTRY_CONTROLLED_CELESTIAL_BODIES
                .with_label_values(&[save, name])
                .set(length as i64)
        });
}

fn get_country_colonized_planets(country: &Map<String, Value>, name: &str, save: &str) {
    trace!("\tSeparating country Colonized Planets");
    country
        .get("owned_planets")
        .and_then(|v| v.as_array())
        .map(|planets| planets.len())
        .map(|length| {
            STELLARIS_COUNTRY_COLONIZED_PLANETS
                .with_label_values(&[save, name])
                .set(length as i64)
        });
}

fn get_country_surveyed_systems(country: &Map<String, Value>, name: &str, save: &str) {
    trace!("\tSeparating country Surveyed Systems");
    country
        .get("surveyed")
        .and_then(|v| v.as_array())
        .map(|surveyed| surveyed.len())
        .map(|len| {
            STELLARIS_COUNTRY_SURVEYED_SYSTEMS
                .with_label_values(&[save, name])
                .set(len as i64)
        });
}

fn get_country_ships_types(
    country: &Map<String, Value>,
    fleets: &Map<String, Value>,
    ships: &Map<String, Value>,
    ship_designs: &Map<String, Value>,
    name: &str,
    save: &str,
) {
    trace!("\tSeparating country Ship Types");
    let mut ships_ids: Vec<Value> = Vec::new();
    country
        .get("fleets_manager")
        .and_then(|f| f.as_object())
        .and_then(|fleets_manager| fleets_manager.get("owned_fleets"))
        .and_then(|v| v.as_array())
        .map(|owned_fleets| {
            for owned in owned_fleets {
                if let Some(fleet) = owned.get("fleet").and_then(|v| v.as_i64()) {
                    if let Some(sps) = fleets
                        .get(fleet.to_string().as_str())
                        .and_then(|flt| flt.get("ships"))
                        .and_then(|v| v.as_array())
                    {
                        ships_ids.extend_from_slice(sps);
                    }
                }
            }
        });

    for ship_id in &ships_ids {
        let id = ship_id.as_i64().unwrap_or(0);
        let design = ships
            .get(id.to_string().as_str())
            .and_then(|v| v.get("ship_design"))
            .and_then(|v| v.as_i64())
            .unwrap_or(-1);
        let size = ship_designs
            .get(design.to_string().as_str())
            .and_then(|v| v.as_object())
            .and_then(|v| v.get("ship_size"))
            .and_then(|v| v.as_str())
            .unwrap_or("no_size");
        let sizes: HashMap<String, Value> = ships
            .iter()
            .filter(|(k, v)| {
                ships_ids.contains(&json!(k.parse::<i64>().unwrap_or(-1)))
                    && v.get("ship_design").and_then(|v| v.as_i64()).unwrap_or(-1) == design
            })
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        STELLARIS_COUNTRY_SHIP_SIZES
            .with_label_values(&[save, name, size])
            .set(sizes.len() as f64);
    }
}

fn get_country_by_id(gm: Gamestate, id: &i64) -> Option<Value> {
    if let Some(countries) = *gm.country.clone() {
        match countries {
            Value::Object(country) => {
                if let Some(c) = country.get(&id.to_string()) {
                    return Some(c.to_owned());
                }
            }
            _ => {
                return None;
            }
        }
    }
    return None;
}

fn get_country_name(country: &Value) -> Option<String> {
    if let Some(name) = country.get("name") {
        if let Ok(country_name) = transform_input_name(name) {
            return Some(country_name);
        }
    }
    return None;
}

pub fn get_wars(gm: Gamestate, save: &str) {
    //
    info!("collecting battles infos");
    if let None = *gm.war {
        return;
    }

    let Some(wars) = gm.war.clone().and_then(|v| v.as_object().cloned()) else {
        return;
    };
    for (id, war) in wars {
        let name = war.get("name").map(|v| transform_input_name(v));
        let start_date = war.get("start_date").and_then(|v| v.as_str().to_owned());

        let attacker_war_exhaustion = war
            .get("attacker_war_exhaustion")
            .and_then(|v| v.as_str().to_owned());
        let defender_war_exhaustion = war
            .get("defender_war_exhaustion")
            .and_then(|v| v.as_str().to_owned());

        let attackers = war.get("attackers").and_then(|v| v.as_array().to_owned());
        let defenders = war.get("defenders").and_then(|v| v.as_array().to_owned());
        let main_attacker_id: Option<i64> = attackers
            .map(|vec| {
                vec.iter().filter(|&attacker| {
                    if let serde_json::Value::Object(map) = attacker {
                        map.get("call_type")
                            .map_or(false, |call_type| call_type == "primary")
                    } else {
                        false
                    }
                })
            })
            .and_then(|mut filter_iter| filter_iter.next())
            .and_then(|v| v.get("country"))
            .and_then(|v| v.as_i64());

        let main_defender_id: Option<i64> = defenders
            .map(|vec| {
                vec.iter().filter(|&defender| {
                    if let serde_json::Value::Object(map) = defender {
                        map.get("call_type")
                            .map_or(false, |call_type| call_type == "primary")
                    } else {
                        false
                    }
                })
            })
            .and_then(|mut filter_iter| filter_iter.next())
            .and_then(|v| v.get("country"))
            .and_then(|v| v.as_i64());

        let attacker_war_goal = war
            .get("attacker_war_goal")
            .and_then(|v| v.get("type"))
            .and_then(|v| v.as_str())
            .map(|v| render_name(format!("war_goal_{}", v)));

        let defender_war_goal = war
            .get("defender_war_goal")
            .and_then(|v| v.get("type"))
            .and_then(|v| v.as_str())
            .map(|v| render_name(format!("war_goal_{}", v)));

        if let (
            Some(m_attacker_id),
            Some(m_defender_id),
            Some(Ok(att_war_goal)),
            Some(Ok(def_war_goal)),
            Some(Ok(war_name)),
            Some(date),
        ) = (
            main_attacker_id,
            main_defender_id,
            attacker_war_goal,
            defender_war_goal,
            name,
            start_date,
        ) {
            if let (Some(main_attacker), Some(main_defender)) = (
                get_country_by_id(gm.clone(), &m_attacker_id),
                get_country_by_id(gm.clone(), &m_defender_id),
            ) {
                let main_attacker_name = get_country_name(&main_attacker);
                let main_defender_name = get_country_name(&main_defender);

                let number_of_battles = war
                    .get("battles")
                    .and_then(|v| v.as_array())
                    .map(|v| v.len() as i64);

                let battles = war.get("battles").and_then(|v| v.as_array());

                STELLARIS_COUNTRY_WAR_BATLLES
                    .with_label_values(&[
                        save,
                        main_attacker_name
                            .unwrap_or(m_attacker_id.to_string())
                            .as_str(),
                        main_defender_name
                            .unwrap_or(m_defender_id.to_string())
                            .as_str(),
                        att_war_goal.as_str(),
                        def_war_goal.as_str(),
                        war_name.as_str(),
                        date,
                        id.as_str(),
                    ])
                    .set(number_of_battles.unwrap_or(0))
            }
        }
    }
}

fn get_war_battles(battles: &Vec<Value>) {}

fn get_war_exhaustion() {}

fn get_country_name_by_id(gm: Gamestate, id: &str) -> Option<String> {
    let name = gm
        .country
        .and_then(|countries| countries.get(id).cloned())
        .and_then(|country| country.get("name").cloned())
        .map(|v| transform_input_name(&v));

    match name {
        Some(nm) => match nm {
            Ok(n) => Some(n),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn get_megastructures(gm: Gamestate, save: &str) {
    info!("Collecting megastructures info");
    if let Some(structures) = *gm.megastructures.clone() {
        match structures {
            Value::Object(structs) => {
                debug!(
                    "Detected {:?} megastructures to collect info from",
                    structs.len(),
                );
                for (_, mstruct) in structs.iter() {
                    let mut name = String::new();
                    let owner = mstruct.get("owner").and_then(|v| v.as_i64()).unwrap_or(-1);
                    let owner_name = get_country_name_by_id(gm.clone(), owner.to_string().as_str());
                    if let Some(Ok(nm)) = mstruct
                        .get("type")
                        .and_then(|v| v.as_str())
                        .map(|v| render_name(v.to_string()))
                    {
                        name = nm;
                    }

                    let same_structres_owner: Map<String, Value> = structs
                        .iter()
                        .filter(|(_, v)| {
                            v.get("type") == mstruct.get("type")
                                && v.get("owner") == mstruct.get("owner")
                        })
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();
                    STELLARIS_MEGASTRUCTURES
                        .with_label_values(&[
                            save,
                            name.as_str(),
                            owner_name.unwrap_or(owner.to_string()).as_str(),
                        ])
                        .set(same_structres_owner.len() as f64);
                }
            }
            _ => {}
        }
    }
}
