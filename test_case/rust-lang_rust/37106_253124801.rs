 rust
$(
    let $name = match try!(visitor.visit()) {
        Some(value) => value,
        None => { return Err(Error::end_of_stream()); }
    };
)+;

try!(visitor.end());

Ok(($($name,)+))
