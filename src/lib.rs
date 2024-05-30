// db
pub use db::cmd::DbCommand;
pub mod db;
// models
pub mod models;
// schema
#[allow(non_snake_case)]
pub mod schema;
// pattern
pub use pattern::human::{bor::HumanPatternBor, buf::HumanPatternBuf, own::HumanPatternOwn};
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
pub mod util {
    //! Utilities that could not be categorized.

    /// Returns a printable table.
    ///
    /// Delibrate typing error in function name to make it more pleasant to read.
    pub fn prin_table<I, T>(records: I) -> tabled::Table
    where
        I: IntoIterator<Item = T>,
        T: tabled::Tabled,
    {
        let mut table = tabled::Table::new(records);
        table.with(tabled::settings::Style::markdown());
        table
    }
}

/// Initializes this app.
///
/// Makes sure that all environment stuff is accessible in standard way.
pub fn init() {
    // Use of .env file.
    drop(dotenvy::dotenv())
}
