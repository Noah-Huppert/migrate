extern crate clap;

use clap::ArgMatches;

trait Command {
    fn from_matches(matches: &ArgMatched) -> Result<Command, String>;
    fn run(&self) -> Result<(), String>;
}