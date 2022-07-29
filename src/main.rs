use clap::Parser;
use regex::Regex;
use tracing::{info,instrument,error};
use tracing_subscriber::FmtSubscriber;
use std::{process::{exit,Command}, error::Error};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Executable directory destination
   #[clap(short, long, value_parser)]
   dest: String,

   /// Check only if a new version is available
   #[clap(short, long)]
   check: bool,
}

const NIGHTLY_URL: &str = "https://github.com/neovim/neovim/releases/tag/nightly";

async fn fetch_latest_version() -> Result<String, reqwest::Error> {
    let response = match reqwest::get(NIGHTLY_URL).await {
        Ok(response) => response,
        Err(e) => return Err(e)
    };

    let text = match response.text().await {
        Ok(text) => text,
        Err(e) => return Err(e)
    };

    Ok(text)
}

async fn fetch_current_version() -> Result<String, Box<dyn Error>> {
    let output = match Command::new("nvim").arg("--version").output() {
        Ok(out) => out,
        Err(e) => return Err(Box::new(e))
    };

    match String::from_utf8(output.stdout) {
        Ok(s) => Ok(s),
        Err(e) => Err(Box::new(e))
    }
}

fn get_version(content: String) -> Option<String> {
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

#[tokio::main]
#[instrument]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Args::parse();

    // Setup logging
    let subscriber = FmtSubscriber::new();
    match tracing::subscriber::set_global_default(subscriber) {
        Ok(()) => (),
        Err(_) => {
            error!("Unable to setup logger");
            exit(1);
        }
    }

    // Fetch latest version from GitHub
    info!("Fetching latest version of neovim");
    let mut content = match fetch_latest_version().await {
        Ok(text) => text,
        Err(e) => {
            error!("Error when fetching nvim latest version: {}", e);
            exit(1);
        }
    };

    let latest_version = match get_version(content) {
        Some(v) => v,
        None => {
            error!("Error when searching nvim latest version: can't find pattern in content");
            exit(1);
        }
    };
    info!("Latest neovim nightly version is {}", latest_version);

    // Fetch current version
    content = match fetch_current_version().await {
        Ok(text) => text,
        Err(e) => {
            error!("Error when getting nvim current version: {}", e);
            exit(1);
        }
    };

    let current_version = match get_version(content) {
        Some(v) => v,
        None => {
            error!("Error when searching nvim current version: can't find pattern in content");
            exit(1);
        }
    };
    info!("Current neovim version is {}", current_version);

    if latest_version == current_version {
        info!("Already at the latest version: latest={} current={}", latest_version, current_version);
        exit(0)
    } else if args.check {
        info!("A new version is available: latest={} current={}", latest_version, current_version);
        exit(0)
    } else {
        info!("A new version is available: latest={} current={}", latest_version, current_version);
    }

    //TODO: Download latest version off Github

    Ok(())
}
