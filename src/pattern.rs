pub mod human {
    pub mod buf {
        use derive_more::Constructor;

        #[derive(Clone, Debug, clap::Args, Constructor)]
        pub struct HumanPatternBuf {
            #[clap(long, short)]
            pub id: Option<i32>,
            #[clap(long, short)]
            pub name: Option<String>,
            #[clap(long, short)]
            pub surname: Option<String>,
            /// `-a` like alias.
            #[clap(long, short = 'a')]
            pub nickname: Option<String>,
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
                }
            }
        }
    }
    pub mod own {
        #[derive(Clone, Debug, clap::Args)]
        pub struct HumanPatternOwn {
            #[clap(long, short)]
            pub id: Option<i32>,
            #[clap(long, short)]
            pub name: Option<Box<str>>,
            #[clap(long, short)]
            pub surname: Option<Box<str>>,
            /// `-a` like alias.
            #[clap(long, short = 'a')]
            pub nickname: Option<Box<str>>,
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
                }
            }
        }
    }
    pub mod bor {
        #[derive(Debug, Clone)]
        pub struct HumanPatternBor<'b, 'c, 'd> {
            pub id: Option<i32>,
            pub name: Option<&'b str>,
            pub surname: Option<&'c str>,
            pub nickname: Option<&'d str>,
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
                }
            }
        }
    }
}
