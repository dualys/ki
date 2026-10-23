use std::fs::create_dir_all;

use clap::Command;
use dotenvy::dotenv;
use inquire::{Confirm, InquireError};

pub mod meikyo;
pub mod schema;

fn cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author("Willy Micieli")
        .subcommand(Command::new("add").about("Create new words"))
        .subcommand(Command::new("man").about("Create man pages"))
}
fn main() -> Result<(), InquireError> {
    let app = cli();
    let matches = app.get_matches();
    create_dir_all(dirs::preference_dir().expect("pref").join("ki")).expect("failed to create dir");
    match matches.subcommand() {
        Some(("add", _)) => loop {
            loop {
                let _ = dotenv().ok();
                meikyo::add_word_interactive().expect("failed to get data");
                if Confirm::new("Add an other words")
                    .with_default(false)
                    .prompt()?
                    .eq(&false)
                {
                    return Ok(());
                }
            }
        },
        Some(("man", _)) => Ok(()),
        _ => Ok(()),
    }
}
