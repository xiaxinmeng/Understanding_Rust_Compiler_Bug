plain
    Checking url v2.1.1
    Checking toml v0.5.7
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
error[E0027]: pattern does not mention field `let_span`
     |
     |
1997 |                 ref match_expr, ref match_arms, MatchSource::IfLetDesugar{ contains_else_clause: false }
     |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `let_span`
help: include the missing field in the pattern
     |
     |
1997 |                 ref match_expr, ref match_arms, MatchSource::IfLetDesugar{ contains_else_clause: false, let_span }
     |                                                                                                       ^^^^^^^^^^^^
help: if you don't care about this missing field, you can explicitly ignore it
     |
1997 |                 ref match_expr, ref match_arms, MatchSource::IfLetDesugar{ contains_else_clause: false, .. }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0027`.
