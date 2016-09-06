//! The `db_schema_ver` module provides an interface for determining and modifying the schema version.
extern crate chrono;
extern crate postgres;

use chrono::datetime::DateTime;
use postgres::Connection;

pub struct DbSchemaVer<'a> {
    conn: &'a Connection,
    db_name: String
}

pub enum VerStatus {
    Ongoing,
    Fail,
    Success,
}

pub struct VerEntry {
    id: i32,
    updated: String,
    version: i32,
    migration_hash: String,
    status: VerStatus,
    lib_ver: i32
}

impl<'a> DbSchemaVer<'a> {
    /// Creates a new DbSchemaVer struct
    ///
    /// - `connection: &Connection` - [Postgres Connection](https://sfackler.github.io/rust-postgres/doc/v0.11.11/postgres/struct.Connection.html)
    /// - *return* - New DbSchemaVer if successful, Error message if not.
    ///
    /// This function will attempt to create the 'schema_ver_status' type and the
    /// 'schema_versions' table if they do not exist.
    ///
    /// # Errors
    /// Following errors occur when SQL queries go awry
    /// - Failed to get name of current database
    /// - Failed to create 'schema_versions' table
    /// - Failed to create 'schema_version_status' type
    fn bootstrap(connection: &'a Connection) -> Result<DbSchemaVer, String> {
        // Get current database name
        let db_name = match connection.execute("SELECT current_database()", &[]) {
            Ok(name) => name,
            Err(err) => {
                error!("Failed to get name of current database, error: {}", err);
                return Err("Failed to get name of current database");
            }
        };

        // Create new schema_ver obj to return
        let schema_ver = DbSchemaVer {
            conn: connection,
            db_name: db_name
        };

        // Create 'schema_ver_status' if type doesn't exist
        let create_typer = schema_ver.conn.execute("IF NOT EXISTS
                                                (SELECT 1 FROM pg_type WHERE typename = '$1') THEN
                                                    CREATE TYPE schema_version_status AS ENUM (
                                                        'Ongoing', 'Success', 'Fail
                                                     )", &[]);
        match create_typer {
            Ok(rows_up) => {
                info!("Created 'schema_version_status' type for {}", schema_ver.db_name);
            },
            Err(err) => {
                error!("Failed to created type 'schema_version_status' type, error: {}", err);
                return Err("Failed to create type 'schema_version_status`");
            }
        }

        // Create 'schema_versions' table if it doesn't exist
        let create_tlbr = schema_ver.conn.execute("CREATE TABLE IF NOT EXISTS schema_versions (
                                id INT PRIMARY KEY NOT NULL,
                                updated TIMESTAMP NOT NULL,
                                version INT NOT NULL,
                                migration_hash TEXT NOT NULL,
                                status schema_ver_status NOT NULL,
                                lib_ver INT NOT NULL)", &[]);
        match create_tlbr {
            Ok(rows_updated) => {
                info!("Created 'schema_versions' table for {}", schema_ver.db_name);
            },
            Err(err) => {
                error!("Failed to create 'schema_versions' table for {}, error: {}", schema_ver.db_name, err);
                return Err("Failed to create 'schema_versions' table");
            }
        }

        Ok(schema_ver)
    }
}
