//! Run the app

use crate::app::args::Args;
use crate::app::config::Config;

/// Run everything.
///
/// Steps:
///
///   * Initialize configuration.
///
///   * Initialize arguments.
///
/// Example:
///
/// ```
/// run();
/// ```
///
pub(crate) fn run() -> Result<(), Error> {
    trace!("run");
    let args: Args = crate::app::clap::args();
    if args.test { println!("{:#?}", args); }
    let config: Config = crate::app::confy::config()?;
    if args.log_level == Some(::log::Level::Trace) { println!("{:?}", config); }
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("Config  ➡ {0:?}")]
    Config(#[from] ::confy::ConfyError)

}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_run() {
        //TODO
    }

}
