#![doc = documentation!()]
#![warn(missing_docs)]

/// Crate level documentation for binary app.
///
/// Serves also as a parameter in construction of CLI interface,
///  so it is convenient to have it be inside macro.
///
/// # Examples
/// Use in attributes.
/// ```
///     #![doc = crate_doc!()]
/// ```
/// Use in normal code as a `&`[`str`] value.
/// ```
///     println!("You're welcome to use my {}", documentation!());
/// ```
#[macro_export]
macro_rules! documentation {
    () => {
        "CLI tool that facilitates management of my company's database.\n\
        \n\
        You can give commands to it like if you were giving them to a database system.\n\
        Yet these commands are shorter, safer and more adjusted to company's daily needs."
    };
}

pub use self::app::cmd::sub::AppSubcmd;
pub mod app;

use mma::DbCommand;

fn main() -> anyhow::Result<()> {
    mma::init::all();
    let app_args: app::Args = clap::Parser::parse();
    app_args.subcmd.execute()
}
