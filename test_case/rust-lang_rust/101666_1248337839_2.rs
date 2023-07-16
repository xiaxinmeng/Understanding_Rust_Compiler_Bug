rust
macro_rules! get_default_provider {
    ([][$name:ident]) => {
        ()
    };

    ([(separate_provide_extern) $($rest:tt)*][$name:ident]) => {
        |_, key| bug!(
            "`tcx.{}({:?})` unsupported by {} crate;", // Removed a chunk of the message here just to make this more readable.
            stringify!($name),
            if key.query_crate_is_local() { "local" } else { "extern" },
            stringify!($name),
        )
    };
    ([$other:tt $($modifiers:tt)*][$($args:tt)*]) => {
        get_default_provider!([$($modifiers)*][$($args)*])
    };
}

