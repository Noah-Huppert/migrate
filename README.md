Project Status: Active Development
# Migrate
A dead simple database migrator

# Commands
Migrate provides 2 different commands, `run` and `create`.

## `run`
The run command runs a series of database migrations on a specified database.

### `--target/-t` option
Specifies the database schema version that Migrate will attempt to reach by running migrations.

### `--migrations-dir/-d` option
The directory to look for migrations in. Defaults to `migrations`.

### `--config/-c` option
This config option specifies an `.ini` file to parse for database connection information. This file can contain specific
sections for different application environments (ex., `production`, `test`, `debug`) or no sections at all.

If the provided `.ini` file contains sections one must also specify the `--environment/-e` option. If left out the
following error will be returned:

```
No config for environment "None"
```

See `test/database.ini` for a sample `.ini` file.

### `--environment/-e` option
Specifies which section in the provided `.ini` file to use.
`--config/-c` option must be provided when in use.

### `--host/-h`, `--user/-u`, and `--password/-p` options
These command line options provide the information needed to connect to the database Migrate will run on.
Values provided with these command line options will override any set in a specified `.ini` file.

### `--backup/-b` option
This option specifies when in the migration process backups should take place.

## `create`
The create command places boilerplate migration files into the specified directory

### Usage
```
create <migration name>
```
Where `migration name` is a short descriptor of the migration. Think of it as a commit message in git.

### `--migrations-dir/-d` option
The directory to put the new migration in. Defaults to `migrations`.

# Migration structure
A typical migration would look as such

```
|- migrations        <-- `--migrations-dir/-d`
|--- add-posts-table <-- Short descriptive name of migration
|----- version      <-- Migration configuration file
|----- up.rs         <-- Rust file to run when performing the migration
|----- down.rs       <-- Rust file which reverses changes made in up.rs

---

# migrations/add-posts-table/version
2                    <-- Specifies which schema version the migration provides

< -- Run SQL queries and other complex logic in Rust -- >

# migrations/add-posts-table/up.rs
extern crate postgres;

use postgres::Connection;

fn run(conn: *postgres::Connection) {
	conn.execute("CREATE TABLE posts;");
}

# migrations/down.rs
extern crate postgres;

use postgres::Connection;

fn run(conn: *postgres::Connection) {
    conn.execute("DROP TABLE posts;");
}
```

