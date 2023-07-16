rust
Term::ident("foo"); // Make an identifier `foo`
Term::raw_ident("foo"); // Make a raw identifier `r#foo`
Term::lifetime("foo"); // Make a lifetime `'foo`, note the lack of starting `'` in the argument (*)
// Term::raw_lifetime("foo") // Maybe, some day
