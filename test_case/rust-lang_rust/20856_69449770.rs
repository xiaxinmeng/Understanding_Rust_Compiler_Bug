 rust
        impl fmt::Show for SqlState {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                let s = match *self {
                    $(SqlState::$error => stringify!($error),)+
                    SqlState::Unknown(ref s) => return write!(fmt, "Unknown({:?})", s),
                };
                fmt.write_str(s)
            }

