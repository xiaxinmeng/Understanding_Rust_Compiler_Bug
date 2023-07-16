rust
cx.span_lint(NON_UPPER_CASE_GLOBALS,
                              span,
                              &format!("{} `{}` should have an upper case name such as `{}`", sort, name, uc));
