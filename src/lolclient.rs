use reqwest;
use std::{path, io::{BufReader, Read} };
use base64;

use crate::runes::Runes;

pub struct Credentials {
    pub password: String,
    pub port: String
}

fn read_file(location: &str) -> String {
    if !path::Path::new(location).exists() {
        return String::from("");
    }
    let file = std::fs::File::open(location).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    return contents;
}

fn champ_id_to_name(id: &str) -> String {
    id.to_string().pop();
    for champ in read_file("champ_ids.txt").split("\n") {
        if champ.split(" ").nth(0).unwrap() == id {
            return champ.split(" ").nth(1).unwrap().trim().to_string();
        }
    }
    return "".to_string();
    
}

fn build_request() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build().unwrap()
}

impl Credentials {
    // pub fn update_creds(&mut self) {
    //     let contents: String = read_file("/home/ilias/Games/league-of-legends/drive_c/Riot Games/League of Legends/lockfile");
    //     self.port = contents.split(":").nth(2).unwrap_or_else(|| -> &str { "" }).to_string();
    //     self.password = contents.split(":").nth(3).unwrap_or_else(|| -> &str { "" }).to_string();
    // }

    pub fn new() -> Result<Credentials, String> {
        let contents: String = read_file("/home/ilias/Games/league-of-legends/drive_c/Riot Games/League of Legends/lockfile");
        if contents == "" { return Err("Empty file".to_string()); }
        Ok(Self {
            port: contents.split(":").nth(2).unwrap_or_else(|| -> &str { "" }).to_string(),
            password: contents.split(":").nth(3).unwrap_or_else(|| -> &str { "" }).to_string(),
        })
    }
    
}
pub struct LolClient {
    pub creds: Credentials,
}

impl LolClient {
    pub fn new() -> Result<LolClient, String> {        
        let temp = Credentials::new()?;
        Ok(Self {
            creds: temp,
        })
    }

    pub fn change_runes(&self) -> Result<(), String> {
        let body = build_request()
            .get(format!("https://127.0.0.1:{}/lol-perks/v1/currentpage", self.creds.port))
            .header("Authorization", format!("Basic {}", base64::encode(format!("riot:{}", self.creds.password))))
            .send().unwrap()
            .text().unwrap();
        let json_body: serde_json::Value = serde_json::from_str(&body).expect("Not well formated json");
        let rune_page_id = match json_body.get("id") {
            Some(t) => { t.to_string() },
            None => { panic!("Wrong id") },
        };

        let current_champ = build_request()
            .get(format!("https://127.0.0.1:{}/lol-champ-select/v1/current-champion", self.creds.port))
            .header("Authorization", format!("Basic {}", base64::encode(format!("riot:{}", self.creds.password))))
            .send()
            .unwrap()
            .text()
            .unwrap();
        let current_champ_name = champ_id_to_name(&current_champ);
        
        if current_champ_name.is_empty() {
            return Err("Could figure out what champion you playing".to_string());
        }

        println!("You are currently playing: {}", current_champ_name);

        
        build_request()
            .delete(format!("https://127.0.0.1:{}/lol-perks/v1/pages/{}", self.creds.port, rune_page_id))
            .header("Authorization", format!("Basic {}", base64::encode(format!("riot:{}", self.creds.password))))
            .send()
            .unwrap();

        let runes: Runes = Runes::new(current_champ_name);
        let body = serde_json::to_string(&runes).unwrap();
        build_request()
            .post(format!("https://127.0.0.1:{}/lol-perks/v1/pages", self.creds.port))
            .header("Authorization", format!("Basic {}", base64::encode(format!("riot:{}", self.creds.password))))
            .body(body)
            .send()
            .unwrap();
        Ok(())

    }
}
