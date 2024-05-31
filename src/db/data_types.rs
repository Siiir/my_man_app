pub use nullable_string::NullableString;
pub mod nullable_string {
    /// Contains the full definition of nullable string stuff and related functionalities.
    use derive_more::{From, Into};
    use diesel::mysql::Mysql;
    use std::{fmt::Display, str::FromStr};

    /// Mimics nullable UTF-8 string from SQL.
    ///
    /// ## Posible future refactorings.
    /// This could be potentially abstracted into `Nullable<T: Display + diesel::Querable>`.
    #[derive(From, Into, Debug, Default, Clone)]
    pub struct NullableString(Option<String>);
    impl NullableString {
        /// Null variant of `Self`.
        pub const NULL: Self = Self(None);
        /// Display for frontend that supports colors, bolding, italic etc.
        pub fn pretty_display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Delegates display to string or colored string.
            if let Some(string) = &self.0 {
                Display::fmt(string, f)
            } else {
                use colored::Colorize;
                // This delegation allowes for clear distinction between `null` and `"null"`.
                // This code part makes this function display `self` prettyly.
                Display::fmt(&"null".bold().red(), f)
            }
        }
    }

    /// Aims to mimic SQL comparisons of nullables in contrast to Rusty way of comparing.
    impl PartialEq for NullableString {
        fn eq(&self, other: &Self) -> bool {
            if let [Some(lhs), Some(rhs)] = [&self.0, &other.0] {
                lhs == rhs
            } else {
                false
            }
        }
    }
    /// Aims to mimic SQL comparisons of nullables in contrast to Rusty way of comparing.
    impl PartialOrd for NullableString {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if let [Some(lhs), Some(rhs)] = [&self.0, &other.0] {
                lhs.partial_cmp(rhs)
            } else {
                None
            }
        }
    }
    /// This is "ugly" display, which doesn't distinguish between `null` and `"null"`.
    ///
    /// Yet it often works as expected. In different environments, where colorful display fails.
    impl Display for NullableString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use std::ops::Deref;
            Display::fmt(self.0.as_ref().map(String::deref).unwrap_or("null"), f)
        }
    }

    // Simple delegations.

    impl FromStr for NullableString {
        type Err = std::convert::Infallible;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(s.into())
        }
    }
    impl From<&str> for NullableString {
        fn from(value: &str) -> Self {
            Some(value.to_owned()).into()
        }
    }
    impl From<String> for NullableString {
        fn from(value: String) -> Self {
            Some(value).into()
        }
    }
    impl From<NullableString> for String {
        fn from(value: NullableString) -> Self {
            value.0.unwrap_or_else(|| "null".into())
        }
    }

    // Diesel traits, that are in fact simple delegations on the semantic level.

    impl diesel::Queryable<diesel::sql_types::Text, Mysql> for NullableString {
        type Row = String;

        fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
            Ok(row.into())
        }
    }
    impl diesel::Queryable<diesel::sql_types::Nullable<diesel::sql_types::Text>, Mysql>
        for NullableString
    {
        type Row = Option<String>;

        fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
            Ok(row.into())
        }
    }
    impl diesel::deserialize::FromSql<diesel::sql_types::Text, Mysql> for NullableString {
        fn from_sql(
            bytes: <Mysql as diesel::backend::Backend>::RawValue<'_>,
        ) -> diesel::deserialize::Result<Self> {
            use diesel::deserialize::FromSql;
            let inner = FromSql::from_sql(bytes)?;

            Ok(NullableString(inner))
        }
    }
}

#[cfg(test)]
mod test {
    mod trait_impls {
        #[allow(non_snake_case)]
        mod Display {
            use crate::db::data_types::NullableString;
            #[test]
            fn pad_non_null() {
                let non_null_str: NullableString = "cat".into();
                let padded = format!("{non_null_str:*<5}");
                assert_eq!(padded, "cat**");
            }
            #[test]
            fn pad_null() {
                let null_str: NullableString = None.into();
                let padded = format!("{null_str:*<5}");
                let expected = format!("{}", "null*");
                assert_eq!(padded, expected);
            }
        }
    }
}
