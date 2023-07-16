rust
// snip
            opt(tuple!(
                tag("@"),
                map(
                    take_while1(none_of(" ".chars())),
                    delimited(
                        tag(";"),
                        tuple!(
                            opt(tag("+")),
                            opt(tuple(host, tag("/"))),
                            take_while(alt2!(letter, number, tag("-"))),
                            opt(tuple(
                                tag("="),
                                take_while(none_of("\0\0xa\0x0d; ".chars()))
                            ))
                        )
                    )
                ),
                space
            )),
// snip
