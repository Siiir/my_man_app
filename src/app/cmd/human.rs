//! Provides utilities to form & execute commands that manage human data.

/// Defines commands, that manage human data.
#[derive(Clone, clap::Subcommand)]
pub enum HumanCmd {
    /// Adds a human to the database, with specific provided parameters.
    Add(mma::models::NewHumanOwn),
    /// Finds a human in the database.
    ///
    /// The search is based on provided parameters that form a pattern.
    /// The more parameters you give, the more specific/narrow the pattern will be.
    Find(mma::HumanPatternOwn),
    // Commands that could be implemented in the future:
    // Info(mma::HumanPatternOwn),
    // Forget { id: String },
}

impl mma::DbCommand for HumanCmd {
    type T = ();
    type E = anyhow::Error;

    fn exec_using(self, connection: &mut diesel::MysqlConnection) -> anyhow::Result<Self::T> {
        match self {
            HumanCmd::Add(new_human) => mma::db::human::add(connection, (&new_human).into()),
            HumanCmd::Find(human_pattern) => {
                let human_records = mma::db::human::find(connection, human_pattern.into())?;
                println!("{}", mma::util::prin_table(human_records));
                Ok(())
            }
        }
    }
}
