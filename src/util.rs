//! Utilities that could not be categorized.

pub type NullableBool = diesel::sql_types::Nullable<diesel::sql_types::Bool>;
pub type BoxedSubfilter<T> = Box<
    dyn diesel::BoxableExpression<crate::schema::Human::table, diesel::mysql::Mysql, SqlType = T>,
>;

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

#[derive(Clone, Copy, strum::EnumIs, Debug, Eq, PartialEq, Hash)]
pub enum StrPatternKind {
    // Only 2 variants.
    Literal,
    Like,
    // No, more variants should be added, without careful update of `impl`.
}

impl StrPatternKind {
    pub fn from_like_flag(is_regex: bool) -> Self {
        if is_regex {
            Self::Like
        } else {
            Self::Literal
        }
    }
}
