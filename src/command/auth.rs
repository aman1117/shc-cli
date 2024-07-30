use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

use crate::consts;
use crate::user_config::UserConfig;

#[derive(Deserialize, Serialize, Clone)]
struct OtpResponse {
    access_token: String,
    refresh_token: String,
    email: String,
    name: String,
    id: String,
}

pub async fn login(user_config: &mut UserConfig) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let name = dialoguer::Input::<String>::new()
        .with_prompt("Name")
        .interact_text()
        .unwrap();

    let email = dialoguer::Input::<String>::new()
        .with_prompt("Email")
        .interact_text()
        .unwrap();

    let pb = ProgressBar::new_spinner();

    pb.enable_steady_tick(Duration::from_millis(200));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.dim.bold} shc: {wide_msg}")
            .unwrap()
            .tick_chars("/|\\- "),
    );
    pb.set_message("Sending OTP...");
    let _ = client
        .post(format!("{}/auth/otp", consts::SHC_BACKEND_API_BASE_URL))
        .json(&json!({
            "name": name,
            "email": email
        }))
        .send()
        .await?;

    pb.finish_and_clear();

    let otp = dialoguer::Input::<String>::new()
        .with_prompt("Check your mail for OTP, Enter")
        .interact_text()
        .unwrap();

    let pb = ProgressBar::new_spinner();

    pb.enable_steady_tick(Duration::from_millis(200));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.dim.bold} shc: {wide_msg}")
            .unwrap()
            .tick_chars("/|\\- "),
    );
    pb.set_message("Verifying OTP...");

    let res = client
        .post(format!("{}/auth/login", consts::SHC_BACKEND_API_BASE_URL))
        .json(&json!(
            {
                "name": name,
                "otp": otp,
                "email": email
            }
        ))
        .send()
        .await?;

    pb.finish_and_clear();
    if res.status().is_success() {
        println!("Login Successfull");
        let res: OtpResponse = res.json().await?;
        user_config.user.email = Some(res.email);
        user_config.user.name = Some(res.name);
        user_config.user.user_id = Some(res.id);
        user_config.user.access_token = Some(res.access_token);
        user_config.user.refresh_token = Some(res.refresh_token);
        user_config.save();
    } else {
        println!("Login Failed");
    }
    Ok(())
}

pub async fn check_for_api_key(
    user_config: &mut UserConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    match user_config.user.access_token.as_ref() {
        Some(_) => {}
        None => {
            println!("Please login first");
            login(user_config).await?;
        }
    }
    Ok(())
}

pub fn logout(user_config: &mut UserConfig) {
    user_config.clear();
    println!("Logged out");
}
