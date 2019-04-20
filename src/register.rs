use std::fs;
use std::fs::OpenOptions;
use std::io;

use std::process::Command;

use std::time::{SystemTime, UNIX_EPOCH};

use git_wrapper;

//holds data for instructor and students
pub struct ClassRegister {
    repo_name: String,
    class_repo_address: String,
    username: String,
    password: String,
}

impl ClassRegister {
    pub fn new(
        repo_name: String,
        class_repo_address: String,
        username: String,
        password: String,
    ) -> ClassRegister {
        ClassRegister {
            repo_name,
            class_repo_address,
            username,
            password,
        }
    }

    pub fn create_repo(&self) {
        git_wrapper::create_repo(&self.username, &self.password, &self.repo_name, "/tmp/");
    }

    pub fn clone_repo_to_private(&self) {
        git_wrapper::clone_class_repo_to_private(
            &self.username,
            &self.password,
            &self.repo_name,
            "/tmp/",
            &self.class_repo_address,
        );
    }
    pub fn clone_repo_to_dir(&self) {
        git_wrapper::clone_repo_to_dir(&self.username, &self.password, &self.repo_name, "/tmp/");
    }

    pub fn add_deploy_key(&self, key: &str) {
        git_wrapper::add_deploy_key(
            &self.username,
            &self.password,
            &self.repo_name,
            "/tmp/",
            key,
        );
    }
    pub fn add_file(&self, filename: &str, repo_path: &str) {
        git_wrapper::pass_add_file(
            filename,
            repo_path,
            &self.username,
            &self.password,
            &self.repo_name,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_repo() {
        let gh = ClassRegister::new(
            "test_class".to_string(),
            "https://github.com/replicatedu/test_class".to_string(),
            "hortinstein".to_string(),
            "ccccc".to_string(),
        );

        gh.create_repo();
        gh.clone_repo_to_private();
    }
}
