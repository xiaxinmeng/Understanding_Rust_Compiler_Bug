
$ rg 'impl \$[^ ]+ for' -trust src
src/libcore/slice/mod.rs
2594:            impl $traitname for $ty { }

src/test/run-pass/hygiene/trait_items.rs
20:    impl $T for () {}
