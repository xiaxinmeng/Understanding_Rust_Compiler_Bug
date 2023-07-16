
[00:57:55] error[E0050]: method `check_pat` has 3 parameters but the declaration in trait `rustc::lint::EarlyLintPass::check_pat` has 4
[00:57:55]    --> tools\clippy\clippy_lints\src\misc_early.rs:219:57
[00:57:55]     |
[00:57:55] 219 |     fn check_pat(&mut self, cx: &EarlyContext<'_>, pat: &Pat) {
[00:57:55]     |                                                         ^^^^ expected 4 parameters, found 3
[00:57:55]     |
[00:57:55]     = note: `check_pat` from trait: `fn(&mut Self, &rustc::lint::EarlyContext<'_>, &syntax::ast::Pat, &mut bool)`
[00:57:55] 
[00:57:55] error: aborting due to previous error
