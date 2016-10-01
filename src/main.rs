#![warn(missing_docs)]
//! Main fie for Migrate
//! A simple database migrator written in Rust

#[macro_use]
extern crate log;
extern crate clap;
extern crate ini;
extern crate postgres;

mod commands;
mod models;

use models::command::Command;

//use std::io::{self, Write};
use clap::{Arg, App, SubCommand};

fn main() {
    let prgm_ver = env!("CARGO_PKG_VERSION");
    info!("Migrate v{}", prgm_ver);

    // Define command line options
    let app_matches = App::new("Migrate")
                            .version(prgm_ver)
                            .about("Lightweight database migration runner")
                            .subcommand(SubCommand::with_name("create")
                                .about("Create scaffold for migration")
                                .arg(Arg::with_name("name")
                                    .help("Name of database migration")
                                    .index(1)
                                    .required(true)
                                )
                                .arg(Arg::with_name("migrations-dir")
                                    .help("The directory to put the new migration in.")
                                    .short("m")
                                    .takes_value(true)
                                    .default_value("migrations")
                                )
                            )
                            .subcommand(commands::run::sub_cmd::sub_cmd())
                            .get_matches();

    match app_matches.subcommand() {
        ("create", Some(sub_matches)) => {
            println!("TODO: create command -> {:?}", sub_matches);
        }
        ("run", Some(sub_matches)) => {
            let cmd = match commands::run::run::RunCmd::from_matches(sub_matches) {
                Ok(cmd) => { cmd }
                Err(err) => {
                    println!("Failed to load \"run\" command: {}", err);
                    commands::run::run::RunCmd::new(String::new(), String::new(), String::new(), String::new())
                }
            };
            match cmd.run() {
                Ok(res) => {
                    println!("OK! => {:?}", res);
                }
                Err(err) => {
                    println!("ERR! => {:?}", err);
                }
            }
            /*
            match commands::run::sub_matches) {
                Ok(_) => {}
                Err(err) => {
                    writeln!(&mut io::stderr(), "{}", err).expect("Failed to write to stderr");
                }
            }
            */
        }
        _ => {}
    }
}
