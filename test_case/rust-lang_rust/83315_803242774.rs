
macro_rules! stringy_enum {
    (pub enum $type:ident (parse error $error_type:ident :: $error_variant:ident) {
        $( $variant:ident = $name:tt ),* ,
    }) => {
        #[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
        pub enum $type {
            $( $variant ),*
        }

        impl $type {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $( $type :: $variant => $name ),*
                }
            }
        }

        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl std::str::FromStr for $type {
            type Err = $error_type;
            fn from_str(s: &str) -> std::result::Result<$type, $error_type> {
                Ok(match s {
                    $( $name => $type :: $variant ),* ,
                    _ => return Err($error_type :: $error_variant(s.to_string())),
                })
            }
        }
    }
}
