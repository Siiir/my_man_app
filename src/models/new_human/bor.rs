use std::ops::Deref;

use derive_more::Constructor;

#[derive(Constructor, Debug, Clone, Copy, diesel::Insertable)]
#[diesel(table_name = crate::schema::Human)]
pub struct NewHumanBor<'a, 'b, 'c> {
    pub name: &'a str,
    pub surname: &'b str,
    pub nickname: Option<&'c str>,
}
impl<'a, 'b, 'c> super::NewHuman for NewHumanBor<'a, 'b, 'c> {}

impl<'o> From<&'o crate::models::NewHumanOwn> for NewHumanBor<'o, 'o, 'o> {
    fn from(owned: &'o crate::models::NewHumanOwn) -> Self {
        Self {
            name: &owned.name,
            surname: &owned.surname,
            nickname: owned.nickname.as_ref().map(Deref::deref),
        }
    }
}
