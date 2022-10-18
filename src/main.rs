mod lolclient;
mod runes;

use crate::lolclient::LolClient;
fn main() {
    let client = match LolClient::new() {
        Ok(t) => { t },
        Err(_) => {
            println!("YOu need to initialize the client first !");
            std::process::exit(1)
        }
    };
    match client.change_runes() {
        Ok(_) => { println!("Runes changed")},
        Err(msg) => { println!("{}", msg) },
    };
}
