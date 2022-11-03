mod lolclient;
mod runes;

use crate::lolclient::LolClient;
fn main() {
    let client = match LolClient::new() {
        Ok(t) => { t },
        Err(_) => {
            println!("You need to open LoL client first !");
            std::process::exit(1)
        }
    };
    match client.change_runes() {
        Ok(_) => { println!("Runes changed")},
        Err(msg) => { println!("{}", msg) },
    };
}
