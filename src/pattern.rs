pub mod human {
    pub mod buf;
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
