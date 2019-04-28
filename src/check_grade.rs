
use class_crypto::ClassCrypto;
use class_crypto;
use class_crypto::serialization::{Message, Participant};
use issue_database::ClassIssues;
use std::env;
use std::fs;
use std::panic;
use std::process;
use std::str;
use std::{thread, time};
use term_painter::Color::*;
use term_painter::ToStyle;
use toml;

pub fn main() {
    let username = &env::var("GITHUB_USERNAME").expect("set the GITHUB_USERNAME env");
    let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");

    let class_api = fs::read_to_string("api_addr".to_owned()).expect(
        "error reading the api_addr file, start daemon in an initialized repo \
         folder or move the api_addr to your location",
    );

    let my_cryto_file_string = fs::read_to_string("my_crypto.toml".to_owned()).expect(
        "error reading the my_crypto file, start daemon in an initialized repo \
         folder or move the my_crypto to your location",
    );

    let my_cryto_obj: Participant =
        toml::from_str(&my_cryto_file_string).expect("error parsing my crypto");

    let my_cryto =
        ClassCrypto::new_from_sk("me", my_cryto_obj.sk, true).expect("error creating cryto obj");

    let issue = ClassIssues::new(class_api, username.to_string(), password.to_string());

    let thirty_seconds = time::Duration::from_secs(30);
    println!("{}", Green.paint("entering grade checking loop"));
    let found = false;
    while found == false {
        let did_panic = panic::catch_unwind(|| {
            let open_regs = issue.view_grades().expect("error getting api");
            for reg in &open_regs {
                let reg_panic = panic::catch_unwind(|| {
                    //dbg!(reg);
                    let issues_d = issue.view_issues(reg);
                    for issued in issues_d {
                        dbg!(&issued[0]);
                        let student_message: Message =
                            toml::from_str(&issued[0]).expect("error reading toml");
                        let dec = my_cryto
                            .decrypt(&student_message.msg, student_message.pk)
                            .expect("error decryting");
                        dbg!(str::from_utf8(&dec));
                    }
                });
                if reg_panic.is_err() {
                    println!("{}", Red.paint("panic: invalid grade...trying next"));
                }
            }
        });
        if did_panic.is_err() {
            println!(
                "{}",
                Red.paint("panic: api url incorrect, make sure running this from student dir ")
            );
        }
        println!("{}", Green.paint("sleeping 30 seconds before next check "));

        thread::sleep(thirty_seconds);
    }
}
