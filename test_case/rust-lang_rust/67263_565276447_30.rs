\n"},"level":"error","spans":[{"file_name":"tests/run-pass/generator.rs","byte_start":1759,"byte_end":1760,"line_start":94,"line_end":94,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"        let _x: (String, !) = (String::new(), { yield 2; return });","highlight_start":26,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see https://github.com/rust-lang/rust/issues/35121","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#![feature(never_type)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: The `!` type is experimental\n  --> tests/run-pass/generator.rs:94:26\n   |\n94 |         let _x: (String, !) = (String::new(), { yield 2; return });\n   |                          ^\n   |\n   = note: for more information, see https://github.com/rust-lang/rust/issues/35121\n   = help: add `#![feature(never_type)]` to the crate attributes to enable\n\n"}
2019-12-13T02:39:57.9688083Z {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
2019-12-13T02:39:57.9688612Z 
2019-12-13T02:39:57.9689108Z ------------------------------------------
2019-12-13T02:39:57.9689334Z 
---
2019-12-13T02:40:04.4370391Z normalized stderr:
2019-12-13T02:40:04.4378366Z error[E0658]: The `!` type is experimental
2019-12-13T02:40:04.4379205Z   --> $DIR/loop-break-value.rs:15:25
2019-12-13T02:40:04.4379285Z    |
2019-12-13T02:40:04.4379369Z 15 |             let _never: ! = loop {
2019-12-13T02:40:04.4379531Z    |
2019-12-13T02:40:04.4379858Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:40:04.4379858Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:40:04.4379965Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T02:40:04.4380114Z error: aborting due to previous error
2019-12-13T02:40:04.4380164Z 
2019-12-13T02:40:04.4380957Z For more information about this error, try `rustc --explain E0658`.
2019-12-13T02:40:04.4386681Z 
---
2019-12-13T02:40:04.4403296Z 
2019-12-13T02:40:04.4423645Z +error[E0658]: The `!` type is experimental
2019-12-13T02:40:04.4424351Z +  --> $DIR/loop-break-value.rs:15:25
2019-12-13T02:40:04.4424453Z +   |
2019-12-13T02:40:04.4424520Z +15 |             let _never: ! = loop {
2019-12-13T02:40:04.4424686Z +   |
2019-12-13T02:40:04.4425015Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:40:04.4425015Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:40:04.4425127Z +   = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T02:40:04.4425480Z +error: aborting due to previous error
2019-12-13T02:40:04.4425548Z +
2019-12-13T02:40:04.4425954Z +For more information about this error, try `rustc --explain E0658`.
2019-12-13T02:40:04.4426045Z +
2019-12-13T02:40:04.4426045Z +
2019-12-13T02:40:04.4426086Z 
2019-12-13T02:40:04.4426173Z The actual stderr differed from the expected stderr.
2019-12-13T02:40:04.4426503Z Actual stderr saved to /tmp/compiletestRnsZyF/loop-break-value.stderr
2019-12-13T02:40:04.4426599Z To update references, run this command from build directory:
2019-12-13T02:40:04.4426922Z tests/run-pass/update-references.sh '/tmp/compiletestRnsZyF' 'loop-break-value.rs'
2019-12-13T02:40:04.4427069Z error: 1 errors occurred comparing output.
2019-12-13T02:40:04.4427140Z status: exit code: 1
2019-12-13T02:40:04.4427140Z status: exit code: 1
2019-12-13T02:40:04.4427991Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletestRnsZyF" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestRnsZyF/loop-break-value.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestRnsZyF/loop-break-value.stage-id.aux" "-A" "unused"
2019-12-13T02:40:04.4428486Z ------------------------------------------
2019-12-13T02:40:04.4428553Z 
2019-12-13T02:40:04.4428790Z ------------------------------------------
2019-12-13T02:40:04.4428888Z stderr:
2019-12-13T02:40:04.4428888Z stderr:
2019-12-13T02:40:04.4429117Z ------------------------------------------
2019-12-13T02:40:04.4432131Z {"message":"The `!` type is experimental","code":{"code":"E0658","explanation":"An unstable feature was used.\n\nErroneous code example:\n\n