//! Utilities that could not be categorized.

use serde::de::{self, Unexpected, Visitor};
use serde::Deserializer;
use std::fmt;

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

pub fn bool_from_checkbox<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean or a string representing a boolean")
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match value {
                "true" => Ok(true),
                "on" => Ok(true),
                "false" => Ok(false),
                "off" => Ok(false),
                "" => Ok(true), // Handle the mere presence of parameter without value
                _ => Err(E::invalid_value(Unexpected::Str(value), &self)),
            }
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(BoolVisitor)
        }
    }

    deserializer.deserialize_any(BoolVisitor)
}
