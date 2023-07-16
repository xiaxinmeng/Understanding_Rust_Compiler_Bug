
src/librustc_resolve/lib.rs:254:13: 260:95 error: mismatched types [E0308]

src/librustc_resolve/lib.rs:254             struct_span_err!(resolver.session,

                                            ^

src/librustc_resolve/lib.rs:254:13: 260:95 help: run `rustc --explain E0308` to see a detailed explanation

src/librustc_resolve/lib.rs:254:13: 260:95 note: expected type `errors::DiagnosticBuilder<'a>`

src/librustc_resolve/lib.rs:254:13: 260:95 note:    found type `()`

src/librustc_resolve/lib.rs:263:13: 269:95 error: mismatched types [E0308]

src/librustc_resolve/lib.rs:263             struct_span_err!(resolver.session,

                                            ^

src/librustc_resolve/lib.rs:263:13: 269:95 help: run `rustc --explain E0308` to see a detailed explanation

src/librustc_resolve/lib.rs:263:13: 269:95 note: expected type `errors::DiagnosticBuilder<'a>`

src/librustc_resolve/lib.rs:263:13: 269:95 note:    found type `()`

src/librustc_resolve/lib.rs:443:13: 447:99 error: mismatched types [E0308]

src/librustc_resolve/lib.rs:443             struct_span_err!(resolver.session,

                                            ^

src/librustc_resolve/lib.rs:443:13: 447:99 help: run `rustc --explain E0308` to see a detailed explanation

src/librustc_resolve/lib.rs:443:13: 447:99 note: expected type `errors::DiagnosticBuilder<'a>`

src/librustc_resolve/lib.rs:443:13: 447:99 note:    found type `()`

error: aborting due to 3 previous errors
