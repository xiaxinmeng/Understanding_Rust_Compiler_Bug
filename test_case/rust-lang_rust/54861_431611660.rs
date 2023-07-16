plain
[00:43:49]     |              ^^^^^^^^^^^^^^^ cannot be resolved, ignoring
[00:43:49]     |
[00:43:49]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:43:49] 
[00:43:51] error: expected item after attributes
[00:43:51]  --> <anon>:1:31
[00:43:51]   |
[00:43:51] 1 | #[feature(exhaustive_patterns)]
[00:43:51] 
[00:43:51] 
[00:43:51] error: macros that expand to items must either be surrounded with braces or followed by a semicolon
[00:43:51]  --> <anon>:2:15
[00:43:51]   |
[00:43:51] 2 | compile_error!("Either feature \"foo\" or \"bar\" must be enabled for this crate.")
[00:43:51] 
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
[00:43:51] 
[00:43:51] 
[00:43:51] error[E0586]: inclusive range with no end
[00:43:51]   --> <anon>:10:29
[00:43:51]    |
[00:43:51] 10 |         _mm256_add_epi64(...);
[00:43:51]    |
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
[00:43:51] 
[00:43:51] 
[00:43:51] error[E0586]: inclusive range with no end
[00:43:51]   --> <anon>:20:25
[00:43:51]    |
[00:43:51] 20 |     _mm256_add_epi64(...);
[00:43:51]    |
[00:43:51]    |
[00:43:51]    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
[00:43:53]     Finished release [optimized] target(s) in 50.60s
[00:43:53] Documenting stage2 test (x86_64-unknown-linux-gnu)
[00:43:54]     Checking term v0.0.0 (/checkout/src/libterm)
[00:43:54]     Checking getopts v0.2.17
---
[00:44:34] travis_fold:end:stage2-error_index_generator

[00:44:34] travis_time:end:stage2-error_index_generator:start=1540064024085535125,finish=1540064027269330388,duration=3183795263

[00:44:34] error: only foreign functions are allowed to be variadic
[00:44:34]  --> <anon>:5:15
[00:44:34]   |
[00:44:34] 5 | fn foo(x: u8, ...) {}
[00:44:34] 
[00:44:34] 
[00:44:34] error[E0178]: expected a path on the left-hand side of `+`, not `&'a Foo`
[00:44:34]  --> <anon>:4:8
[00:44:34]   |
[00:44:34] 4 |     w: &'a Foo + Copy,   // error, use &'a (Foo + Copy)
[00:44:34]   |        ^^^^^^^^^^^^^^ help: try adding parentheses: `&'a (Foo + Copy)`
[00:44:34] 
[00:44:34] error[E0178]: expected a path on the left-hand side of `+`, not `&'a Foo`
[00:44:34]  --> <anon>:5:8
[00:44:34]   |
[00:44:34] 5 |     x: &'a Foo + 'a,     // error, use &'a (Foo + 'a)
[00:44:34]   |        ^^^^^^^^^^^^ help: try adding parentheses: `&'a (Foo + 'a)`
[00:44:34] 
[00:44:34] error[E0178]: expected a path on the left-hand side of `+`, not `&'a mut Foo`
[00:44:34]  --> <anon>:6:8
[00:44:34]   |
[00:44:34] 6 |     y: &'a mut Foo + 'a, // error, use &'a mut (Foo + 'a)
[00:44:34]   |        ^^^^^^^^^^^^^^^^ help: try adding parentheses: `&'a mut (Foo + 'a)`
[00:44:34] 
[00:44:34] error[E0178]: expected a path on the left-hand side of `+`, not `fn() -> Foo`
[00:44:34]  --> <anon>:7:8
[00:44:34]   |
[00:44:34] 7 |     z: fn() -> Foo + 'a, // error, use fn() -> (Foo + 'a)
[00:44:34]   |        ^^^^^^^^^^^^^^^^ perhaps you forgot parentheses?
[00:44:34] 
[00:44:34] error: unexpected close delimiter: `}`
[00:44:34]  --> <anon>:5:1
[00:44:34] 5 | }
[00:44:34] 5 | }
[00:44:34]   | ^ unexpected close delimiter
[00:44:34] 
[00:44:34] 
[00:44:34] 
[00:44:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/error_index_generator" "html" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc/error-index.html"
[00:44:34] 
[00:44:34] 
[00:44:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:44:34] Build completed unsuccessfully in 0:05:24
[00:44:34] Build completed unsuccessfully in 0:05:24
[00:44:34] Makefile:28: recipe for target 'all' failed
[00:44:34] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17be19a8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
