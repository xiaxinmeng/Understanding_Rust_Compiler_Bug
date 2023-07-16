plain
    Checking url v2.1.1
    Checking toml v0.5.7
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
error[E0599]: no variant named `IfLetDesugar` found for enum `MatchSource`
     |
     |
2000 |                 ref match_expr, ref match_arms, MatchSource::IfLetDesugar{ contains_else_clause: false }
     |                                                              ^^^^^^^^^^^^ help: there is a variant with a similar name: `AwaitDesugar`

error[E0599]: no variant named `IfLetDesugar` found for enum `MatchSource`
    |
    |
214 |             MatchSource::IfLetDesugar {
    |                          ^^^^^^^^^^^^ help: there is a variant with a similar name: `AwaitDesugar`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_lints`
