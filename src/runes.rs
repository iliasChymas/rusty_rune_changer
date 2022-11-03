use reqwest;
use serde::{Deserialize, Serialize};
use regex;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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

        let mut output: Vec<i32> = Vec::new();
        let mut flag: bool = true;
        for rune in runes {
            if flag {
                output.push(rune);
            }
            flag = !flag;
        }
        

        println!("{:?}", output);
        
        Self {
            name: "Quandale".to_string(),
            primary_style_id: Self::get_tree(output[0]),
            sub_style_id: Self::get_tree(output[4]),
            selected_perk_ids: output,
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
