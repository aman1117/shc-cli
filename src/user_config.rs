use serde::{Deserialize, Serialize};

// what is meaning of PathBuf?
use std::{fs, path::PathBuf};

// when do we have to write crate?
use crate::consts::{SHC_CLI_FOLDER_NAME, USER_CONFIG_FILE_NAME};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserInfo {
    // what is meaning of Option<String>?
    pub email: Option<String>,
    pub name: Option<String>,
    pub user_id: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]

pub struct UserConfig {
    pub user: UserInfo,
    pub config_path: PathBuf,
}

// what is impl?
impl UserConfig {
    // what is meaning of Self is it similar to self in python?
    pub fn new() -> Self {
        // what is meaning of unwrap and join?
        // what does below code do?
        let shc_folder = dirs::home_dir().unwrap().join(SHC_CLI_FOLDER_NAME);

        // what is meaning of join?
        // shc_folder is immutable so how can we call join on it?
        let config_path = shc_folder.join(USER_CONFIG_FILE_NAME);

        // what is meaning of exists?
        if !shc_folder.exists() {
            // what is meaning of create_dir_all?
            std::fs::create_dir_all(&shc_folder).unwrap();
        }
        if !config_path.exists() {
            // are we intializing user_config here?
            let user_config = UserConfig {
                user: UserInfo {
                    email: None,
                    name: None,
                    user_id: None,
                    access_token: None,
                    refresh_token: None,
                },
                config_path: config_path.clone(),
            };
            user_config.save();
            return user_config;
        }

        // what is meaning of read_to_string? and what does below code do?
        let contents =
            fs::read_to_string(&config_path).expect("Something went wrong reading the file");

        // what is meaning of toml & from_str & expect?
        let user: UserInfo = toml::from_str(&contents).expect("Could not parse TOML");

        UserConfig {
            user,
            config_path: config_path.clone(),
        }
    }

    // what is meaning of &self? and what does below code do?
    pub fn save(&self) {
        let toml = toml::to_string(&self.user).unwrap();
        fs::write(&self.config_path, toml).unwrap();
    }

    // FIXME: not working
    pub fn clear(&mut self) {
        self.user = UserInfo {
            email: None,
            name: None,
            user_id: None,
            access_token: None,
            refresh_token: None,
        };
        self.save();
    }
}
