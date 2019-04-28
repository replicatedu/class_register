use class_crypto;
use class_crypto::serialization::{Message, Participant};
use class_crypto::ClassCrypto;
use issue_database::ClassIssues;
use std::env;
use std::fs;
use std::panic;
use std::str;
use std::{thread, time};
use term_painter::Color::*;
use term_painter::ToStyle;
use toml;

pub fn main() {
    let args: Vec<String> = env::args().collect();

     if args.len() != 4 {
        panic!("args: --grade my_reposity_url coord_key");
    }
    let username = &env::var("GITHUB_USERNAME").expect("set the GITHUB_USERNAME env");
    let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
    let my_repo = &args[2];
    let sk = &args[4];
    let class_api = fs::read_to_string(&(my_repo.clone() + &"/api_addr".to_owned()))
        .expect("Something went wrong reading the api_addr");
    dbg!(&class_api);
    //"https://api.github.com/repos/replicatedu/issue_database"
    let issue = ClassIssues::new(class_api, username.to_string(), password.to_string());



}
