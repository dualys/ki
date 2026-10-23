use clap::Command;
use diesel::{Connection, PgConnection};
use inquire::Confirm;
use std::{env::var, fs::create_dir_all};

pub mod meikyo;
pub mod schema;

fn cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author("Willy Micieli")
        .subcommand(Command::new("add").about("Create new words"))
        .subcommand(Command::new("gen").about("Generate the dictionary PDF"))
        .subcommand(Command::new("fill").about("Fill NULL columns in words table"))
        .subcommand(Command::new("import").about("Import word list"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&database_url).expect("Failed to connect to database");

    create_dir_all(dirs::preference_dir().expect("pref dir").join("ki"))
        .expect("failed to create dir");

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("add", _)) => {
            loop {
                meikyo::interactive_generation_loop(&mut conn)?;
                if !Confirm::new("Add another word?")
                    .with_default(false)
                    .prompt()?
                {
                    break;
                }
            }
            Ok(())
        }
        Some(("gen", _)) => {
            meikyo::export_dictionary(&mut conn)?;
            Ok(())
        }
        Some(("fill", _)) => {
            meikyo::fill_null_columns(&mut conn)?;
            println!("Colonnes mises à jour.");
            Ok(())
        }
        Some(("import", _)) => {
            meikyo::import_wordlist(&mut conn, "/usr/share/dict/words")?;
            println!("Import terminé.");
            Ok(())
        }
        _ => {
            println!("Usage: ki [add|gen|fill|import]");
            Ok(())
        }
    }
}
