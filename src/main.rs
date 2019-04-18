use std::str;

use std::fs::File;
use std::fs::OpenOptions;

use std::io::prelude::*;
use std::env;

use class_crypto::ClassCrypto;
use class_crypto::serialization::Participant;
use class_crypto::participant_to_str;

use class_crypto::convert_me_to_serializable;
use class_register::ClassRegister;


use term_painter::Color::*;
use term_painter::ToStyle;
use gag::Gag;

#[macro_use]
extern crate serde_derive;
extern crate toml;

fn read_file_to_string(filepath:&str) -> String {
    let mut file = File::open(filepath).expect("key not there");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("error reading key");
    contents
}

fn write_file(filepath: &str, contents: &str) {
    match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filepath)
    {
        Ok(ref mut file) => {
            file.set_len(0);
            writeln!(file, "{}",contents).unwrap();
        }
        Err(err) => {
            panic!("Failed to open log file: {}", err);
        }
    }
}

fn parse_key_mat(repo_name: &str, coord:&ClassCrypto) -> String{
    let mut deploy_key_path = "".to_string();
    deploy_key_path.push_str(&format!("/tmp/{}/deploy_key.toml",repo_name));

    let deploy_key_enc = read_file_to_string(&deploy_key_path);
    
    let deploy_key = coord.decrypt_from_toml(&deploy_key_enc)
                          .expect("error decrypting deploy");
    
    str::from_utf8(&deploy_key).unwrap().to_string()
   
}

fn main (){
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        panic!("args: reposity_url repo_name public_key");
    }
    let class_repo = &args[1];
    let repo_name = &args[2];
    let sk = &args[3];

    let path = "/tmp/";


    let username = &env::var("GITHUB_USERNAME").expect("set the GITHUB_USERNAME env");
    let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
    let mut repo_path: String = path.to_owned();
    repo_path.push_str(&format!("/{}",repo_name));

    let register = ClassRegister::new(repo_name.to_string(),
                                      class_repo.to_string(),
                                      username.to_string(),
                                      password.to_string());

    println!("{}", Yellow.paint("creating student crypto: "));
    let mut print_gag = Gag::stdout().unwrap();
    
    let student_crypto = ClassCrypto::new(username, false);
    let studend_crypto_toml = participant_to_str( convert_me_to_serializable(&student_crypto));
        
    drop(print_gag);
    println!("{}", Green.paint("\tdone"));


    println!("{}", Yellow.paint("creating student private repo: "));
    let mut print_gag = Gag::stdout().unwrap();
    
    register.create_repo();
    register.clone_repo_to_private();
    register.clone_repo_to_dir();
        
    drop(print_gag);
    println!("{}", Green.paint("\tdone"));


    println!("{}", Yellow.paint("adding crypto to repo: "));
    let mut print_gag = Gag::stdout().unwrap();
    
    write_file(&(repo_path.clone()+&"/my_crypto.toml".to_owned()), &studend_crypto_toml);    
    register.add_file("my_crypto.toml", &repo_path);
        
    drop(print_gag);
    println!("{}", Green.paint("\tdone"));


    println!("{}", Yellow.paint("adding deploy key to repo: "));
    let mut print_gag = Gag::stdout().unwrap();    

    let coord_key = ClassCrypto::new_from_sk("coord", sk.to_string(), false).expect("crypto failed");     
    let deploy_key = parse_key_mat(repo_name,&coord_key);
    register.add_deploy_key(&deploy_key);

    drop(print_gag);
    println!("{}", Green.paint("\tdone"));

}