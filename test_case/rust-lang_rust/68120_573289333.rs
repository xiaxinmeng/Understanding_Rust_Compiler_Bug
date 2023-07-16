plain
2020-01-11T05:55:44.7264145Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T05:55:44.7614657Z ##[command]git config gc.auto 0
2020-01-11T05:55:45.7062530Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T05:55:45.7067046Z ##[command]git config --get-all http.proxy
2020-01-11T05:55:45.7072336Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68120/merge:refs/remotes/pull/68120/merge
---
2020-01-11T06:54:13.5948331Z ........................................i...............i........................................... 4900/9512
2020-01-11T06:54:22.3989658Z .................................................................................................... 5000/9512
2020-01-11T06:54:28.8523323Z .....................................................................................i.............. 5100/9512
2020-01-11T06:54:34.3800142Z .................................................................................................... 5200/9512
2020-01-11T06:54:44.3400173Z ....................................................ii.ii...........i............................... 5300/9512
2020-01-11T06:54:53.1952883Z .................................................................................................... 5500/9512
2020-01-11T06:55:03.0427480Z .................................................................................................... 5600/9512
2020-01-11T06:55:09.7526656Z ....................................i............................................................... 5700/9512
2020-01-11T06:55:16.0466816Z .................................................................................................... 5800/9512
2020-01-11T06:55:16.0466816Z .................................................................................................... 5800/9512
2020-01-11T06:55:26.7660602Z .................................................................................................... 5900/9512
2020-01-11T06:55:36.8373104Z ...........................ii...i...ii..........i................................................... 6000/9512
2020-01-11T06:55:54.3926731Z .................................................................................................... 6200/9512
2020-01-11T06:56:02.4739700Z .................................................................................................... 6300/9512
2020-01-11T06:56:02.4739700Z .................................................................................................... 6300/9512
2020-01-11T06:56:13.3780611Z ......................................................i..ii......................................... 6400/9512
2020-01-11T06:56:39.5848126Z .................................................................................................... 6600/9512
2020-01-11T06:56:41.5712969Z ........F....................i...................................................................... 6700/9512
2020-01-11T06:56:43.7961005Z .................................................................................................... 6800/9512
2020-01-11T06:56:46.2255828Z ..............................i...................................F...................F............. 6900/9512
---
2020-01-11T06:58:21.4087409Z .................................................................................................... 7500/9512
2020-01-11T06:58:25.1794698Z .................................................................................................... 7600/9512
2020-01-11T06:58:30.9726438Z .................................................................................................... 7700/9512
2020-01-11T06:58:38.2893042Z .................................................................................................... 7800/9512
2020-01-11T06:58:47.5746592Z ..............................................................................iiii.................. 7900/9512
2020-01-11T06:59:03.3933035Z ............i......i................................................................................ 8100/9512
2020-01-11T06:59:08.3947730Z .................................................................................................... 8200/9512
2020-01-11T06:59:21.6882538Z .................................................................................................... 8300/9512
2020-01-11T06:59:31.1392601Z .................................................................................................... 8400/9512
---
2020-01-11T07:01:25.6854899Z diff of stderr:
2020-01-11T07:01:25.6855063Z 
2020-01-11T07:01:25.6855618Z 2   --> $DIR/E0586.rs:3:22
2020-01-11T07:01:25.6856009Z 3    |
2020-01-11T07:01:25.6856183Z 4 LL |     let x = &tmp[1..=];
2020-01-11T07:01:25.6857275Z +    |                      ^ help: use `..` instead
2020-01-11T07:01:25.6857446Z 6    |
2020-01-11T07:01:25.6857446Z 6    |
2020-01-11T07:01:25.6858056Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6858421Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6858751Z 9 error: aborting due to previous error
2020-01-11T07:01:25.6858889Z 10 
2020-01-11T07:01:25.6859224Z 
2020-01-11T07:01:25.6859449Z 
2020-01-11T07:01:25.6859449Z 
2020-01-11T07:01:25.6859648Z The actual stderr differed from the expected stderr.
2020-01-11T07:01:25.6860346Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0586/E0586.stderr
2020-01-11T07:01:25.6861006Z To update references, rerun the tests and pass the `--bless` flag
2020-01-11T07:01:25.6861597Z To only update this specific test, also pass `--test-args error-codes/E0586.rs`
2020-01-11T07:01:25.6862155Z error: 1 errors occurred comparing output.
2020-01-11T07:01:25.6862325Z status: exit code: 1
2020-01-11T07:01:25.6862325Z status: exit code: 1
2020-01-11T07:01:25.6863731Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0586.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0586" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0586/auxiliary" "-A" "unused"
2020-01-11T07:01:25.6864671Z ------------------------------------------
2020-01-11T07:01:25.6865069Z 
2020-01-11T07:01:25.6865548Z ------------------------------------------
2020-01-11T07:01:25.6865966Z stderr:
2020-01-11T07:01:25.6865966Z stderr:
2020-01-11T07:01:25.6866449Z ------------------------------------------
2020-01-11T07:01:25.6866883Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.6867465Z   --> /checkout/src/test/ui/error-codes/E0586.rs:3:22
2020-01-11T07:01:25.6867828Z    |
2020-01-11T07:01:25.6868008Z LL |     let x = &tmp[1..=]; //~ ERROR E0586
2020-01-11T07:01:25.6868383Z    |
2020-01-11T07:01:25.6868383Z    |
2020-01-11T07:01:25.6868673Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6868977Z error: aborting due to previous error
2020-01-11T07:01:25.6869104Z 
2020-01-11T07:01:25.6869827Z For more information about this error, try `rustc --explain E0586`.
2020-01-11T07:01:25.6870114Z 
---
2020-01-11T07:01:25.6874458Z 4 LL |     ..=;
2020-01-11T07:01:25.6874968Z -    |        ^
2020-01-11T07:01:25.6875320Z +    |        ^ help: use `..` instead
2020-01-11T07:01:25.6875488Z 6    |
2020-01-11T07:01:25.6876073Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6876431Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6876758Z 9 error[E0586]: inclusive range with no end
2020-01-11T07:01:25.6877689Z 10   --> $DIR/impossible_range.rs:15:9
2020-01-11T07:01:25.6878181Z 
2020-01-11T07:01:25.6878682Z 11    |
2020-01-11T07:01:25.6878682Z 11    |
2020-01-11T07:01:25.6879103Z 12 LL |     0..=;
2020-01-11T07:01:25.6879700Z -    |         ^
2020-01-11T07:01:25.6880131Z +    |         ^ help: use `..` instead
2020-01-11T07:01:25.6880767Z 14    |
2020-01-11T07:01:25.6881721Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6882127Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6883053Z 17 error: aborting due to 2 previous errors
2020-01-11T07:01:25.6883272Z 18 
2020-01-11T07:01:25.6883464Z 
2020-01-11T07:01:25.6883803Z 
2020-01-11T07:01:25.6883803Z 
2020-01-11T07:01:25.6883999Z The actual stderr differed from the expected stderr.
2020-01-11T07:01:25.6884714Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impossible_range/impossible_range.stderr
2020-01-11T07:01:25.6885381Z To update references, rerun the tests and pass the `--bless` flag
2020-01-11T07:01:25.6891154Z To only update this specific test, also pass `--test-args impossible_range.rs`
2020-01-11T07:01:25.6891275Z error: 1 errors occurred comparing output.
2020-01-11T07:01:25.6891322Z status: exit code: 1
2020-01-11T07:01:25.6891322Z status: exit code: 1
2020-01-11T07:01:25.6892190Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impossible_range.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impossible_range" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impossible_range/auxiliary" "-A" "unused"
2020-01-11T07:01:25.6892842Z ------------------------------------------
2020-01-11T07:01:25.6892915Z 
2020-01-11T07:01:25.6893197Z ------------------------------------------
2020-01-11T07:01:25.6893244Z stderr:
2020-01-11T07:01:25.6893244Z stderr:
2020-01-11T07:01:25.6893474Z ------------------------------------------
2020-01-11T07:01:25.6893524Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.6893754Z   --> /checkout/src/test/ui/impossible_range.rs:8:8
2020-01-11T07:01:25.6893812Z    |
2020-01-11T07:01:25.6893874Z LL |     ..=; //~ERROR inclusive range with no end
2020-01-11T07:01:25.6893962Z    |
2020-01-11T07:01:25.6893962Z    |
2020-01-11T07:01:25.6894024Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6894103Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.6894354Z   --> /checkout/src/test/ui/impossible_range.rs:15:9
2020-01-11T07:01:25.6894401Z    |
2020-01-11T07:01:25.6894401Z    |
2020-01-11T07:01:25.6894447Z LL |     0..=; //~ERROR inclusive range with no end
2020-01-11T07:01:25.6894558Z    |
2020-01-11T07:01:25.6894558Z    |
2020-01-11T07:01:25.6894605Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6894693Z error: aborting due to 2 previous errors
2020-01-11T07:01:25.6894722Z 
2020-01-11T07:01:25.6894976Z For more information about this error, try `rustc --explain E0586`.
2020-01-11T07:01:25.6895013Z 
---
2020-01-11T07:01:25.6895588Z diff of stderr:
2020-01-11T07:01:25.6895617Z 
2020-01-11T07:01:25.6895839Z 342   --> $DIR/attr-stmt-expr-attr-bad.rs:94:35
2020-01-11T07:01:25.6895886Z 343    |
2020-01-11T07:01:25.6895948Z 344 LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] 10 => () } }
2020-01-11T07:01:25.6896220Z +    |                                   ^^^ help: use `..` instead
2020-01-11T07:01:25.6896428Z 346    |
2020-01-11T07:01:25.6896428Z 346    |
2020-01-11T07:01:25.6896720Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6896777Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6896822Z 348 
2020-01-11T07:01:25.6896959Z 349 error: expected one of `=>`, `if`, or `|`, found `#`
2020-01-11T07:01:25.6897261Z 
2020-01-11T07:01:25.6897498Z 356   --> $DIR/attr-stmt-expr-attr-bad.rs:97:35
2020-01-11T07:01:25.6897543Z 357    |
2020-01-11T07:01:25.6897543Z 357    |
2020-01-11T07:01:25.6897788Z 358 LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] -10 => () } }
2020-01-11T07:01:25.6898075Z +    |                                   ^^^ help: use `..` instead
2020-01-11T07:01:25.6898119Z 360    |
2020-01-11T07:01:25.6898119Z 360    |
2020-01-11T07:01:25.6898365Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6898450Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6898494Z 362 
2020-01-11T07:01:25.6898540Z 363 error: expected one of `=>`, `if`, or `|`, found `#`
2020-01-11T07:01:25.6898828Z 
2020-01-11T07:01:25.6899049Z 376   --> $DIR/attr-stmt-expr-attr-bad.rs:102:35
2020-01-11T07:01:25.6899110Z 377    |
2020-01-11T07:01:25.6899110Z 377    |
2020-01-11T07:01:25.6899159Z 378 LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] FOO => () } }
2020-01-11T07:01:25.6899431Z +    |                                   ^^^ help: use `..` instead
2020-01-11T07:01:25.6899489Z 380    |
2020-01-11T07:01:25.6899489Z 380    |
2020-01-11T07:01:25.6899735Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6899791Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6899857Z 382 
2020-01-11T07:01:25.6899903Z 383 error: expected one of `=>`, `if`, or `|`, found `#`
2020-01-11T07:01:25.6900177Z 
2020-01-11T07:01:25.6900203Z 
2020-01-11T07:01:25.6900247Z The actual stderr differed from the expected stderr.
2020-01-11T07:01:25.6900572Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr-stmt-expr-attr-bad/attr-stmt-expr-attr-bad.stderr
2020-01-11T07:01:25.6900572Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr-stmt-expr-attr-bad/attr-stmt-expr-attr-bad.stderr
2020-01-11T07:01:25.6900844Z To update references, rerun the tests and pass the `--bless` flag
2020-01-11T07:01:25.6901142Z To only update this specific test, also pass `--test-args parser/attr-stmt-expr-attr-bad.rs`
2020-01-11T07:01:25.6901240Z error: 1 errors occurred comparing output.
2020-01-11T07:01:25.6901286Z status: exit code: 1
2020-01-11T07:01:25.6901286Z status: exit code: 1
2020-01-11T07:01:25.6902138Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr-stmt-expr-attr-bad" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr-stmt-expr-attr-bad/auxiliary" "-A" "unused"
2020-01-11T07:01:25.6902499Z ------------------------------------------
2020-01-11T07:01:25.6902781Z 
2020-01-11T07:01:25.6903066Z ------------------------------------------
2020-01-11T07:01:25.6903112Z stderr:
2020-01-11T07:01:25.6903112Z stderr:
2020-01-11T07:01:25.6903341Z ------------------------------------------
2020-01-11T07:01:25.6903394Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6903640Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:5:36
2020-01-11T07:01:25.6903824Z    |
2020-01-11T07:01:25.6903872Z LL | #[cfg(FALSE)] fn e() { let _ = box #![attr] 0; }
2020-01-11T07:01:25.6903988Z    |
2020-01-11T07:01:25.6903988Z    |
2020-01-11T07:01:25.6904106Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6904217Z error: expected expression, found `]`
2020-01-11T07:01:25.6904511Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:7:40
2020-01-11T07:01:25.6904560Z    |
2020-01-11T07:01:25.6904560Z    |
2020-01-11T07:01:25.6904621Z LL | #[cfg(FALSE)] fn e() { let _ = [#[attr]]; }
2020-01-11T07:01:25.6904700Z 
2020-01-11T07:01:25.6904700Z 
2020-01-11T07:01:25.6904764Z error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `#`
2020-01-11T07:01:25.6905070Z    |
2020-01-11T07:01:25.6905070Z    |
2020-01-11T07:01:25.6905132Z LL | #[cfg(FALSE)] fn e() { let _ = foo#[attr](); }
2020-01-11T07:01:25.6905183Z    |                                   ^ expected one of 7 possible tokens
2020-01-11T07:01:25.6905265Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6905531Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:11:36
2020-01-11T07:01:25.6905580Z    |
2020-01-11T07:01:25.6905580Z    |
2020-01-11T07:01:25.6905624Z LL | #[cfg(FALSE)] fn e() { let _ = foo(#![attr]); }
2020-01-11T07:01:25.6905730Z    |
2020-01-11T07:01:25.6905730Z    |
2020-01-11T07:01:25.6905793Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6905892Z error: expected expression, found `)`
2020-01-11T07:01:25.6906148Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:11:44
2020-01-11T07:01:25.6906194Z    |
2020-01-11T07:01:25.6906194Z    |
2020-01-11T07:01:25.6906257Z LL | #[cfg(FALSE)] fn e() { let _ = foo(#![attr]); }
2020-01-11T07:01:25.6906345Z 
2020-01-11T07:01:25.6906403Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6906651Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:14:38
2020-01-11T07:01:25.6906697Z    |
2020-01-11T07:01:25.6906697Z    |
2020-01-11T07:01:25.6906741Z LL | #[cfg(FALSE)] fn e() { let _ = x.foo(#![attr]); }
2020-01-11T07:01:25.6906846Z    |
2020-01-11T07:01:25.6906846Z    |
2020-01-11T07:01:25.6906909Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6907017Z error: expected expression, found `)`
2020-01-11T07:01:25.6907262Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:14:46
2020-01-11T07:01:25.6907323Z    |
2020-01-11T07:01:25.6907323Z    |
2020-01-11T07:01:25.6907367Z LL | #[cfg(FALSE)] fn e() { let _ = x.foo(#![attr]); }
2020-01-11T07:01:25.6907457Z 
2020-01-11T07:01:25.6907516Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6907761Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:17:36
2020-01-11T07:01:25.6907807Z    |
2020-01-11T07:01:25.6907807Z    |
2020-01-11T07:01:25.6907867Z LL | #[cfg(FALSE)] fn e() { let _ = 0 + #![attr] 0; }
2020-01-11T07:01:25.6907955Z    |
2020-01-11T07:01:25.6907955Z    |
2020-01-11T07:01:25.6908032Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6908246Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6908557Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:19:33
2020-01-11T07:01:25.6908605Z    |
2020-01-11T07:01:25.6908605Z    |
2020-01-11T07:01:25.6908727Z LL | #[cfg(FALSE)] fn e() { let _ = !#![attr] 0; }
2020-01-11T07:01:25.6908838Z    |
2020-01-11T07:01:25.6908838Z    |
2020-01-11T07:01:25.6908900Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6909001Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6909278Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:21:33
2020-01-11T07:01:25.6909325Z    |
2020-01-11T07:01:25.6909325Z    |
2020-01-11T07:01:25.6909567Z LL | #[cfg(FALSE)] fn e() { let _ = -#![attr] 0; }
2020-01-11T07:01:25.6909670Z    |
2020-01-11T07:01:25.6909670Z    |
2020-01-11T07:01:25.6909755Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6909798Z 
2020-01-11T07:01:25.6909844Z error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `#`
2020-01-11T07:01:25.6910156Z    |
2020-01-11T07:01:25.6910156Z    |
2020-01-11T07:01:25.6910200Z LL | #[cfg(FALSE)] fn e() { let _ = x #![attr] as Y; }
2020-01-11T07:01:25.6910250Z    |                                  ^ expected one of 7 possible tokens
2020-01-11T07:01:25.6910340Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6910585Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:25:35
2020-01-11T07:01:25.6910658Z    |
2020-01-11T07:01:25.6910658Z    |
2020-01-11T07:01:25.6910703Z LL | #[cfg(FALSE)] fn e() { let _ = || #![attr] foo; }
2020-01-11T07:01:25.6910791Z    |
2020-01-11T07:01:25.6910791Z    |
2020-01-11T07:01:25.6910876Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6910962Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6911225Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:27:40
2020-01-11T07:01:25.6911271Z    |
2020-01-11T07:01:25.6911271Z    |
2020-01-11T07:01:25.6911316Z LL | #[cfg(FALSE)] fn e() { let _ = move || #![attr] foo; }
2020-01-11T07:01:25.6911461Z    |
2020-01-11T07:01:25.6911461Z    |
2020-01-11T07:01:25.6911523Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6911632Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6911894Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:29:35
2020-01-11T07:01:25.6911941Z    |
2020-01-11T07:01:25.6911941Z    |
2020-01-11T07:01:25.6912002Z LL | #[cfg(FALSE)] fn e() { let _ = || #![attr] {foo}; }
2020-01-11T07:01:25.6912090Z    |
2020-01-11T07:01:25.6912090Z    |
2020-01-11T07:01:25.6912168Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6912253Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6912782Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:31:40
2020-01-11T07:01:25.6912971Z    |
2020-01-11T07:01:25.6912971Z    |
2020-01-11T07:01:25.6913018Z LL | #[cfg(FALSE)] fn e() { let _ = move || #![attr] {foo}; }
2020-01-11T07:01:25.6913127Z    |
2020-01-11T07:01:25.6913127Z    |
2020-01-11T07:01:25.6913246Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6913351Z error: expected expression, found `..`
2020-01-11T07:01:25.6913687Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:33:40
2020-01-11T07:01:25.6913736Z    |
2020-01-11T07:01:25.6913736Z    |
2020-01-11T07:01:25.6913796Z LL | #[cfg(FALSE)] fn e() { let _ = #[attr] ..#[attr] 0; }
2020-01-11T07:01:25.6913877Z 
2020-01-11T07:01:25.6913933Z error: expected expression, found `..`
2020-01-11T07:01:25.6914193Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:35:40
2020-01-11T07:01:25.6914239Z    |
2020-01-11T07:01:25.6914239Z    |
2020-01-11T07:01:25.6914283Z LL | #[cfg(FALSE)] fn e() { let _ = #[attr] ..; }
2020-01-11T07:01:25.6914379Z 
2020-01-11T07:01:25.6914431Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6914696Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:37:41
2020-01-11T07:01:25.6914743Z    |
2020-01-11T07:01:25.6914743Z    |
2020-01-11T07:01:25.6914788Z LL | #[cfg(FALSE)] fn e() { let _ = #[attr] &#![attr] 0; }
2020-01-11T07:01:25.6914890Z    |
2020-01-11T07:01:25.6914890Z    |
2020-01-11T07:01:25.6914953Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6915062Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6915311Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:39:45
2020-01-11T07:01:25.6915357Z    |
2020-01-11T07:01:25.6915357Z    |
2020-01-11T07:01:25.6915419Z LL | #[cfg(FALSE)] fn e() { let _ = #[attr] &mut #![attr] 0; }
2020-01-11T07:01:25.6915519Z    |
2020-01-11T07:01:25.6915519Z    |
2020-01-11T07:01:25.6915595Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6915680Z error: attributes are not yet allowed on `if` expressions
2020-01-11T07:01:25.6915928Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:41:32
2020-01-11T07:01:25.6915990Z    |
2020-01-11T07:01:25.6915990Z    |
2020-01-11T07:01:25.6916034Z LL | #[cfg(FALSE)] fn e() { let _ = #[attr] if 0 {}; }
2020-01-11T07:01:25.6916133Z 
2020-01-11T07:01:25.6916133Z 
2020-01-11T07:01:25.6916174Z error: expected `{`, found `#`
2020-01-11T07:01:25.6916483Z    |
2020-01-11T07:01:25.6916483Z    |
2020-01-11T07:01:25.6916527Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 #[attr] {}; }
2020-01-11T07:01:25.6916823Z    |                                --   ^       --- help: try placing this code inside a block: `{ {}; }`
2020-01-11T07:01:25.6916944Z    |                                |    expected `{`
2020-01-11T07:01:25.6917000Z    |                                this `if` expression has a condition, but no block
2020-01-11T07:01:25.6917034Z 
2020-01-11T07:01:25.6917096Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6917096Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6917369Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:45:38
2020-01-11T07:01:25.6917419Z    |
2020-01-11T07:01:25.6917585Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {#![attr]}; }
2020-01-11T07:01:25.6917682Z    |
2020-01-11T07:01:25.6917682Z    |
2020-01-11T07:01:25.6917818Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6917867Z 
2020-01-11T07:01:25.6917915Z error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
2020-01-11T07:01:25.6918287Z    |
2020-01-11T07:01:25.6918287Z    |
2020-01-11T07:01:25.6918336Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} #[attr] else {}; }
2020-01-11T07:01:25.6918392Z    |                                        ^ expected one of `.`, `;`, `?`, `else`, or an operator
2020-01-11T07:01:25.6918441Z 
2020-01-11T07:01:25.6918484Z error: expected `{`, found `#`
2020-01-11T07:01:25.6918827Z    |
2020-01-11T07:01:25.6918827Z    |
2020-01-11T07:01:25.6918877Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else #[attr] {}; }
2020-01-11T07:01:25.6919197Z    |                                             ^       --- help: try placing this code inside a block: `{ {}; }`
2020-01-11T07:01:25.6919331Z    |                                             expected `{`
2020-01-11T07:01:25.6919363Z 
2020-01-11T07:01:25.6919410Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6919699Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:51:46
2020-01-11T07:01:25.6919699Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:51:46
2020-01-11T07:01:25.6919749Z    |
2020-01-11T07:01:25.6919797Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else {#![attr]}; }
2020-01-11T07:01:25.6919908Z    |
2020-01-11T07:01:25.6919908Z    |
2020-01-11T07:01:25.6920120Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6920240Z error: attributes are not yet allowed on `if` expressions
2020-01-11T07:01:25.6927225Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:53:45
2020-01-11T07:01:25.6927301Z    |
2020-01-11T07:01:25.6927301Z    |
2020-01-11T07:01:25.6927373Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else #[attr] if 0 {}; }
2020-01-11T07:01:25.6927455Z 
2020-01-11T07:01:25.6927455Z 
2020-01-11T07:01:25.6927512Z error: expected `{`, found `#`
2020-01-11T07:01:25.6927908Z    |
2020-01-11T07:01:25.6927908Z    |
2020-01-11T07:01:25.6927955Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else #[attr] if 0 {}; }
2020-01-11T07:01:25.6928278Z    |                                             ^       -------- help: try placing this code inside a block: `{ if 0 {}; }`
2020-01-11T07:01:25.6928396Z    |                                             expected `{`
2020-01-11T07:01:25.6928443Z 
2020-01-11T07:01:25.6928443Z 
2020-01-11T07:01:25.6928483Z error: expected `{`, found `#`
2020-01-11T07:01:25.6928803Z    |
2020-01-11T07:01:25.6928803Z    |
2020-01-11T07:01:25.6928849Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else if 0 #[attr] {}; }
2020-01-11T07:01:25.6929143Z    |                                             --   ^       --- help: try placing this code inside a block: `{ {}; }`
2020-01-11T07:01:25.6929266Z    |                                             |    expected `{`
2020-01-11T07:01:25.6929322Z    |                                             this `if` expression has a condition, but no block
2020-01-11T07:01:25.6929498Z 
2020-01-11T07:01:25.6929562Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6929562Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6929857Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:58:51
2020-01-11T07:01:25.6929904Z    |
2020-01-11T07:01:25.6929967Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else if 0 {#![attr]}; }
2020-01-11T07:01:25.6930155Z    |
2020-01-11T07:01:25.6930155Z    |
2020-01-11T07:01:25.6930235Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6930323Z error: attributes are not yet allowed on `if` expressions
2020-01-11T07:01:25.6930620Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:60:32
2020-01-11T07:01:25.6930668Z    |
2020-01-11T07:01:25.6930668Z    |
2020-01-11T07:01:25.6930714Z LL | #[cfg(FALSE)] fn e() { let _ = #[attr] if let _ = 0 {}; }
2020-01-11T07:01:25.6930814Z 
2020-01-11T07:01:25.6930814Z 
2020-01-11T07:01:25.6930854Z error: expected `{`, found `#`
2020-01-11T07:01:25.6931161Z    |
2020-01-11T07:01:25.6931161Z    |
2020-01-11T07:01:25.6931215Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 #[attr] {}; }
2020-01-11T07:01:25.6931507Z    |                                --           ^       --- help: try placing this code inside a block: `{ {}; }`
2020-01-11T07:01:25.6931628Z    |                                |            expected `{`
2020-01-11T07:01:25.6931681Z    |                                this `if` expression has a condition, but no block
2020-01-11T07:01:25.6931714Z 
2020-01-11T07:01:25.6931773Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6931773Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6932020Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:64:46
2020-01-11T07:01:25.6932078Z    |
2020-01-11T07:01:25.6932140Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {#![attr]}; }
2020-01-11T07:01:25.6932231Z    |
2020-01-11T07:01:25.6932231Z    |
2020-01-11T07:01:25.6932316Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6932358Z 
2020-01-11T07:01:25.6932404Z error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
2020-01-11T07:01:25.6933021Z    |
2020-01-11T07:01:25.6933021Z    |
2020-01-11T07:01:25.6933068Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} #[attr] else {}; }
2020-01-11T07:01:25.6933124Z    |                                                ^ expected one of `.`, `;`, `?`, `else`, or an operator
2020-01-11T07:01:25.6933174Z 
2020-01-11T07:01:25.6933225Z error: expected `{`, found `#`
2020-01-11T07:01:25.6933541Z    |
2020-01-11T07:01:25.6933541Z    |
2020-01-11T07:01:25.6933588Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else #[attr] {}; }
2020-01-11T07:01:25.6933894Z    |                                                     ^       --- help: try placing this code inside a block: `{ {}; }`
2020-01-11T07:01:25.6934024Z    |                                                     expected `{`
2020-01-11T07:01:25.6934057Z 
2020-01-11T07:01:25.6934103Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6934396Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:70:54
2020-01-11T07:01:25.6934396Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:70:54
2020-01-11T07:01:25.6934446Z    |
2020-01-11T07:01:25.6934497Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else {#![attr]}; }
2020-01-11T07:01:25.6934737Z    |
2020-01-11T07:01:25.6934737Z    |
2020-01-11T07:01:25.6934803Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6934972Z error: attributes are not yet allowed on `if` expressions
2020-01-11T07:01:25.6935288Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:72:53
2020-01-11T07:01:25.6935355Z    |
2020-01-11T07:01:25.6935355Z    |
2020-01-11T07:01:25.6935408Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else #[attr] if let _ = 0 {}; }
2020-01-11T07:01:25.6935493Z 
2020-01-11T07:01:25.6935493Z 
2020-01-11T07:01:25.6935551Z error: expected `{`, found `#`
2020-01-11T07:01:25.6935868Z    |
2020-01-11T07:01:25.6935868Z    |
2020-01-11T07:01:25.6935946Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else #[attr] if let _ = 0 {}; }
2020-01-11T07:01:25.6941893Z    |                                                     ^       ---------------- help: try placing this code inside a block: `{ if let _ = 0 {}; }`
2020-01-11T07:01:25.6947021Z    |                                                     expected `{`
2020-01-11T07:01:25.6947116Z 
2020-01-11T07:01:25.6947116Z 
2020-01-11T07:01:25.6947164Z error: expected `{`, found `#`
2020-01-11T07:01:25.6947651Z    |
2020-01-11T07:01:25.6947651Z    |
2020-01-11T07:01:25.6947702Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else if let _ = 0 #[attr] {}; }
2020-01-11T07:01:25.6948026Z    |                                                     --           ^       --- help: try placing this code inside a block: `{ {}; }`
2020-01-11T07:01:25.6948180Z    |                                                     |            expected `{`
2020-01-11T07:01:25.6948240Z    |                                                     this `if` expression has a condition, but no block
2020-01-11T07:01:25.6948297Z 
2020-01-11T07:01:25.6948351Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6948351Z error: an inner attribute is not permitted in this context
2020-01-11T07:01:25.6948641Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:77:67
2020-01-11T07:01:25.6948710Z    |
2020-01-11T07:01:25.6948762Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else if let _ = 0 {#![attr]}; }
2020-01-11T07:01:25.6948880Z    |
2020-01-11T07:01:25.6948880Z    |
2020-01-11T07:01:25.6948946Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6949052Z error: an inner attribute is not permitted following an outer attribute
2020-01-11T07:01:25.6949345Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:80:32
2020-01-11T07:01:25.6949394Z    |
2020-01-11T07:01:25.6949394Z    |
2020-01-11T07:01:25.6949444Z LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] let _ = 0; }
2020-01-11T07:01:25.6949755Z    |                        ------- ^^^^^^^^ not permitted following an outer attibute
2020-01-11T07:01:25.6949860Z    |                        previous outer attribute
2020-01-11T07:01:25.6949922Z    |
2020-01-11T07:01:25.6949922Z    |
2020-01-11T07:01:25.6949987Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6950094Z error: an inner attribute is not permitted following an outer attribute
2020-01-11T07:01:25.6950560Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:82:32
2020-01-11T07:01:25.6950611Z    |
2020-01-11T07:01:25.6950611Z    |
2020-01-11T07:01:25.6950676Z LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] 0; }
2020-01-11T07:01:25.6950962Z    |                        ------- ^^^^^^^^ not permitted following an outer attibute
2020-01-11T07:01:25.6951167Z    |                        previous outer attribute
2020-01-11T07:01:25.6951229Z    |
2020-01-11T07:01:25.6951229Z    |
2020-01-11T07:01:25.6951294Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6951401Z error: an inner attribute is not permitted following an outer attribute
2020-01-11T07:01:25.6951703Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:84:32
2020-01-11T07:01:25.6951753Z    |
2020-01-11T07:01:25.6951753Z    |
2020-01-11T07:01:25.6951817Z LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo!(); }
2020-01-11T07:01:25.6952118Z    |                        ------- ^^^^^^^^ not permitted following an outer attibute
2020-01-11T07:01:25.6952234Z    |                        previous outer attribute
2020-01-11T07:01:25.6952280Z    |
2020-01-11T07:01:25.6952280Z    |
2020-01-11T07:01:25.6952353Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6952461Z error: an inner attribute is not permitted following an outer attribute
2020-01-11T07:01:25.6953048Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:86:32
2020-01-11T07:01:25.6953104Z    |
2020-01-11T07:01:25.6953104Z    |
2020-01-11T07:01:25.6953170Z LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo![]; }
2020-01-11T07:01:25.6953438Z    |                        ------- ^^^^^^^^ not permitted following an outer attibute
2020-01-11T07:01:25.6953565Z    |                        previous outer attribute
2020-01-11T07:01:25.6953609Z    |
2020-01-11T07:01:25.6953609Z    |
2020-01-11T07:01:25.6953679Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6953785Z error: an inner attribute is not permitted following an outer attribute
2020-01-11T07:01:25.6954036Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:88:32
2020-01-11T07:01:25.6954099Z    |
2020-01-11T07:01:25.6954099Z    |
2020-01-11T07:01:25.6954145Z LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo!{}; }
2020-01-11T07:01:25.6954405Z    |                        ------- ^^^^^^^^ not permitted following an outer attibute
2020-01-11T07:01:25.6954517Z    |                        previous outer attribute
2020-01-11T07:01:25.6954568Z    |
2020-01-11T07:01:25.6954568Z    |
2020-01-11T07:01:25.6954631Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-01-11T07:01:25.6954734Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.6954989Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:94:35
2020-01-11T07:01:25.6955052Z    |
2020-01-11T07:01:25.6955052Z    |
2020-01-11T07:01:25.6955099Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] 10 => () } }
2020-01-11T07:01:25.6955207Z    |
2020-01-11T07:01:25.6955207Z    |
2020-01-11T07:01:25.6955254Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6955285Z 
2020-01-11T07:01:25.6955328Z error: expected one of `=>`, `if`, or `|`, found `#`
2020-01-11T07:01:25.6955763Z    |
2020-01-11T07:01:25.6955763Z    |
2020-01-11T07:01:25.6955810Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] 10 => () } }
2020-01-11T07:01:25.6955879Z    |                                      ^ expected one of `=>`, `if`, or `|`
2020-01-11T07:01:25.6955954Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.6956314Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:97:35
2020-01-11T07:01:25.6956389Z    |
2020-01-11T07:01:25.6956389Z    |
2020-01-11T07:01:25.6956659Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] -10 => () } }
2020-01-11T07:01:25.6956773Z    |
2020-01-11T07:01:25.6956773Z    |
2020-01-11T07:01:25.6956821Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6956853Z 
2020-01-11T07:01:25.6956912Z error: expected one of `=>`, `if`, or `|`, found `#`
2020-01-11T07:01:25.6957217Z    |
2020-01-11T07:01:25.6957217Z    |
2020-01-11T07:01:25.6957453Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] -10 => () } }
2020-01-11T07:01:25.6957524Z    |                                      ^ expected one of `=>`, `if`, or `|`
2020-01-11T07:01:25.6957596Z error: unexpected token: `#`
2020-01-11T07:01:25.6957865Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:100:39
2020-01-11T07:01:25.6957912Z    |
2020-01-11T07:01:25.6957912Z    |
2020-01-11T07:01:25.6958149Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=-#[attr] 10 => () } }
2020-01-11T07:01:25.6958244Z 
2020-01-11T07:01:25.6958285Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.6958527Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:102:35
2020-01-11T07:01:25.6958588Z    |
2020-01-11T07:01:25.6958588Z    |
2020-01-11T07:01:25.6958634Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] FOO => () } }
2020-01-11T07:01:25.6958749Z    |
2020-01-11T07:01:25.6958749Z    |
2020-01-11T07:01:25.6959018Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6959051Z 
2020-01-11T07:01:25.6959094Z error: expected one of `=>`, `if`, or `|`, found `#`
2020-01-11T07:01:25.6966169Z    |
2020-01-11T07:01:25.6966169Z    |
2020-01-11T07:01:25.6966218Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] FOO => () } }
2020-01-11T07:01:25.6966293Z    |                                      ^ expected one of `=>`, `if`, or `|`
2020-01-11T07:01:25.6966367Z error: unexpected token: `#`
2020-01-11T07:01:25.6966641Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:106:34
2020-01-11T07:01:25.6966706Z    |
2020-01-11T07:01:25.6966706Z    |
2020-01-11T07:01:25.6966751Z LL | #[cfg(FALSE)] fn e() { let _ = x.#![attr]foo(); }
2020-01-11T07:01:25.6966844Z 
2020-01-11T07:01:25.6966844Z 
2020-01-11T07:01:25.6966890Z error: expected one of `.`, `;`, `?`, or an operator, found `#`
2020-01-11T07:01:25.6967214Z    |
2020-01-11T07:01:25.6967214Z    |
2020-01-11T07:01:25.6967260Z LL | #[cfg(FALSE)] fn e() { let _ = x.#![attr]foo(); }
2020-01-11T07:01:25.6967320Z    |                                  ^ expected one of `.`, `;`, `?`, or an operator
2020-01-11T07:01:25.6967408Z error: unexpected token: `#`
2020-01-11T07:01:25.6967654Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:109:34
2020-01-11T07:01:25.6967701Z    |
2020-01-11T07:01:25.6967701Z    |
2020-01-11T07:01:25.6967763Z LL | #[cfg(FALSE)] fn e() { let _ = x.#[attr]foo(); }
2020-01-11T07:01:25.6967838Z 
2020-01-11T07:01:25.6967838Z 
2020-01-11T07:01:25.6967882Z error: expected one of `.`, `;`, `?`, or an operator, found `#`
2020-01-11T07:01:25.6968191Z    |
2020-01-11T07:01:25.6968191Z    |
2020-01-11T07:01:25.6968396Z LL | #[cfg(FALSE)] fn e() { let _ = x.#[attr]foo(); }
2020-01-11T07:01:25.6968468Z    |                                  ^ expected one of `.`, `;`, `?`, or an operator
2020-01-11T07:01:25.6968545Z error: expected statement after outer attribute
2020-01-11T07:01:25.6968928Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:114:44
2020-01-11T07:01:25.6969005Z    |
2020-01-11T07:01:25.6969005Z    |
2020-01-11T07:01:25.6969051Z LL | #[cfg(FALSE)] fn e() { { fn foo() { #[attr]; } } }
2020-01-11T07:01:25.6969145Z 
2020-01-11T07:01:25.6969188Z error: expected statement after outer attribute
2020-01-11T07:01:25.6969468Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:116:45
2020-01-11T07:01:25.6969516Z    |
2020-01-11T07:01:25.6969516Z    |
2020-01-11T07:01:25.6969576Z LL | #[cfg(FALSE)] fn e() { { fn foo() { #[attr] } } }
2020-01-11T07:01:25.6969653Z 
2020-01-11T07:01:25.6969719Z error: aborting due to 57 previous errors
2020-01-11T07:01:25.6969748Z 
2020-01-11T07:01:25.6969994Z For more information about this error, try `rustc --explain E0586`.
---
2020-01-11T07:01:25.6977242Z 4 LL |     for _ in 1..= {}
2020-01-11T07:01:25.6977478Z -    |                   ^
2020-01-11T07:01:25.6977529Z +    |                   ^ help: use `..` instead
2020-01-11T07:01:25.6977591Z 6    |
2020-01-11T07:01:25.6977850Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6977906Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6978029Z 9 error: aborting due to previous error
2020-01-11T07:01:25.6978072Z 10 
2020-01-11T07:01:25.6978101Z 
2020-01-11T07:01:25.6978127Z 
2020-01-11T07:01:25.6978127Z 
2020-01-11T07:01:25.6978187Z The actual stderr differed from the expected stderr.
2020-01-11T07:01:25.6978508Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/range_inclusive/range_inclusive.stderr
2020-01-11T07:01:25.6978758Z To update references, rerun the tests and pass the `--bless` flag
2020-01-11T07:01:25.6979079Z To only update this specific test, also pass `--test-args parser/range_inclusive.rs`
2020-01-11T07:01:25.6979164Z error: 1 errors occurred comparing output.
2020-01-11T07:01:25.6979225Z status: exit code: 1
2020-01-11T07:01:25.6979225Z status: exit code: 1
2020-01-11T07:01:25.6980061Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/range_inclusive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/range_inclusive" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/range_inclusive/auxiliary" "-A" "unused"
2020-01-11T07:01:25.6980427Z ------------------------------------------
2020-01-11T07:01:25.6980465Z 
2020-01-11T07:01:25.6980706Z ------------------------------------------
2020-01-11T07:01:25.6980769Z stderr:
2020-01-11T07:01:25.6980769Z stderr:
2020-01-11T07:01:25.6980999Z ------------------------------------------
2020-01-11T07:01:25.6981050Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.6981319Z   --> /checkout/src/test/ui/parser/range_inclusive.rs:4:19
2020-01-11T07:01:25.6981371Z    |
2020-01-11T07:01:25.6981424Z LL |     for _ in 1..= {} //~ERROR inclusive range with no end
2020-01-11T07:01:25.6981714Z    |
2020-01-11T07:01:25.6981714Z    |
2020-01-11T07:01:25.6981764Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6981860Z error: aborting due to previous error
2020-01-11T07:01:25.6981891Z 
2020-01-11T07:01:25.6982291Z For more information about this error, try `rustc --explain E0586`.
2020-01-11T07:01:25.6982336Z 
---
2020-01-11T07:01:25.6983313Z diff of stderr:
2020-01-11T07:01:25.6983341Z 
2020-01-11T07:01:25.6983557Z 44   --> $DIR/recover-range-pats.rs:69:13
2020-01-11T07:01:25.6983617Z 45    |
2020-01-11T07:01:25.6983660Z 46 LL |     if let 0..= = 0 {}
2020-01-11T07:01:25.6983905Z +    |             ^^^ help: use `..` instead
2020-01-11T07:01:25.6983977Z 48    |
2020-01-11T07:01:25.6983977Z 48    |
2020-01-11T07:01:25.6984226Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6984284Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6984398Z 51 error[E0586]: inclusive range with no end
2020-01-11T07:01:25.6984619Z 52   --> $DIR/recover-range-pats.rs:70:13
2020-01-11T07:01:25.6984651Z 
2020-01-11T07:01:25.6984712Z 53    |
2020-01-11T07:01:25.6984712Z 53    |
2020-01-11T07:01:25.6984754Z 54 LL |     if let X..= = 0 {}
2020-01-11T07:01:25.6985015Z +    |             ^^^ help: use `..` instead
2020-01-11T07:01:25.6985057Z 56    |
2020-01-11T07:01:25.6985057Z 56    |
2020-01-11T07:01:25.6985303Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6985375Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6985472Z 59 error[E0586]: inclusive range with no end
2020-01-11T07:01:25.6985709Z 60   --> $DIR/recover-range-pats.rs:71:16
2020-01-11T07:01:25.6985743Z 
2020-01-11T07:01:25.6985782Z 61    |
2020-01-11T07:01:25.6985782Z 61    |
2020-01-11T07:01:25.6985824Z 62 LL |     if let true..= = 0 {}
2020-01-11T07:01:25.6986098Z +    |                ^^^ help: use `..` instead
2020-01-11T07:01:25.6986141Z 64    |
2020-01-11T07:01:25.6986141Z 64    |
2020-01-11T07:01:25.6986387Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6986458Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6986546Z 67 error: float literals must have an integer part
2020-01-11T07:01:25.6986780Z 68   --> $DIR/recover-range-pats.rs:73:12
2020-01-11T07:01:25.6986810Z 
2020-01-11T07:01:25.6987020Z 74   --> $DIR/recover-range-pats.rs:73:14
2020-01-11T07:01:25.6987020Z 74   --> $DIR/recover-range-pats.rs:73:14
2020-01-11T07:01:25.6987079Z 75    |
2020-01-11T07:01:25.6987121Z 76 LL |     if let .0..= = 0 {}
2020-01-11T07:01:25.6987374Z +    |              ^^^ help: use `..` instead
2020-01-11T07:01:25.6987433Z 78    |
2020-01-11T07:01:25.6987433Z 78    |
2020-01-11T07:01:25.6987677Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6987742Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6987846Z 81 error[E0586]: inclusive range with no end
2020-01-11T07:01:25.6988064Z 82   --> $DIR/recover-range-pats.rs:79:13
2020-01-11T07:01:25.6988097Z 
2020-01-11T07:01:25.6988151Z 83    |
2020-01-11T07:01:25.6988151Z 83    |
2020-01-11T07:01:25.6988192Z 84 LL |     if let 0... = 0 {}
2020-01-11T07:01:25.6988450Z +    |             ^^^ help: use `..` instead
2020-01-11T07:01:25.6988492Z 86    |
2020-01-11T07:01:25.6988492Z 86    |
2020-01-11T07:01:25.6988734Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6988805Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6989022Z 89 error[E0586]: inclusive range with no end
2020-01-11T07:01:25.6989290Z 90   --> $DIR/recover-range-pats.rs:80:13
2020-01-11T07:01:25.6989324Z 
2020-01-11T07:01:25.6989363Z 91    |
2020-01-11T07:01:25.6989363Z 91    |
2020-01-11T07:01:25.6989404Z 92 LL |     if let X... = 0 {}
2020-01-11T07:01:25.6989769Z +    |             ^^^ help: use `..` instead
2020-01-11T07:01:25.6989809Z 94    |
2020-01-11T07:01:25.6989809Z 94    |
2020-01-11T07:01:25.6990092Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6990166Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6990254Z 97 error[E0586]: inclusive range with no end
2020-01-11T07:01:25.6990487Z 98   --> $DIR/recover-range-pats.rs:81:16
2020-01-11T07:01:25.6990519Z 
2020-01-11T07:01:25.6990557Z 99    |
2020-01-11T07:01:25.6990557Z 99    |
2020-01-11T07:01:25.6990615Z 100 LL |     if let true... = 0 {}
2020-01-11T07:01:25.6990874Z +    |                ^^^ help: use `..` instead
2020-01-11T07:01:25.6990917Z 102    |
2020-01-11T07:01:25.6990917Z 102    |
2020-01-11T07:01:25.6991178Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6991241Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6991345Z 105 error: float literals must have an integer part
2020-01-11T07:01:25.6991566Z 106   --> $DIR/recover-range-pats.rs:83:12
2020-01-11T07:01:25.6991598Z 
2020-01-11T07:01:25.6991809Z 112   --> $DIR/recover-range-pats.rs:83:14
2020-01-11T07:01:25.6991809Z 112   --> $DIR/recover-range-pats.rs:83:14
2020-01-11T07:01:25.6991870Z 113    |
2020-01-11T07:01:25.6991911Z 114 LL |     if let .0... = 0 {}
2020-01-11T07:01:25.6992168Z +    |              ^^^ help: use `..` instead
2020-01-11T07:01:25.6992210Z 116    |
2020-01-11T07:01:25.6992210Z 116    |
2020-01-11T07:01:25.6992455Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6992767Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.6992868Z 119 error: float literals must have an integer part
2020-01-11T07:01:25.6993167Z 120   --> $DIR/recover-range-pats.rs:93:15
2020-01-11T07:01:25.6993200Z 
2020-01-11T07:01:25.6993431Z 132   --> $DIR/recover-range-pats.rs:109:12
2020-01-11T07:01:25.6993431Z 132   --> $DIR/recover-range-pats.rs:109:12
2020-01-11T07:01:25.6993476Z 133    |
2020-01-11T07:01:25.6993534Z 134 LL |     if let ...3 = 0 {}
2020-01-11T07:01:25.6993906Z -    |
2020-01-11T07:01:25.6994125Z -    = help: use `..=` instead
2020-01-11T07:01:25.6994175Z +    |            ^^^ help: use `..=` instead
2020-01-11T07:01:25.6994218Z 138 
2020-01-11T07:01:25.6994218Z 138 
2020-01-11T07:01:25.6994447Z 139 error: range-to patterns with `...` are not allowed
2020-01-11T07:01:25.6994717Z 
2020-01-11T07:01:25.6994756Z 141    |
2020-01-11T07:01:25.6994756Z 141    |
2020-01-11T07:01:25.6994805Z 142 LL |     if let ...Y = 0 {}
2020-01-11T07:01:25.6995191Z -    |
2020-01-11T07:01:25.6995390Z -    = help: use `..=` instead
2020-01-11T07:01:25.6995457Z +    |            ^^^ help: use `..=` instead
2020-01-11T07:01:25.6995500Z 146 
2020-01-11T07:01:25.6995500Z 146 
2020-01-11T07:01:25.6995736Z 147 error: range-to patterns with `...` are not allowed
2020-01-11T07:01:25.6996006Z 
2020-01-11T07:01:25.6996046Z 149    |
2020-01-11T07:01:25.6996088Z 150 LL |     if let ...true = 0 {}
2020-01-11T07:01:25.6996296Z -    |            ^^^
---
2020-01-11T07:01:25.6997042Z 156   --> $DIR/recover-range-pats.rs:116:15
2020-01-11T07:01:25.6997088Z 
2020-01-11T07:01:25.6997464Z 162   --> $DIR/recover-range-pats.rs:116:12
2020-01-11T07:01:25.6997512Z 163    |
2020-01-11T07:01:25.6997621Z 164 LL |     if let ....3 = 0 {}
2020-01-11T07:01:25.6997998Z -    |
2020-01-11T07:01:25.6998198Z -    = help: use `..=` instead
2020-01-11T07:01:25.6998343Z +    |            ^^^ help: use `..=` instead
2020-01-11T07:01:25.6998391Z 168 
2020-01-11T07:01:25.6998391Z 168 
2020-01-11T07:01:25.6998646Z 169 error: range-to patterns with `...` are not allowed
2020-01-11T07:01:25.6998917Z 
2020-01-11T07:01:25.6998956Z 171    |
2020-01-11T07:01:25.6998997Z 172 LL |             let ...$e;
2020-01-11T07:01:25.6999208Z -    |                 ^^^
---
2020-01-11T07:01:25.7000350Z 180 error[E0586]: inclusive range with no end
2020-01-11T07:01:25.7000617Z 181   --> $DIR/recover-range-pats.rs:141:19
2020-01-11T07:01:25.7000652Z 
2020-01-11T07:01:25.7000701Z 182    |
2020-01-11T07:01:25.7000742Z 183 LL |             let $e...;
2020-01-11T07:01:25.7001016Z +    |                   ^^^ help: use `..` instead
2020-01-11T07:01:25.7001059Z 185 ...
2020-01-11T07:01:25.7001116Z 186 LL |     mac!(0);
2020-01-11T07:01:25.7001336Z 187    |     -------- in this macro invocation
2020-01-11T07:01:25.7001336Z 187    |     -------- in this macro invocation
2020-01-11T07:01:25.7001368Z 
2020-01-11T07:01:25.7001407Z 188    |
2020-01-11T07:01:25.7001669Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7001726Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7001838Z 191 error[E0586]: inclusive range with no end
2020-01-11T07:01:25.7002059Z 192   --> $DIR/recover-range-pats.rs:142:19
2020-01-11T07:01:25.7002090Z 
2020-01-11T07:01:25.7002129Z 193    |
2020-01-11T07:01:25.7002129Z 193    |
2020-01-11T07:01:25.7002186Z 194 LL |             let $e..=;
2020-01-11T07:01:25.7002443Z +    |                   ^^^ help: use `..` instead
2020-01-11T07:01:25.7002501Z 196 ...
2020-01-11T07:01:25.7003007Z 197 LL |     mac!(0);
2020-01-11T07:01:25.7003300Z 198    |     -------- in this macro invocation
2020-01-11T07:01:25.7003300Z 198    |     -------- in this macro invocation
2020-01-11T07:01:25.7003334Z 
2020-01-11T07:01:25.7003392Z 199    |
2020-01-11T07:01:25.7003640Z -    = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7003696Z +    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7003800Z 202 error: `...` range patterns are deprecated
2020-01-11T07:01:25.7004019Z 203   --> $DIR/recover-range-pats.rs:42:13
2020-01-11T07:01:25.7004063Z 
2020-01-11T07:01:25.7004104Z 
2020-01-11T07:01:25.7004104Z 
2020-01-11T07:01:25.7004149Z The actual stderr differed from the expected stderr.
2020-01-11T07:01:25.7004459Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-range-pats/recover-range-pats.stderr
2020-01-11T07:01:25.7004733Z To update references, rerun the tests and pass the `--bless` flag
2020-01-11T07:01:25.7005036Z To only update this specific test, also pass `--test-args parser/recover-range-pats.rs`
2020-01-11T07:01:25.7005137Z error: 1 errors occurred comparing output.
2020-01-11T07:01:25.7005183Z status: exit code: 1
2020-01-11T07:01:25.7005183Z status: exit code: 1
2020-01-11T07:01:25.7006054Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/recover-range-pats.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-range-pats" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-range-pats/auxiliary" "-A" "unused"
2020-01-11T07:01:25.7006688Z ------------------------------------------
2020-01-11T07:01:25.7006743Z 
2020-01-11T07:01:25.7006983Z ------------------------------------------
2020-01-11T07:01:25.7007030Z stderr:
2020-01-11T07:01:25.7007030Z stderr:
2020-01-11T07:01:25.7007261Z ------------------------------------------
2020-01-11T07:01:25.7007330Z error: float literals must have an integer part
2020-01-11T07:01:25.7007587Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:22:12
2020-01-11T07:01:25.7007640Z    |
2020-01-11T07:01:25.7007707Z LL |     if let .0..Y = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7007759Z    |            ^^ help: must have an integer part: `0.0`
2020-01-11T07:01:25.7007862Z error: float literals must have an integer part
2020-01-11T07:01:25.7008128Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:24:16
2020-01-11T07:01:25.7008178Z    |
2020-01-11T07:01:25.7008178Z    |
2020-01-11T07:01:25.7008226Z LL |     if let X.. .0 = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7008304Z    |                ^^ help: must have an integer part: `0.0`
2020-01-11T07:01:25.7008381Z error: float literals must have an integer part
2020-01-11T07:01:25.7008662Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:35:12
2020-01-11T07:01:25.7008712Z    |
2020-01-11T07:01:25.7008712Z    |
2020-01-11T07:01:25.7008760Z LL |     if let .0..=Y = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7008825Z    |            ^^ help: must have an integer part: `0.0`
2020-01-11T07:01:25.7008903Z error: float literals must have an integer part
2020-01-11T07:01:25.7009163Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:37:16
2020-01-11T07:01:25.7009236Z    |
2020-01-11T07:01:25.7009236Z    |
2020-01-11T07:01:25.7009285Z LL |     if let X..=.0 = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7009337Z    |                ^^ help: must have an integer part: `0.0`
2020-01-11T07:01:25.7009428Z error: float literals must have an integer part
2020-01-11T07:01:25.7009699Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:50:12
2020-01-11T07:01:25.7009748Z    |
2020-01-11T07:01:25.7009748Z    |
2020-01-11T07:01:25.7009812Z LL |     if let .0...Y = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7009863Z    |            ^^ help: must have an integer part: `0.0`
2020-01-11T07:01:25.7009941Z error: float literals must have an integer part
2020-01-11T07:01:25.7010219Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:53:17
2020-01-11T07:01:25.7010269Z    |
2020-01-11T07:01:25.7010269Z    |
2020-01-11T07:01:25.7010317Z LL |     if let X... .0 = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7010385Z    |                 ^^ help: must have an integer part: `0.0`
2020-01-11T07:01:25.7010471Z error: float literals must have an integer part
2020-01-11T07:01:25.7010749Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:63:12
2020-01-11T07:01:25.7010799Z    |
2020-01-11T07:01:25.7010799Z    |
2020-01-11T07:01:25.7010845Z LL |     if let .0.. = 0 {}
2020-01-11T07:01:25.7010904Z    |            ^^ help: must have an integer part: `0.0`
2020-01-11T07:01:25.7010996Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.7011255Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:69:13
2020-01-11T07:01:25.7011320Z    |
2020-01-11T07:01:25.7011320Z    |
2020-01-11T07:01:25.7011369Z LL |     if let 0..= = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7011464Z    |
2020-01-11T07:01:25.7011464Z    |
2020-01-11T07:01:25.7011530Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7011607Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.7012008Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:70:13
2020-01-11T07:01:25.7012058Z    |
2020-01-11T07:01:25.7012058Z    |
2020-01-11T07:01:25.7012107Z LL |     if let X..= = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7012217Z    |
2020-01-11T07:01:25.7012217Z    |
2020-01-11T07:01:25.7012341Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7012443Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.7012993Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:71:16
2020-01-11T07:01:25.7013051Z    |
2020-01-11T07:01:25.7013051Z    |
2020-01-11T07:01:25.7013118Z LL |     if let true..= = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7013211Z    |
2020-01-11T07:01:25.7013211Z    |
2020-01-11T07:01:25.7013259Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7013351Z error: float literals must have an integer part
2020-01-11T07:01:25.7013615Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:73:12
2020-01-11T07:01:25.7013678Z    |
2020-01-11T07:01:25.7013678Z    |
2020-01-11T07:01:25.7013724Z LL |     if let .0..= = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7013773Z    |            ^^ help: must have an integer part: `0.0`
2020-01-11T07:01:25.7013870Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.7014112Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:73:14
2020-01-11T07:01:25.7014159Z    |
2020-01-11T07:01:25.7014159Z    |
2020-01-11T07:01:25.7014221Z LL |     if let .0..= = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7014312Z    |
2020-01-11T07:01:25.7014312Z    |
2020-01-11T07:01:25.7014373Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7014446Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.7014684Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:79:13
2020-01-11T07:01:25.7014756Z    |
2020-01-11T07:01:25.7014756Z    |
2020-01-11T07:01:25.7014802Z LL |     if let 0... = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7014907Z    |
2020-01-11T07:01:25.7014907Z    |
2020-01-11T07:01:25.7014961Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7015034Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.7015288Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:80:13
2020-01-11T07:01:25.7015334Z    |
2020-01-11T07:01:25.7015334Z    |
2020-01-11T07:01:25.7015380Z LL |     if let X... = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7015486Z    |
2020-01-11T07:01:25.7015486Z    |
2020-01-11T07:01:25.7015533Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7015625Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.7015876Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:81:16
2020-01-11T07:01:25.7015922Z    |
2020-01-11T07:01:25.7015922Z    |
2020-01-11T07:01:25.7015984Z LL |     if let true... = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7016075Z    |
2020-01-11T07:01:25.7016075Z    |
2020-01-11T07:01:25.7016146Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7016220Z error: float literals must have an integer part
2020-01-11T07:01:25.7016458Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:83:12
2020-01-11T07:01:25.7016520Z    |
2020-01-11T07:01:25.7016520Z    |
2020-01-11T07:01:25.7016566Z LL |     if let .0... = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7016615Z    |            ^^ help: must have an integer part: `0.0`
2020-01-11T07:01:25.7016701Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.7016936Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:83:14
2020-01-11T07:01:25.7017099Z    |
2020-01-11T07:01:25.7017099Z    |
2020-01-11T07:01:25.7017164Z LL |     if let .0... = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7017255Z    |
2020-01-11T07:01:25.7017255Z    |
2020-01-11T07:01:25.7017371Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7017450Z error: float literals must have an integer part
2020-01-11T07:01:25.7017746Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:93:15
2020-01-11T07:01:25.7017794Z    |
2020-01-11T07:01:25.7017794Z    |
2020-01-11T07:01:25.7017836Z LL |     if let .. .0 = 0 {}
2020-01-11T07:01:25.7017884Z    |               ^^ help: must have an integer part: `0.0`
2020-01-11T07:01:25.7017973Z error: float literals must have an integer part
2020-01-11T07:01:25.7018213Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:103:15
2020-01-11T07:01:25.7018276Z    |
2020-01-11T07:01:25.7018276Z    |
2020-01-11T07:01:25.7018317Z LL |     if let ..=.0 = 0 {}
2020-01-11T07:01:25.7018372Z    |               ^^ help: must have an integer part: `0.0`
2020-01-11T07:01:25.7018402Z 
2020-01-11T07:01:25.7018643Z error: range-to patterns with `...` are not allowed
2020-01-11T07:01:25.7018929Z    |
2020-01-11T07:01:25.7018929Z    |
2020-01-11T07:01:25.7018994Z LL |     if let ...3 = 0 {}
2020-01-11T07:01:25.7019039Z    |            ^^^ help: use `..=` instead
2020-01-11T07:01:25.7019069Z 
2020-01-11T07:01:25.7019291Z error: range-to patterns with `...` are not allowed
2020-01-11T07:01:25.7019592Z    |
2020-01-11T07:01:25.7019592Z    |
2020-01-11T07:01:25.7019633Z LL |     if let ...Y = 0 {}
2020-01-11T07:01:25.7019694Z    |            ^^^ help: use `..=` instead
2020-01-11T07:01:25.7019723Z 
2020-01-11T07:01:25.7019943Z error: range-to patterns with `...` are not allowed
2020-01-11T07:01:25.7020253Z    |
2020-01-11T07:01:25.7020294Z LL |     if let ...true = 0 {}
2020-01-11T07:01:25.7020339Z    |            ^^^ help: use `..=` instead
2020-01-11T07:01:25.7020369Z 
2020-01-11T07:01:25.7020369Z 
2020-01-11T07:01:25.7020425Z error: float literals must have an integer part
2020-01-11T07:01:25.7020673Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:116:15
2020-01-11T07:01:25.7020719Z    |
2020-01-11T07:01:25.7020779Z LL |     if let ....3 = 0 {}
2020-01-11T07:01:25.7020826Z    |               ^^ help: must have an integer part: `0.3`
2020-01-11T07:01:25.7020856Z 
2020-01-11T07:01:25.7021095Z error: range-to patterns with `...` are not allowed
2020-01-11T07:01:25.7021382Z    |
2020-01-11T07:01:25.7021382Z    |
2020-01-11T07:01:25.7021424Z LL |     if let ....3 = 0 {}
2020-01-11T07:01:25.7021486Z    |            ^^^ help: use `..=` instead
2020-01-11T07:01:25.7021514Z 
2020-01-11T07:01:25.7021735Z error: range-to patterns with `...` are not allowed
2020-01-11T07:01:25.7022049Z    |
2020-01-11T07:01:25.7022090Z LL |             let ...$e;
2020-01-11T07:01:25.7022136Z    |                 ^^^ help: use `..=` instead
2020-01-11T07:01:25.7022193Z ...
2020-01-11T07:01:25.7022193Z ...
2020-01-11T07:01:25.7022242Z LL |     mac!(0);
2020-01-11T07:01:25.7022461Z    |     -------- in this macro invocation
2020-01-11T07:01:25.7022712Z 
2020-01-11T07:01:25.7022779Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.7023071Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:141:19
2020-01-11T07:01:25.7023117Z    |
2020-01-11T07:01:25.7023183Z LL |             let $e...; //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7023275Z ...
2020-01-11T07:01:25.7023334Z LL |     mac!(0);
2020-01-11T07:01:25.7023832Z    |     -------- in this macro invocation
2020-01-11T07:01:25.7023878Z    |
2020-01-11T07:01:25.7023878Z    |
2020-01-11T07:01:25.7024070Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7024147Z error[E0586]: inclusive range with no end
2020-01-11T07:01:25.7024704Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:142:19
2020-01-11T07:01:25.7024786Z    |
2020-01-11T07:01:25.7024786Z    |
2020-01-11T07:01:25.7024951Z LL |             let $e..=; //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7025065Z ...
2020-01-11T07:01:25.7025106Z LL |     mac!(0);
2020-01-11T07:01:25.7025542Z    |     -------- in this macro invocation
2020-01-11T07:01:25.7025592Z    |
2020-01-11T07:01:25.7025592Z    |
2020-01-11T07:01:25.7025656Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-01-11T07:01:25.7025731Z error: `...` range patterns are deprecated
2020-01-11T07:01:25.7025991Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:42:13
2020-01-11T07:01:25.7026038Z    |
2020-01-11T07:01:25.7026038Z    |
2020-01-11T07:01:25.7026095Z LL |     if let 0...3 = 0 {} //~ ERROR `...` range patterns are deprecated
2020-01-11T07:01:25.7026204Z    |
2020-01-11T07:01:25.7026246Z note: lint level defined here
2020-01-11T07:01:25.7026493Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:8:9
2020-01-11T07:01:25.7026555Z    |
2020-01-11T07:01:25.7026555Z    |
2020-01-11T07:01:25.7026599Z LL | #![deny(ellipsis_inclusive_range_patterns)]
2020-01-11T07:01:25.7026646Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-11T07:01:25.7026689Z 
2020-01-11T07:01:25.7028610Z error: `...` range patterns are deprecated
2020-01-11T07:01:25.7029208Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:43:13
2020-01-11T07:01:25.7029266Z    |
2020-01-11T07:01:25.7029341Z LL |     if let 0...Y = 0 {} //~ ERROR `...` range patterns are deprecated
2020-01-11T07:01:25.7029423Z 
2020-01-11T07:01:25.7029496Z error: `...` range patterns are deprecated
2020-01-11T07:01:25.7029749Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:44:13
2020-01-11T07:01:25.7029795Z    |
2020-01-11T07:01:25.7029795Z    |
2020-01-11T07:01:25.7029859Z LL |     if let X...3 = 0 {} //~ ERROR `...` range patterns are deprecated
2020-01-11T07:01:25.7029952Z 
2020-01-11T07:01:25.7029998Z error: `...` range patterns are deprecated
2020-01-11T07:01:25.7030280Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:45:13
2020-01-11T07:01:25.7030331Z    |
2020-01-11T07:01:25.7030331Z    |
2020-01-11T07:01:25.7030381Z LL |     if let X...Y = 0 {} //~ ERROR `...` range patterns are deprecated
2020-01-11T07:01:25.7030484Z 
2020-01-11T07:01:25.7030529Z error: `...` range patterns are deprecated
2020-01-11T07:01:25.7030792Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:46:16
2020-01-11T07:01:25.7030865Z    |
2020-01-11T07:01:25.7030865Z    |
2020-01-11T07:01:25.7030915Z LL |     if let true...Y = 0 {} //~ ERROR only char and numeric types
2020-01-11T07:01:25.7031017Z 
2020-01-11T07:01:25.7031062Z error: `...` range patterns are deprecated
2020-01-11T07:01:25.7031331Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:48:13
2020-01-11T07:01:25.7031382Z    |
2020-01-11T07:01:25.7031382Z    |
2020-01-11T07:01:25.7031448Z LL |     if let X...true = 0 {} //~ ERROR only char and numeric types
2020-01-11T07:01:25.7031533Z 
2020-01-11T07:01:25.7031592Z error: `...` range patterns are deprecated
2020-01-11T07:01:25.7031856Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:50:14
2020-01-11T07:01:25.7031904Z    |
2020-01-11T07:01:25.7031904Z    |
2020-01-11T07:01:25.7031967Z LL |     if let .0...Y = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7032190Z 
2020-01-11T07:01:25.7032235Z error: `...` range patterns are deprecated
2020-01-11T07:01:25.7032814Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:53:13
2020-01-11T07:01:25.7032871Z    |
2020-01-11T07:01:25.7032871Z    |
2020-01-11T07:01:25.7032916Z LL |     if let X... .0 = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7033135Z 
2020-01-11T07:01:25.7033178Z error: `...` range patterns are deprecated
2020-01-11T07:01:25.7033508Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:126:20
2020-01-11T07:01:25.7033576Z    |
2020-01-11T07:01:25.7033576Z    |
2020-01-11T07:01:25.7033618Z LL |             let $e1...$e2;
2020-01-11T07:01:25.7033726Z ...
2020-01-11T07:01:25.7033726Z ...
2020-01-11T07:01:25.7033768Z LL |     mac2!(0, 1);
2020-01-11T07:01:25.7034025Z 
2020-01-11T07:01:25.7034087Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7034340Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:20:12
2020-01-11T07:01:25.7034386Z    |
2020-01-11T07:01:25.7034386Z    |
2020-01-11T07:01:25.7034448Z LL |     if let true..Y = 0 {} //~ ERROR only char and numeric types
2020-01-11T07:01:25.7034671Z    |            ^^^^  - this is of type `u8`
2020-01-11T07:01:25.7034791Z    |            this is of type `bool` but it should be `char` or numeric
2020-01-11T07:01:25.7034824Z 
2020-01-11T07:01:25.7034869Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7035111Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:21:15
2020-01-11T07:01:25.7035111Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:21:15
2020-01-11T07:01:25.7035175Z    |
2020-01-11T07:01:25.7035222Z LL |     if let X..true = 0 {} //~ ERROR only char and numeric types
2020-01-11T07:01:25.7035478Z    |            -  ^^^^ this is of type `bool` but it should be `char` or numeric
2020-01-11T07:01:25.7035608Z    |            this is of type `u8`
2020-01-11T07:01:25.7035637Z 
2020-01-11T07:01:25.7035694Z error[E0308]: mismatched types
2020-01-11T07:01:25.7035937Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:22:12
2020-01-11T07:01:25.7035937Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:22:12
2020-01-11T07:01:25.7035983Z    |
2020-01-11T07:01:25.7036037Z LL |     if let .0..Y = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7036277Z    |            ^^  - this is of type `u8`
2020-01-11T07:01:25.7036808Z    |            expected integer, found floating-point number
2020-01-11T07:01:25.7036872Z 
2020-01-11T07:01:25.7036915Z error[E0308]: mismatched types
2020-01-11T07:01:25.7038085Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:24:16
2020-01-11T07:01:25.7038085Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:24:16
2020-01-11T07:01:25.7038152Z    |
2020-01-11T07:01:25.7038219Z LL |     if let X.. .0 = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7038593Z    |            |   |
2020-01-11T07:01:25.7038870Z    |            |   expected integer, found floating-point number
2020-01-11T07:01:25.7038921Z    |            this is of type `u8`
2020-01-11T07:01:25.7038951Z 
2020-01-11T07:01:25.7038951Z 
2020-01-11T07:01:25.7038997Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7039264Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:33:12
2020-01-11T07:01:25.7039309Z    |
2020-01-11T07:01:25.7039355Z LL |     if let true..=Y = 0 {} //~ ERROR only char and numeric types
2020-01-11T07:01:25.7039644Z    |            |
2020-01-11T07:01:25.7039692Z    |            this is of type `bool` but it should be `char` or numeric
2020-01-11T07:01:25.7039740Z 
2020-01-11T07:01:25.7039785Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7039785Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7040177Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:34:16
2020-01-11T07:01:25.7040245Z    |
2020-01-11T07:01:25.7040292Z LL |     if let X..=true = 0 {} //~ ERROR only char and numeric types
2020-01-11T07:01:25.7040746Z    |            -   ^^^^ this is of type `bool` but it should be `char` or numeric
2020-01-11T07:01:25.7040863Z    |            this is of type `u8`
2020-01-11T07:01:25.7040893Z 
2020-01-11T07:01:25.7041018Z error[E0308]: mismatched types
2020-01-11T07:01:25.7041316Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:35:12
2020-01-11T07:01:25.7041316Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:35:12
2020-01-11T07:01:25.7041363Z    |
2020-01-11T07:01:25.7041408Z LL |     if let .0..=Y = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7041964Z    |            |
2020-01-11T07:01:25.7042202Z    |            expected integer, found floating-point number
2020-01-11T07:01:25.7042237Z 
2020-01-11T07:01:25.7042295Z error[E0308]: mismatched types
2020-01-11T07:01:25.7042295Z error[E0308]: mismatched types
2020-01-11T07:01:25.7042772Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:37:16
2020-01-11T07:01:25.7042828Z    |
2020-01-11T07:01:25.7042887Z LL |     if let X..=.0 = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7043245Z    |            |   |
2020-01-11T07:01:25.7043481Z    |            |   expected integer, found floating-point number
2020-01-11T07:01:25.7043555Z    |            this is of type `u8`
2020-01-11T07:01:25.7043584Z 
2020-01-11T07:01:25.7043584Z 
2020-01-11T07:01:25.7043631Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7043889Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:46:12
2020-01-11T07:01:25.7043936Z    |
2020-01-11T07:01:25.7043982Z LL |     if let true...Y = 0 {} //~ ERROR only char and numeric types
2020-01-11T07:01:25.7048149Z    |            |
2020-01-11T07:01:25.7068700Z    |            this is of type `bool` but it should be `char` or numeric
2020-01-11T07:01:25.7068736Z 
2020-01-11T07:01:25.7068785Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7068785Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7069303Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:48:16
2020-01-11T07:01:25.7069355Z    |
2020-01-11T07:01:25.7069404Z LL |     if let X...true = 0 {} //~ ERROR only char and numeric types
2020-01-11T07:01:25.7069711Z    |            -   ^^^^ this is of type `bool` but it should be `char` or numeric
2020-01-11T07:01:25.7069811Z    |            this is of type `u8`
2020-01-11T07:01:25.7069842Z 
2020-01-11T07:01:25.7069896Z error[E0308]: mismatched types
2020-01-11T07:01:25.7070254Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:50:12
2020-01-11T07:01:25.7070254Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:50:12
2020-01-11T07:01:25.7070303Z    |
2020-01-11T07:01:25.7070360Z LL |     if let .0...Y = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7070631Z    |            |
2020-01-11T07:01:25.7070882Z    |            expected integer, found floating-point number
2020-01-11T07:01:25.7070926Z 
2020-01-11T07:01:25.7070968Z error[E0308]: mismatched types
2020-01-11T07:01:25.7070968Z error[E0308]: mismatched types
2020-01-11T07:01:25.7071212Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:53:17
2020-01-11T07:01:25.7071270Z    |
2020-01-11T07:01:25.7071317Z LL |     if let X... .0 = 0 {} //~ ERROR mismatched types
2020-01-11T07:01:25.7071856Z    |            |    |
2020-01-11T07:01:25.7072121Z    |            |    expected integer, found floating-point number
2020-01-11T07:01:25.7072171Z    |            this is of type `u8`
2020-01-11T07:01:25.7072201Z 
2020-01-11T07:01:25.7072201Z 
2020-01-11T07:01:25.7072255Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7072498Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:61:12
2020-01-11T07:01:25.7072979Z    |
2020-01-11T07:01:25.7073038Z LL |     if let true.. = 0 {}
2020-01-11T07:01:25.7073089Z    |            ^^^^ this is of type `bool` but it should be `char` or numeric
2020-01-11T07:01:25.7073313Z error[E0308]: mismatched types
2020-01-11T07:01:25.7073670Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:63:12
2020-01-11T07:01:25.7073719Z    |
2020-01-11T07:01:25.7073719Z    |
2020-01-11T07:01:25.7073761Z LL |     if let .0.. = 0 {}
2020-01-11T07:01:25.7074269Z    |            ^^ expected integer, found floating-point number
2020-01-11T07:01:25.7074378Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7074692Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:71:12
2020-01-11T07:01:25.7074757Z    |
2020-01-11T07:01:25.7074757Z    |
2020-01-11T07:01:25.7075376Z LL |     if let true..= = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7075475Z    |            ^^^^ this is of type `bool` but it should be `char` or numeric
2020-01-11T07:01:25.7075567Z error[E0308]: mismatched types
2020-01-11T07:01:25.7075924Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:73:12
2020-01-11T07:01:25.7075997Z    |
2020-01-11T07:01:25.7075997Z    |
2020-01-11T07:01:25.7076044Z LL |     if let .0..= = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7076291Z    |            ^^ expected integer, found floating-point number
2020-01-11T07:01:25.7076377Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7076626Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:81:12
2020-01-11T07:01:25.7076672Z    |
2020-01-11T07:01:25.7076672Z    |
2020-01-11T07:01:25.7076725Z LL |     if let true... = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7076980Z    |            ^^^^ this is of type `bool` but it should be `char` or numeric
2020-01-11T07:01:25.7077055Z error[E0308]: mismatched types
2020-01-11T07:01:25.7077360Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:83:12
2020-01-11T07:01:25.7077408Z    |
2020-01-11T07:01:25.7077408Z    |
2020-01-11T07:01:25.7077454Z LL |     if let .0... = 0 {} //~ ERROR inclusive range with no end
2020-01-11T07:01:25.7077844Z    |            ^^ expected integer, found floating-point number
2020-01-11T07:01:25.7077939Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7078181Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:91:14
2020-01-11T07:01:25.7078240Z    |
2020-01-11T07:01:25.7078240Z    |
2020-01-11T07:01:25.7078291Z LL |     if let ..true = 0 {}
2020-01-11T07:01:25.7078340Z    |              ^^^^ this is of type `bool` but it should be `char` or numeric
2020-01-11T07:01:25.7078425Z error[E0308]: mismatched types
2020-01-11T07:01:25.7078668Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:93:15
2020-01-11T07:01:25.7078725Z    |
2020-01-11T07:01:25.7078725Z    |
2020-01-11T07:01:25.7078768Z LL |     if let .. .0 = 0 {}
2020-01-11T07:01:25.7079008Z    |               ^^ expected integer, found floating-point number
2020-01-11T07:01:25.7079094Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7079334Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:101:15
2020-01-11T07:01:25.7079389Z    |
2020-01-11T07:01:25.7079389Z    |
2020-01-11T07:01:25.7079444Z LL |     if let ..=true = 0 {}
2020-01-11T07:01:25.7079495Z    |               ^^^^ this is of type `bool` but it should be `char` or numeric
2020-01-11T07:01:25.7079569Z error[E0308]: mismatched types
2020-01-11T07:01:25.7079823Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:103:15
2020-01-11T07:01:25.7079870Z    |
2020-01-11T07:01:25.7079870Z    |
2020-01-11T07:01:25.7079911Z LL |     if let ..=.0 = 0 {}
2020-01-11T07:01:25.7080550Z    |               ^^ expected integer, found floating-point number
2020-01-11T07:01:25.7080635Z error[E0029]: only char and numeric types are allowed in range patterns
2020-01-11T07:01:25.7080879Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:113:15
2020-01-11T07:01:25.7080933Z    |
2020-01-11T07:01:25.7080975Z LL |     if let ...true = 0 {}
2020-01-11T07:01:25.7080975Z LL |     if let ...true = 0 {}
2020-01-11T07:01:25.7081025Z    |               ^^^^ this is of type `bool` but it should be `char` or numeric
2020-01-11T07:01:25.7081205Z 
2020-01-11T07:01:25.7081248Z error[E0308]: mismatched types
2020-01-11T07:01:25.7081525Z   --> /checkout/src/test/ui/parser/recover-range-pats.rs:116:15
2020-01-11T07:01:25.7081572Z    |
2020-01-11T07:01:25.7081620Z LL |     if let ....3 = 0 {}
2020-01-11T07:01:25.7082077Z    |               ^^ expected integer, found floating-point number
2020-01-11T07:01:25.7082179Z error: aborting due to 60 previous errors
2020-01-11T07:01:25.7082207Z 
2020-01-11T07:01:25.7082251Z Some errors have detailed explanations: E0029, E0308, E0586.
2020-01-11T07:01:25.7082777Z For more information about an error, try `rustc --explain E0029`.
---
2020-01-11T07:01:25.7084970Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:386:22
2020-01-11T07:01:25.7085036Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-11T07:01:25.7085068Z 
2020-01-11T07:01:25.7085094Z 
2020-01-11T07:01:25.7086808Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-11T07:01:25.7087061Z 
2020-01-11T07:01:25.7087089Z 
2020-01-11T07:01:25.7087149Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-11T07:01:25.7087205Z Build completed unsuccessfully in 0:59:07
2020-01-11T07:01:25.7087205Z Build completed unsuccessfully in 0:59:07
2020-01-11T07:01:25.7087252Z == clock drift check ==
2020-01-11T07:01:25.7087310Z   local time: Sat Jan 11 07:01:25 UTC 2020
2020-01-11T07:01:25.8671922Z   network time: Sat, 11 Jan 2020 07:01:25 GMT
2020-01-11T07:01:25.8672676Z == end clock drift check ==
2020-01-11T07:01:26.3000452Z 
2020-01-11T07:01:26.3074780Z ##[error]Bash exited with code '1'.
2020-01-11T07:01:26.3106747Z ##[section]Starting: Checkout
2020-01-11T07:01:26.3108390Z ==============================================================================
2020-01-11T07:01:26.3108468Z Task         : Get sources
2020-01-11T07:01:26.3108659Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
