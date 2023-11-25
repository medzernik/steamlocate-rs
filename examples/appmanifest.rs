use std::{env, process::exit};

use steamlocate::InstallDir;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 || args[1].parse::<u32>().is_err() {
        eprintln!("Usage: cargo run --example appmanifest -- <STEAM_APP_ID>");
        exit(1);
    }
    let app_id: u32 = args[1].parse().expect("<STEAM_APP_ID> should be a u32");

    let steam_dir = InstallDir::locate().unwrap();
    match steam_dir.app(app_id) {
        Ok(Some(app)) => println!("Found app - {:#?}", app),
        Ok(None) => println!("No app found for {}", app_id),
        Err(err) => println!("Failed reading app: {err}"),
    }
}
