 rust
pub struct Utf8Error {
    // if there's an error, produce a sanitized output with replacement chars
    recovered: ~str, 

    // the byte position and exact sequence of each error
    errors: ~[(uint, ~[u8])], 
}

pub fn from_utf8_owned_explain(vv: ~[u8]) -> Result<~str, Utf8Error> { ... }

pub fn from_utf8_explain<'a>(v: &'a [u8]) -> Result<&'a str, Utf8Error> { ... }
