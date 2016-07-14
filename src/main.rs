extern crate clap;
extern crate ini;

use std::env;
use clap::{Arg, App, SubCommand};
use ini::Ini;

#[derive(Debug)]
struct DatabaseConf {
    host: String,
    user: String,
    password: String
}

impl DatabaseConf {
    fn new(host: String, user: String, password: String) -> DatabaseConf {
        DatabaseConf {
            host: host,
            user: user,
            password: password
         }
    }
}

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
    /*
    let matches = App::new("Migrate")
                        .version(prgm_ver)
                        .about("A lightweight tool for running database migration written in rust")
                        .subcommand(SubCommand::with_name("create")
                            .about("Create scaffold for a migration")
                        )
                        .subcommand(SubCommand::with_name("run")
                            .about("Run migrations")
                            .arg(Arg::with_name("config").help("Path to database access configuration file")
                                .
                            )
                        )
                        */
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
            println!("TODO: create command");
        }
        ("run", Some(sub_matches)) => {
            cmd_run(&sub_matches);
        }
        _ => {}
    }

    /*
    let mut opts_conf = Options::new();

    opts_conf.optopt("c", "config", "Config file for specifying host, user, and password of database. Values set in file will be overridden by their respective flags", "database_auth.ini");
    opts_conf.optopt("h", "host", "Database host", "localhost:5432");
    opts_conf.optopt("u", "user", "Database user", "migrator");
    opts_conf.optopt("p", "password", "Database password", "password123");
    opts_conf.optopt("e", "env", "Application environment", "dev");

    // Parse command line options
    let opts = match opts_conf.parse(&args) {
        Ok(matches) => { matches }
        Err(err) => {
            panic!("Err parsing command line options: {}", err.to_string())
        }
    };

    // Load Db configuration
    let db_conf = get_db_conf(&opts);
    println!("{:?}", db_conf);
    */
}
