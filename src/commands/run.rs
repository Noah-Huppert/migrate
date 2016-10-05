///! Run sub-command

use clap::{ArgMatches, Arg, App, SubCommand};
use ini::Ini;
//use postgres::{Connection, SslMode};

use traits::command::Command;

#[derive(Debug)]
pub struct RunCmd {
    host: String,
    user: String,
    password: String,
    database: String
}

impl RunCmd {
    fn make_db_conn_str(&self) -> String {
        format!("postgresql://{}:{}@{}/{}", self.user, self.password, self.host, self.database)
    }
}

impl Command for RunCmd {
    fn new() -> RunCmd {
        RunCmd {
            host: "",
            user: "",
            password: "",
            database: ""
         }
    }

    fn mk_cli_config() -> App {
        SubCommand::with_name("run")
        .about("Run migrations")
        .arg(Arg::with_name("target")
            .help("Database schema version that \"run\" command should attempt to reach")
            .short("t")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("migrations-dir")
            .help("The directory to look for migrations in.")
            .short("m")
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
            .required_unless_all(&["host", "user", "password", "database"])
        )
        .arg(Arg::with_name("host")
            .help("Database server host")
            .short("h")
            .takes_value(true)
            .required_unless("config")
        )
        .arg(Arg::with_name("user")
            .help("Database server user")
            .short("u")
            .takes_value(true)
            .required_unless("config")
        )
        .arg(Arg::with_name("password")
            .help("Database server password")
            .short("p")
            .takes_value(true)
            .required_unless("config")
        )
        .arg(Arg::with_name("database")
            .help("Database to run migrations on")
            .short("d")
            .takes_value(true)
            .required_unless("config")
        )
    }

    fn parse_args(args: &ArgMatches) -> Result<RunCmd, String> {
        let mut obj = RunCmd::new(String::new(), String::new(), String::new(), String::new());

        // Config from ini file
        if let Some(config_path) = args.value_of("config") {
            let confr = Ini::load_from_file(config_path);
            if let Err(err) = confr {
                return Err(err.to_string())
            }

            let conf = confr.unwrap();

            let sectionr = conf.section(args.value_of("environment"));
            if sectionr.is_none() {
                return Err(format!("No config for environment \"{}\"", args.value_of("environment").unwrap_or("None")))
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
        if let Some(host) = args.value_of("host") {
            obj.host = host.to_owned();
        }

        if let Some(user) = args.value_of("user") {
            obj.user = user.to_owned();
        }

        if let Some(password) = args.value_of("password") {
            obj.password = password.to_owned();
        }

        if let Some(database) = args.value_of("database") {
            obj.database = database.to_owned();
        }

        Ok(obj)
    }

    fn run(&self) -> Result<(), String> {
        /*
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
        */

        Ok(())
    }
}
