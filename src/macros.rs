macro_rules! enum_str {
    {$($(#[$meta: meta])* pub enum $name: ident { $($variant: ident => $str: expr),+ $(,)? })+} => {
        $(
            $(#[$meta])*
            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum $name {
                $(
                    $variant,
                )+
            }

            impl ::std::str::FromStr for $name {
                type Err = ();

                fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                    let part = match s {
                        $(
                            $str => Self::$variant,
                        )+
                        _ => return Err(()),
                    };

                    Ok(part)
                }
            }
        )+
    }
}
