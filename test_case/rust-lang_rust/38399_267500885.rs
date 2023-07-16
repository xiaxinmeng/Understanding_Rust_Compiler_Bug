
    OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect() // wrong
    OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect()   // good
