use chrono::{DateTime, Utc};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

use crate::api_client;
use crate::tui::shc_file_input;

pub async fn list_files(
    search: &str,
    api_client: &mut api_client::ApiClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let pb = ProgressBar::new_spinner();

    pb.enable_steady_tick(Duration::from_millis(200));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.dim.bold} shc: {wide_msg}")
            .unwrap()
            .tick_chars("/|\\- "),
    );

    pb.set_message("Fetching files...");

    let res = api_client.list_files(search).await?;
    pb.finish_and_clear();

    if res.results.is_empty() {
        println!("No files found.");
        return Ok(());
    }

    let file_count = res.results.len();
    let prompt = if file_count > 100 {
        "Select a file to see more info. (Last 100 files, use filter to get more specific results)"
            .to_string()
    } else {
        format!("Select a file to see more info.  ({} files)", file_count)
    };

    let selection = shc_file_input(&res.results, &prompt);

    let file = &res.results[selection];
    let file_name = &file.name;
    let upload_status = &file.upload_status;
    let updated_at = DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&file.updated_at)?)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    let size = if file.size < 1024 {
        format!("{:.3} KB", file.size as f64 / 1024.0)
    } else {
        format!("{:.3} MB", file.size as f64 / 1024.0 / 1024.0)
    };
    let visibility = if file.is_public { "Public" } else { "Private" };
    let shareable_link = format!("https://shc.ajaysharma.dev/share/{}", file.id);

    console::Term::stdout()
        .write_line( format!(
        "\nFile Name: {}\nUpload Status: {}\nUpdated At: {}\nSize: {}\nVisibility: {}\nShareable Link: {}",
        style(file_name).cyan(),
        style(upload_status).yellow(),
        style(updated_at).green(),
        style(size).magenta(),
        style(visibility).blue(),
        style(shareable_link).underlined().bright().blue()
    ).as_ref())?;

    Ok(())
}
