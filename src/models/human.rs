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
