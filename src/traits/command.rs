use clap::{ArgMatches, App};

/// Logic needed to run command via cli
pub trait Command {
    /// Create a new command
    ///
    /// - *returns* - Created command
    fn new() -> Command;

    /// Makes clap cli configuration object which represents signature of logic (arguments and such)
    ///
    /// - *returns* - Constructed clap configuration
    fn mk_cli_config() -> App;

    /// Parse command line arguments provided by clap
    ///
    /// - `args` - Arguments parsed by clap
    /// - *returns* - Empty tuple on success, error code on fail
    ///
    /// Typically when parsing arguments the method will also assign arguments values to fields in a
    /// corresponding structure.
    fn parse_args(&self, args: &ArgMatches) -> Result<(), String>;

    /// Executes core logic of command
    ///
    /// - *returns* - Empty tuple on success, error code on fail
    fn run(&self) -> Result<(), String>;
}