# TODO: Figure out how to export a static `command: Command` object from this module.
# TODO: Convert infro `Command`
extern crate clap;
extern crate ini;
extern crate postgres;

use clap::ArgMatches;
use ini::Ini;
use postgres::{Connection, SslMode};

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

    fn from_matches(matches: &ArgMatches) -> Result<DatabaseConf, String> {
        let mut db_conf = DatabaseConf::new("".to_string(), "".to_string(), "".to_string());

        // Config from ini file
        if let Some(config_path) = matches.value_of("config") {
            let confr = Ini::load_from_file(config_path);
            if let Err(err) = confr {
                return Err(err.to_string())
            }

            let conf = confr.unwrap();

            let sectionr = conf.section(matches.value_of("environment"));
            if sectionr.is_none() {
                return Err(format!("No config for environment \"{}\"", matches.value_of("environment").unwrap_or("None")))
            }

            let section = sectionr.unwrap();

            if let Some(host) = section.get("host") {
                db_conf.host = host.to_owned();
            }

            if let Some(user) = section.get("user") {
                db_conf.user = user.to_owned();
            }

            if let Some(password) = section.get("password") {
                db_conf.password = password.to_owned();
            }
        }

        // Config from options
        if let Some(host) = matches.value_of("host") {
            db_conf.host = host.to_owned();
        }

        if let Some(user) = matches.value_of("user") {
            db_conf.user = user.to_owned();
        }

        if let Some(password) = matches.value_of("password") {
            db_conf.password = password.to_owned();
        }

        Ok(db_conf)
    }
}

pub fn cmd_run(matches: &ArgMatches) -> Result<(), String> {
    let db_conf = try!(DatabaseConf::from_matches(matches));

    let db_connr = Connection::connect(&format!("postgresql://{}:{}@{}", db_conf.user, db_conf.password, db_conf.host)[..], SslMode::None);
    if let Err(err) = db_connr {
        return Err(err.to_string())
    }

    let db_conn = db_connr.unwrap();

    Ok(())
}