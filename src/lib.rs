// db
pub use db::{cmd::DbCommand, DbPool};
pub mod db;
// serving
pub use serv::file::{file_service, serve_file, static_file_service};
pub use serv::human::query::HumanHttpQuery;
pub mod serv;
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
pub mod util;

pub mod init;
