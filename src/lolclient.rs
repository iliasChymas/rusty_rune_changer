use reqwest;
use sysinfo::{ProcessExt, System, SystemExt};
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
    pub fn new() -> Result<Credentials, String> {
        let mut sys = System::new_all();
        sys.refresh_all();
        let mut t_port = "";
        let mut t_pass = "";
        for (_pid, process) in sys.processes() {
            if process.name().contains("LeagueClient.ex") {
                println!("Lol process is: {}", process.name());
                let cmd = process.cmd();
                t_port = cmd[2].split("=").nth(1).unwrap();
                t_pass = cmd[1].split("=").nth(1).unwrap();
                break;
            }
        }
        if t_port == "" && t_pass == "" {
            return Err("You need to have league client opened!".to_string());
        }
        Ok(Self {
            port: t_port.to_string(),
            password: t_pass.to_string()
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
