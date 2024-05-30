//! Provides utilities to form & execute the specific command user gives to this app.

/// App's subcommand that defines specific tasks you can give to this app.
#[derive(clap::Subcommand)]
pub enum AppSubcmd {
    /// Manages data associated with humans.  
    ///
    /// Has subcommands that manage human-data in different ways.
    Human {
        /// Subcommand of `human`, that defines what specific thing to do with human(s).
        #[command(subcommand)]
        subcmd: crate::app::HumanCmd,
    },
    /// Will cause app to serve its content.
    Serve(crate::app::ServeCmd),
    // Commands that could be implemented in the future:
    // Project {},
}
impl mma::DbCommand for AppSubcmd {
    type T = ();
    type E = anyhow::Error;

    fn exec_using(self, connection: &mut diesel::MysqlConnection) -> Result<Self::T, Self::E> {
        match self {
            AppSubcmd::Human { subcmd } => subcmd.exec_using(connection),
            AppSubcmd::Serve(subcmd) => subcmd.exec(),
        }
    }
}
