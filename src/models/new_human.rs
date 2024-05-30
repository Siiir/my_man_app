pub mod bor;
pub mod own;

use std::fmt::Debug;

/// Currently useless since it needs unstable feature or alternative trait bounds to be usefull.
pub trait NewHuman: Debug
// + diesel::Insertable<crate::schema::Human::table, Values: QueryFragment<_>>
{
}
