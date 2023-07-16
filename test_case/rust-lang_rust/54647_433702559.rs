rust
    let c = if env::var("CARGO_PKG_NAME").unwrap() == "mycrate" {
        quote!( crate )
    } else {
        quote!( mycrate )
    };
