rust
assert_eq!(
        ".,\"foo1bar\".,';".trim_matches(|c| char::is_ascii_punctuation(&c)),
        "foo1bar"
    );
