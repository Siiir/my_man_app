use anyhow::Context;

// db
pub use db::cmd::DbCommand;
pub mod db;
// serving
pub mod serv;
// models
pub mod models;
// schema
#[allow(non_snake_case)]
pub mod schema;
// pattern
pub use pattern::human::{bor::HumanPatternBor, buf::HumanPatternBuf, own::HumanPatternOwn};
use tracing_subscriber::FmtSubscriber;
pub mod pattern;
// name
pub mod name_of {
    //! Contains names of things that haven't been categorized.
    pub mod env_var {
        //! Contains names of envirnonment variables.
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __db_url {
            () => {
                "DATABASE_URL"
            };
        }
        pub use __db_url as db_url;
    }
}

// Util
pub mod util;

/// Initializes this app.
///
/// Makes sure that all environment stuff is accessible in standard way.
pub fn init() {
    // Use of .env file.
    _ = dotenvy::dotenv();

    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // will be written to stdout.
        .with_max_level(tracing::Level::TRACE)
        // completes the builder.
        .finish();

    match tracing::subscriber::set_global_default(subscriber).context(
        "Correct logs may not be produced, because app could not set global log subscriber.",
    ) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("WARNING: {e}");
        }
    }
}
