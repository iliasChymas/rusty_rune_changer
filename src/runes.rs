use reqwest;
use serde::{Deserialize, Serialize};
use regex;

#[derive(Serialize, Deserialize, Debug)]
pub struct Runes {
    name: String,
    primary_style_id: i32,
    sub_style_id: i32,
    selected_perk_ids: Vec<i32>,
    current: bool
}

impl Runes {
    pub fn new(champion_name: String) -> Runes {
        let mut runes: Vec<i32> = Vec::new();
        let body: String = reqwest::blocking::get(format!("https://champion.gg/champion/{}", champion_name)).unwrap().text().unwrap();
        let urls_regex = regex::Regex::new(r"/active/\d{4}.png").unwrap();
        let mut result: i32;
        
        for capture in urls_regex.captures_iter(&body) {
            match capture.get(0) {
                Some(m) => { result = m.as_str()[8..12].parse().unwrap() },
                None => { result = -1 },
            }
            runes.push(result);
        }
        
        runes.dedup();
        runes.push(5003);

        println!("{:?}", runes);
        
        Self {
            name: "Quandale".to_string(),
            primary_style_id: Self::get_tree(runes[0]),
            sub_style_id: Self::get_tree(runes[4]),
            selected_perk_ids: runes,
            current: true,
            
        }
            
    }

    fn get_tree(rune: i32) -> i32 {
        let temp = rune.to_string();
        let output = match &temp[0..2] {
            "81" => return 8100,
            "83" => return 8300,
            "80" => return 8000,
            "84" => return 8400,
            "82" => return 8200,
            &_ => -1

        };
        output
    }
}