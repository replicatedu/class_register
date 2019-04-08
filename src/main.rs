use std::str;

use std::fs::File;
use std::fs::OpenOptions;

use std::io::prelude::*;
use std::env;

use class_crypto::ClassCrypto;
use class_crypto::serialization::Participant;
use class_register::ClassRegister;

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
    let mut database_pub_path = "".to_string();
    database_pub_path.push_str(&format!("/tmp/{}/database_keys.toml",repo_name));
    let mut database_priv_path = "".to_string();
    database_priv_path.push_str(&format!("/tmp/{}/database_keys_priv.toml",repo_name));
    let mut deploy_key_path = "".to_string();
    deploy_key_path.push_str(&format!("/tmp/{}/deploy_key.toml",repo_name));

    let deploy_key_enc = read_file_to_string(&deploy_key_path);
    let database_priv_key_enc = read_file_to_string(&database_priv_path);
    let database_pub_key_enc = read_file_to_string(&deploy_key_path);

    let deploy_key = coord.decrypt_from_toml(&deploy_key_enc)
                          .expect("error decrypting deploy");
    let database_priv_key = coord.decrypt_from_toml(&database_priv_key_enc)
                                 .expect("error decrypting deploy");
    let database_pub_key = coord.decrypt_from_toml(&database_pub_key_enc)
                                .expect("error decrypting deploy");

    write_file("/tmp/id_rsa", str::from_utf8(&database_priv_key).unwrap());
    write_file("/tmp/id_rsa.pub",str::from_utf8(&database_pub_key).unwrap());
    
    str::from_utf8(&deploy_key).unwrap().to_string()
   
}

fn main (){
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("args: reposity_url public_key");
    }
    let class_repo = &args[1];
    let sk = &args[2];
    let repo_name = "student_test_class";
    let username = "hortinstein";
    let path = "/tmp/";
    let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
    let mut repo_path: String = path.to_owned();
    repo_path.push_str(&format!("/{}",repo_name));

    let register = ClassRegister::new(repo_name.to_string(),
                                      class_repo.to_string(),
                                      username.to_string(),
                                      password.to_string());

    register.create_repo();
    register.clone_repo_to_private();
    register.clone_repo_to_dir();
    let coord_key = ClassCrypto::new_from_sk("coord", sk.to_string(), false).expect("crypto failed");
     
    let deploy_key = parse_key_mat(repo_name,&coord_key);
    register.add_deploy_key(&deploy_key);
}