rust
OutputTypeParameterMismatch(
    Binder(<[closure@ICE.rs:43:24: 43:40] as std::ops::FnOnce<(&<for<'a> fn(&'a str) -> (&'a str, &'a str) {main::bar} as Parser<'_>>::Output,)>>, 
    Binder(<[closure@ICE.rs:43:24: 43:40] as std::ops::FnOnce<(&&str,)>>), 
    Sorts(
        ExpectedFound { 
            expected: &str, 
            found: <for<'a> fn(&'a str) -> (&'a str, &'a str) {main::bar} as Parser<'_>>::Output 
        }
    )
)
