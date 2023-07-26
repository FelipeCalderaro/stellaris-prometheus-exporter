use lazy_static::__Deref;
use log::{debug, error, info, trace, warn};
use serde_json::{Map, Value};
use std::error::Error;

use super::exporter::{
    STELLARIS_COUNTRY_BALANCE, STELLARIS_COUNTRY_BATTLE_LOSSES, STELLARIS_COUNTRY_FLEETS,
    STELLARIS_COUNTRY_POWER, STELLARIS_COUNTRY_VICTORY_STATUS, STELLARIS_MEGASTRUCTURES,
};
use crate::{
    exporter::{
        exporter::{STELLARIS_COUNTRY_WAR_ALLIES, STELLARIS_COUNTRY_WAR_BATLLES},
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

                let battles = war
                    .get("battles")
                    .and_then(|v| v.as_array())
                    .map(|v| v.len() as i64);

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
                    .set(battles.unwrap_or(0))
            }
        }
    }
}

fn get_war_battles() {}

fn get_war_exhaustion() {}
// pub fn get_battles(gm: Gamestate, save: &str) {
//     info!("Collecting battles info");
//     if let Some(wr) = *gm.war.clone() {
//         match wr {
//             VecOrMap::Map(wars) => {
//                 for (key, war) in wars {
//                     if let Some(battles) = war.battles {
//                         for battle in battles {
//                             let mut main_attacker_name = String::new();
//                             let mut main_defender_name = String::new();
//                             if let Some(attackers) = battle.attackers {
//                                 if attackers.len() > 0 {
//                                     if let Some(country) =
//                                         get_country_by_id(&gm, &attackers.get(0).unwrap().to_i64())
//                                     {
//                                         let name_temp = get_country_name(&country)
//                                             .unwrap_or(format!("{:?}", key));
//                                         let name = name_temp.as_str();
//                                         main_attacker_name = name.to_string();
//                                     }
//                                 }
//                             }
//                             if let Some(defenders) = battle.defenders {
//                                 if defenders.len() > 0 {
//                                     if let Some(country) =
//                                         get_country_by_id(&gm, &defenders.get(0).unwrap().to_i64())
//                                     {
//                                         let name_temp = get_country_name(&country)
//                                             .unwrap_or(format!("{:?}", key));
//                                         let name = name_temp.as_str();
//                                         main_defender_name = name.to_string();
//                                     }
//                                 }
//                             }

//                             let battle_type_temp =
//                                 battle.battle_type.unwrap_or("undefined".to_string());
//                             let battle_type = battle_type_temp.as_str();

//                             STELLARIS_COUNTRY_BATTLE_LOSSES
//                                 .with_label_values(&[
//                                     save,
//                                     main_attacker_name.as_str(),
//                                     main_defender_name.as_str(),
//                                     battle_type,
//                                     "defender",
//                                     key.as_str(),
//                                 ])
//                                 .set(battle.defender_losses.unwrap_or(Number::Int(0)).to_f64());
//                             STELLARIS_COUNTRY_BATTLE_LOSSES
//                                 .with_label_values(&[
//                                     save,
//                                     main_attacker_name.as_str(),
//                                     main_defender_name.as_str(),
//                                     battle_type,
//                                     "attacker",
//                                     key.as_str(),
//                                 ])
//                                 .set(battle.attacker_losses.unwrap_or(Number::Int(0)).to_f64());
//                         }
//                     }
//                 }
//             }
//             _ => {}
//         }
//     }
// }

pub fn get_megastructures(gm: Gamestate, save: &str) {
    info!("Collecting megastructures info");
    if let Some(structures) = *gm.megastructures {
        match structures {
            Value::Object(structs) => {
                debug!(
                    "Detected {:?} megastructures to collect info from",
                    structs.len(),
                );
                for (_, mstruct) in structs.iter() {
                    let mut name = String::new();
                    let owner = mstruct.get("owner").and_then(|v| v.as_i64()).unwrap_or(-1);
                    if let Some(Ok(nm)) = mstruct.get("type").map(|v| transform_input_name(v)) {
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
                        .with_label_values(&[save, name.as_str(), owner.to_string().as_str()])
                        .set(same_structres_owner.len() as f64);
                }
            }
            _ => {}
        }
    }
}
