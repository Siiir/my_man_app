//! Organizes functionalities designed for this binary
//!  app, that may not be reusable by other binaries.

pub use args::Args;
pub mod args;
pub use cmd::human::HumanCmd;
pub mod cmd;
