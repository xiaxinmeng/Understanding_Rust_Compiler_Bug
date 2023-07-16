
error[E0308]: mismatched types
    --> /Users/gilescope/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-174.0.0/attr.rs:1314:64
     |
1314 |                 Token::Literal(token::Lit::Str_(Symbol::intern(&escaped)), None)
     |                                                                ^^^^^^^^ expected str, found struct `std::str::EscapeDefault`
     |
     = note: expected type `&str`
                found type `&std::str::EscapeDefault<'_>`
