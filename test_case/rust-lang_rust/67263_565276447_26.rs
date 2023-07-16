\n"},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":800,"byte_end":801,"line_start":29,"line_end":29,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"    let res: Result<!, usize> = Ok(42).map(diverge);","highlight_start":21,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see https://github.com/rust-lang/rust/issues/35121","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#![feature(never_type)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: The `!` type is experimental\n  --> tests/ui/result_map_unit_fn_unfixable.rs:29:21\n   |\nLL |     let res: Result<!, usize> = Ok(42).map(diverge);\n   |                     ^\n   |\n   = note: for more information, see https://github.com/rust-lang/rust/issues/35121\n   = help: add `#![feature(never_type)]` to the crate attributes to enable\n\n"}
2019-12-13T01:59:32.0894224Z {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
2019-12-13T01:59:32.0894812Z {"message":"Some errors have detailed explanations: E0425, E0658.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"Some errors have detailed explanations: E0425, E0658.\n"}
2019-12-13T01:59:32.0895390Z {"message":"For more information about an error, try `rustc --explain E0425`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0425`.\n"}
2019-12-13T01:59:32.0896258Z ------------------------------------------
2019-12-13T01:59:32.0896336Z 
2019-12-13T01:59:32.0896427Z test [ui] ui/result_map_unit_fn_unfixable.rs ... FAILED
2019-12-13T01:59:32.6651473Z test [ui] ui/same_functions_in_if_condition.rs ... ok
---
2019-12-13T02:39:51.0011011Z normalized stderr:
2019-12-13T02:39:51.0011174Z error[E0658]: The `!` type is experimental
2019-12-13T02:39:51.0011985Z   --> $DIR/async-fn.rs:44:22
2019-12-13T02:39:51.0012127Z    |
2019-12-13T02:39:51.0012210Z 44 |     let _x: (String, !) = (String::new(), return async { x + x }.await);
2019-12-13T02:39:51.0012389Z    |
2019-12-13T02:39:51.0013849Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:39:51.0013849Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:39:51.0013979Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T02:39:51.0014659Z error: aborting due to previous error
2019-12-13T02:39:51.0014717Z 
2019-12-13T02:39:51.0015073Z For more information about this error, try `rustc --explain E0658`.
2019-12-13T02:39:51.0015125Z 
---
2019-12-13T02:39:51.0016920Z 
2019-12-13T02:39:51.0020416Z +error[E0658]: The `!` type is experimental
2019-12-13T02:39:51.0021278Z +  --> $DIR/async-fn.rs:44:22
2019-12-13T02:39:51.0024281Z +   |
2019-12-13T02:39:51.0024418Z +44 |     let _x: (String, !) = (String::new(), return async { x + x }.await);
2019-12-13T02:39:51.0029293Z +   |
2019-12-13T02:39:51.0033802Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:39:51.0033802Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:39:51.0033985Z +   = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T02:39:51.0037160Z +error: aborting due to previous error
2019-12-13T02:39:51.0040479Z +
2019-12-13T02:39:51.0040938Z +For more information about this error, try `rustc --explain E0658`.
2019-12-13T02:39:51.0042767Z +
2019-12-13T02:39:51.0042767Z +
2019-12-13T02:39:51.0046715Z 
2019-12-13T02:39:51.0047392Z The actual stderr differed from the expected stderr.
2019-12-13T02:39:51.0050938Z Actual stderr saved to /tmp/compiletestRnsZyF/async-fn.stderr
2019-12-13T02:39:51.0059521Z To update references, run this command from build directory:
2019-12-13T02:39:51.0063825Z tests/run-pass/update-references.sh '/tmp/compiletestRnsZyF' 'async-fn.rs'
2019-12-13T02:39:51.0068909Z error: 1 errors occurred comparing output.
2019-12-13T02:39:51.0069269Z status: exit code: 1
2019-12-13T02:39:51.0069269Z status: exit code: 1
2019-12-13T02:39:51.0071205Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestRnsZyF" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestRnsZyF/async-fn.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestRnsZyF/async-fn.stage-id.aux" "-A" "unused"
2019-12-13T02:39:51.0072021Z ------------------------------------------
2019-12-13T02:39:51.0072077Z 
2019-12-13T02:39:51.0072343Z ------------------------------------------
2019-12-13T02:39:51.0072420Z stderr:
2019-12-13T02:39:51.0072420Z stderr:
2019-12-13T02:39:51.0072777Z ------------------------------------------
2019-12-13T02:39:51.0075903Z {"message":"The `!` type is experimental","code":{"code":"E0658","explanation":"An unstable feature was used.\n\nErroneous code example:\n\n