extern crate clap;

use clap::ArgMatches;

trait Command<MatchesMapObj> {
    fn from_matches(&self, matches: &ArgMatched) -> Result<MatchesMapObj, String>;
}