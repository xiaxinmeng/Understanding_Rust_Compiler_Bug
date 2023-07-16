plain
2019-12-14T14:03:09.2683488Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-14T14:03:09.2938641Z ##[command]git config gc.auto 0
2019-12-14T14:03:09.3398985Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-14T14:03:09.3468139Z ##[command]git config --get-all http.proxy
2019-12-14T14:03:09.3627376Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67224/merge:refs/remotes/pull/67224/merge
---
2019-12-14T15:34:50.5341391Z ---- /checkout/src/doc/reference/src/types/never.md - Never_type (line 10) stdout ----
2019-12-14T15:34:50.5341444Z error[E0658]: The `!` type is experimental
2019-12-14T15:34:50.5341669Z  --> /checkout/src/doc/reference/src/types/never.md:11:8
2019-12-14T15:34:50.5341713Z   |
2019-12-14T15:34:50.5341770Z 3 | let x: ! = panic!();
2019-12-14T15:34:50.5341849Z   |
2019-12-14T15:34:50.5341849Z   |
2019-12-14T15:34:50.5342225Z   = note: for more information, see ***/issues/35121
2019-12-14T15:34:50.5342282Z   = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-14T15:34:50.5342370Z error: aborting due to previous error
2019-12-14T15:34:50.5342397Z 
2019-12-14T15:34:50.5342641Z For more information about this error, try `rustc --explain E0658`.
2019-12-14T15:34:50.5343051Z Couldn't compile the test.
---
2019-12-14T15:48:21.8197960Z  53 │ variables. [`UniversalRegions`] contains indices for all the free regions in
2019-12-14T15:48:21.8198382Z     │            ^ Server responded with 404 Not Found
2019-12-14T15:48:21.8198756Z     │
2019-12-14T15:48:21.8198914Z 
2019-12-14T15:48:21.8199467Z error: The server responded with 404 Not Found for "***/tree/master/src/librustc_mir/borrow_check/nll/region_infer/"
2019-12-14T15:48:21.8200063Z     ┌── borrow_check/region_inference.md:81:34 ───
2019-12-14T15:48:21.8200408Z     │
2019-12-14T15:48:21.8201865Z  81 │ for all regions is maintained in [the
2019-12-14T15:48:21.8202436Z     │                                  ^ Server responded with 404 Not Found
---
2019-12-14T16:43:26.4965891Z normalized stderr:
2019-12-14T16:43:26.4966361Z error[E0658]: The `!` type is experimental
2019-12-14T16:43:26.4967545Z   --> $DIR/async-fn.rs:44:22
2019-12-14T16:43:26.4967604Z    |
2019-12-14T16:43:26.4967772Z 44 |     let _x: (String, !) = (String::new(), return async { x + x }.await);
2019-12-14T16:43:26.4967867Z    |
2019-12-14T16:43:26.4967867Z    |
2019-12-14T16:43:26.4968297Z    = note: for more information, see ***/issues/35121
2019-12-14T16:43:26.4968360Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-14T16:43:26.4968445Z error: aborting due to previous error
2019-12-14T16:43:26.4968494Z 
2019-12-14T16:43:26.4968765Z For more information about this error, try `rustc --explain E0658`.
2019-12-14T16:43:26.4968801Z 
---
2019-12-14T16:43:26.4969025Z 
2019-12-14T16:43:26.4969095Z +error[E0658]: The `!` type is experimental
2019-12-14T16:43:26.4969310Z +  --> $DIR/async-fn.rs:44:22
2019-12-14T16:43:26.4969357Z +   |
2019-12-14T16:43:26.4969434Z +44 |     let _x: (String, !) = (String::new(), return async { x + x }.await);
2019-12-14T16:43:26.4969528Z +   |
2019-12-14T16:43:26.4969528Z +   |
2019-12-14T16:43:26.4969830Z +   = note: for more information, see ***/issues/35121
2019-12-14T16:43:26.4969890Z +   = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-14T16:43:26.4970000Z +error: aborting due to previous error
2019-12-14T16:43:26.4970042Z +
2019-12-14T16:43:26.4970296Z +For more information about this error, try `rustc --explain E0658`.
2019-12-14T16:43:26.4970343Z +
2019-12-14T16:43:26.4970343Z +
2019-12-14T16:43:26.4970389Z 
2019-12-14T16:43:26.4970435Z The actual stderr differed from the expected stderr.
2019-12-14T16:43:26.4970677Z Actual stderr saved to /tmp/compiletestnSA66k/async-fn.stderr
2019-12-14T16:43:26.4970966Z To update references, run this command from build directory:
2019-12-14T16:43:26.4971258Z tests/run-pass/update-references.sh '/tmp/compiletestnSA66k' 'async-fn.rs'
2019-12-14T16:43:26.4971438Z error: 1 errors occurred comparing output.
2019-12-14T16:43:26.4971511Z status: exit code: 1
2019-12-14T16:43:26.4971511Z status: exit code: 1
2019-12-14T16:43:26.4972213Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestnSA66k" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestnSA66k/async-fn.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestnSA66k/async-fn.stage-id.aux" "-A" "unused"
2019-12-14T16:43:26.4972544Z ------------------------------------------
2019-12-14T16:43:26.4972588Z 
2019-12-14T16:43:26.4972834Z ------------------------------------------
2019-12-14T16:43:26.4972882Z stderr:
2019-12-14T16:43:26.4972882Z stderr:
2019-12-14T16:43:26.4973096Z ------------------------------------------
2019-12-14T16:43:26.4975610Z {"message":"The `!` type is experimental","code":{"code":"E0658","explanation":"An unstable feature was used.\n\nErroneous code example:\n\n