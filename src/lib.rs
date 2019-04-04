use std::fs;
use std::fs::OpenOptions;
use std::io;

use std::process::{Command};

use std::time::{SystemTime, UNIX_EPOCH};

//returns a command setup ready to run the tests
fn command_wrapper(test_command: &str, command_directory: &str) -> Command {
    let mut command = if cfg!(target_os = "windows") {
        {
            let mut c = Command::new("cmd");
            c.args(&["/C", test_command]);
            c
        }
    } else {
        {
            let mut c = Command::new("sh");
            c.arg("-c");
            c.arg(test_command);
            c
        }
    };
    command.current_dir(command_directory);
    command
}



//holds data for instructor and students
pub struct ClassRegister {
	repo_name: String,
    class_repo_address: String,
    username: String,
    password: String
}

impl ClassRegister {
    pub fn new( repo_name: String,
                class_repo_address: String, 
                username: String, 
                password: String) -> ClassRegister {
        ClassRegister {
            repo_name,
            class_repo_address,
            username,
            password
        }
    }

    pub fn create_repo( &self){
        //curl --url url -K- <<< "--user user:password"
        let mut command = String::new();
        command.push_str("curl --url https://api.github.com/user/repos ");
        command.push_str(&format!("-d '{{\"name\":\"{}\",\"private\":true}}' ", 
                         &self.repo_name));
        command.push_str(&format!("--user \"{}:{}\"", &self.username,&self.password));
        println!("{}", command);

        let mut c = command_wrapper(&command, "/tmp/");
        println!("{}",String::from_utf8_lossy(&c.output().unwrap().stdout));
    }

    // git clone --bare https://github.com/exampleuser/public-repo.git
    // cd public-repo.git
    // git push --mirror https://github.com/yourname/private-repo.git
    // cd ..
    // rm -rf public-repo.git
    pub fn clone_repo_to_private( &self){
        let mut command = String::new();
        command.push_str(&format!("rm -rf {}.git && ", &self.repo_name));
        command.push_str(&format!("git clone --bare {} && ",&self.class_repo_address));
        command.push_str(&format!("cd {}.git && ", &self.repo_name));
        command.push_str(&format!("git push --mirror https://{}:{}@github.com/{}/{}.git && ", 
                         &self.username,
                         &self.password,
                         &self.username,
                         &self.repo_name));
        command.push_str("cd .. && ");
        command.push_str(&format!("rm -rf {}.git", &self.repo_name));
        println!("{}", command);

        let mut c = command_wrapper(&command, "/tmp/");
        let output = &c.output().unwrap();
        println!("{}",String::from_utf8_lossy(&output.stdout));
        println!("{}",String::from_utf8_lossy(&output.stderr));
    }

    //update from old repo
    // cd private-repo
    // git remote add public https://github.com/exampleuser/public-repo.git
    // git pull public master # Creates a merge commit
    // git push origin master

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_repo() {

        let gh = ClassRegister::new("test_class".to_string(),
                                    "https://github.com/replicatedu/test_class".to_string(),
                                    "hortinstein".to_string(),
                                    "ccccc".to_string());

        gh.create_repo();
        gh.clone_repo_to_private();
    }
}