
[00:43:51] error: expected item after attributes
[00:43:51]  --> <anon>:1:31
[00:43:51]   |
[00:43:51] 1 | #[feature(exhaustive_patterns)]
[00:43:51]   |                               ^
[00:43:51] 
[00:43:51] error: macros that expand to items must either be surrounded with braces or followed by a semicolon
[00:43:51]  --> <anon>:2:15
[00:43:51]   |
[00:43:51] 2 | compile_error!("Either feature \"foo\" or \"bar\" must be enabled for this crate.")
[00:43:51]   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:43:51] 
[00:43:51] error: unexpected token: `...`
[00:43:51]   --> <anon>:10:26
[00:43:51]    |
[00:43:51] 10 |         _mm256_add_epi64(...);
[00:43:51]    |                          ^^^
[00:43:51] help: use `..` for an exclusive range
[00:43:51]    |
[00:43:51] 10 |         _mm256_add_epi64(..);
[00:43:51]    |                          ^^
[00:43:51] help: or `..=` for an inclusive range
[00:43:51]    |
[00:43:51] 10 |         _mm256_add_epi64(..=);
[00:43:51]    |                          ^^^
[00:43:51] 
[00:43:51] error[E0586]: inclusive range with no end
[00:43:51]   --> <anon>:10:29
[00:43:51]    |
[00:43:51] 10 |         _mm256_add_epi64(...);
[00:43:51]    |                             ^
[00:43:51]    |
[00:43:51]    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
[00:43:51] 
[00:43:51] error: unexpected token: `...`
[00:43:51]   --> <anon>:20:22
[00:43:51]    |
[00:43:51] 20 |     _mm256_add_epi64(...);
[00:43:51]    |                      ^^^
[00:43:51] help: use `..` for an exclusive range
[00:43:51]    |
[00:43:51] 20 |     _mm256_add_epi64(..);
[00:43:51]    |                      ^^
[00:43:51] help: or `..=` for an inclusive range
[00:43:51]    |
[00:43:51] 20 |     _mm256_add_epi64(..=);
[00:43:51]    |                      ^^^
[00:43:51] 
[00:43:51] error[E0586]: inclusive range with no end
[00:43:51]   --> <anon>:20:25
[00:43:51]    |
[00:43:51] 20 |     _mm256_add_epi64(...);
[00:43:51]    |                         ^
[00:43:51]    |
[00:43:51]    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
