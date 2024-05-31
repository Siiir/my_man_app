pub mod human {
    pub mod buf {
        use derive_more::Constructor;

        #[derive(Clone, Debug, clap::Args, Constructor)]
        pub struct HumanPatternBuf {
            // Id
            #[clap(long, short = 'i')]
            pub id: Option<i32>,

            // Name
            #[clap(long, short = 'n')]
            pub name: Option<String>,
            #[clap(long, short = 'N')]
            pub name_is_re: bool,

            // Surname
            #[clap(long, short = 's')]
            pub surname: Option<String>,
            #[clap(long, short = 'S')]
            pub surname_is_re: bool,

            /// Nickname. `-a` like alias.
            #[clap(long, short = 'a')]
            pub nickname: Option<String>,
            #[clap(long, short = 'A')]
            pub nickname_is_re: bool,
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
                    name_is_re: bor.name_is_re,
                    surname_is_re: bor.surname_is_re,
                    nickname_is_re: bor.nickname_is_re,
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
                    name_is_re: own.name_is_re,
                    surname_is_re: own.surname_is_re,
                    nickname_is_re: own.nickname_is_re,
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
            pub name_is_re: bool,

            // Surname
            #[clap(long, short = 's')]
            pub surname: Option<Box<str>>,
            #[clap(long, short = 'S')]
            pub surname_is_re: bool,

            /// Nickname. `-a` like alias.
            #[clap(long, short = 'a')]
            pub nickname: Option<Box<str>>,
            #[clap(long, short = 'A')]
            pub nickname_is_re: bool,
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
                    name_is_re: bor.name_is_re,
                    surname_is_re: bor.surname_is_re,
                    nickname_is_re: bor.nickname_is_re,
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
                    name_is_re: bufferful.name_is_re,
                    surname_is_re: bufferful.surname_is_re,
                    nickname_is_re: bufferful.nickname_is_re,
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
            pub name_is_re: bool,
            // Surname
            pub surname: Option<&'c str>,
            pub surname_is_re: bool,
            // Nickname
            pub nickname: Option<&'d str>,
            pub nickname_is_re: bool,
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
                    name_is_re: owned.name_is_re,
                    surname_is_re: owned.surname_is_re,
                    nickname_is_re: owned.nickname_is_re,
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
                    name_is_re: owned.name_is_re,
                    surname_is_re: owned.surname_is_re,
                    nickname_is_re: owned.nickname_is_re,
                }
            }
        }
    }
}
