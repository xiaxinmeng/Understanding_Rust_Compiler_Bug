
error: some variants are not matched explicitly
   |
note: the lint level is defined here
  --> selene-lib/src/rules/high_cyclomatic_complexity.rs:77:14
   |
77 |         deny(non_exhaustive_omitted_patterns)
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: ensure that all variants are matched explicitly by adding the suggested match arms
   = note: the matched value is of type `full_moon::ast::Value` and the `non_exhaustive_omitted_patterns` attribute was found

error: some variants are not matched explicitly
  |
  = help: ensure that all variants are matched explicitly by adding the suggested match arms
  = note: the matched value is of type `full_moon::ast::Expression` and the `non_exhaustive_omitted_patterns` attribute was found

error: some variants are not matched explicitly
    |
note: the lint level is defined here
   --> selene-lib/src/rules/high_cyclomatic_complexity.rs:180:18
    |
180 |             deny(non_exhaustive_omitted_patterns)
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: ensure that all variants are matched explicitly by adding the suggested match arms
    = note: the matched value is of type `full_moon::ast::Stmt` and the `non_exhaustive_omitted_patterns` attribute was found

error: could not compile `selene-lib` due to 3 previous errors
