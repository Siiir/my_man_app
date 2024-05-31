use derive_more::Constructor;

#[derive(Constructor, Debug, tabled::Tabled, Clone, diesel::Queryable, diesel::Selectable)]
#[diesel(table_name = crate::schema::Human)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Human {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub nickname: crate::db::data_types::NullableString,
}

impl<'h> IntoIterator for Human {
    type Item = String;

    type IntoIter = std::array::IntoIter<String, 4>;

    fn into_iter(self) -> Self::IntoIter {
        let Self {
            id,
            name,
            surname,
            nickname,
        } = self;
        [id.to_string(), name, surname, nickname.into()].into_iter()
    }
}
