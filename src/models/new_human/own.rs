use derive_more::Constructor;

#[derive(clap::Args, Constructor, Debug, Clone, diesel::Insertable)]
#[diesel(table_name = crate::schema::Human)]
pub struct NewHumanOwn {
    pub name: String,
    pub surname: String,
    pub nickname: Option<String>,
}
impl super::NewHuman for NewHumanOwn {}

impl<'a, 'b, 'c> From<crate::models::NewHumanBor<'a, 'b, 'c>> for NewHumanOwn {
    fn from(bor: crate::models::NewHumanBor<'a, 'b, 'c>) -> Self {
        NewHumanOwn::new(
            bor.name.to_owned(),
            bor.surname.to_owned(),
            bor.nickname.map(ToOwned::to_owned),
        )
    }
}
