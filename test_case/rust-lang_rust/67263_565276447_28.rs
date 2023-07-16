\n"},"level":"error","spans":[{"file_name":"tests/run-pass/async-fn.rs","byte_start":941,"byte_end":942,"line_start":44,"line_end":44,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let _x: (String, !) = (String::new(), return async { x + x }.await);","highlight_start":22,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see https://github.com/rust-lang/rust/issues/35121","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#![feature(never_type)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: The `!` type is experimental\n  --> tests/run-pass/async-fn.rs:44:22\n   |\n44 |     let _x: (String, !) = (String::new(), return async { x + x }.await);\n   |                      ^\n   |\n   = note: for more information, see https://github.com/rust-lang/rust/issues/35121\n   = help: add `#![feature(never_type)]` to the crate attributes to enable\n\n"}
2019-12-13T02:39:51.0077252Z {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
2019-12-13T02:39:51.0077379Z 
2019-12-13T02:39:51.0077635Z ------------------------------------------
2019-12-13T02:39:51.0077691Z 
---
2019-12-13T02:39:57.9661412Z normalized stderr:
2019-12-13T02:39:57.9662211Z error[E0658]: The `!` type is experimental
2019-12-13T02:39:57.9663400Z   --> $DIR/generator.rs:94:26
2019-12-13T02:39:57.9663509Z    |
2019-12-13T02:39:57.9663651Z 94 |         let _x: (String, !) = (String::new(), { yield 2; return });
2019-12-13T02:39:57.9663833Z    |
2019-12-13T02:39:57.9664190Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:39:57.9664190Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:39:57.9664370Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T02:39:57.9664521Z error: aborting due to previous error
2019-12-13T02:39:57.9664587Z 
2019-12-13T02:39:57.9664881Z For more information about this error, try `rustc --explain E0658`.
2019-12-13T02:39:57.9664939Z 
---
2019-12-13T02:39:57.9665295Z 
2019-12-13T02:39:57.9665362Z +error[E0658]: The `!` type is experimental
2019-12-13T02:39:57.9665635Z +  --> $DIR/generator.rs:94:26
2019-12-13T02:39:57.9665917Z +   |
2019-12-13T02:39:57.9666491Z +94 |         let _x: (String, !) = (String::new(), { yield 2; return });
2019-12-13T02:39:57.9666705Z +   |
2019-12-13T02:39:57.9667140Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:39:57.9667140Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:39:57.9667249Z +   = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T02:39:57.9667422Z +error: aborting due to previous error
2019-12-13T02:39:57.9667510Z +
2019-12-13T02:39:57.9667790Z +For more information about this error, try `rustc --explain E0658`.
2019-12-13T02:39:57.9668031Z +
2019-12-13T02:39:57.9668031Z +
2019-12-13T02:39:57.9675518Z 
2019-12-13T02:39:57.9675908Z The actual stderr differed from the expected stderr.
2019-12-13T02:39:57.9676161Z Actual stderr saved to /tmp/compiletestRnsZyF/generator.stderr
2019-12-13T02:39:57.9676386Z To update references, run this command from build directory:
2019-12-13T02:39:57.9677669Z tests/run-pass/update-references.sh '/tmp/compiletestRnsZyF' 'generator.rs'
2019-12-13T02:39:57.9678131Z error: 1 errors occurred comparing output.
2019-12-13T02:39:57.9678325Z status: exit code: 1
2019-12-13T02:39:57.9678325Z status: exit code: 1
2019-12-13T02:39:57.9679377Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestRnsZyF" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestRnsZyF/generator.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestRnsZyF/generator.stage-id.aux" "-A" "unused"
2019-12-13T02:39:57.9682052Z ------------------------------------------
2019-12-13T02:39:57.9682286Z 
2019-12-13T02:39:57.9682715Z ------------------------------------------
2019-12-13T02:39:57.9682957Z stderr:
2019-12-13T02:39:57.9682957Z stderr:
2019-12-13T02:39:57.9683350Z ------------------------------------------
2019-12-13T02:39:57.9686281Z {"message":"The `!` type is experimental","code":{"code":"E0658","explanation":"An unstable feature was used.\n\nErroneous code example:\n\n