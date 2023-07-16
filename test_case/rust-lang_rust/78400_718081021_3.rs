rust")]
    /// fn has_ampersand(v: &OsStr) -> Result<(), String> {
    ///     if v.as_bytes().iter().any(|b| *b == b'&') { return Ok(()); }
    ///     Err(String::from("The value did not contain the required & sigil"))
    /// }
