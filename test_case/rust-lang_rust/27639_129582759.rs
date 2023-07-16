 rust
    {
        let result = match ::std::iter::IntoIterator::into_iter(<head>) {
            mut iter => {
                [opt_ident]: loop {
                    match ::std::iter::Iterator::next(&mut iter) {
                        ::std::option::Option::Some(<pat>) => <body>,
                        ::std::option::Option::None => break
                    }
                }
            }
        };
        result
    }
