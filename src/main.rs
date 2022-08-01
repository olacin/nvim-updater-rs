use clap::Parser;
use std::process::exit;
use console::Emoji;
use console::style;

type Result<T> = nvim_updater_rs::Result<T>;


/// Update to latest nightly nvim version.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Executable directory destination
   #[clap(short, long, default_value_t = String::from("/usr/bin/nvim"))]
   dest: String,

   /// Check only if a new version is available
   #[clap(short, long)]
   check: bool,
}

#[tokio::main]
async fn main() -> Result<()>{
    let args = Args::parse();
    let client = reqwest::Client::new();

    eprintln!("{} information on versions", style("Gathering").green());
    let mut content = match nvim_updater_rs::fetch_latest_version(&client).await {
        Ok(text) => text,
        Err(e) => {
            eprintln!("{} when fetching nvim latest version: {}", style("Error").red(), e);
            exit(1);
        }
    };

    let latest_version = match nvim_updater_rs::get_version(content) {
        Some(v) => v,
        None => {
            eprintln!("{} when searching nvim latest version: can't find pattern in content", style("Error").red());
            exit(1);
        }
    };

    content = match nvim_updater_rs::fetch_current_version().await {
        Ok(text) => text,
        Err(e) => {
            eprintln!("{} when getting nvim current version: {}", style("Error").red(), e);
            exit(1);
        }
    };

    let current_version = match nvim_updater_rs::get_version(content) {
        Some(v) => v,
        None => {
            eprintln!("{} when searching nvim current version: can't find pattern in content", style("Error").red());
            exit(1);
        }
    };

    if latest_version == current_version {
        eprintln!("{}Already at the latest version: latest={} current={}", Emoji("✅ ", ""), style(latest_version).green(), style(current_version).green());
        exit(0)
    } else if args.check {
        eprintln!("{}A new version is available: latest={} current={}", Emoji("✨ ", ""), style(&latest_version).green(), style(current_version).yellow());
        exit(0)
    } else {
        eprintln!("{}A new version is available: latest={} current={}", Emoji("✨ ", ""), style(&latest_version).green(), style(current_version).yellow());
    }

    match nvim_updater_rs::download(&client, &args.dest).await {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{} happened while downloading latest nvim version: {}", style("Error").red(), e);
            exit(1);
        }
    }

    eprintln!("{}Successfully updated {} to version {}", Emoji("✅ ", ""), style(args.dest).green(), style(&latest_version).green());
    Ok(())
}
