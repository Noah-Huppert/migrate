extern crate clap;
extern crate ini;
extern crate postgres;

pub mod cmd_run;

use std::io::{self, Write};
use clap::{Arg, App, SubCommand};

fn main() {
    let prgm_ver = env!("CARGO_PKG_VERSION");

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
                                    .short("d")
                                    .takes_value(true)
                                    .default_value("migrations")
                                )
                            )
                            .subcommand(SubCommand::with_name("run")
                                .about("Run migrations")
                                .arg(Arg::with_name("target")
                                    .help("Database schema version that \"run\" command should attempt to reach")
                                    .short("t")
                                    .takes_value(true)
                                    .required(true)
                                )
                                .arg(Arg::with_name("migrations-dir")
                                    .help("The directory to look for migrations in.")
                                    .short("d")
                                    .takes_value(true)
                                    .default_value("migrations")
                                )
                                .arg(Arg::with_name("environment")
                                    .help("Environment which application is running in")
                                    .short("e")
                                    .takes_value(true)
                                    .requires("config")
                                )
                                .arg(Arg::with_name("config")
                                    .help("Path to database access configuration file")
                                    .short("c")
                                    .takes_value(true)
                                    .required_unless_all(&["host", "user", "password"])
                                )
                                .arg(Arg::with_name("host")
                                    .help("Database host")
                                    .short("h")
                                    .takes_value(true)
                                    .required_unless("config")
                                )
                                .arg(Arg::with_name("user")
                                    .help("Database user")
                                    .short("u")
                                    .takes_value(true)
                                    .required_unless("config")
                                )
                                .arg(Arg::with_name("password")
                                    .help("Database password")
                                    .short("p")
                                    .takes_value(true)
                                    .required_unless("config")
                                )
                            )
                            .get_matches();

    match app_matches.subcommand() {
        ("create", Some(sub_matches)) => {
            println!("TODO: create command -> {:?}", sub_matches);
        }
        ("run", Some(sub_matches)) => {
            match cmd_run::cmd_run(&sub_matches) {
                Ok(_) => {}
                Err(err) => {
                    writeln!(&mut io::stderr(), "{}", err).expect("Failed to write to stderr");
                }
            }
        }
        _ => {}
    }
}
