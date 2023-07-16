plain
[00:16:50]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:16:50]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:17:17]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:17:17]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:17:17] error: incorrect close delimiter: `}`
[00:17:17]     |
[00:17:17]     |
[00:17:17] 280 |         if let ast::ExprKind::Paren(ref inner) = value.node {
[00:17:17]     |                                                             - close delimiter possibly meant for this
[00:17:17] 281 |             if struct_lit_needs_parens &&
[00:17:17] 282 |                     (parser::contains_exterior_struct_lit(&inner) ||
[00:17:17]     |                     - un-closed delimiter
[00:17:17] 323 |         }
[00:17:17]     |         ^ incorrect close delimiter
[00:17:17] 
[00:17:17] 
[00:17:17] error: expected one of `)`, `,`, `.`, `?`, or an operator, found `{`
[00:17:17]     |
[00:17:17]     |
[00:17:17] 283 |                     is_break(inner) {
[00:17:17]     |                                     ^ expected one of `)`, `,`, `.`, `?`, or an operator here
[00:17:17]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:17:17] error: unused import: `syntax::errors::Applicability`
[00:17:17]   --> librustc_lint/unused.rs:20:5
[00:17:17]    |
---
[00:17:17] 
[00:17:18] error[E0308]: mismatched types
[00:17:18]    --> librustc_lint/unused.rs:268:16
[00:17:18]     |
[00:17:18] 268 |         if let ast::ExprKind::Break(..) = e {
[00:17:18]     |                ^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `syntax::ast::Expr`, found enum `syntax::ast::ExprKind`
[00:17:18]     = note: expected type `syntax::ast::Expr`
[00:17:18]                found type `syntax::ast::ExprKind`
[00:17:18] 
[00:17:18] error: aborting due to 6 previous errors
---
travis_time:end:1d31ecd8:start=1538997715075704716,finish=1538997715080232183,duration=4527467
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b42e8f2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f6147c0
travis_time:start:0f6147c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0700659c
$ dmesg | grep -i kill
