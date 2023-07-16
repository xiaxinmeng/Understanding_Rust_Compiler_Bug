
            let _ = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["rec "],
                    &match (&recursor,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                    },
                ));
                res
            };
            let _ = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["len: "],
                    &match (&tc.env_store().rec_rules.len(),) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            };
