
       * Source String: `"hel\u{0x6c}o"`
       * `.to_string()` -> hello
       * foo.sub_span(2..4) -> `"hel\u{0x6c}o"`
                                   ^^^^^^^^^
       