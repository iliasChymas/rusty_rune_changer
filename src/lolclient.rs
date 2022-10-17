use reqwest;
use std::{fs,path, io::{BufReader, Read}, thread::Builder};
use base64;

pub struct Credentials {
    pub password: String,
    pub port: String
}

fn build_request() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build().unwrap()
}



impl Credentials {
    pub fn read_lockfile() -> String {
        let location: &str = "/home/ilias/Games/league-of-legends/drive_c/Riot Games/League of Legends/lockfile";
        if !path::Path::new(location).exists() {
            return "".to_string();
        }
        let file = std::fs::File::open(location).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        return contents;
    }

    pub fn update_creds(&mut self) {
        let contents: String = Self::read_lockfile();
        self.port = contents.split(":").nth(2).unwrap_or_else(|| -> &str { "" }).to_string();
        self.password = contents.split(":").nth(3).unwrap_or_else(|| -> &str { "" }).to_string();
    }

    pub fn new() -> Credentials {
        let contents: String = Self::read_lockfile();
        Self {
            port: contents.split(":").nth(2).unwrap_or_else(|| -> &str { "" }).to_string(),
            password: contents.split(":").nth(3).unwrap_or_else(|| -> &str { "" }).to_string(),
        }
    }
    
}
pub struct LolClient {
    pub creds: Credentials,
}

impl LolClient {
    pub fn new() -> LolClient {        
        Self {
            creds: Credentials::new(),
        }
    }

    pub fn get_current_summoner(& self) {
        let body = build_request()
            .get(format!("https://127.0.0.1:{}/lol-summoner/v1/current-summoner", self.creds.port))
            .header("Authorization", format!("Basic {}", base64::encode(format!("riot:{}", self.creds.password))))
            .send().unwrap()
            .text().unwrap();
        println!("{}", body);
    }



    pub fn change_runes(&self) {
        let body = build_request()
            .get(format!("https://127.0.0.1:{}/lol-perks/v1/currentpage", self.creds.port))
            .header("Authorization", format!("Basic {}", base64::encode(format!("riot:{}", self.creds.password))))
            .send().unwrap()
            .text().unwrap();
        println!("{}", body);

    }
}
