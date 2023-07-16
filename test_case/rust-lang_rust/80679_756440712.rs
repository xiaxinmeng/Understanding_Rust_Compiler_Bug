plain
    Checking url v2.1.1
    Checking toml v0.5.7
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.51 (/checkout/src/tools/clippy/clippy_lints)
error: expected one of `,` or `}`, found `(`
   --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:120:29
    |
119 |                     Some(ty::PredicateKind::Trait(pred, _)) if pred.def_id() != sized_trait {
    |                                                                                 ----------- while parsing this struct
120 |                         Some(pred)
    |                             ^ expected one of `,` or `}`

error: expected one of `.`, `=>`, `?`, or an operator, found `,`
   --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:121:22
121 |                     },
121 |                     },
    |                      ^ expected one of `.`, `=>`, `?`, or an operator

error[E0282]: type annotations needed for `&&T`
   --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:165:50
    |
165 |                 let preds = preds.iter().filter(|t| t.self_ty() == ty).collect::<Vec<_>>();
    |                                                  ^ consider giving this closure parameter the explicit type `&&T`, where the type parameter `T` is specified
    |
    = note: type must be known at this point
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0282`.
error: could not compile `clippy_lints`
