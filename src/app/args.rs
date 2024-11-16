//! Args for the application.
//!
//! These args correspond to the matches in the file `clap.rs`.
//! 
//! We favor our convention of doing args in a file named `args.rs`,
//! rather than elsewhere, because we like the separation of concerns.
//!
//! Because the args are in their own file, we're able to be more flexible,
//! such as being able to start our app with a GUI, or headless runtime, etc.
//!
#[derive(Debug)]
pub struct Args {

    /// Test flag that sets whether the app prints diagnostics.
    /// Example: true means print diagnostics.
    pub(crate) test: bool,

    /// Log level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.
    /// Example: 5 means print debug diagnostics.
    pub(crate) log_level: Option<::log::Level>,

}

impl std::default::Default for Args {
    fn default() -> Self { Self {
        test: false,
        log_level: None,
    }}
}
