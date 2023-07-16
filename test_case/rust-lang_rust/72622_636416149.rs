rust

    macro_rules! visit_number {
        ($l: expr, $to: ident, $ty: ident) => {
            if $l.token == Token::$to {
                paste::expr! {
                    let result = $l.slice().parse().unwrap();
                    $l.advance();
                    visitor.[<visit_$ty>](result)
                }
            } else {
                unexpected_token!($l, "<number>")
            }
        };
    }
