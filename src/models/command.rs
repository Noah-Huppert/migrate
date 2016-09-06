extern crate clap;

use clap::ArgMatches;

pub trait Command <CmdT> {
    fn from_matches(matches: &ArgMatches) -> Result<CmdT, String>;
    fn run(&self) -> Result<(), String>;
}