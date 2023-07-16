\n"},"level":"error","spans":[{"file_name":"tests/run-pass/loop-break-value.rs","byte_start":240,"byte_end":241,"line_start":15,"line_end":15,"column_start":25,"column_end":26,"is_primary":true,"text":[{"text":"            let _never: ! = loop {","highlight_start":25,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see ***/issues/35121","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#![feature(never_type)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: The `!` type is experimental\n  --> tests/run-pass/loop-break-value.rs:15:25\n   |\n15 |             let _never: ! = loop {\n   |                         ^\n   |\n   = note: for more information, see ***/issues/35121\n   = help: add `#![feature(never_type)]` to the crate attributes to enable\n\n"}
2019-12-14T16:43:39.3578772Z {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
2019-12-14T16:43:39.3579097Z 
2019-12-14T16:43:39.3579893Z ------------------------------------------
2019-12-14T16:43:39.3580166Z 
---
2019-12-14T16:43:41.4442076Z normalized stderr:
2019-12-14T16:43:41.4444006Z error[E0658]: The `!` type is experimental
2019-12-14T16:43:41.4445031Z   --> $DIR/catch_panic.rs:23:53
2019-12-14T16:43:41.4445438Z    |
2019-12-14T16:43:41.4446725Z 23 | fn do_panic_counter(do_panic: impl FnOnce(usize) -> !) {
2019-12-14T16:43:41.4447984Z    |
2019-12-14T16:43:41.4447984Z    |
2019-12-14T16:43:41.4448767Z    = note: for more information, see ***/issues/35121
2019-12-14T16:43:41.4449247Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-14T16:43:41.4449843Z error[E0658]: The `!` type is experimental
2019-12-14T16:43:41.4450445Z   --> $DIR/catch_panic.rs:70:41
2019-12-14T16:43:41.4450903Z    |
2019-12-14T16:43:41.4450903Z    |
2019-12-14T16:43:41.4451527Z 70 | fn test(do_panic: impl FnOnce(usize) -> !) {
2019-12-14T16:43:41.4452588Z    |
2019-12-14T16:43:41.4452588Z    |
2019-12-14T16:43:41.4453560Z    = note: for more information, see ***/issues/35121
2019-12-14T16:43:41.4454491Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-14T16:43:41.4455085Z error: aborting due to 2 previous errors
2019-12-14T16:43:41.4455287Z 
2019-12-14T16:43:41.4456015Z For more information about this error, try `rustc --explain E0658`.
2019-12-14T16:43:41.4457247Z 
---
2019-12-14T16:43:41.4528194Z -Success!
2019-12-14T16:43:41.4528384Z +error[E0658]: The `!` type is experimental
2019-12-14T16:43:41.4528757Z +  --> $DIR/catch_panic.rs:23:53
2019-12-14T16:43:41.4528943Z +   |
2019-12-14T16:43:41.4529356Z +23 | fn do_panic_counter(do_panic: impl FnOnce(usize) -> !) {
2019-12-14T16:43:41.4529854Z +   |
2019-12-14T16:43:41.4529854Z +   |
2019-12-14T16:43:41.4530316Z +   = note: for more information, see ***/issues/35121
2019-12-14T16:43:41.4530521Z +   = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-14T16:43:41.4530812Z +error[E0658]: The `!` type is experimental
2019-12-14T16:43:41.4531158Z +  --> $DIR/catch_panic.rs:70:41
2019-12-14T16:43:41.4531354Z +   |
2019-12-14T16:43:41.4531354Z +   |
2019-12-14T16:43:41.4531723Z +70 | fn test(do_panic: impl FnOnce(usize) -> !) {
2019-12-14T16:43:41.4532046Z +   |
2019-12-14T16:43:41.4532046Z +   |
2019-12-14T16:43:41.4532708Z +   = note: for more information, see ***/issues/35121
2019-12-14T16:43:41.4532907Z +   = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-14T16:43:41.4533206Z +error: aborting due to 2 previous errors
2019-12-14T16:43:41.4533467Z +
2019-12-14T16:43:41.4533891Z +For more information about this error, try `rustc --explain E0658`.
2019-12-14T16:43:41.4534093Z  
2019-12-14T16:43:41.4534093Z  
2019-12-14T16:43:41.4534218Z 
2019-12-14T16:43:41.4534375Z The actual stderr differed from the expected stderr.
2019-12-14T16:43:41.4534521Z Actual stderr saved to /tmp/compiletestnSA66k/panic/catch_panic.stderr
2019-12-14T16:43:41.4534668Z To update references, run this command from build directory:
2019-12-14T16:43:41.4535092Z tests/run-pass/update-references.sh '/tmp/compiletestnSA66k' 'panic/catch_panic.rs'
2019-12-14T16:43:41.4535398Z error: 1 errors occurred comparing output.
2019-12-14T16:43:41.4535532Z status: exit code: 1
2019-12-14T16:43:41.4535532Z status: exit code: 1
2019-12-14T16:43:41.4536963Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/panic/catch_panic.rs" "-L" "/tmp/compiletestnSA66k" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestnSA66k/panic/catch_panic.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestnSA66k/panic/catch_panic.stage-id.aux" "-A" "unused"
2019-12-14T16:43:41.4537713Z ------------------------------------------
2019-12-14T16:43:41.4537886Z 
2019-12-14T16:43:41.4538276Z ------------------------------------------
2019-12-14T16:43:41.4538456Z stderr:
2019-12-14T16:43:41.4538456Z stderr:
2019-12-14T16:43:41.4538802Z ------------------------------------------
2019-12-14T16:43:41.4541427Z {"message":"The `!` type is experimental","code":{"code":"E0658","explanation":"An unstable feature was used.\n\nErroneous code example:\n\n