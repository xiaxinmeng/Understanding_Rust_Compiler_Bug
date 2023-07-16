\n"},"level":"error","spans":[{"file_name":"tests/run-pass/loop-break-value.rs","byte_start":240,"byte_end":241,"line_start":15,"line_end":15,"column_start":25,"column_end":26,"is_primary":true,"text":[{"text":"            let _never: ! = loop {","highlight_start":25,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see https://github.com/rust-lang/rust/issues/35121","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#![feature(never_type)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: The `!` type is experimental\n  --> tests/run-pass/loop-break-value.rs:15:25\n   |\n15 |             let _never: ! = loop {\n   |                         ^\n   |\n   = note: for more information, see https://github.com/rust-lang/rust/issues/35121\n   = help: add `#![feature(never_type)]` to the crate attributes to enable\n\n"}
2019-12-13T02:40:04.4433748Z {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
2019-12-13T02:40:04.4433872Z 
2019-12-13T02:40:04.4434292Z ------------------------------------------
2019-12-13T02:40:04.4434341Z 
---
2019-12-13T02:40:06.5532368Z normalized stderr:
2019-12-13T02:40:06.5537871Z error[E0658]: The `!` type is experimental
2019-12-13T02:40:06.5538753Z   --> $DIR/catch_panic.rs:23:53
2019-12-13T02:40:06.5538833Z    |
2019-12-13T02:40:06.5539167Z 23 | fn do_panic_counter(do_panic: impl FnOnce(usize) -> !) {
2019-12-13T02:40:06.5539355Z    |
2019-12-13T02:40:06.5539660Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:40:06.5539660Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:40:06.5539766Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T02:40:06.5539908Z error[E0658]: The `!` type is experimental
2019-12-13T02:40:06.5540162Z   --> $DIR/catch_panic.rs:70:41
2019-12-13T02:40:06.5540232Z    |
2019-12-13T02:40:06.5540232Z    |
2019-12-13T02:40:06.5540506Z 70 | fn test(do_panic: impl FnOnce(usize) -> !) {
2019-12-13T02:40:06.5541179Z    |
2019-12-13T02:40:06.5541515Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:40:06.5541515Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:40:06.5541640Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T02:40:06.5541809Z error: aborting due to 2 previous errors
2019-12-13T02:40:06.5541861Z 
2019-12-13T02:40:06.5542203Z For more information about this error, try `rustc --explain E0658`.
2019-12-13T02:40:06.5546428Z 
---
2019-12-13T02:40:06.5595493Z -Success!
2019-12-13T02:40:06.5595561Z +error[E0658]: The `!` type is experimental
2019-12-13T02:40:06.5595802Z +  --> $DIR/catch_panic.rs:23:53
2019-12-13T02:40:06.5595880Z +   |
2019-12-13T02:40:06.5596146Z +23 | fn do_panic_counter(do_panic: impl FnOnce(usize) -> !) {
2019-12-13T02:40:06.5596322Z +   |
2019-12-13T02:40:06.5596609Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:40:06.5596609Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:40:06.5596708Z +   = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T02:40:06.5596868Z +error[E0658]: The `!` type is experimental
2019-12-13T02:40:06.5597114Z +  --> $DIR/catch_panic.rs:70:41
2019-12-13T02:40:06.5597207Z +   |
2019-12-13T02:40:06.5597207Z +   |
2019-12-13T02:40:06.5597441Z +70 | fn test(do_panic: impl FnOnce(usize) -> !) {
2019-12-13T02:40:06.5597607Z +   |
2019-12-13T02:40:06.5597896Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:40:06.5597896Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T02:40:06.5598166Z +   = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T02:40:06.5598322Z +error: aborting due to 2 previous errors
2019-12-13T02:40:06.5598406Z +
2019-12-13T02:40:06.5598720Z +For more information about this error, try `rustc --explain E0658`.
2019-12-13T02:40:06.5598797Z  
2019-12-13T02:40:06.5598797Z  
2019-12-13T02:40:06.5598833Z 
2019-12-13T02:40:06.5599012Z The actual stderr differed from the expected stderr.
2019-12-13T02:40:06.5599098Z Actual stderr saved to /tmp/compiletestRnsZyF/panic/catch_panic.stderr
2019-12-13T02:40:06.5599205Z To update references, run this command from build directory:
2019-12-13T02:40:06.5599601Z tests/run-pass/update-references.sh '/tmp/compiletestRnsZyF' 'panic/catch_panic.rs'
2019-12-13T02:40:06.5599749Z error: 1 errors occurred comparing output.
2019-12-13T02:40:06.5599819Z status: exit code: 1
2019-12-13T02:40:06.5599819Z status: exit code: 1
2019-12-13T02:40:06.5601720Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/panic/catch_panic.rs" "-L" "/tmp/compiletestRnsZyF" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestRnsZyF/panic/catch_panic.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestRnsZyF/panic/catch_panic.stage-id.aux" "-A" "unused"
2019-12-13T02:40:06.5602319Z ------------------------------------------
2019-12-13T02:40:06.5602389Z 
2019-12-13T02:40:06.5602637Z ------------------------------------------
2019-12-13T02:40:06.5602729Z stderr:
2019-12-13T02:40:06.5602729Z stderr:
2019-12-13T02:40:06.5602977Z ------------------------------------------
2019-12-13T02:40:06.5605969Z {"message":"The `!` type is experimental","code":{"code":"E0658","explanation":"An unstable feature was used.\n\nErroneous code example:\n\n