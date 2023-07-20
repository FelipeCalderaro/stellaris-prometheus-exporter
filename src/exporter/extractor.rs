use lazy_static::__Deref;
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, error::Error};

use super::exporter::{
    STELLARIS_COUNTRY_BALANCE, STELLARIS_COUNTRY_BATTLE_LOSSES, STELLARIS_COUNTRY_FLEETS,
    STELLARIS_COUNTRY_POWER, STELLARIS_COUNTRY_VICTORY_STATUS, STELLARIS_MEGASTRUCTURES,
};
use crate::{
    exporter::renderers::render_name,
    models::gamestate_model::{
        CountryClass, CountryValue, FleetsManagerUnion, Gamestate, Megastructure, Number,
        PurpleName, VecOrMap,
    },
};

const NAME_PREFIX: [&str; 3] = ["SPEC_", "NAME_", "EMPIRE_DESIGN_"];

pub fn get_country_infos(gm: Gamestate, save: &str) {
    info!("Collecting Country Infos");
    if let None = *gm.country {
        debug!("Gamestate has no Countries");
        return;
    }
    // let x = match gm.country.clone().unwrap() {
    //     VecOrMap::Vec(v) => todo!(),
    //     VecOrMap::Map(m) => todo!(),
    //     VecOrMap::Unknown(u) => todo!(),
    //     VecOrMap::VecNone(n) => todo!(),
    // };

    if let VecOrMap::Map(countries) = gm.country.clone().unwrap() {
        debug!(
            "Detected {} Countries to extract info from",
            countries.len()
        );
        for (key, value) in countries {
            trace!("Analysing current country: {}", key);
            match value {
                CountryValue::CountryClass(country) => {
                    let name_temp = get_country_name(&country).unwrap_or(format!("{:?}", key));
                    let name = name_temp.as_str();

                    get_country_powers(&country, name, save);
                    get_country_fleets(&country, name, save);
                    get_country_balance(&country, name, save);
                    get_country_victory_score_n_rank(&country, name, save);
                }
                _ => {}
            }
        }
    }
    //  else if let VecOrMap::Vec(countries) = gm.country.clone().unwrap() {
    //     warn!("Gamestate.Country is not a HashMap. Analysing as Vec");
    //     for (id, value) in countries.iter().enumerate() {
    //         trace!("Analysing current country: {}", id);
    //         // match value {
    //         //     CountryValue::CountryClass(country) => {
    //         //         let name_temp = get_country_name(&country).unwrap_or(format!("{:?}", id));
    //         //         let name = name_temp.as_str();

    //         //         get_country_powers(&country, name, save);
    //         //         get_country_fleets(&country, name, save);
    //         //         get_country_balance(&country, name, save);
    //         //         get_country_victory_score_n_rank(&country, name, save);
    //         //     }
    //         //     _ => {}
    //         // }
    //     }
    // } else {
    //     error!("No country to parse: {:?}", gm.country.clone());
    // }
}

fn get_country_powers(country: &CountryClass, name: &str, save: &str) {
    trace!("\tSeparating country powers");
    if let Some(mv) = *country.military_power.clone() {
        STELLARIS_COUNTRY_POWER
            .with_label_values(&[save, "military".as_ref(), name])
            .set(mv.to_f64());
    }
    if let Some(tv) = *country.tech_power.clone() {
        STELLARIS_COUNTRY_POWER
            .with_label_values(&[save, "tech".as_ref(), name])
            .set(tv.to_f64());
    }
    if let Some(ev) = *country.economy_power.clone() {
        STELLARIS_COUNTRY_POWER
            .with_label_values(&[save, "economic".as_ref(), name])
            .set(ev.to_f64());
    }
}

fn get_country_fleets(country: &CountryClass, name: &str, save: &str) {
    trace!("\tSeparating country fleets");

    if let Some(fleet_manager) = *country.fleets_manager.clone() {
        match fleet_manager {
            FleetsManagerUnion::FleetsManagerClass(manager) => {
                if let Some(fleets) = *manager.owned_fleets {
                    STELLARIS_COUNTRY_FLEETS
                        .with_label_values(&[save, name])
                        .set(fleets.len() as i64);
                }
            }
            _ => {}
        }
    }
}

fn get_country_balance(country: &CountryClass, name: &str, save: &str) {
    trace!("\tSeparating country balance");
    if let Some(budget) = *country.budget.clone() {
        if let Some(Some(current)) = budget
            .income_high_water_mark
            .deref()
            .clone()
            .map(|income| *income.current)
        {
            if let VecOrMap::Map(values) = current {
                for (key, value) in values {
                    STELLARIS_COUNTRY_BALANCE
                        .with_label_values(&[save, name, key.as_str()])
                        .set(value.to_f64())
                }
            }
        } else {
            debug!("Current country {} has no current income", name);
        }
    } else {
        debug!("Current country {} has no budget", name);
    }
}

fn get_country_victory_score_n_rank(country: &CountryClass, name: &str, save: &str) {
    trace!("\tSeparating country victory rank and score");
    if let Some(rank) = *country.victory_rank.clone() {
        STELLARIS_COUNTRY_VICTORY_STATUS
            .with_label_values(&[save, name, "rank"])
            .set(rank.to_f64())
    }
    if let Some(score) = *country.victory_score.clone() {
        STELLARIS_COUNTRY_VICTORY_STATUS
            .with_label_values(&[save, name, "score"])
            .set(score.to_f64())
    }
}

fn get_country_by_id(gm: &Gamestate, id: &i64) -> Option<CountryClass> {
    if let Some(c) = *gm.country.clone() {
        match c {
            VecOrMap::Map(countries) => {
                if countries.contains_key(&id.to_string()) {
                    match countries.get(&id.to_string()).unwrap() {
                        CountryValue::CountryClass(country) => {
                            return Some(country.to_owned());
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    return None;
}

fn get_name_from_purple_name(name: &PurpleName) -> Result<Option<String>, Box<dyn Error>> {
    let mut name_parts: Vec<String> = vec![];
    if let Some(literal) = &*name.literal {
        name_parts.push(literal.to_string());
    }

    if let Some(variables) = &*name.variables {
        for variable in variables {
            match &*variable.key {
                Some(key) if key == "adjective" || key == "1" => {
                    if let Some(var_value) = &*variable.value {
                        if let Some(value_key) = &*var_value.key {
                            let name = render_name(value_key.clone().trim().replace("\"", ""))
                                .unwrap_or(value_key.to_owned());
                            name_parts.push(name)
                        }
                    }
                }
                _ => (),
            }
        }
    }

    if let Some(key) = &*name.key {
        match key.as_str() {
            "%ADJECTIVE%" | "%ADJ%" => (),
            _ => name_parts.push(key.to_string()),
        }
    }
    let mut joined: String = name_parts.join(" ");
    for prefix in NAME_PREFIX {
        joined = joined.replace(prefix, "");
    }

    Ok(Some(
        joined.split("_").into_iter().collect::<Vec<_>>().join(" "),
    ))
}

fn get_country_name(country: &CountryClass) -> Option<String> {
    if let Some(name) = *country.name.clone() {
        match get_name_from_purple_name(&name) {
            Ok(res) => {
                trace!(
                    "Country name: {:?}",
                    &res.clone().unwrap_or("none".to_owned())
                );
                return res;
            }
            Err(_) => return None,
        }
    }
    return None;
}

pub fn get_battles(gm: Gamestate, save: &str) {
    info!("Collecting battles info");
    if let Some(wr) = *gm.war.clone() {
        match wr {
            VecOrMap::Map(wars) => {
                for (key, war) in wars {
                    if let Some(battles) = war.battles {
                        for battle in battles {
                            let mut main_attacker_name = String::new();
                            let mut main_defender_name = String::new();
                            if let Some(attackers) = battle.attackers {
                                if attackers.len() > 0 {
                                    if let Some(country) =
                                        get_country_by_id(&gm, &attackers.get(0).unwrap().to_i64())
                                    {
                                        let name_temp = get_country_name(&country)
                                            .unwrap_or(format!("{:?}", key));
                                        let name = name_temp.as_str();
                                        main_attacker_name = name.to_string();
                                    }
                                }
                            }
                            if let Some(defenders) = battle.defenders {
                                if defenders.len() > 0 {
                                    if let Some(country) =
                                        get_country_by_id(&gm, &defenders.get(0).unwrap().to_i64())
                                    {
                                        let name_temp = get_country_name(&country)
                                            .unwrap_or(format!("{:?}", key));
                                        let name = name_temp.as_str();
                                        main_defender_name = name.to_string();
                                    }
                                }
                            }

                            let battle_type_temp =
                                battle.battle_type.unwrap_or("undefined".to_string());
                            let battle_type = battle_type_temp.as_str();

                            STELLARIS_COUNTRY_BATTLE_LOSSES
                                .with_label_values(&[
                                    save,
                                    main_attacker_name.as_str(),
                                    main_defender_name.as_str(),
                                    battle_type,
                                    "defender",
                                    key.as_str(),
                                ])
                                .set(battle.defender_losses.unwrap_or(Number::Int(0)).to_f64());
                            STELLARIS_COUNTRY_BATTLE_LOSSES
                                .with_label_values(&[
                                    save,
                                    main_attacker_name.as_str(),
                                    main_defender_name.as_str(),
                                    battle_type,
                                    "attacker",
                                    key.as_str(),
                                ])
                                .set(battle.attacker_losses.unwrap_or(Number::Int(0)).to_f64());
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn get_megastructures(gm: Gamestate, save: &str) {
    info!("Collecting megastructures info");
    if let Some(structures) = *gm.megastructures.clone() {
        debug!(
            "Detected {} megastructures to collect info from",
            structures.len()
        );
        // if let VecOrMap::Map(structures) = value {
        for (_, mstr) in structures.iter() {
            if let Some(tp) = mstr.megastructure_type.clone() {
                let mut curr_name = String::new();
                if let Ok(name) = render_name(tp) {
                    curr_name = name;
                }
                let owner_id = mstr.owner.clone().unwrap_or(Number::Int(-1));
                let owner_name = match get_country_by_id(&gm, &owner_id.to_i64()) {
                    Some(c) => {
                        let temp_name = String::new();
                        let res = get_country_name(&c).unwrap_or(temp_name);
                        res
                    }
                    None => mstr
                        .owner
                        .clone()
                        .unwrap_or(Number::Int(-1))
                        .to_i64()
                        .to_string(),
                };

                let same_structres_owner: HashMap<String, Megastructure> = structures
                    .iter()
                    .filter(|(_, v)| {
                        v.megastructure_type == mstr.megastructure_type && v.owner == mstr.owner
                    })
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();

                STELLARIS_MEGASTRUCTURES
                    .with_label_values(&[save, curr_name.as_str(), owner_name.as_str()])
                    .set(same_structres_owner.len() as f64);
            }
        }
    }
    // }
}
