extern crate clap;
extern crate ini;
extern crate postgres;

pub mod cmd_run;

use std::io::{self, Write};
use clap::{Arg, App, SubCommand};

/*
fn get_db_conf(opts: &clap::ArgMatches) -> DatabaseConf {
    let mut db_conf = DatabaseConf::new("".to_string(), "".to_string(), "".to_string());
    
    // Load configuration from ini file
    match opts.opt_str("c") {
        Some(fconf_path) => {
            let fconf = match Ini::load_from_file(&*fconf_path) {
                Ok(fconf) => { fconf }
                Err(err) => {
                    panic!("Error opening config file: {}", err.to_string())
                }
            };

            let e = opts.opt_str("e");
            let section = match fconf.section(e.clone()) {
                Some(s) => { s }
                None => {
                    panic!("\"{}\" environment section not found in config file provided", e.clone().unwrap_or("None".to_owned()))
                }
            };
            // Check for host
            if section.contains_key("host") {
                db_conf.host = section.get("host").unwrap().to_string();
            }

            // Check for user
            if section.contains_key("user") {
                db_conf.user = section.get("user").unwrap().to_string();
            }

            // Check for password
            if section.contains_key("password") {
                db_conf.password = section.get("password").unwrap().to_string();
            }
        }
        None => {}
    };

    // Load configuration from options
    match opts.opt_str("h") {
        Some(host) => {
            db_conf.host = host;
        }
        None => {}
    }

    match opts.opt_str("u") {
        Some(user) => {
            db_conf.user = user;
        }
        None => {}
    }

    match opts.opt_str("p") {
        Some(password) => {
            db_conf.password = password;
        }
        None => {}
    }

    // Check that necessary configuration values are set
    if db_conf.host.is_empty() {
        panic!("Database host must be provided either with -h or in an .ini file specified with -c");
    }

    if db_conf.user.is_empty() {
        panic!("Database user must be provided either with -u or in an .ini file specified with -c");
    }

    if db_conf.password.is_empty() {
        panic!("Database password must be provided either with -p or in an .ini file specified with -c");
    }

    db_conf
}
*/

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
                            )
                            .subcommand(SubCommand::with_name("run")
                                .about("Run migrations")
                                .arg(Arg::with_name("target")
                                    .help("Database schema version that \"run\" command should attempt to reach")
                                    .short("t")
                                    .takes_value(true)
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
