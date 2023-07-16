\n"},"level":"error","spans":[{"file_name":"tests/run-pass/generator.rs","byte_start":1759,"byte_end":1760,"line_start":94,"line_end":94,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"        let _x: (String, !) = (String::new(), { yield 2; return });","highlight_start":26,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see ***/issues/35121","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#![feature(never_type)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: The `!` type is experimental\n  --> tests/run-pass/generator.rs:94:26\n   |\n94 |         let _x: (String, !) = (String::new(), { yield 2; return });\n   |                          ^\n   |\n   = note: for more information, see ***/issues/35121\n   = help: add `#![feature(never_type)]` to the crate attributes to enable\n\n"}
2019-12-14T16:43:33.0267635Z {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
2019-12-14T16:43:33.0267962Z 
2019-12-14T16:43:33.0268552Z ------------------------------------------
2019-12-14T16:43:33.0269581Z 
---
2019-12-14T16:43:39.3523212Z normalized stderr:
2019-12-14T16:43:39.3524793Z error[E0658]: The `!` type is experimental
2019-12-14T16:43:39.3525592Z   --> $DIR/loop-break-value.rs:15:25
2019-12-14T16:43:39.3525646Z    |
2019-12-14T16:43:39.3525690Z 15 |             let _never: ! = loop {
2019-12-14T16:43:39.3526060Z    |
2019-12-14T16:43:39.3526060Z    |
2019-12-14T16:43:39.3526619Z    = note: for more information, see ***/issues/35121
2019-12-14T16:43:39.3526705Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-14T16:43:39.3526792Z error: aborting due to previous error
2019-12-14T16:43:39.3526828Z 
2019-12-14T16:43:39.3527156Z For more information about this error, try `rustc --explain E0658`.
2019-12-14T16:43:39.3531595Z 
---
2019-12-14T16:43:39.3537853Z 
2019-12-14T16:43:39.3546753Z +error[E0658]: The `!` type is experimental
2019-12-14T16:43:39.3547317Z +  --> $DIR/loop-break-value.rs:15:25
2019-12-14T16:43:39.3547579Z +   |
2019-12-14T16:43:39.3547633Z +15 |             let _never: ! = loop {
2019-12-14T16:43:39.3550787Z +   |
2019-12-14T16:43:39.3550787Z +   |
2019-12-14T16:43:39.3553881Z +   = note: for more information, see ***/issues/35121
2019-12-14T16:43:39.3556971Z +   = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-14T16:43:39.3557576Z +error: aborting due to previous error
2019-12-14T16:43:39.3558337Z +
2019-12-14T16:43:39.3559666Z +For more information about this error, try `rustc --explain E0658`.
2019-12-14T16:43:39.3561370Z +
2019-12-14T16:43:39.3561370Z +
2019-12-14T16:43:39.3563755Z 
2019-12-14T16:43:39.3564084Z The actual stderr differed from the expected stderr.
2019-12-14T16:43:39.3564706Z Actual stderr saved to /tmp/compiletestnSA66k/loop-break-value.stderr
2019-12-14T16:43:39.3565017Z To update references, run this command from build directory:
2019-12-14T16:43:39.3565543Z tests/run-pass/update-references.sh '/tmp/compiletestnSA66k' 'loop-break-value.rs'
2019-12-14T16:43:39.3566551Z error: 1 errors occurred comparing output.
2019-12-14T16:43:39.3566777Z status: exit code: 1
2019-12-14T16:43:39.3566777Z status: exit code: 1
2019-12-14T16:43:39.3568327Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletestnSA66k" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestnSA66k/loop-break-value.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestnSA66k/loop-break-value.stage-id.aux" "-A" "unused"
2019-12-14T16:43:39.3568815Z ------------------------------------------
2019-12-14T16:43:39.3571997Z 
2019-12-14T16:43:39.3572614Z ------------------------------------------
2019-12-14T16:43:39.3572919Z stderr:
2019-12-14T16:43:39.3572919Z stderr:
2019-12-14T16:43:39.3573363Z ------------------------------------------
2019-12-14T16:43:39.3577043Z {"message":"The `!` type is experimental","code":{"code":"E0658","explanation":"An unstable feature was used.\n\nErroneous code example:\n\n