use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::min;
use std::path::Path;
use std::time::Duration;
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;

use crate::api_client;
use crate::utils::zip_directory_recursive;

pub async fn upload_file(
    file_path: &Path,
    api_client: &mut api_client::ApiClient,
) -> Result<(), Box<dyn std::error::Error>> {
    if !file_path.exists() {
        println!("ShcFile or Folder does not exist");
        return Ok(());
    }

    let is_dir = file_path.is_dir();
    let file_path = if is_dir {
        let pb = ProgressBar::new_spinner();

        pb.enable_steady_tick(Duration::from_millis(200));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.dim.bold} shc: {wide_msg}")
                .unwrap()
                .tick_chars("/|\\- "),
        );

        pb.set_message("Compressing folder...");
        let zip_file_path = zip_directory_recursive(file_path, 30 * 1024 * 1024)?;
        pb.finish_and_clear();
        zip_file_path
    } else {
        file_path.to_path_buf()
    };

    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let mime_type = mime_guess::from_path(&file_path).first_or_octet_stream();
    let file = tokio::fs::File::open(&file_path)
        .await
        .expect("Cannot open input file for HTTPS read");
    let total_size = file
        .metadata()
        .await
        .expect("Cannot determine input file size for HTTPS read")
        .len();
    let client = reqwest::Client::new();

    let pb = ProgressBar::new_spinner();

    pb.enable_steady_tick(Duration::from_millis(200));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.dim.bold} shc: {wide_msg}")
            .unwrap()
            .tick_chars("/|\\- "),
    );

    pb.set_message("Preparing for upload...");

    let res = api_client
        .add_file(file_name, mime_type.as_ref(), total_size)
        .await?;
    pb.finish_and_clear();

    let file_id = res.file_id;
    let file_name = res.file_name;
    let upload_url = res.upload_url;

    let mut uploaded = 0;

    let mut reader_stream = ReaderStream::new(file);
    let bar = ProgressBar::new(total_size);
    bar.set_style(
        ProgressStyle::with_template(
            "{msg}\n{spinner:.green} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta}) {bytes_per_sec} \n",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    let res = api_client.update_upload_status(&file_id, "uploading").await;

    match res {
        Ok(_) => {}
        Err(_) => {
            // TODO: better error handling
            println!("Failed to upload file");
            std::process::exit(1);
        }
    }

    bar.reset_eta();
    bar.set_message(format!("Uploading {}", file_name));
    let async_stream = async_stream::stream! {
        while let Some(chunk) = reader_stream.next().await {
            if let Ok(chunk) = &chunk {
                let new = min(uploaded + (chunk.len() as u64), total_size);
                uploaded = new;
                bar.set_position(new);
                if uploaded >= total_size {
                    //TODO: fix this
                        bar.finish_and_clear();
                }
            }
            yield chunk;
        }
    };

    let res = client
        .put(upload_url)
        .body(reqwest::Body::wrap_stream(async_stream))
        .header("Content-Type", mime_type.as_ref())
        .header("Content-Length", total_size.to_string())
        .send()
        .await?;

    let pb = ProgressBar::new_spinner();

    pb.enable_steady_tick(Duration::from_millis(200));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.dim.bold} shc: {wide_msg}")
            .unwrap()
            .tick_chars("/|\\- "),
    );

    pb.set_message("Adding file...");

    match res.status() {
        reqwest::StatusCode::OK => {
            let res = api_client.update_upload_status(&file_id, "uploaded").await;
            pb.finish_and_clear();
            match res {
                Ok(_) => {
                    print!(
                "\n{} added successfully\nShcFile Link: https://shc.ajaysharma.dev/share/{}\n",
                file_name, file_id
            );
                }
                Err(_) => {
                    print!("Failed to add file");
                }
            }
        }
        _ => {
            let res = api_client
                .update_upload_status(file_id.as_str(), "failed")
                .await;
            pb.finish_and_clear();
            match res {
                Ok(_) => {
                    print!("Failed to upload file");
                }

                Err(_) => {
                    print!("Something went wrong!");
                }
            }
        }
    }

    // Delete the zip file if it was created by the app
    if is_dir {
        std::fs::remove_file(&file_path)?;
    }

    Ok(())
}
