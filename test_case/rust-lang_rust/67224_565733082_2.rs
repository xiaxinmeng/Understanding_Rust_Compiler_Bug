\n"},"level":"error","spans":[{"file_name":"tests/run-pass/async-fn.rs","byte_start":941,"byte_end":942,"line_start":44,"line_end":44,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let _x: (String, !) = (String::new(), return async { x + x }.await);","highlight_start":22,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see ***/issues/35121","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#![feature(never_type)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: The `!` type is experimental\n  --> tests/run-pass/async-fn.rs:44:22\n   |\n44 |     let _x: (String, !) = (String::new(), return async { x + x }.await);\n   |                      ^\n   |\n   = note: for more information, see ***/issues/35121\n   = help: add `#![feature(never_type)]` to the crate attributes to enable\n\n"}
2019-12-14T16:43:26.4976434Z {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
2019-12-14T16:43:26.4976510Z 
2019-12-14T16:43:26.4976736Z ------------------------------------------
2019-12-14T16:43:26.4976770Z 
---
2019-12-14T16:43:33.0233082Z normalized stderr:
2019-12-14T16:43:33.0233816Z error[E0658]: The `!` type is experimental
2019-12-14T16:43:33.0236770Z   --> $DIR/generator.rs:94:26
2019-12-14T16:43:33.0237712Z    |
2019-12-14T16:43:33.0238107Z 94 |         let _x: (String, !) = (String::new(), { yield 2; return });
2019-12-14T16:43:33.0247912Z    |
2019-12-14T16:43:33.0247912Z    |
2019-12-14T16:43:33.0248803Z    = note: for more information, see ***/issues/35121
2019-12-14T16:43:33.0249321Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-14T16:43:33.0249784Z error: aborting due to previous error
2019-12-14T16:43:33.0249980Z 
2019-12-14T16:43:33.0250710Z For more information about this error, try `rustc --explain E0658`.
2019-12-14T16:43:33.0251451Z 
---
2019-12-14T16:43:33.0252955Z 
2019-12-14T16:43:33.0253297Z +error[E0658]: The `!` type is experimental
2019-12-14T16:43:33.0253948Z +  --> $DIR/generator.rs:94:26
2019-12-14T16:43:33.0254271Z +   |
2019-12-14T16:43:33.0254507Z +94 |         let _x: (String, !) = (String::new(), { yield 2; return });
2019-12-14T16:43:33.0254980Z +   |
2019-12-14T16:43:33.0254980Z +   |
2019-12-14T16:43:33.0255643Z +   = note: for more information, see ***/issues/35121
2019-12-14T16:43:33.0255968Z +   = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-14T16:43:33.0256456Z +error: aborting due to previous error
2019-12-14T16:43:33.0256674Z +
2019-12-14T16:43:33.0257183Z +For more information about this error, try `rustc --explain E0658`.
2019-12-14T16:43:33.0257511Z +
2019-12-14T16:43:33.0257511Z +
2019-12-14T16:43:33.0257710Z 
2019-12-14T16:43:33.0257938Z The actual stderr differed from the expected stderr.
2019-12-14T16:43:33.0258187Z Actual stderr saved to /tmp/compiletestnSA66k/generator.stderr
2019-12-14T16:43:33.0258421Z To update references, run this command from build directory:
2019-12-14T16:43:33.0258962Z tests/run-pass/update-references.sh '/tmp/compiletestnSA66k' 'generator.rs'
2019-12-14T16:43:33.0259455Z error: 1 errors occurred comparing output.
2019-12-14T16:43:33.0259833Z status: exit code: 1
2019-12-14T16:43:33.0259833Z status: exit code: 1
2019-12-14T16:43:33.0260771Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestnSA66k" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestnSA66k/generator.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestnSA66k/generator.stage-id.aux" "-A" "unused"
2019-12-14T16:43:33.0261806Z ------------------------------------------
2019-12-14T16:43:33.0262072Z 
2019-12-14T16:43:33.0262579Z ------------------------------------------
2019-12-14T16:43:33.0262874Z stderr:
2019-12-14T16:43:33.0262874Z stderr:
2019-12-14T16:43:33.0263336Z ------------------------------------------
2019-12-14T16:43:33.0266053Z {"message":"The `!` type is experimental","code":{"code":"E0658","explanation":"An unstable feature was used.\n\nErroneous code example:\n\n