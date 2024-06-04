use core::fmt;

use serde::{Deserialize, Serialize};

use crate::util::{self, BoxedSubfilter, NullableBool};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HumanPatternBuf {
    // Id
    pub id: Option<i32>,

    // Name
    pub name: Option<String>,
    #[serde(default)]
    pub like_name: bool,

    // Surname
    pub surname: Option<String>,
    #[serde(default)]
    pub like_surname: bool,

    // Nickname
    pub nickname: Option<String>,
    #[serde(default)]
    pub like_nickname: bool,
}

impl HumanPatternBuf {
    // CRUD-R: Properties
    pub fn name_pattern_kind(&self) -> util::StrPatternKind {
        util::StrPatternKind::from_like_flag(self.like_name)
    }
    pub fn surname_pattern_kind(&self) -> util::StrPatternKind {
        util::StrPatternKind::from_like_flag(self.like_surname)
    }
    pub fn nickname_pattern_kind(&self) -> util::StrPatternKind {
        util::StrPatternKind::from_like_flag(self.like_nickname)
    }
    pub fn pattren_kind_for(&self, field_name: &str) -> util::StrPatternKind {
        match field_name {
                "name" => self.name_pattern_kind(),
                "surname" => self.surname_pattern_kind(),
                "nickname" => self.nickname_pattern_kind(),
                _ => panic!(
                    "Invalid field name for `{}`. Or field name hasn't been registered in this function.",
                    std::any::type_name::<Self>()
                ),
            }
    }
    pub fn id_filter(&mut self) -> Option<BoxedSubfilter<NullableBool>> {
        use diesel::ExpressionMethods;
        let taken_field = self.id.take();

        if let Some(field_val) = taken_field {
            let sf = crate::schema::Human::id.eq(field_val);
            Some(Box::new(diesel::NullableExpressionMethods::nullable(sf)))
        } else {
            None
        }
    }
    // CRUD-U: Stealing properties
    pub fn take_name_filter(&mut self) -> Option<BoxedSubfilter<NullableBool>> {
        {
            let taken_field = self.name.take();
            let pattern_kind = self.name_pattern_kind();
            use diesel::{ExpressionMethods, TextExpressionMethods};

            if let Some(field_val) = taken_field {
                Some(match pattern_kind {
                    util::StrPatternKind::Literal => {
                        let sf = crate::schema::Human::name.eq(field_val);
                        Box::new(diesel::NullableExpressionMethods::nullable(sf))
                    }
                    util::StrPatternKind::Like => {
                        let sf = crate::schema::Human::name.like(field_val);
                        Box::new(diesel::NullableExpressionMethods::nullable(sf))
                    }
                })
            } else {
                None
            }
        }
    }
    pub fn take_surname_filter(&mut self) -> Option<BoxedSubfilter<NullableBool>> {
        {
            let taken_field = self.surname.take();
            let pattern_kind = self.surname_pattern_kind();
            use diesel::{ExpressionMethods, TextExpressionMethods};

            if let Some(field_val) = taken_field {
                Some(match pattern_kind {
                    util::StrPatternKind::Literal => {
                        let sf = crate::schema::Human::surname.eq(field_val);
                        Box::new(diesel::NullableExpressionMethods::nullable(sf))
                    }
                    util::StrPatternKind::Like => {
                        let sf = crate::schema::Human::surname.like(field_val);
                        Box::new(diesel::NullableExpressionMethods::nullable(sf))
                    }
                })
            } else {
                None
            }
        }
    }
    pub fn take_nickname_filter(&mut self) -> Option<BoxedSubfilter<NullableBool>> {
        {
            let taken_field = self.nickname.take();
            let pattern_kind = self.nickname_pattern_kind();
            use diesel::{ExpressionMethods, TextExpressionMethods};

            if let Some(field_val) = taken_field {
                Some(match pattern_kind {
                    util::StrPatternKind::Literal => {
                        let sf = crate::schema::Human::nickname.eq(field_val);
                        Box::new(diesel::NullableExpressionMethods::nullable(sf))
                    }
                    util::StrPatternKind::Like => {
                        let sf = crate::schema::Human::nickname.like(field_val);
                        Box::new(diesel::NullableExpressionMethods::nullable(sf))
                    }
                })
            } else {
                None
            }
        }
    }
}

// CRUD-C: Conversions into `Self`
impl From<crate::HumanHttpQuery> for HumanPatternBuf {
    fn from(value: crate::HumanHttpQuery) -> Self {
        let crate::HumanHttpQuery {
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
impl<'b, 'c, 'd> From<crate::HumanPatternBor<'b, 'c, 'd>> for HumanPatternBuf {
    fn from(bor: crate::HumanPatternBor<'b, 'c, 'd>) -> Self {
        let [name, surname, nickname] =
            [bor.name, bor.surname, bor.nickname].map(|o| o.map(String::from));
        HumanPatternBuf {
            id: bor.id,
            name,
            surname,
            nickname,
            like_name: bor.like_name,
            like_surname: bor.like_surname,
            like_nickname: bor.like_nickname,
        }
    }
}
impl From<crate::HumanPatternOwn> for HumanPatternBuf {
    fn from(own: crate::HumanPatternOwn) -> Self {
        let [name, surname, nickname] =
            [own.name, own.surname, own.nickname].map(|o| o.map(String::from));
        HumanPatternBuf {
            id: own.id,
            name,
            surname,
            nickname,
            like_name: own.like_name,
            like_surname: own.like_surname,
            like_nickname: own.like_nickname,
        }
    }
}
// CRUD-R: Display

impl fmt::Display for HumanPatternBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(id) = &self.id {
            write!(f, "{id}")?;
        }
        let expressions = [
            ("name", self.like_name, self.name.as_ref()),
            ("surname", self.like_surname, self.surname.as_ref()),
            ("nickname", self.like_nickname, self.nickname.as_ref()),
        ];
        for (field_name, is_like_pattern, field_val) in expressions {
            if let Some(field_val) = field_val {
                f.write_str(field_name)?;
                f.write_str(if is_like_pattern { " LIKE " } else { " = " })?;
                write!(f, "{field_val}")?;
            }
        }
        Ok(())
    }
}
