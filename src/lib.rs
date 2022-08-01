use regex::Regex;
use reqwest::Client;
use std::process::Command;
use std::fs::File;
use std::io::Write;
use indicatif::{ProgressBar,ProgressStyle};
use std::cmp::min;
use futures_util::StreamExt;

use crate::error::UpdaterError;

mod error;

pub type Result<T> = std::result::Result<T, UpdaterError>;

const NIGHTLY_URL: &str = "https://github.com/neovim/neovim/releases/tag/nightly";
const DOWNLOAD_URL: &str = "https://github.com/neovim/neovim/releases/download/nightly/nvim.appimage";

pub async fn download(client: &Client, path: &str) -> Result<()> {
    let response = match client.get(DOWNLOAD_URL).send().await {
        Ok(response) => response,
        Err(e) => return Err(UpdaterError::Http(e))
    };
    let total_size = match response.content_length() {
        Some(length) => length,
        None => return Err(UpdaterError::Base)
    };

    // Setup progress bar
    let pb = ProgressBar::new(total_size);
    let style = "{msg}\n[{elapsed_precise}] [{bar:.green}] {bytes}/{total_bytes} [ETA: {eta}] [speed: {bytes_per_sec}]";
    pb.set_style(ProgressStyle::default_bar().template(style).unwrap());
    pb.set_message(format!("Downloading {}", DOWNLOAD_URL));

    // Setup output file
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(_) => return Err(UpdaterError::Base)
    };
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    // Write to fs
    while let Some(item) = stream.next().await {
        let chunk = match item {
            Ok(c) => c,
            Err(_) => break
        };
        match file.write_all(&chunk) {
            Ok(()) => (),
            Err(_) => break
        };
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    if downloaded != total_size {
        return Err(UpdaterError::Base);
    }

    pb.finish();
    Ok(())
}

pub async fn fetch_latest_version(client: &Client) -> Result<String> {
    let response = match client.get(NIGHTLY_URL).send().await {
        Ok(response) => response,
        Err(e) => return Err(UpdaterError::Http(e))
    };

    let text = match response.text().await {
        Ok(text) => text,
        Err(e) => return Err(UpdaterError::Http(e))
    };

    Ok(text)
}

pub async fn fetch_current_version() -> Result<String> {
    let output = match Command::new("nvim").arg("--version").output() {
        Ok(out) => out,
        Err(_) => return Err(UpdaterError::Base)
    };

    match String::from_utf8(output.stdout) {
        Ok(s) => Ok(s),
        Err(e) => Err(UpdaterError::StringErr(e))
    }
}

pub fn get_version(content: String) -> Option<String> {
    let re = match Regex::new(r"NVIM v.*-[a-z](?P<Commit>\w{9})") {
        Ok(regex) => regex,
        Err(_) => return None
    };
    let captures = match re.captures(&content) {
        Some(capts) => capts,
        None => return None
    };
    let commit = match captures.name("Commit") {
        Some(v) => v,
        None => return None
    };

    return Some(commit.as_str().to_string())
}

#[cfg(test)]
mod test {
    use std::{fs, env};

    use super::*;

    #[test]
    fn test_get_local_version() {
        let root = env::var("CARGO_MANIFEST_DIR").unwrap();
        let content = fs::read_to_string(root + "/samples/local.txt").unwrap();
        let version = get_version(content);
        assert_ne!(version, None);
        assert_eq!(version.unwrap(), "8952def50");
    }

    #[test]
    fn test_get_remote_version() {
        let root = env::var("CARGO_MANIFEST_DIR").unwrap();
        let content = fs::read_to_string(root + "/samples/remote.html").unwrap();
        let version = get_version(content);
        assert_ne!(version, None);
        assert_eq!(version.unwrap(), "8952def50");
    }
}
