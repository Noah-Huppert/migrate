extern crate clap;
extern crate ini;
extern crate postgres;

use clap::ArgMatches;
use ini::Ini;
use postgres::{Connection, SslMode};

use models;
use db_schema_ver;

#[derive(Debug)]
pub struct RunCmd {
    host: String,
    user: String,
    password: String,
    database: String
}

impl RunCmd {
    pub fn new(host: String, user: String, password: String, database: String) -> RunCmd {
        RunCmd {
            host: host,
            user: user,
            password: password,
            database: database
         }
    }

    fn make_db_conn_str(&self) -> String {
        format!("postgresql://{}:{}@{}/{}", self.user, self.password, self.host, self.database)
    }
}

impl models::command::Command <RunCmd> for RunCmd {
    fn from_matches(matches: &ArgMatches) -> Result<RunCmd, String> {
        let mut obj = RunCmd::new(String::new(), String::new(), String::new(), String::new());

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
                obj.host = host.to_owned();
            }

            if let Some(user) = section.get("user") {
                obj.user = user.to_owned();
            }

            if let Some(password) = section.get("password") {
                obj.password = password.to_owned();
            }

            if let Some(database) = section.get("database") {
                obj.database = database.to_owned();
            }
        }

        // Config from options
        if let Some(host) = matches.value_of("host") {
            obj.host = host.to_owned();
        }

        if let Some(user) = matches.value_of("user") {
            obj.user = user.to_owned();
        }

        if let Some(password) = matches.value_of("password") {
            obj.password = password.to_owned();
        }

        if let Some(database) = matches.value_of("database") {
            obj.database = database.to_owned();
        }

        Ok(obj)
    }

    fn run(&self) -> Result<(), String> {
        let db_connr = Connection::connect(self.make_db_conn_str(), SslMode::None);
        if let Err(err) = db_connr {
            println!("Error connecting to database: {:?}", err);
            return Err(err.to_string())
        }

        let db_conn = match db_connr {
            Ok(conn) => conn,
            Err(err) => {
                error!("Error connecting to database: {}", err);
                panic!("Error connecting to database");
            }
        };


        Ok(())
    }
}
