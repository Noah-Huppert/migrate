//! The `db_schema_ver` module provides an interface for determining and modifying the schema version.
extern crate chrono;
extern crate postgres;

use chrono::datetime::DateTime;
use postgres::{Connection, Row};

pub struct DbSchemaVer<'a> {
    conn: &'a Connection,
    db_name: String
}

pub enum VerStatus {
    Ongoing,
    Fail,
    Success,
}

impl VerStatus {
    fn from_string(str: String) -> Option<VerStatus> {
        str = str.to_lowercase();
        return match str {
            "ongoing" => Some(VerStatus::Ongoing),
            "fail" => Some(VerStatus::Fail),
            "success" => Some(VerStatus::Success),
            _ => None
        }
    }
}

pub struct VerEntry {
    id: i32,
    updated: DateTime,
    version: i32,
    migration_hash: String,
    status: VerStatus,
    lib_ver: i32
}

impl<'a> DbSchemaVer<'a> {
    /// Creates a new DbSchemaVer struct
    ///
    /// - `connection: &Connection` - [Postgres Connection](https://sfackler.github.io/rust-postgres/doc/v0.11.11/postgres/struct.Connection.html)
    /// - *returns*: `DbSchemaVer` - New DbSchemaVer if successful, Error code if not.
    ///
    /// This function will attempt to create the 'schema_ver_status' type and the
    /// 'schema_versions' table if they do not exist.
    ///
    /// # Errors
    /// - `get_db_name_fail` - Query to get current database name failed
    /// - `type_create_fail` - Query to create `schema_version_status` enum failed
    /// - `table_create_fail` - Query to create `schema_versions` table failed
    fn bootstrap(connection: &'a Connection) -> Result<DbSchemaVer, String> {
        // Get current database name
        let db_name = match connection.execute("SELECT current_database()", &[]) {
            Ok(name) => name,
            Err(err) => {
                error!("Failed to get name of current database, error: {}", err);
                return Err("get_db_name_fail");
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
                return Err("type_create_fail");
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
                return Err("table_create_fail");
            }
        }

        Ok(schema_ver)
    }

    /// Construct a struct (VerEntry struct) which represents a row in the `schema_versions` table
    ///
    /// - `row: &Row - [Postgres Row](https://sfackler.github.io/rust-postgres/doc/v0.11.11/postgres/rows/struct.Row.html)
    ///                 to construct row struct from
    /// - *returns*: `VerEntry` - VerEntry which represents provided Row, error code if fail
    ///
    /// # Errors
    /// - `missing_cols` - The Row provided does not contain all the columns necessary
    /// - `date_parse_fail` - Failed to parse a DateTime struct from `updated` column in row
    /// - `status_parse_fail` - Failed to find VerStatus enum value which matched `status` column in row
    fn construct_row(row: &'a Row) -> Result<VerEntry, String> {
        // Check that required columns are provided
        let req_cols = vec!["id", "updated", "version", "migration_hash", "status", "lib_ver"];
        let mut prov_cols = Vec::new();

        row.columns().iter().map(|col| {
            prov_cols.push(col.name());
        });

        let prov_cols_slice = prov_cols.as_slice();
        let mut missing_cols = Vec::new();

        req_cols.iter().map(|col| {
            if prov_cols_slice.contains(col) == false {
                missing_cols.push(col);
            }
        });

        if missing_cols.len() > 0 {
            error!("Cannot construct row, columns missing: {}", missing_cols);
            return err("missing_cols")
        }

        // Map values
        // -- Create and error handle more complex values ahead of time
        let updatedv = match DateTime.parse_from_rfc3339(row.get("update")) {
            Ok(up) => up,
            Err(err) => {
                error!("Failed to parse date time string \"{}\" to DateTime, error: {}", row.get("update"), err.description());
                return Err("date_parse_fail")
            }
        };

        let statusv = match VerStatus::from_string(row.get("status")) {
            Some(status) => status,
            None => {
                error!("Failed to parse string \"{}\" into VerStatus enum, did not match any text", row.get("status"));
                return Err("status_parse_fail")
            }
        };

        VerEntry {
            id: row.get("id"),
            updated: updatedv,// Parse date time from ISO 8601 string (RFC-3339 == ISO-8601 in this case)
            version: row.get("version"),
            migration_hash: row.get("migration_hash"),
            status: statusv,
            lib_ver: row.get("lib_ver")
        }
    }

    /// Retrieves most recent schema version information
    ///
    /// - *returns*: `VerEntry` - VerEntry struct representing current schema version, error code if fail
    ///
    /// # Errors
    /// - `row_parse_fail` - Failed to parse retrieved row from db
    /// - `query_error` - Query to retrieve most recent schema version information failed
    fn current_version(&self) -> Result<VerEntry, String> {
        let currentr = self.conn.query("SELECT * FROM schema_versions ORDER BY updated, desc LIMIT 1", &[]);

        match currentr {
            Ok(rows) => {
                // Expecting exactly one row
                if rows.is_empty() || rows.len() > 1 {
                    return Err("incorrect_row_count")
                }

                return match DbSchemaVer::construct_row(rows.get(0)) {
                    Ok(v) => Ok(v),
                    Err(err) => {
                        error!("Failed to construct current version row, error: {}", err);
                        return Err("row_parse_fail")
                    }
                }
            },
            Err(err) => {
                error!("Failed to execute query, error: {}", err.description());
                return Err("query_error")
            }
        };
    }

    /// Retrieves schema version information based on provided `version` column value
    ///
    /// - `ver: i32` - Version to retrieve
    /// - *returns*: `VerEntry` - VerEntry representing requested version, error code if fail
    ///
    /// # Errors
    fn by_version_num(&self, ver: i32) -> Result<VerEntry, String> {
        let verr = self.conn.query("SELECT * FROM schema_version WHERE version = $1", &vec![ver]);

        match verr {
            Ok(rows) => {
                // Check for duplicates
                if rows.len() > 1 {
                    warn!("Multiple rows found for same version, merging");
                    // TODO: call and handle resolve_dup_ver_rows
                }

                // TODO: Check for 0 rows
                // TODO: Contruct row struct and return
            }
        };
    }

    /// In the rare event that multiple rows represent the same schema version this function will resolve that version
    ///
    /// - `ver: i32` - Version with duplicate rows
    /// - *returns*: `Row` - The correct row to use for the specified version, error code if fail
    ///
    /// Resolves duplicate version rows by keeping the most recent and deleting the rest
    ///
    /// # Errors
    /// - `no_dup_rows` - No duplicate rows where found for the supplied version
    /// - `date_parse_fail` - Failed to parse `updated` column in a row
    fn resolve_dup_ver_rows(&self, ver: i32) -> Result<Row, String> {
        let verr = self.conn.query("SELECT * FROM schema_version WHERE version = $1", &vec![ver]);

        match verr{
            Ok(rows) => {
                // Check that there are in fact duplicate rows
                if rows.length <= 1 {
                    error!("No duplicate rows for schema version {}", ver);
                    return Err("no_dup_rows")
                }

                let i = 0;
                let newest_date = (0, DateTime::new());// TODO: Figure out what actual date a new DateTime is created with
                rows.iter().map(|row| {
                    let date = match DateTime.from_rfc3331(row.get("updated")) {
                        Ok(date) => date,
                        Err(err) => {
                            error!("Error parsing DateTime from date string \"{}\" (row.id: {}), error: {}", row.get("updated"), row.get("id"), err.description());
                            return Err("date_parse_fail")
                        }
                    };

                    if date > newest_date {
                        newest_date = (i, date);
                    }

                    i += 1;
                });

                // TODO: Delete old rows, return correct row
            }
        };
    }
}
