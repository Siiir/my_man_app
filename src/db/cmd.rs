/// Represents database command.
///
/// Applicable for commands that need a database (in form of connection) to be executed.
pub trait DbCommand {
    type T;
    type E: Into<anyhow::Error>;

    // Required methods.

    fn exec_using(self, connection: &mut diesel::MysqlConnection) -> Result<Self::T, Self::E>
    where
        Self: Sized;

    // Provided methods

    /// Establishes connection. Then uses it to perform this command.
    ///
    /// A convenience method for `exec_using` that provides its argument automatically.
    fn execute(self) -> anyhow::Result<Self::T>
    where
        Self: Sized,
    {
        let mut connection = crate::db::establish_connection()?;
        self.exec_using(&mut connection).map_err(|e| e.into())
    }
}
