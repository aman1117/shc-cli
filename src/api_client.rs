use async_recursion::async_recursion;
use serde_json::json;
use std::io::{Error, ErrorKind};

use crate::consts::SHC_BACKEND_API_BASE_URL;
use crate::models::{AddFileResponse, RefreshTokenResponse, ShcFile, ShcFileResponse};
use crate::user_config::UserConfig;

pub struct ApiClient {
    api_base_url: String,
    tried_refreshing_token: bool,
    user_config: UserConfig,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(user_config: UserConfig) -> ApiClient {
        ApiClient {
            api_base_url: SHC_BACKEND_API_BASE_URL.to_string(),
            tried_refreshing_token: false,
            user_config,
            client: reqwest::Client::new(),
        }
    }

    pub fn login_again(&mut self) {
        self.tried_refreshing_token = true;
        println!("Logged out, please login again");
        // TODO: logout - clear config
        // TODO: run login command (can we continue after login command?)
        std::process::exit(1);
    }

    async fn refresh_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.tried_refreshing_token {
            self.user_config.clear();
            self.login_again();
        }

        let res = self
            .client
            .get(format!("{}/auth/refresh-token", self.api_base_url))
            .header(
                "Authorization",
                self.user_config.user.refresh_token.as_ref().unwrap(),
            )
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => match res.json::<RefreshTokenResponse>().await {
                Ok(res) => {
                    self.user_config.user.email = Some(res.user.email);
                    self.user_config.user.name = Some(res.user.name);
                    self.user_config.user.user_id = Some(res.user.id);
                    self.user_config.user.access_token = Some(res.access_token);
                    self.user_config.user.refresh_token = Some(res.refresh_token);
                    self.user_config.save();
                }
                Err(e) => {
                    return Err(e.into());
                }
            },
            _ => {
                self.login_again();
            }
        }
        Ok(())
    }

    #[async_recursion]
    pub async fn list_files(
        &mut self,
        search: &str,
    ) -> Result<ShcFileResponse, Box<dyn std::error::Error>> {
        let access_token = self.user_config.user.access_token.as_ref().unwrap();

        let res = self
            .client
            .get(format!(
                "{}/api/files?search={}&page=1&limit=100",
                self.api_base_url, search
            ))
            .header("Authorization", access_token)
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let res = res.json::<ShcFileResponse>().await?;
                Ok(res)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                self.refresh_token().await?;
                return self.list_files(search).await;
            }
            _ => {
                // TODO: use server error message
                Err(Error::new(ErrorKind::Other, "Something went wrong").into())
            }
        }
    }

    #[async_recursion]
    pub async fn remove_file(&mut self, file_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let access_token = self.user_config.user.access_token.as_ref().unwrap();

        let res = self
            .client
            .delete(format!(
                "{}/api/files/remove/{}",
                self.api_base_url, file_id
            ))
            .header("Authorization", access_token)
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::UNAUTHORIZED => {
                self.refresh_token().await?;
                return self.remove_file(file_id).await;
            }
            _ => Err(Error::new(ErrorKind::Other, "Something went wrong").into()),
        }
    }

    #[async_recursion]
    pub async fn toggle_file_visibility(
        &mut self,
        file_id: &str,
    ) -> Result<ShcFile, Box<dyn std::error::Error>> {
        let access_token = self.user_config.user.access_token.as_ref().unwrap();

        let res = self
            .client
            .patch(format!(
                "{}/api/files/toggle-visibility/{}",
                self.api_base_url, file_id
            ))
            .header("Authorization", access_token)
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let res = res.json::<ShcFile>().await?;
                Ok(res)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                self.refresh_token().await?;
                return self.toggle_file_visibility(file_id).await;
            }
            _ => Err(Error::new(ErrorKind::Other, "Something went wrong").into()),
        }
    }

    #[async_recursion]
    pub async fn rename_file(
        &mut self,
        file_id: &str,
        new_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let access_token = self.user_config.user.access_token.as_ref().unwrap();

        let res = self
            .client
            .patch(format!(
                "{}/api/files/rename/{}",
                self.api_base_url, file_id
            ))
            .header("Authorization", access_token)
            .json(&json!({
                "name": new_name,
            }))
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::UNAUTHORIZED => {
                self.refresh_token().await?;
                return self.rename_file(file_id, new_name).await;
            }
            _ => Err(Error::new(ErrorKind::Other, "Something went wrong").into()),
        }
    }

    #[async_recursion]
    pub async fn add_file(
        &mut self,
        file_name: &str,
        mime_type: &str,
        file_size: u64,
    ) -> Result<AddFileResponse, Box<dyn std::error::Error>> {
        let access_token = self.user_config.user.access_token.as_ref().unwrap();

        let res = self
            .client
            .post(format!("{}/api/files/add", self.api_base_url))
            .header("Authorization", access_token)
            .json(&json!(
                {
                    "file_name": file_name,
                    "mime_type": mime_type,
                    "file_size": file_size,
                }
            ))
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let res = res.json::<AddFileResponse>().await?;
                Ok(res)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                self.refresh_token().await?;
                return self.add_file(file_name, mime_type, file_size).await;
            }
            _ => Err(Error::new(ErrorKind::Other, "Something went wrong").into()),
        }
    }

    #[async_recursion]
    pub async fn update_upload_status(
        &mut self,
        file_id: &str,
        upload_status: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let access_token = self.user_config.user.access_token.as_ref().unwrap();

        let res = self
            .client
            .patch(format!(
                "{}/api/files/update-upload-status/{}",
                self.api_base_url, file_id
            ))
            .json(&json!(
                {
                    "upload_status": upload_status,
                }
            ))
            .header("Authorization", access_token)
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::UNAUTHORIZED => {
                self.refresh_token().await?;
                return self.update_upload_status(file_id, upload_status).await;
            }
            _ => Err(Error::new(ErrorKind::Other, "Something went wrong").into()),
        }
    }

    #[async_recursion]
    pub async fn get_file_download_url(
        &mut self,
        file_id: &str,
    ) -> Result<ShcFile, Box<dyn std::error::Error>> {
        let access_token = self.user_config.user.access_token.as_ref().unwrap();

        let res = self
            .client
            .get(format!("{}/api/files/{}", self.api_base_url, file_id))
            .header("Authorization", access_token)
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let res = res.json::<ShcFile>().await?;
                Ok(res)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                self.refresh_token().await?;
                return self.get_file_download_url(file_id).await;
            }
            _ => Err(Error::new(ErrorKind::Other, "Something went wrong").into()),
        }
    }

    #[async_recursion]
    pub async fn increment_download_count(
        &mut self,
        file_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let access_token = self.user_config.user.access_token.as_ref().unwrap();

        let res = self
            .client
            .patch(format!(
                "{}/api/files/increment-download-count/{}",
                self.api_base_url, file_id
            ))
            .header("Authorization", access_token)
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::UNAUTHORIZED => {
                self.refresh_token().await?;
                return self.increment_download_count(file_id).await;
            }
            _ => Err(Error::new(ErrorKind::Other, "Something went wrong").into()),
        }
    }
}
