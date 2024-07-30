use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::consts::{SHC_CLI_FOLDER_NAME, USER_CONFIG_FILE_NAME};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserInfo {
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

impl UserConfig {
    pub fn new() -> Self {
        let shc_folder = dirs::home_dir().unwrap().join(SHC_CLI_FOLDER_NAME);
        let config_path = shc_folder.join(USER_CONFIG_FILE_NAME);
        if !shc_folder.exists() {
            std::fs::create_dir_all(&shc_folder).unwrap();
        }
        if !config_path.exists() {
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

        let contents =
            fs::read_to_string(&config_path).expect("Something went wrong reading the file");

        let user: UserInfo = toml::from_str(&contents).expect("Could not parse TOML");
        UserConfig {
            user,
            config_path: config_path.clone(),
        }
    }

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
