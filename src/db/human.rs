//! Defines database operations centered around human data management.

use crate::{db::table_name, models, schema};
use anyhow::Context;
use diesel::{
    BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};

/// Adds a new human to the database.
///
/// Prints the created record to stderr.
pub fn add(
    connection: &mut diesel::MysqlConnection,
    new_human: crate::models::NewHumanBor,
) -> Result<(), anyhow::Error> {
    connection
        .transaction(
            |connection: &mut diesel::MysqlConnection| -> anyhow::Result<()> {
                insert(connection, new_human)?;
                let inserted_human: models::Human =
                    crate::db::human::that_was_just_inserted(connection)?;
                tracing::info!(
                    "Successfully inserted human record:\n{:}",
                    crate::util::prin_table([&inserted_human])
                );
                Ok(())
            },
        )
        .with_context(|| {
            format!("Failed to add the new human to database. New human object: `{new_human:?}`")
        })
}

/// Find all humans in the database, who match the provided pattern/filter.
///
/// This function was previously intended to be used for finding a concrete person.
/// Yet finding a concrete person is often done by narrowing the criterion,
///  and so the narrowing of the search group.
/// So it meets the goal that this function can actually return many humans (the search group).
/// You can then try to narrow the search group by repetitive calls.
pub fn search_with_context(
    connection: &mut diesel::MysqlConnection,
    pattern: crate::HumanPatternBuf,
) -> anyhow::Result<Vec<models::Human>> {
    search(pattern, connection)
        .context(concat!(
            "Couldn't load records from table `",
            table_name::human!(),
            "`."
        ))
        .context("Search for humans failed.")
}

fn search(
    mut pattern: crate::HumanPatternBuf,
    connection: &mut diesel::prelude::MysqlConnection,
) -> Result<Vec<models::Human>, diesel::result::Error> {
    // Prepare filter
    let filter = {
        let f = [
            pattern.id_filter(),
            pattern.take_name_filter(),
            pattern.take_surname_filter(),
            pattern.take_nickname_filter(),
        ];
        // It has been spoiled by ".take_*" methods.
        std::mem::drop(pattern);
        f
    };
    let filter = filter
        .into_iter()
        .flatten()
        .reduce(|lhs, rhs| Box::new(lhs.and(rhs)));

    // Prepare query
    let query = schema::Human::table.select(models::Human::as_select());
    if let Some(filter) = filter {
        query.filter(filter).load(connection)
    } else {
        query.load(connection)
    }
}

/// Finds the human with the highest id number if exists.
pub fn that_was_just_inserted(
    connection: &mut diesel::MysqlConnection,
) -> Result<models::Human, anyhow::Error> {
    let out = schema::Human::table
        .order(schema::Human::id.desc())
        .select(models::Human::as_select())
        .first(connection)
        .context(concat!(
            "Failed to select the record that was just inserted into `",
            table_name::human!(),
            "` table."
        ))
        .context("Transaction is deemed unsuccessful and was rolled back.")?;
    Ok(out)
}

/// Inserts the human into the database.
///
/// Subprocedure of `add`. It focuses on doing the pure insertion logic.
/// Doesn't print the created database record and is therefore more low-level & user-unfriendly.
fn insert(
    connection: &mut diesel::MysqlConnection,
    new_human: crate::models::NewHumanBor,
) -> Result<(), anyhow::Error> {
    // The insertion query and its execution.
    let ins_rows_count = diesel::insert_into(crate::schema::Human::table)
        .values(new_human)
        .execute(connection)
        .context(concat!(
            "Failed to perform record insertion into `",
            table_name::human!(),
            "` table."
        ))?;
    // We are inserting only one human and are on success path, so this assertion should hold.
    debug_assert_eq!(ins_rows_count, 1);

    Ok(())
}
