use crate::util;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HumanHttpQuery {
    // Id
    pub id: Option<i32>,

    // Name
    pub name: Option<String>,
    #[serde(default, deserialize_with = "util::bool_from_checkbox")]
    pub like_name: bool,

    // Surname
    pub surname: Option<String>,
    #[serde(default, deserialize_with = "util::bool_from_checkbox")]
    pub like_surname: bool,

    // Nickname
    pub nickname: Option<String>,
    #[serde(default, deserialize_with = "util::bool_from_checkbox")]
    pub like_nickname: bool,
}
impl From<crate::HumanPatternBuf> for crate::HumanHttpQuery {
    fn from(value: crate::HumanPatternBuf) -> Self {
        let crate::HumanPatternBuf {
            id,
            name,
            like_name,
            surname,
            like_surname,
            nickname,
            like_nickname,
        } = value;
        Self {
            id,
            name,
            like_name,
            surname,
            like_surname,
            nickname,
            like_nickname,
        }
    }
}
