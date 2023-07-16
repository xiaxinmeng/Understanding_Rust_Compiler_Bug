plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0609]: no field `feature` on type `&rustc_attr::Stability`
   --> src/librustdoc/clean/inline.rs:359:53
    |
359 |                 if stab.level.is_unstable() && stab.feature == sym::rustc_private {
    |
    = note: available fields are: `level`

error[E0609]: no field `feature` on type `&rustc_attr::Stability`
error[E0609]: no field `feature` on type `&rustc_attr::Stability`
   --> src/librustdoc/clean/inline.rs:391:53
    |
391 |                 if stab.level.is_unstable() && stab.feature == sym::rustc_private {
    |
    = note: available fields are: `level`


error[E0609]: no field `feature` on type `&&&rustc_attr::Stability`
   --> src/librustdoc/html/render/mod.rs:646:29
    |
646 |         .filter(|stab| stab.feature != sym::rustc_private)
    |
    = note: available fields are: `level`


error[E0609]: no field `feature` on type `&&rustc_attr::Stability`
   --> src/librustdoc/html/render/mod.rs:647:39
    |
647 |         .map(|stab| (stab.level, stab.feature))
    |
    = note: available fields are: `level`

error[E0027]: pattern does not mention field `feature`
error[E0027]: pattern does not mention field `feature`
   --> src/librustdoc/html/render/mod.rs:804:48
    |
804 |         (Some(v), Some(ConstStability { level: StabilityLevel::Stable { since }, .. }))
    |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `feature`
help: include the missing field in the pattern
    |
    |
804 |         (Some(v), Some(ConstStability { level: StabilityLevel::Stable { since, feature }, .. }))
    |                                                                              ^^^^^^^^^^^
help: if you don't care about this missing field, you can explicitly ignore it
    |
804 |         (Some(v), Some(ConstStability { level: StabilityLevel::Stable { since, .. }, .. }))


error[E0026]: struct `ConstStability` does not have a field named `feature`
   --> src/librustdoc/html/render/mod.rs:816:82
    |
816 |             Some(ConstStability { level: StabilityLevel::Unstable { issue, .. }, feature, .. }),
    |                                                                                  ^^^^^^^ struct `ConstStability` does not have this field

error[E0609]: no field `feature` on type `&&rustc_attr::Stability`
   --> src/librustdoc/html/render/print_item.rs:420:45
    |
420 |         .map(|s| s.level.is_unstable() && s.feature != sym::rustc_private)
    |
    = note: available fields are: `level`

Some errors have detailed explanations: E0026, E0027, E0609.
