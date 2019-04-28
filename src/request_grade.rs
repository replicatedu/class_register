
use issue_database::ClassIssues;
use std::env;
use std::fs;
use std::panic;
use std::str;
use std::{thread, time};
use term_painter::Color::*;
use term_painter::ToStyle;
use toml;
use std::fs::File;
use std::io::prelude::*;
use class_crypto;
use class_crypto::serialization::{Message, Participant};
use class_crypto::ClassCrypto;
fn read_file_to_string(filepath: &str) -> String {
    let mut file = File::open(filepath).expect("key not there");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("error reading key");
    contents
}
fn parse_key_mat(repo_name: &str, coord: &ClassCrypto) -> String {
    let mut deploy_key_path = "".to_string();
    deploy_key_path.push_str(&format!("/tmp/{}/deploy_key.toml", repo_name));

    let deploy_key_enc = read_file_to_string(&deploy_key_path);

    let deploy_key = coord
        .decrypt_from_toml(&deploy_key_enc)
        .expect("error decrypting deploy");

    str::from_utf8(&deploy_key).unwrap().to_string()
}


pub fn main() {
    let args: Vec<String> = env::args().collect();

     if args.len() != 4 {
        panic!("args: --grade my_reposity_url coord_key");
    }
    let username = &env::var("GITHUB_USERNAME").expect("set the GITHUB_USERNAME env");
    let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
    let my_repo = &args[2];
    let sk = &args[3];

    //loading it to send back to the student to confirm
    let student_cryto_file_string = fs::read_to_string("my_crytpo.toml".to_owned())
        .expect(
            "error reading the instructor_keys.toml file, start daemon in an initialized repo \
             folder or move the instructor_keys.toml to your location",
        );

    let my_cryto_obj: Participant =
        toml::from_str(&student_cryto_file_string).expect("error parsing coord crypto");
    let student_crypto =
        ClassCrypto::new_from_sk("me", my_cryto_obj.sk, true).expect("error creating cryto obj");

    let coord_key =
        ClassCrypto::new_from_sk("coord", sk.to_string(), false).expect("crypto failed");
    let deploy_key = parse_key_mat(my_repo, &coord_key);
    //register.add_deploy_key(&deploy_key);

    let class_api = fs::read_to_string("api_addr".to_owned())
        .expect("Something went wrong reading the api_addr");
    dbg!(&class_api);
    //"https://api.github.com/repos/replicatedu/issue_database"
    let issue = ClassIssues::new(class_api, username.to_string(), password.to_string());
    //encrypt the repo for registry
    let enc_my_repo =
        student_crypto.encrypt_to_toml(my_repo.as_bytes().to_vec(), coord_key.return_pk());
    issue.request_grade(&enc_my_repo);

}
