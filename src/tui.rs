use chrono::DateTime;
use dialoguer::{theme, Select};

use crate::consts::MAX_NAME_WIDTH_LENGTH;
use crate::models::ShcFile;
use crate::utils::format_bytes;

pub fn shc_file_input(files: &[ShcFile], prompt: &str) -> usize {
    let size_width = 10;
    let updated_at_width = 20;
    let visibility_width = 10;
    let date_formatter = timeago::Formatter::new();

    let files =
        files
            .iter()
            .map(|file| -> Result<String, Box<dyn std::error::Error>> {
                let mut name = file.name.clone();
                println!("{} {}", name, name.len());
                if name.len() > MAX_NAME_WIDTH_LENGTH {
                    name.truncate(name.len() - 5);
                    name.push_str("...");
                }

                let time_ago = date_formatter.convert_chrono(
                    DateTime::parse_from_rfc3339(file.updated_at.as_str())?,
                    chrono::Utc::now(),
                );
                let size = format_bytes(file.size);
                let visibility = if file.is_public {
                    "Public".to_string()
                } else {
                    "Private".to_string()
                };
                Ok(format!(
                "{:<name_width$}\t\t{:<size_width$}\t{:<updated_at_width$}\t{:<visibility_width$}",
                name, size, time_ago, visibility,
                name_width = name.len(),
                size_width = size_width,
                updated_at_width = updated_at_width,
                visibility_width = visibility_width
            ))
            })
            .collect::<Result<Vec<String>, Box<dyn std::error::Error>>>();

    let files = match files {
        Ok(items) => items,
        Err(_) => vec![],
    };

    let selection = Select::with_theme(&theme::ColorfulTheme::default())
        .max_length(20)
        .with_prompt(prompt)
        .default(0)
        .items(&files)
        .interact()
        .unwrap();

    selection
}
