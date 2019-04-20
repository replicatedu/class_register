use replicatedu_student::main_register;
use replicatedu_student::check_reg;
use std::env;
use std::thread;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("args: check readme");
    }
    if args[1] == "--register" || args[1] == "-r" {
        main_register();
    } else if args[1] == "--grade" || args[1] == "-g" {

    } else if args[1] == "--check_grade" || args[1] == "-e" {

    } else if args[1] == "--check_registration" || args[1] == "-e" {
        check_reg::main();
    }
}
