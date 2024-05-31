pub mod human {
    pub mod buf {
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
    }
    pub mod own {
        #[derive(Clone, Debug, clap::Args)]
        pub struct HumanPatternOwn {
            // Id
            #[clap(long, short = 'i')]
            pub id: Option<i32>,

            // Name
            #[clap(long, short = 'n')]
            pub name: Option<Box<str>>,
            #[clap(long, short = 'N')]
            pub like_name: bool,

            // Surname
            #[clap(long, short = 's')]
            pub surname: Option<Box<str>>,
            #[clap(long, short = 'S')]
            pub like_surname: bool,

            /// Nickname. `-a` like alias.
            #[clap(long, short = 'a')]
            pub nickname: Option<Box<str>>,
            #[clap(long, short = 'A')]
            pub like_nickname: bool,
        }

        impl<'b, 'c, 'd> From<crate::HumanPatternBor<'b, 'c, 'd>> for HumanPatternOwn {
            fn from(bor: crate::HumanPatternBor<'b, 'c, 'd>) -> Self {
                let [name, surname, nickname] =
                    [bor.name, bor.surname, bor.nickname].map(|o| o.map(Box::<str>::from));
                HumanPatternOwn {
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
        impl From<crate::HumanPatternBuf> for HumanPatternOwn {
            fn from(bufferful: crate::HumanPatternBuf) -> Self {
                let [name, surname, nickname] =
                    [bufferful.name, bufferful.surname, bufferful.nickname]
                        .map(|o| o.map(Box::from));
                HumanPatternOwn {
                    id: bufferful.id,
                    name,
                    surname,
                    nickname,
                    like_name: bufferful.like_name,
                    like_surname: bufferful.like_surname,
                    like_nickname: bufferful.like_nickname,
                }
            }
        }
    }
    pub mod bor {
        #[derive(Debug, Clone)]
        pub struct HumanPatternBor<'b, 'c, 'd> {
            pub id: Option<i32>,
            // Name
            pub name: Option<&'b str>,
            pub like_name: bool,
            // Surname
            pub surname: Option<&'c str>,
            pub like_surname: bool,
            // Nickname
            pub nickname: Option<&'d str>,
            pub like_nickname: bool,
        }

        impl<'o> From<&'o crate::HumanPatternOwn> for HumanPatternBor<'o, 'o, 'o> {
            fn from(owned: &'o crate::HumanPatternOwn) -> Self {
                use std::ops::Deref;

                let [name, surname, nickname] = [&owned.name, &owned.surname, &owned.nickname]
                    .map(Option::as_ref)
                    .map(|o| o.map(Deref::deref));

                HumanPatternBor {
                    id: owned.id,
                    name,
                    surname,
                    nickname,
                    like_name: owned.like_name,
                    like_surname: owned.like_surname,
                    like_nickname: owned.like_nickname,
                }
            }
        }
        impl<'o> From<&'o crate::HumanPatternBuf> for HumanPatternBor<'o, 'o, 'o> {
            fn from(owned: &'o crate::HumanPatternBuf) -> Self {
                use std::ops::Deref;

                let [name, surname, nickname] = [&owned.name, &owned.surname, &owned.nickname]
                    .map(Option::as_ref)
                    .map(|o| o.map(Deref::deref));

                HumanPatternBor {
                    id: owned.id,
                    name,
                    surname,
                    nickname,
                    like_name: owned.like_name,
                    like_surname: owned.like_surname,
                    like_nickname: owned.like_nickname,
                }
            }
        }
    }
}
