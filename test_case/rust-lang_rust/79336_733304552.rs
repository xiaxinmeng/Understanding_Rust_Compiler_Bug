
❯ rg -i '(oibit|opt-?in,?[ |_]?built-?in)' -g '!src/tools/rust-analyzer'
compiler/rustc_feature/src/removed.rs
74:    /// Allows features specific to OIBIT (now called auto traits).
76:    (removed, optin_builtin_traits, "1.0.0", Some(13231), None,

compiler/rustc_feature/src/active.rs
216:    /// Renamed from `optin_builtin_traits`.

compiler/rustc_span/src/symbol.rs
773:        optin_builtin_traits,

compiler/rustc_error_codes/src/error_codes/E0380.md
12:the [opt-in builtin traits RFC][RFC 19].

compiler/rustc_error_codes/src/error_codes/E0192.md
18:information see the [opt-in builtin traits RFC][RFC 19].

compiler/rustc_data_structures/src/lib.rs
18:#![cfg_attr(bootstrap, feature(optin_builtin_traits))]

src/tools/cargo/tests/testsuite/custom_target.rs
65:        // Requires features no_core, lang_items, optin_builtin_traits
87:                #![feature(optin_builtin_traits)]

library/std/src/lib.rs
289:#![cfg_attr(bootstrap, feature(optin_builtin_traits))]

library/core/src/lib.rs
122:#![cfg_attr(bootstrap, feature(optin_builtin_traits))]

library/rtstartup/rsbegin.rs
17:#![cfg_attr(bootstrap, feature(optin_builtin_traits))]

library/rtstartup/rsend.rs
5:#![cfg_attr(bootstrap, feature(optin_builtin_traits))]

library/proc_macro/src/lib.rs
31:#![cfg_attr(bootstrap, feature(optin_builtin_traits))]

library/alloc/src/lib.rs
115:#![cfg_attr(bootstrap, feature(optin_builtin_traits))]

RELEASES.md
7837:* The final bits of [OIBIT landed][oibit-final], meaning that traits
7941:[oibit-final]: https://github.com/rust-lang/rust/pull/21689
8117:      footguns and are collectively known as [opt-in built-in
8118:      traits][oibit] (though `Sync` and `Send` will soon become pure
8225:[oibit]: https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md
