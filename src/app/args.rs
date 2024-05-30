//! Defines arguments for this app.

/// Contains all this app's arguments.
#[derive(clap::Parser)]
#[command(author, version, about, long_about = crate::documentation!())]
pub struct Args {
    /// Defines specific subcommands of this app.
    #[command(subcommand)]
    pub subcmd: crate::AppSubcmd,
}
