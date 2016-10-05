//! The `db_schema_ver` module provides an interface for determining and modifying the schema version.
use chrono::datetime;
use postgres::Connection;
use postgres::rows::Row;

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
    // TODO: Find out how to either have match statement with keys of type String, or take type &str as an argument and figure out how to lowercase
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

        /*
        row.columns().any(|row| {
            debug!("{}", row);
            false
        });
        */

        Err("BREAK")
        /*
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
            return Err("missing_cols")
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

        Some(VerEntry {
            id: row.get("id"),
            updated: updatedv,// Parse date time from ISO 8601 string (RFC-3339 == ISO-8601 in this case)
            version: row.get("version"),
            migration_hash: row.get("migration_hash"),
            status: statusv,
            lib_ver: row.get("lib_ver")
        })
        */
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
    /// `resolve_dup_rows_fail` - Failed to resolve multiple schema version rows, see resolve_dup_ver_rows(ver: i32, rows: Option<[Row]>)
    /// `row_construct_fail` - Failed to construct row from db row provided
    fn by_version_num(&self, ver: i32) -> Result<VerEntry, String> {
        let verr = self.conn.query("SELECT * FROM schema_version WHERE version = $1", &vec![ver]);

        let row = match verr {
            Ok(rows) => {
                // Check for duplicates
                if rows.len() > 1 {
                    warn!("Multiple rows found for same version, merging");

                    let resolver = self.resolve_dup_ver_rows(ver);
                    match resolver {
                        Ok(rrow) => return rrow,
                        Err(err) => {
                            error!("Failed to resolve duplicate schema version rows, error: {}", err);
                            return Err("resolve_dup_rows_fail")
                        }
                    };
                }
            },
            Err(err) => {}
        };

        let contructr = DbSchemaVer::construct_row(&row);

        match contructr {
            Ok(crow) => return Ok(crow),
            Err(err) => {
                warn!("Error constructing row from db row provided, error: {}", err.describe());
                return Err("row_construct_fail")
            }
        }
    }

    /// Proxy method for `resolve_dup_ver_rows_backend(&self, ver: i32, rows: [Row]`
    ///
    /// See parent method for more information
    ///
    /// # Errors
    /// - `db_query_fail` - Query to retrieve version failed
    fn resolve_dup_ver_rows(&self, ver: i32) -> Result<Row, String> {
        let verr = self.conn.query("SELECT * FROM schema_version WHERE version = $1", &vec![ver]);

        match verr {
            Ok(rows) => {
                return self.resolve_dup_ver_rows_backend(ver, Some(rows))
            },
            Err(err) => {
                return Err("db_query_fail")
            }
        }
    }

    /// In the rare event that multiple rows represent the same schema version this function will resolve that version
    ///
    /// - `ver: i32` - Version with duplicate rows
    /// - `rows: [Row]` - Array of db rows to analyse when resolving duplicate
    /// - *returns*: `Row` - The correct row to use for the specified version, error code if fail
    ///
    /// Resolves duplicate version rows by keeping the most recent and deleting the rest
    ///
    /// # Errors
    /// - `no_dup_rows` - No duplicate rows where found for the supplied version
    /// - `date_parse_fail` - Failed to parse `updated` column in a row
    /// - `disappearing_ver_row` - The correct version row that was determined earlier in the
    ///                             function did not survive the deletion process
    fn resolve_dup_ver_rows_backend(&self, ver: i32, rows: [Row]) -> Result<Row, String> {
        // Check that there are in fact duplicate rows
        if rows.length <= 1 {
            error!("No duplicate rows for schema version {}", ver);
            return Err("no_dup_rows")
        }

        // Find id of row with most recent updated date
        // (id, updated)
        let newest_date = (-1, None);
        rows.iter().map(|row| {
            let date = match DateTime.from_rfc3331(row.get("updated")) {
                Ok(date) => date,
                Err(err) => {
                    error!("Error parsing DateTime from date string \"{}\" (row.id: {}), error: {}", row.get("updated"), row.get("id"), err.description());
                    return Err("date_parse_fail")
                }
            };

            if newest_date == None || date > newest_date {
                newest_date = (row.get("id"), Some(date));
            }
        });

        // Store history of all db transactions for after action report
        // (deletes, bad_deletes, restores, bad_restores)
        let transactions_status = (0, 0, 0, 0);

        // Delete non most recent rows
        let correct_ver_row: Option<Row> = None;
        rows.iter().map(|row| {
            let id = row.get("id");

            if id == newest_date.0 {// If the current row is the row determined to me the most recent, mark it as so and do nothing
                correct_ver_row = Some(row);
            } else {// If not most recent row (aka a duplicate) delete from table
                let delr = self.conn.execute("DELETE FROM schema_versions WHERE id = $1", &vec![id]);

                match delr {
                    Ok(rows_changed) => {
                        if rows_changed != 1 {
                            error!("Unexpected behavior when deleting duplicate schema version entry, rows changed: {} (Should be 1)", rows_changed);
                            transactions_status.1 += 1;
                            return Err("dup_del_fail")
                        }

                        transactions_status.0 += 1;
                    },
                    Err(err) => {
                        transactions_status.1 += 1;
                        error!("Failed to execute query to delete duplicate schema version entry, error: {}", err.describe());
                        return Err("dup_del_fail")
                    }
                }
            }
        });

        // Deal with the highly unlikely case that the code above just deleted every single row of
        // the specified version.
        //
        // However unlikely it may be (Maybe only possible via something crazly unlikely like a
        // random bit being flipped in RAM due to a hardware failure, idk :/), still need to support
        // this case, this table is kinda important :)
       if correct_ver_row == None {
           error!("Could not find correct version entry row, restoring rows");

           let columns = vec!["id", "updated", "version", "migration_hash", "status", "lib_version"];

           // Put values into array in order expected by SQL VALUES
           let mut value_tuples: Vec<Vec<String>> = Vec::new();

           rows.iter().map(|row| {
               let mut values: Vec<String> = Vec::new();

               columns.iter().map(|column| {
                   values.push(row.get(column));
               });

               value_tuples.push(values);
           });

           // Convert each tuple in value_tuples array into SQL format
           let mut value_blocks: Vec<String> = Vec::new();
           value_blocks.iter().map(|block| {
               value_blocks.push(format!("({})", block.join(", ")));
           });

           // Generate a list of value tuples in SQL format
           let mut value_blocks_str = "";
           let i = 0;
           value_blocks.map(|block| {
               let mut separator = ",\n";

               if i == value_blocks.len() {
                   separator = "\n";
               }

               value_blocks_str = format!("{}{}", block.join(", "), separator);
               i += 1;
           });

           // Execute query
           let query_str = r#"INSERT INTO schema_versions
               ($1)
               VALUES
                    $2
               ON CONFLICT DO UPDATE"#;

           let queryr = self.conn.execute(query_str, &vec![columns.join(", "), value_blocks_str]);

           match queryr {
               Ok(rows_upserted) => {
                   transactions_status.3 += rows_upserted;
               },
               Err(err) => {
                   error!("Error restoring schema version rows, error: {}", err.describe());
                   transactions_status.4 += value_tuples.len();
               }
           };

           // Print after action report
           error!("Finished restoring rows, database hopefully now in state it was before attempted dup resolve, \
                    {tot_restores}/{tot_deletes} deleted rows restored \
                    (row deletions: {deletes}, failed deletions: {bad_deletes}, restores: {restores}, failed restores: {bad_restores})",
                tot_restores=transactions_status.3 - transactions_status.4,
                tot_deletes=transactions_status.0 - transactions_status.1,
                deletes=transactions_status.0,
                bad_deletes=transactions_status.1,
                restores=transactions_status.3,
                bad_restores=transactions_status.4);

           return Err("disappearing_ver_row")
       };

        return Ok(correct_ver_row.unwrap())
    }
}
