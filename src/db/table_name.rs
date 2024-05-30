//! Organizes constants representing table names.
//!
//! Currently their values need to be manually updated if real table names change..

#[macro_export]
#[doc(hidden)]
macro_rules! __human {
    () => {
        "Human"
    };
}
pub use __human as human;
