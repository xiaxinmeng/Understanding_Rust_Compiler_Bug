\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/regions/regions-static-bound.rs","byte_start":945,"byte_end":958,"line_start":25,"line_end":25,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    static_id(&u); //[ll]~ ERROR explicit lifetime required in the type of `u` [E0621]","highlight_start":5,"highlight_end":18}],"label":"lifetime `'static` required","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add explicit lifetime `'static` to the type of `u`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/regions/regions-static-bound.rs","byte_start":926,"byte_end":929,"line_start":24,"line_end":24,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"fn error(u: &(), v: &()) {","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":"&'static ()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0621]: explicit lifetime required in the type of `u`\n  --> /checkout/src/test/ui/regions/regions-static-bound.rs:25:5\n   |\nLL | fn error(u: &(), v: &()) {\n   |             --- help: add explicit lifetime `'static` to the type of `u`: `&'static ()`\nLL |     static_id(&u); //[ll]~ ERROR explicit lifetime required in the type of `u` [E0621]\n   |     ^^^^^^^^^^^^^ lifetime `'static` required\n\n"}
[00:48:57] thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:345:21
[00:48:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:57] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:48:57] {"message":"For more information about this error, try `rustc --explain E0621`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0621`.\n"}
[00:48:57] error: internal compiler error: unexpected panic
[00:48:57] 
[00:48:57] note: the compiler unexpectedly panicked. this is a bug.
[00:48:57] 
[00:48:57] 
[00:48:57] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:48:57] 
[00:48:57] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:48:57] 
[00:48:57] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -C prefer-dynamic -C rpath
[00:48:57] 
[00:48:57] ------------------------------------------
[00:48:57] 
[00:48:57] thread '[ui] ui/regions/regions-static-bound.rs#nll' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
