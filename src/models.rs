use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShcFile {
    pub name: String,
    pub id: String,
    pub extension: String,
    pub mime_type: String,
    pub size: u64,
    pub is_public: bool,
    pub updated_at: String,
    pub user_id: String,
    pub download_url: Option<String>,
    pub upload_status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShcFileResponse {
    pub results: Vec<ShcFile>,
    pub total_results: u64,
    pub total_pages: u64,
    pub current_page: u64,
    pub previous_page: Option<u64>,
    pub next_page: Option<u64>,
    pub per_page: u64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AddFileResponse {
    pub upload_url: String,
    pub file_id: String,
    pub file_name: String,
    pub is_public: bool,
}
