rust
pub fn signal_describe(sig: i32) -> String;
pub fn signal_string_raw(sig: i32) -> Option<&'static str>;
pub fn signal_abbrev(sig: i32) -> Option<&'static str>;
