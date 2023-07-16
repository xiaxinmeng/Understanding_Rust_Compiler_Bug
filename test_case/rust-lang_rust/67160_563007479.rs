plain
2019-12-08T22:03:29.4367528Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-08T22:03:29.4379965Z ##[command]git config gc.auto 0
2019-12-08T22:03:29.4383774Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-08T22:03:29.4386117Z ##[command]git config --get-all http.proxy
2019-12-08T22:03:29.4388028Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67160/merge:refs/remotes/pull/67160/merge
---
2019-12-08T23:00:54.6585762Z .................................................................................................... 1600/9344
2019-12-08T23:00:59.2445588Z .................................................................................................... 1700/9344
2019-12-08T23:01:11.0370990Z ...............................................i.................................................... 1800/9344
2019-12-08T23:01:18.9303241Z .................................................................................................... 1900/9344
2019-12-08T23:01:32.5153911Z ................................iiiii.......................................................F....... 2000/9344
2019-12-08T23:01:42.1149354Z .................................................................................................... 2200/9344
2019-12-08T23:01:44.6002758Z .................................................................................................... 2300/9344
2019-12-08T23:01:48.8225065Z .................................................................................................... 2400/9344
2019-12-08T23:02:09.5289997Z .................................................................................................... 2500/9344
---
2019-12-08T23:04:36.5435043Z .................................................................................................... 4700/9344
2019-12-08T23:04:41.4930767Z ........................................................i...............i........................... 4800/9344
2019-12-08T23:04:49.1700289Z .................................................................................................... 4900/9344
2019-12-08T23:04:56.9116410Z .................................................................................................... 5000/9344
2019-12-08T23:05:01.9880681Z i................................................................................................... 5100/9344
2019-12-08T23:05:12.0618560Z .................................................................ii.ii...........i.................. 5200/9344
2019-12-08T23:05:20.7317954Z .i.................................................................................................. 5400/9344
2019-12-08T23:05:30.6916387Z .................................................................F.........F........................ 5500/9344
2019-12-08T23:05:36.7366251Z ...............................................i.................................................... 5600/9344
2019-12-08T23:05:43.6433504Z .................................................................................................... 5700/9344
2019-12-08T23:05:43.6433504Z .................................................................................................... 5700/9344
2019-12-08T23:05:54.2847511Z .................................................................................................... 5800/9344
2019-12-08T23:06:03.8278365Z ..............F...................ii...i..ii...........i............................................ 5900/9344
2019-12-08T23:06:21.7760451Z .................................................................................................... 6100/9344
2019-12-08T23:06:29.7390240Z .................................................................................................... 6200/9344
2019-12-08T23:06:29.7390240Z .................................................................................................... 6200/9344
2019-12-08T23:06:40.3051275Z .........................................................i..ii...................................... 6300/9344
2019-12-08T23:07:06.4752694Z .................................................................................................... 6500/9344
2019-12-08T23:07:08.4306303Z .............................i..............................F....................................... 6600/9344
2019-12-08T23:07:10.6072596Z .....................................................................................F.............. 6700/9344
2019-12-08T23:07:12.8646685Z ....................i..............F...........................................F.................... 6800/9344
---
2019-12-08T23:08:46.9472656Z .................................................................................................F.. 7400/9344
2019-12-08T23:08:51.4781821Z ..............................................................................F..................... 7500/9344
2019-12-08T23:08:58.1791015Z ....................................................................................F............... 7600/9344
2019-12-08T23:09:10.0749498Z .................................................................................................... 7700/9344
2019-12-08T23:09:15.4754637Z ......................iiii.......................................................................... 7800/9344
2019-12-08T23:09:29.2328566Z ...............F......................F............................................................. 8000/9344
2019-12-08T23:09:39.2463686Z .................................................................................................... 8100/9344
2019-12-08T23:09:51.7823797Z .................................................................................................... 8200/9344
2019-12-08T23:09:58.6780698Z ................................F.........F..............................F..............F........... 8300/9344
---
2019-12-08T23:11:50.5425832Z + LL | fn foo() -> &'static str {
2019-12-08T23:11:50.5427024Z +    |          ^^^^^^^^^^^^^^^
2019-12-08T23:11:50.5427288Z + help: consider dereferencing the borrow
2019-12-08T23:11:50.5428525Z +    |
2019-12-08T23:11:50.5429665Z + LL |     *"bar            boo"
2019-12-08T23:11:50.5431122Z 14 
2019-12-08T23:11:50.5432693Z 15 error: aborting due to 2 previous errors
2019-12-08T23:11:50.5433025Z 16 
2019-12-08T23:11:50.5433382Z 
2019-12-08T23:11:50.5433382Z 
2019-12-08T23:11:50.5434468Z 
2019-12-08T23:11:50.5434784Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5437203Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/tab/tab.stderr
2019-12-08T23:11:50.5439468Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5441028Z To only update this specific test, also pass `--test-args codemap_tests/tab.rs`
2019-12-08T23:11:50.5443155Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5445387Z status: exit code: 1
2019-12-08T23:11:50.5445387Z status: exit code: 1
2019-12-08T23:11:50.5447425Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/tab.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/tab" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/tab/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5451450Z ------------------------------------------
2019-12-08T23:11:50.5453301Z 
2019-12-08T23:11:50.5454791Z ------------------------------------------
2019-12-08T23:11:50.5456890Z stderr:
---
2019-12-08T23:11:50.5528663Z 
2019-12-08T23:11:50.5528812Z error[E0308]: mismatched types
2019-12-08T23:11:50.5529447Z   --> /checkout/src/test/ui/codemap_tests/tab.rs:8:2
2019-12-08T23:11:50.5529671Z    |
2019-12-08T23:11:50.5529849Z LL |     "bar            boo" //~ ERROR mismatched types
2019-12-08T23:11:50.5530005Z    |     ^^^^^^^^^^^^^^^^^^^^ expected `()`, found `&str`
2019-12-08T23:11:50.5530314Z help: try adding a return type
2019-12-08T23:11:50.5530463Z    |
2019-12-08T23:11:50.5530831Z LL | fn foo() -> &'static str {
2019-12-08T23:11:50.5531050Z    |          ^^^^^^^^^^^^^^^
2019-12-08T23:11:50.5531050Z    |          ^^^^^^^^^^^^^^^
2019-12-08T23:11:50.5531204Z help: consider dereferencing the borrow
2019-12-08T23:11:50.5531348Z    |
2019-12-08T23:11:50.5531525Z LL |     *"bar            boo" //~ ERROR mismatched types
2019-12-08T23:11:50.5532702Z 
2019-12-08T23:11:50.5532932Z error: aborting due to 2 previous errors
2019-12-08T23:11:50.5533108Z 
2019-12-08T23:11:50.5533327Z Some errors have detailed explanations: E0308, E0425.
---
2019-12-08T23:11:50.5536092Z diff of stderr:
2019-12-08T23:11:50.5536242Z 
2019-12-08T23:11:50.5536597Z 61   --> $DIR/dst-bad-coercions.rs:24:25
2019-12-08T23:11:50.5536789Z 62    |
2019-12-08T23:11:50.5536955Z 63 LL |     let x: *mut dyn T = &S;
2019-12-08T23:11:50.5537327Z -    |            ----------   ^^ types differ in mutability
2019-12-08T23:11:50.5538095Z +    |            ----------   ^^
2019-12-08T23:11:50.5538292Z +    |            |            |
2019-12-08T23:11:50.5538453Z +    |            |            types differ in mutability
2019-12-08T23:11:50.5538453Z +    |            |            types differ in mutability
2019-12-08T23:11:50.5538642Z +    |            |            help: consider dereferencing the borrow: `*&S`
2019-12-08T23:11:50.5538952Z 67    |
2019-12-08T23:11:50.5539122Z 68    = note: expected raw pointer `*mut dyn T`
2019-12-08T23:11:50.5539252Z 
2019-12-08T23:11:50.5539615Z 72   --> $DIR/dst-bad-coercions.rs:25:21
2019-12-08T23:11:50.5539615Z 72   --> $DIR/dst-bad-coercions.rs:25:21
2019-12-08T23:11:50.5540764Z 73    |
2019-12-08T23:11:50.5540992Z 74 LL |     let x: *mut S = &S;
2019-12-08T23:11:50.5541445Z -    |            ------   ^^ types differ in mutability
2019-12-08T23:11:50.5543302Z +    |            ------   ^^
2019-12-08T23:11:50.5543562Z +    |            |        |
2019-12-08T23:11:50.5543759Z +    |            |        types differ in mutability
2019-12-08T23:11:50.5543759Z +    |            |        types differ in mutability
2019-12-08T23:11:50.5543952Z +    |            |        help: consider dereferencing the borrow: `*&S`
2019-12-08T23:11:50.5544624Z 78    |
2019-12-08T23:11:50.5544813Z 79    = note: expected raw pointer `*mut S`
2019-12-08T23:11:50.5545145Z 
2019-12-08T23:11:50.5545318Z 
2019-12-08T23:11:50.5545318Z 
2019-12-08T23:11:50.5545843Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5546699Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-bad-coercions/dst-bad-coercions.stderr
2019-12-08T23:11:50.5655252Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5656126Z To only update this specific test, also pass `--test-args dst/dst-bad-coercions.rs`
2019-12-08T23:11:50.5656244Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5656283Z status: exit code: 1
2019-12-08T23:11:50.5656283Z status: exit code: 1
2019-12-08T23:11:50.5658526Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dst/dst-bad-coercions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-bad-coercions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-bad-coercions/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5659820Z ------------------------------------------
2019-12-08T23:11:50.5659928Z 
2019-12-08T23:11:50.5660159Z ------------------------------------------
2019-12-08T23:11:50.5660364Z stderr:
2019-12-08T23:11:50.5660364Z stderr:
2019-12-08T23:11:50.5660693Z ------------------------------------------
2019-12-08T23:11:50.5660750Z error[E0308]: mismatched types
2019-12-08T23:11:50.5660944Z   --> /checkout/src/test/ui/dst/dst-bad-coercions.rs:14:17
2019-12-08T23:11:50.5660982Z    |
2019-12-08T23:11:50.5661034Z LL |     let y: &S = x; //~ ERROR mismatched types
2019-12-08T23:11:50.5661230Z    |            --   ^ expected `&S`, found *-ptr
2019-12-08T23:11:50.5661322Z    |            expected due to this
2019-12-08T23:11:50.5661365Z    |
2019-12-08T23:11:50.5661399Z    = note: expected reference `&S`
2019-12-08T23:11:50.5661399Z    = note: expected reference `&S`
2019-12-08T23:11:50.5661435Z             found raw pointer `*const S`
2019-12-08T23:11:50.5661510Z error[E0308]: mismatched types
2019-12-08T23:11:50.5661899Z   --> /checkout/src/test/ui/dst/dst-bad-coercions.rs:15:21
2019-12-08T23:11:50.5661938Z    |
2019-12-08T23:11:50.5661938Z    |
2019-12-08T23:11:50.5662804Z LL |     let y: &dyn T = x; //~ ERROR mismatched types
2019-12-08T23:11:50.5663041Z    |            ------   ^
2019-12-08T23:11:50.5663089Z    |            |        |
2019-12-08T23:11:50.5663329Z    |            |        expected `&dyn T`, found *-ptr
2019-12-08T23:11:50.5663384Z    |            |        help: consider borrowing here: `&x`
2019-12-08T23:11:50.5663496Z    |
2019-12-08T23:11:50.5663496Z    |
2019-12-08T23:11:50.5663550Z    = note: expected reference `&dyn T`
2019-12-08T23:11:50.5663597Z             found raw pointer `*const S`
2019-12-08T23:11:50.5663697Z error[E0308]: mismatched types
2019-12-08T23:11:50.5663933Z   --> /checkout/src/test/ui/dst/dst-bad-coercions.rs:19:17
2019-12-08T23:11:50.5663980Z    |
2019-12-08T23:11:50.5663980Z    |
2019-12-08T23:11:50.5664048Z LL |     let y: &S = x; //~ ERROR mismatched types
2019-12-08T23:11:50.5664271Z    |            --   ^ expected `&S`, found *-ptr
2019-12-08T23:11:50.5664385Z    |            expected due to this
2019-12-08T23:11:50.5664428Z    |
2019-12-08T23:11:50.5664472Z    = note: expected reference `&S`
2019-12-08T23:11:50.5664517Z             found raw pointer `*mut S`
2019-12-08T23:11:50.5664517Z             found raw pointer `*mut S`
2019-12-08T23:11:50.5664568Z 
2019-12-08T23:11:50.5664609Z error[E0308]: mismatched types
2019-12-08T23:11:50.5664843Z   --> /checkout/src/test/ui/dst/dst-bad-coercions.rs:20:21
2019-12-08T23:11:50.5664909Z    |
2019-12-08T23:11:50.5665127Z LL |     let y: &dyn T = x; //~ ERROR mismatched types
2019-12-08T23:11:50.5665377Z    |            ------   ^
2019-12-08T23:11:50.5665424Z    |            |        |
2019-12-08T23:11:50.5666119Z    |            |        expected `&dyn T`, found *-ptr
2019-12-08T23:11:50.5666327Z    |            |        help: consider borrowing here: `&x`
2019-12-08T23:11:50.5666450Z    |
2019-12-08T23:11:50.5666450Z    |
2019-12-08T23:11:50.5666484Z    = note: expected reference `&dyn T`
2019-12-08T23:11:50.5666521Z             found raw pointer `*mut S`
2019-12-08T23:11:50.5666589Z error[E0308]: mismatched types
2019-12-08T23:11:50.5666779Z   --> /checkout/src/test/ui/dst/dst-bad-coercions.rs:23:25
2019-12-08T23:11:50.5666815Z    |
2019-12-08T23:11:50.5666815Z    |
2019-12-08T23:11:50.5666873Z LL |     let x: &mut dyn T = &S; //~ ERROR mismatched types
2019-12-08T23:11:50.5667055Z    |            ----------   ^^ types differ in mutability
2019-12-08T23:11:50.5667155Z    |            expected due to this
2019-12-08T23:11:50.5667188Z    |
2019-12-08T23:11:50.5667222Z    = note: expected mutable reference `&mut dyn T`
2019-12-08T23:11:50.5667280Z                       found reference `&S`
2019-12-08T23:11:50.5667280Z                       found reference `&S`
2019-12-08T23:11:50.5667304Z 
2019-12-08T23:11:50.5667337Z error[E0308]: mismatched types
2019-12-08T23:11:50.5667524Z   --> /checkout/src/test/ui/dst/dst-bad-coercions.rs:24:25
2019-12-08T23:11:50.5667573Z    |
2019-12-08T23:11:50.5667610Z LL |     let x: *mut dyn T = &S; //~ ERROR mismatched types
2019-12-08T23:11:50.5667772Z    |            ----------   ^^
2019-12-08T23:11:50.5667868Z    |            |            types differ in mutability
2019-12-08T23:11:50.5667868Z    |            |            types differ in mutability
2019-12-08T23:11:50.5667910Z    |            |            help: consider dereferencing the borrow: `*&S`
2019-12-08T23:11:50.5668002Z    |
2019-12-08T23:11:50.5668037Z    = note: expected raw pointer `*mut dyn T`
2019-12-08T23:11:50.5668096Z                 found reference `&S`
2019-12-08T23:11:50.5668120Z 
2019-12-08T23:11:50.5668120Z 
2019-12-08T23:11:50.5668152Z error[E0308]: mismatched types
2019-12-08T23:11:50.5668346Z   --> /checkout/src/test/ui/dst/dst-bad-coercions.rs:25:21
2019-12-08T23:11:50.5668397Z    |
2019-12-08T23:11:50.5668433Z LL |     let x: *mut S = &S; //~ ERROR mismatched types
2019-12-08T23:11:50.5668591Z    |            ------   ^^
2019-12-08T23:11:50.5668685Z    |            |        types differ in mutability
2019-12-08T23:11:50.5668685Z    |            |        types differ in mutability
2019-12-08T23:11:50.5668725Z    |            |        help: consider dereferencing the borrow: `*&S`
2019-12-08T23:11:50.5668811Z    |
2019-12-08T23:11:50.5668844Z    = note: expected raw pointer `*mut S`
2019-12-08T23:11:50.5668879Z                 found reference `&S`
2019-12-08T23:11:50.5668918Z 
---
2019-12-08T23:11:50.5669433Z 
2019-12-08T23:11:50.5669641Z ---- [ui] ui/feature-gates/feature-gate-generic_associated_types.rs stdout ----
2019-12-08T23:11:50.5669681Z diff of stderr:
2019-12-08T23:11:50.5669702Z 
2019-12-08T23:11:50.5670020Z 61    = note: for more information, see ***/issues/44265
2019-12-08T23:11:50.5670083Z 62    = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable
2019-12-08T23:11:50.5670299Z - error: aborting due to 7 previous errors
2019-12-08T23:11:50.5670505Z + error: type-generic associated types are not yet implemented
2019-12-08T23:11:50.5670690Z +   --> $DIR/feature-gate-generic_associated_types.rs:4:5
2019-12-08T23:11:50.5670726Z +    |
2019-12-08T23:11:50.5670726Z +    |
2019-12-08T23:11:50.5670776Z + LL |     type Pointer<T>: Deref<Target = T>;
2019-12-08T23:11:50.5670934Z +    |
2019-12-08T23:11:50.5670934Z +    |
2019-12-08T23:11:50.5671202Z +    = note: for more information, see ***/issues/44265
2019-12-08T23:11:50.5672679Z + error: type-generic associated types are not yet implemented
2019-12-08T23:11:50.5672931Z +   --> $DIR/feature-gate-generic_associated_types.rs:6:5
2019-12-08T23:11:50.5672976Z +    |
2019-12-08T23:11:50.5672976Z +    |
2019-12-08T23:11:50.5673023Z + LL |     type Pointer2<T>: Deref<Target = T> where T: Clone, U: Clone;
2019-12-08T23:11:50.5673131Z +    |
2019-12-08T23:11:50.5673131Z +    |
2019-12-08T23:11:50.5673423Z +    = note: for more information, see ***/issues/44265
2019-12-08T23:11:50.5673530Z + error: aborting due to 9 previous errors
2019-12-08T23:11:50.5673573Z 65 
2019-12-08T23:11:50.5673841Z 66 For more information about this error, try `rustc --explain E0658`.
2019-12-08T23:11:50.5673887Z 67 
2019-12-08T23:11:50.5673887Z 67 
2019-12-08T23:11:50.5673925Z 
2019-12-08T23:11:50.5673951Z 
2019-12-08T23:11:50.5674011Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5674371Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-generic_associated_types/feature-gate-generic_associated_types.stderr
2019-12-08T23:11:50.5674621Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5674929Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-generic_associated_types.rs`
2019-12-08T23:11:50.5675016Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5675076Z status: exit code: 1
2019-12-08T23:11:50.5675076Z status: exit code: 1
2019-12-08T23:11:50.5676002Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-generic_associated_types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-generic_associated_types" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-generic_associated_types/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5676277Z ------------------------------------------
2019-12-08T23:11:50.5676304Z 
2019-12-08T23:11:50.5676492Z ------------------------------------------
2019-12-08T23:11:50.5676528Z stderr:
2019-12-08T23:11:50.5676528Z stderr:
2019-12-08T23:11:50.5676698Z ------------------------------------------
2019-12-08T23:11:50.5676753Z error[E0658]: generic associated types are unstable
2019-12-08T23:11:50.5676967Z   --> /checkout/src/test/ui/feature-gates/feature-gate-generic_associated_types.rs:4:5
2019-12-08T23:11:50.5677008Z    |
2019-12-08T23:11:50.5677062Z LL |     type Pointer<T>: Deref<Target = T>;
2019-12-08T23:11:50.5677141Z    |
2019-12-08T23:11:50.5677141Z    |
2019-12-08T23:11:50.5677390Z    = note: for more information, see ***/issues/44265
2019-12-08T23:11:50.5677448Z    = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable
2019-12-08T23:11:50.5677518Z error[E0658]: generic associated types are unstable
2019-12-08T23:11:50.5677779Z   --> /checkout/src/test/ui/feature-gates/feature-gate-generic_associated_types.rs:6:5
2019-12-08T23:11:50.5677820Z    |
2019-12-08T23:11:50.5677820Z    |
2019-12-08T23:11:50.5677861Z LL |     type Pointer2<T>: Deref<Target = T> where T: Clone, U: Clone;
2019-12-08T23:11:50.5677957Z    |
2019-12-08T23:11:50.5677957Z    |
2019-12-08T23:11:50.5678214Z    = note: for more information, see ***/issues/44265
2019-12-08T23:11:50.5678263Z    = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable
2019-12-08T23:11:50.5678426Z error[E0658]: where clauses on associated types are unstable
2019-12-08T23:11:50.5678711Z   --> /checkout/src/test/ui/feature-gates/feature-gate-generic_associated_types.rs:6:5
2019-12-08T23:11:50.5678819Z    |
2019-12-08T23:11:50.5678819Z    |
2019-12-08T23:11:50.5678859Z LL |     type Pointer2<T>: Deref<Target = T> where T: Clone, U: Clone;
2019-12-08T23:11:50.5678949Z    |
2019-12-08T23:11:50.5678949Z    |
2019-12-08T23:11:50.5679218Z    = note: for more information, see ***/issues/44265
2019-12-08T23:11:50.5679280Z    = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable
2019-12-08T23:11:50.5679346Z error[E0658]: generic associated types are unstable
2019-12-08T23:11:50.5679597Z   --> /checkout/src/test/ui/feature-gates/feature-gate-generic_associated_types.rs:14:5
2019-12-08T23:11:50.5679638Z    |
2019-12-08T23:11:50.5679638Z    |
2019-12-08T23:11:50.5679675Z LL |     type Pointer<Usize> = Box<Usize>;
2019-12-08T23:11:50.5679769Z    |
2019-12-08T23:11:50.5679769Z    |
2019-12-08T23:11:50.5680009Z    = note: for more information, see ***/issues/44265
2019-12-08T23:11:50.5680077Z    = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable
2019-12-08T23:11:50.5680145Z error[E0658]: generic associated types are unstable
2019-12-08T23:11:50.5680374Z   --> /checkout/src/test/ui/feature-gates/feature-gate-generic_associated_types.rs:16:5
2019-12-08T23:11:50.5680428Z    |
2019-12-08T23:11:50.5680428Z    |
2019-12-08T23:11:50.5680465Z LL |     type Pointer2<U32> = Box<U32>;
2019-12-08T23:11:50.5680547Z    |
2019-12-08T23:11:50.5680547Z    |
2019-12-08T23:11:50.5680954Z    = note: for more information, see ***/issues/44265
2019-12-08T23:11:50.5681005Z    = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable
2019-12-08T23:11:50.5681090Z error[E0658]: where clauses on associated types are unstable
2019-12-08T23:11:50.5681333Z   --> /checkout/src/test/ui/feature-gates/feature-gate-generic_associated_types.rs:21:5
2019-12-08T23:11:50.5681391Z    |
2019-12-08T23:11:50.5681428Z LL |     type Assoc where Self: Sized;
2019-12-08T23:11:50.5681428Z LL |     type Assoc where Self: Sized;
2019-12-08T23:11:50.5681466Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-08T23:11:50.5681500Z    |
2019-12-08T23:11:50.5682481Z    = note: for more information, see ***/issues/44265
2019-12-08T23:11:50.5682546Z    = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable
2019-12-08T23:11:50.5682644Z error[E0658]: where clauses on associated types are unstable
2019-12-08T23:11:50.5682935Z   --> /checkout/src/test/ui/feature-gates/feature-gate-generic_associated_types.rs:26:5
2019-12-08T23:11:50.5682985Z    |
2019-12-08T23:11:50.5683047Z LL |     type Assoc where Self: Sized = Foo;
2019-12-08T23:11:50.5683047Z LL |     type Assoc where Self: Sized = Foo;
2019-12-08T23:11:50.5683097Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-08T23:11:50.5683140Z    |
2019-12-08T23:11:50.5683460Z    = note: for more information, see ***/issues/44265
2019-12-08T23:11:50.5683519Z    = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable
2019-12-08T23:11:50.5683830Z error: type-generic associated types are not yet implemented
2019-12-08T23:11:50.5684112Z   --> /checkout/src/test/ui/feature-gates/feature-gate-generic_associated_types.rs:4:5
2019-12-08T23:11:50.5684162Z    |
2019-12-08T23:11:50.5684162Z    |
2019-12-08T23:11:50.5684627Z LL |     type Pointer<T>: Deref<Target = T>;
2019-12-08T23:11:50.5684726Z    |
2019-12-08T23:11:50.5684726Z    |
2019-12-08T23:11:50.5685125Z    = note: for more information, see ***/issues/44265
2019-12-08T23:11:50.5685415Z error: type-generic associated types are not yet implemented
2019-12-08T23:11:50.5685934Z   --> /checkout/src/test/ui/feature-gates/feature-gate-generic_associated_types.rs:6:5
2019-12-08T23:11:50.5685989Z    |
2019-12-08T23:11:50.5685989Z    |
2019-12-08T23:11:50.5686141Z LL |     type Pointer2<T>: Deref<Target = T> where T: Clone, U: Clone;
2019-12-08T23:11:50.5686286Z    |
2019-12-08T23:11:50.5686286Z    |
2019-12-08T23:11:50.5686546Z    = note: for more information, see ***/issues/44265
2019-12-08T23:11:50.5686625Z error: aborting due to 9 previous errors
2019-12-08T23:11:50.5686648Z 
2019-12-08T23:11:50.5686847Z For more information about this error, try `rustc --explain E0658`.
2019-12-08T23:11:50.5686874Z 
---
2019-12-08T23:11:50.5687324Z diff of stderr:
2019-12-08T23:11:50.5687347Z 
2019-12-08T23:11:50.5687516Z 2   --> $DIR/fn-trait-formatting.rs:6:17
2019-12-08T23:11:50.5687554Z 3    |
2019-12-08T23:11:50.5687781Z 4 LL |     let _: () = (box |_: isize| {}) as Box<dyn FnOnce(isize)>;
2019-12-08T23:11:50.5688026Z -    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5688411Z +    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-08T23:11:50.5688451Z +    |            |    |
2019-12-08T23:11:50.5688489Z +    |            |    expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5688489Z +    |            |    expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5688550Z +    |            |    help: consider dereferencing the type: `*(box |_: isize| {}) as Box<dyn FnOnce(isize)>`
2019-12-08T23:11:50.5688627Z 8    |
2019-12-08T23:11:50.5688858Z 9    = note: expected unit type `()`
2019-12-08T23:11:50.5688882Z 
2019-12-08T23:11:50.5689063Z 13   --> $DIR/fn-trait-formatting.rs:10:17
2019-12-08T23:11:50.5689063Z 13   --> $DIR/fn-trait-formatting.rs:10:17
2019-12-08T23:11:50.5689101Z 14    |
2019-12-08T23:11:50.5689156Z 15 LL |     let _: () = (box |_: isize, isize| {}) as Box<dyn Fn(isize, isize)>;
2019-12-08T23:11:50.5689416Z -    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5689945Z +    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-08T23:11:50.5690000Z +    |            |    |
2019-12-08T23:11:50.5690041Z +    |            |    expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5690041Z +    |            |    expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5690103Z +    |            |    help: consider dereferencing the type: `*(box |_: isize, isize| {}) as Box<dyn Fn(isize, isize)>`
2019-12-08T23:11:50.5690181Z 19    |
2019-12-08T23:11:50.5690227Z 20    = note: expected unit type `()`
2019-12-08T23:11:50.5690251Z 
2019-12-08T23:11:50.5690470Z 24   --> $DIR/fn-trait-formatting.rs:14:17
2019-12-08T23:11:50.5690470Z 24   --> $DIR/fn-trait-formatting.rs:14:17
2019-12-08T23:11:50.5690507Z 25    |
2019-12-08T23:11:50.5690738Z 26 LL |     let _: () = (box || -> isize { unimplemented!() }) as Box<dyn FnMut() -> isize>;
2019-12-08T23:11:50.5691008Z -    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5691567Z +    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-08T23:11:50.5691608Z +    |            |    |
2019-12-08T23:11:50.5691647Z +    |            |    expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5691647Z +    |            |    expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5692646Z +    |            |    help: consider dereferencing the type: `*(box || -> isize { unimplemented!() }) as Box<dyn FnMut() -> isize>`
2019-12-08T23:11:50.5692747Z 30    |
2019-12-08T23:11:50.5692807Z 31    = note: expected unit type `()`
2019-12-08T23:11:50.5692836Z 
2019-12-08T23:11:50.5692862Z 
2019-12-08T23:11:50.5692862Z 
2019-12-08T23:11:50.5692906Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5693360Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-trait-formatting/fn-trait-formatting.stderr
2019-12-08T23:11:50.5693642Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5694010Z To only update this specific test, also pass `--test-args fn/fn-trait-formatting.rs`
2019-12-08T23:11:50.5694108Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5694153Z status: exit code: 1
2019-12-08T23:11:50.5694153Z status: exit code: 1
2019-12-08T23:11:50.5694955Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/fn-trait-formatting.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-trait-formatting" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-trait-formatting/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5695289Z ------------------------------------------
2019-12-08T23:11:50.5695332Z 
2019-12-08T23:11:50.5695568Z ------------------------------------------
2019-12-08T23:11:50.5695615Z stderr:
2019-12-08T23:11:50.5695615Z stderr:
2019-12-08T23:11:50.5696020Z ------------------------------------------
2019-12-08T23:11:50.5696060Z error[E0308]: mismatched types
2019-12-08T23:11:50.5696431Z   --> /checkout/src/test/ui/fn/fn-trait-formatting.rs:6:17
2019-12-08T23:11:50.5696766Z    |
2019-12-08T23:11:50.5696806Z LL |     let _: () = (box |_: isize| {}) as Box<dyn FnOnce(isize)>;
2019-12-08T23:11:50.5697005Z    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-08T23:11:50.5697094Z    |            |    expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5697094Z    |            |    expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5697140Z    |            |    help: consider dereferencing the type: `*(box |_: isize| {}) as Box<dyn FnOnce(isize)>`
2019-12-08T23:11:50.5697235Z    |
2019-12-08T23:11:50.5697270Z    = note: expected unit type `()`
2019-12-08T23:11:50.5697324Z                  found struct `std::boxed::Box<dyn std::ops::FnOnce(isize)>`
2019-12-08T23:11:50.5697351Z 
2019-12-08T23:11:50.5697351Z 
2019-12-08T23:11:50.5697385Z error[E0308]: mismatched types
2019-12-08T23:11:50.5697583Z   --> /checkout/src/test/ui/fn/fn-trait-formatting.rs:10:17
2019-12-08T23:11:50.5697757Z    |
2019-12-08T23:11:50.5697796Z LL |     let _: () = (box |_: isize, isize| {}) as Box<dyn Fn(isize, isize)>;
2019-12-08T23:11:50.5698001Z    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-08T23:11:50.5698094Z    |            |    expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5698094Z    |            |    expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5698140Z    |            |    help: consider dereferencing the type: `*(box |_: isize, isize| {}) as Box<dyn Fn(isize, isize)>`
2019-12-08T23:11:50.5698234Z    |
2019-12-08T23:11:50.5698268Z    = note: expected unit type `()`
2019-12-08T23:11:50.5698268Z    = note: expected unit type `()`
2019-12-08T23:11:50.5698323Z                  found struct `std::boxed::Box<dyn std::ops::Fn(isize, isize)>`
2019-12-08T23:11:50.5698382Z error[E0308]: mismatched types
2019-12-08T23:11:50.5698576Z   --> /checkout/src/test/ui/fn/fn-trait-formatting.rs:14:17
2019-12-08T23:11:50.5698624Z    |
2019-12-08T23:11:50.5698624Z    |
2019-12-08T23:11:50.5698836Z LL |     let _: () = (box || -> isize { unimplemented!() }) as Box<dyn FnMut() -> isize>;
2019-12-08T23:11:50.5699047Z    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-08T23:11:50.5699320Z    |            |    expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5699320Z    |            |    expected `()`, found struct `std::boxed::Box`
2019-12-08T23:11:50.5699576Z    |            |    help: consider dereferencing the type: `*(box || -> isize { unimplemented!() }) as Box<dyn FnMut() -> isize>`
2019-12-08T23:11:50.5699762Z    |
2019-12-08T23:11:50.5699798Z    = note: expected unit type `()`
2019-12-08T23:11:50.5700101Z                  found struct `std::boxed::Box<dyn std::ops::FnMut() -> isize>`
2019-12-08T23:11:50.5700131Z 
2019-12-08T23:11:50.5700131Z 
2019-12-08T23:11:50.5700170Z error[E0277]: expected a `std::ops::Fn<(isize,)>` closure, found `{integer}`
2019-12-08T23:11:50.5700580Z    |
2019-12-08T23:11:50.5700580Z    |
2019-12-08T23:11:50.5700764Z LL | fn needs_fn<F>(x: F) where F: Fn(isize) -> isize {}
2019-12-08T23:11:50.5700978Z    |    --------                   ------------------ required by this bound in `needs_fn`
2019-12-08T23:11:50.5701062Z LL |     needs_fn(1);
2019-12-08T23:11:50.5701062Z LL |     needs_fn(1);
2019-12-08T23:11:50.5701103Z    |              ^ expected an `Fn<(isize,)>` closure, found `{integer}`
2019-12-08T23:11:50.5701148Z    |
2019-12-08T23:11:50.5701187Z    = help: the trait `std::ops::Fn<(isize,)>` is not implemented for `{integer}`
2019-12-08T23:11:50.5701257Z error: aborting due to 4 previous errors
2019-12-08T23:11:50.5701294Z 
2019-12-08T23:11:50.5701336Z Some errors have detailed explanations: E0277, E0308.
2019-12-08T23:11:50.5701538Z For more information about an error, try `rustc --explain E0277`.
---
2019-12-08T23:11:50.5703166Z diff of stderr:
2019-12-08T23:11:50.5703194Z 
2019-12-08T23:11:50.5703649Z 2   --> $DIR/issue-41742.rs:24:7
2019-12-08T23:11:50.5703700Z 3    |
2019-12-08T23:11:50.5703754Z 4 LL |     H["?"].f();
2019-12-08T23:11:50.5703974Z -    |       ^^^ expected `u32`, found `&str`
2019-12-08T23:11:50.5704072Z +    |       |
2019-12-08T23:11:50.5704116Z +    |       expected `u32`, found `&str`
2019-12-08T23:11:50.5704116Z +    |       expected `u32`, found `&str`
2019-12-08T23:11:50.5704177Z +    |       help: consider dereferencing the borrow: `*"?"`
2019-12-08T23:11:50.5704283Z 7 error: aborting due to previous error
2019-12-08T23:11:50.5704324Z 8 
2019-12-08T23:11:50.5704351Z 
2019-12-08T23:11:50.5704376Z 
2019-12-08T23:11:50.5704376Z 
2019-12-08T23:11:50.5704435Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5704736Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41742/issue-41742.stderr
2019-12-08T23:11:50.5704978Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5705260Z To only update this specific test, also pass `--test-args issues/issue-41742.rs`
2019-12-08T23:11:50.5705338Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5705394Z status: exit code: 1
2019-12-08T23:11:50.5705394Z status: exit code: 1
2019-12-08T23:11:50.5706150Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-41742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41742" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41742/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5706606Z ------------------------------------------
2019-12-08T23:11:50.5706633Z 
2019-12-08T23:11:50.5706983Z ------------------------------------------
2019-12-08T23:11:50.5707020Z stderr:
2019-12-08T23:11:50.5707020Z stderr:
2019-12-08T23:11:50.5707192Z ------------------------------------------
2019-12-08T23:11:50.5707391Z error[E0308]: mismatched types
2019-12-08T23:11:50.5707761Z   --> /checkout/src/test/ui/issues/issue-41742.rs:24:7
2019-12-08T23:11:50.5707800Z    |
2019-12-08T23:11:50.5707836Z LL |     H["?"].f(); //~ ERROR mismatched types
2019-12-08T23:11:50.5708048Z    |       |
2019-12-08T23:11:50.5708095Z    |       expected `u32`, found `&str`
2019-12-08T23:11:50.5708095Z    |       expected `u32`, found `&str`
2019-12-08T23:11:50.5708528Z    |       help: consider dereferencing the borrow: `*"?"`
2019-12-08T23:11:50.5708596Z error: aborting due to previous error
2019-12-08T23:11:50.5708620Z 
2019-12-08T23:11:50.5708901Z For more information about this error, try `rustc --explain E0308`.
2019-12-08T23:11:50.5708932Z 
---
2019-12-08T23:11:50.5709547Z diff of stderr:
2019-12-08T23:11:50.5709570Z 
2019-12-08T23:11:50.5709756Z 113   --> $DIR/loop-break-value.rs:14:15
2019-12-08T23:11:50.5709794Z 114    |
2019-12-08T23:11:50.5709828Z 115 LL |         break "asdf";
2019-12-08T23:11:50.5710015Z -    |               ^^^^^^ expected `i32`, found `&str`
2019-12-08T23:11:50.5710107Z +    |               |
2019-12-08T23:11:50.5710107Z +    |               |
2019-12-08T23:11:50.5710144Z +    |               expected `i32`, found `&str`
2019-12-08T23:11:50.5710203Z +    |               help: consider dereferencing the borrow: `*"asdf"`
2019-12-08T23:11:50.5710275Z 118 error[E0308]: mismatched types
2019-12-08T23:11:50.5710468Z 119   --> $DIR/loop-break-value.rs:19:31
2019-12-08T23:11:50.5710494Z 
2019-12-08T23:11:50.5710526Z 120    |
2019-12-08T23:11:50.5710526Z 120    |
2019-12-08T23:11:50.5710866Z 121 LL |             break 'outer_loop "nope";
2019-12-08T23:11:50.5711160Z -    |                               ^^^^^^ expected `i32`, found `&str`
2019-12-08T23:11:50.5711239Z +    |                               |
2019-12-08T23:11:50.5711239Z +    |                               |
2019-12-08T23:11:50.5711288Z +    |                               expected `i32`, found `&str`
2019-12-08T23:11:50.5711330Z +    |                               help: consider dereferencing the borrow: `*"nope"`
2019-12-08T23:11:50.5711421Z 124 error[E0308]: mismatched types
2019-12-08T23:11:50.5712097Z 125   --> $DIR/loop-break-value.rs:71:26
2019-12-08T23:11:50.5712144Z 
2019-12-08T23:11:50.5712169Z 
2019-12-08T23:11:50.5712169Z 
2019-12-08T23:11:50.5712231Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5712760Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value/loop-break-value.stderr
2019-12-08T23:11:50.5713018Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5713319Z To only update this specific test, also pass `--test-args loops/loop-break-value.rs`
2019-12-08T23:11:50.5713402Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5713462Z status: exit code: 1
2019-12-08T23:11:50.5713462Z status: exit code: 1
2019-12-08T23:11:50.5714244Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/loops/loop-break-value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5714591Z ------------------------------------------
2019-12-08T23:11:50.5714627Z 
2019-12-08T23:11:50.5714854Z ------------------------------------------
2019-12-08T23:11:50.5714916Z stderr:
2019-12-08T23:11:50.5714916Z stderr:
2019-12-08T23:11:50.5715321Z ------------------------------------------
2019-12-08T23:11:50.5715529Z warning: denote infinite loops with `loop { ... }`
2019-12-08T23:11:50.5715733Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:24:5
2019-12-08T23:11:50.5715772Z    |
2019-12-08T23:11:50.5715967Z LL |     'while_loop: while true { //~ WARN denote infinite loops with
2019-12-08T23:11:50.5716117Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ help: use `loop`
2019-12-08T23:11:50.5716162Z    |
2019-12-08T23:11:50.5716242Z    = note: `#[warn(while_true)]` on by default
2019-12-08T23:11:50.5716266Z 
2019-12-08T23:11:50.5716317Z error[E0571]: `break` with value from a `while` loop
2019-12-08T23:11:50.5716575Z    |
2019-12-08T23:11:50.5716575Z    |
2019-12-08T23:11:50.5716630Z LL |         break (); //~ ERROR `break` with value from a `while` loop
2019-12-08T23:11:50.5716674Z    |         ^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-08T23:11:50.5716713Z    |
2019-12-08T23:11:50.5716771Z help: instead, use `break` on its own without a value inside this `while` loop
2019-12-08T23:11:50.5716808Z    |
2019-12-08T23:11:50.5716846Z LL |         break; //~ ERROR `break` with value from a `while` loop
2019-12-08T23:11:50.5716922Z 
2019-12-08T23:11:50.5716922Z 
2019-12-08T23:11:50.5716964Z error[E0571]: `break` with value from a `while` loop
2019-12-08T23:11:50.5717224Z    |
2019-12-08T23:11:50.5717396Z LL |             break 'while_loop 123;
2019-12-08T23:11:50.5717442Z    |             ^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-08T23:11:50.5717494Z    |
2019-12-08T23:11:50.5717494Z    |
2019-12-08T23:11:50.5717532Z help: instead, use `break` on its own without a value inside this `while` loop
2019-12-08T23:11:50.5717615Z LL |             break;
2019-12-08T23:11:50.5717650Z    |             ^^^^^
2019-12-08T23:11:50.5717673Z 
2019-12-08T23:11:50.5717673Z 
2019-12-08T23:11:50.5717709Z error[E0571]: `break` with value from a `while` loop
2019-12-08T23:11:50.5717956Z    |
2019-12-08T23:11:50.5717956Z    |
2019-12-08T23:11:50.5717994Z LL |         if break () { //~ ERROR `break` with value from a `while` loop
2019-12-08T23:11:50.5718056Z    |            ^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-08T23:11:50.5718093Z    |
2019-12-08T23:11:50.5718138Z help: instead, use `break` on its own without a value inside this `while` loop
2019-12-08T23:11:50.5718185Z    |
2019-12-08T23:11:50.5718223Z LL |         if break { //~ ERROR `break` with value from a `while` loop
2019-12-08T23:11:50.5718282Z 
2019-12-08T23:11:50.5718282Z 
2019-12-08T23:11:50.5718328Z error[E0571]: `break` with value from a `while` loop
2019-12-08T23:11:50.5718561Z    |
2019-12-08T23:11:50.5718608Z LL |         break None;
2019-12-08T23:11:50.5718649Z    |         ^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-08T23:11:50.5718686Z    |
2019-12-08T23:11:50.5718686Z    |
2019-12-08T23:11:50.5718738Z help: instead, use `break` on its own without a value inside this `while` loop
2019-12-08T23:11:50.5718809Z LL |         break;
2019-12-08T23:11:50.5718850Z    |         ^^^^^
2019-12-08T23:11:50.5718887Z 
2019-12-08T23:11:50.5718887Z 
2019-12-08T23:11:50.5718922Z error[E0571]: `break` with value from a `while` loop
2019-12-08T23:11:50.5719173Z    |
2019-12-08T23:11:50.5719173Z    |
2019-12-08T23:11:50.5719349Z LL |             break 'while_let_loop "nope";
2019-12-08T23:11:50.5719396Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-08T23:11:50.5719445Z    |
2019-12-08T23:11:50.5719485Z help: instead, use `break` on its own without a value inside this `while` loop
2019-12-08T23:11:50.5719563Z LL |             break;
2019-12-08T23:11:50.5719598Z    |             ^^^^^
2019-12-08T23:11:50.5719620Z 
2019-12-08T23:11:50.5719656Z error[E0571]: `break` with value from a `for` loop
2019-12-08T23:11:50.5719656Z error[E0571]: `break` with value from a `for` loop
2019-12-08T23:11:50.5719855Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:54:9
2019-12-08T23:11:50.5719892Z    |
2019-12-08T23:11:50.5719990Z LL |         break (); //~ ERROR `break` with value from a `for` loop
2019-12-08T23:11:50.5720051Z    |         ^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-08T23:11:50.5720135Z    |
2019-12-08T23:11:50.5720174Z help: instead, use `break` on its own without a value inside this `for` loop
2019-12-08T23:11:50.5720210Z    |
2019-12-08T23:11:50.5720266Z LL |         break; //~ ERROR `break` with value from a `for` loop
2019-12-08T23:11:50.5720326Z 
2019-12-08T23:11:50.5720378Z error[E0571]: `break` with value from a `for` loop
2019-12-08T23:11:50.5720595Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:55:9
2019-12-08T23:11:50.5720633Z    |
2019-12-08T23:11:50.5720633Z    |
2019-12-08T23:11:50.5720667Z LL |         break [()];
2019-12-08T23:11:50.5720718Z    |         ^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-08T23:11:50.5720755Z    |
2019-12-08T23:11:50.5720800Z help: instead, use `break` on its own without a value inside this `for` loop
2019-12-08T23:11:50.5720884Z LL |         break;
2019-12-08T23:11:50.5720925Z    |         ^^^^^
2019-12-08T23:11:50.5720947Z 
2019-12-08T23:11:50.5720995Z error[E0571]: `break` with value from a `for` loop
2019-12-08T23:11:50.5720995Z error[E0571]: `break` with value from a `for` loop
2019-12-08T23:11:50.5721193Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:62:13
2019-12-08T23:11:50.5721231Z    |
2019-12-08T23:11:50.5722340Z LL |             break 'for_loop Some(17);
2019-12-08T23:11:50.5722404Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-08T23:11:50.5722451Z    |
2019-12-08T23:11:50.5722518Z help: instead, use `break` on its own without a value inside this `for` loop
2019-12-08T23:11:50.5722602Z LL |             break;
2019-12-08T23:11:50.5722656Z    |             ^^^^^
2019-12-08T23:11:50.5722685Z 
2019-12-08T23:11:50.5722726Z error[E0308]: mismatched types
2019-12-08T23:11:50.5722726Z error[E0308]: mismatched types
2019-12-08T23:11:50.5723020Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:2:31
2019-12-08T23:11:50.5723082Z    |
2019-12-08T23:11:50.5723126Z LL |     let val: ! = loop { break break; };
2019-12-08T23:11:50.5723182Z    |                               ^^^^^ expected `!`, found `()`
2019-12-08T23:11:50.5723278Z    = note:   expected type `!`
2019-12-08T23:11:50.5723322Z            found unit type `()`
2019-12-08T23:11:50.5723351Z 
2019-12-08T23:11:50.5723405Z error[E0308]: mismatched types
---
2019-12-08T23:11:50.5723820Z 
2019-12-08T23:11:50.5723860Z error[E0308]: mismatched types
2019-12-08T23:11:50.5724107Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:14:15
2019-12-08T23:11:50.5724152Z    |
2019-12-08T23:11:50.5724407Z LL |         break "asdf"; //~ ERROR mismatched types
2019-12-08T23:11:50.5724519Z    |               |
2019-12-08T23:11:50.5724519Z    |               |
2019-12-08T23:11:50.5724572Z    |               expected `i32`, found `&str`
2019-12-08T23:11:50.5724637Z    |               help: consider dereferencing the borrow: `*"asdf"`
2019-12-08T23:11:50.5724710Z error[E0308]: mismatched types
2019-12-08T23:11:50.5724987Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:19:31
2019-12-08T23:11:50.5725045Z    |
2019-12-08T23:11:50.5725045Z    |
2019-12-08T23:11:50.5725283Z LL |             break 'outer_loop "nope"; //~ ERROR mismatched types
2019-12-08T23:11:50.5725386Z    |                               |
2019-12-08T23:11:50.5725386Z    |                               |
2019-12-08T23:11:50.5725603Z    |                               expected `i32`, found `&str`
2019-12-08T23:11:50.5725648Z    |                               help: consider dereferencing the borrow: `*"nope"`
2019-12-08T23:11:50.5725980Z error[E0308]: mismatched types
2019-12-08T23:11:50.5726204Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:71:26
2019-12-08T23:11:50.5726306Z    |
2019-12-08T23:11:50.5726306Z    |
2019-12-08T23:11:50.5726521Z LL |                 break 'c 123; //~ ERROR mismatched types
2019-12-08T23:11:50.5726564Z    |                          ^^^ expected `()`, found integer
2019-12-08T23:11:50.5726623Z error[E0308]: mismatched types
2019-12-08T23:11:50.5726826Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:78:15
2019-12-08T23:11:50.5726863Z    |
2019-12-08T23:11:50.5726863Z    |
2019-12-08T23:11:50.5726900Z LL |         break (break, break); //~ ERROR mismatched types
2019-12-08T23:11:50.5726958Z    |               ^^^^^^^^^^^^^^ expected `()`, found tuple
2019-12-08T23:11:50.5727026Z    = note: expected unit type `()`
2019-12-08T23:11:50.5727026Z    = note: expected unit type `()`
2019-12-08T23:11:50.5727076Z                   found tuple `(!, !)`
2019-12-08T23:11:50.5727134Z error[E0308]: mismatched types
2019-12-08T23:11:50.5727330Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:83:15
2019-12-08T23:11:50.5727383Z    |
2019-12-08T23:11:50.5727425Z LL |         break 2; //~ ERROR mismatched types
---
2019-12-08T23:11:50.5727760Z    |
2019-12-08T23:11:50.5727807Z LL |         break; //~ ERROR mismatched types
2019-12-08T23:11:50.5727844Z    |         ^^^^^
2019-12-08T23:11:50.5727878Z    |         |
2019-12-08T23:11:50.5727923Z    |         expected integer, found `()`
2019-12-08T23:11:50.5727965Z    |         help: give it a value of the expected type: `break value`
2019-12-08T23:11:50.5728025Z error: aborting due to 16 previous errors
2019-12-08T23:11:50.5728058Z 
2019-12-08T23:11:50.5728094Z Some errors have detailed explanations: E0308, E0571.
2019-12-08T23:11:50.5728302Z For more information about an error, try `rustc --explain E0308`.
2019-12-08T23:11:50.5728302Z For more information about an error, try `rustc --explain E0308`.
2019-12-08T23:11:50.5728329Z 
2019-12-08T23:11:50.5728518Z ------------------------------------------
2019-12-08T23:11:50.5728545Z 
2019-12-08T23:11:50.5728565Z 
2019-12-08T23:11:50.5728744Z ---- [ui] ui/lub-glb/old-lub-glb-object.rs stdout ----
2019-12-08T23:11:50.5728792Z diff of stderr:
2019-12-08T23:11:50.5728815Z 
2019-12-08T23:11:50.5728848Z 6 LL | |         0 => x,
2019-12-08T23:11:50.5729065Z 7    | |              - this is found to be of type `&dyn for<'a, 'b> Foo<&'a u8, &'b u8>`
2019-12-08T23:11:50.5729122Z 8 LL | |         _ => y,
2019-12-08T23:11:50.5729331Z -    | |              ^ expected bound lifetime parameter 'a, found concrete lifetime
2019-12-08T23:11:50.5729423Z +    | |              |
2019-12-08T23:11:50.5729423Z +    | |              |
2019-12-08T23:11:50.5729630Z +    | |              expected bound lifetime parameter 'a, found concrete lifetime
2019-12-08T23:11:50.5729681Z +    | |              help: consider dereferencing the borrow: `*y`
2019-12-08T23:11:50.5729918Z 11    | |_____- `match` arms have incompatible types
2019-12-08T23:11:50.5729962Z 12    |
2019-12-08T23:11:50.5729983Z 
2019-12-08T23:11:50.5730016Z 
2019-12-08T23:11:50.5730016Z 
2019-12-08T23:11:50.5730052Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5730303Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-object/old-lub-glb-object.stderr
2019-12-08T23:11:50.5730512Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5730727Z To only update this specific test, also pass `--test-args lub-glb/old-lub-glb-object.rs`
2019-12-08T23:11:50.5730790Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5730836Z status: exit code: 1
2019-12-08T23:11:50.5730836Z status: exit code: 1
2019-12-08T23:11:50.5731521Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lub-glb/old-lub-glb-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-object" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-object/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5731848Z ------------------------------------------
2019-12-08T23:11:50.5731876Z 
2019-12-08T23:11:50.5732067Z ------------------------------------------
2019-12-08T23:11:50.5732104Z stderr:
2019-12-08T23:11:50.5732104Z stderr:
2019-12-08T23:11:50.5732273Z ------------------------------------------
2019-12-08T23:11:50.5732398Z error[E0308]: match arms have incompatible types
2019-12-08T23:11:50.5732594Z   --> /checkout/src/test/ui/lub-glb/old-lub-glb-object.rs:12:14
2019-12-08T23:11:50.5732634Z    |
2019-12-08T23:11:50.5732868Z LL |       let z = match 22 {
2019-12-08T23:11:50.5733311Z LL | |         0 => x,
2019-12-08T23:11:50.5733311Z LL | |         0 => x,
2019-12-08T23:11:50.5733578Z    | |              - this is found to be of type `&dyn for<'a, 'b> Foo<&'a u8, &'b u8>`
2019-12-08T23:11:50.5733650Z LL | |         _ => y, //~ ERROR match arms have incompatible types
2019-12-08T23:11:50.5733751Z    | |              |
2019-12-08T23:11:50.5733751Z    | |              |
2019-12-08T23:11:50.5734002Z    | |              expected bound lifetime parameter 'a, found concrete lifetime
2019-12-08T23:11:50.5734057Z    | |              help: consider dereferencing the borrow: `*y`
2019-12-08T23:11:50.5734344Z    | |_____- `match` arms have incompatible types
2019-12-08T23:11:50.5734389Z    |
2019-12-08T23:11:50.5734389Z    |
2019-12-08T23:11:50.5734622Z    = note:   expected type `&dyn for<'a, 'b> Foo<&'a u8, &'b u8>`
2019-12-08T23:11:50.5734869Z            found reference `&dyn for<'a> Foo<&'a u8, &'a u8>`
2019-12-08T23:11:50.5734953Z error: aborting due to previous error
2019-12-08T23:11:50.5734982Z 
2019-12-08T23:11:50.5735235Z For more information about this error, try `rustc --explain E0308`.
2019-12-08T23:11:50.5735276Z 
---
2019-12-08T23:11:50.5735831Z diff of stderr:
2019-12-08T23:11:50.5735858Z 
2019-12-08T23:11:50.5736078Z 2   --> $DIR/overloaded-calls-bad.rs:28:17
2019-12-08T23:11:50.5736123Z 3    |
2019-12-08T23:11:50.5736165Z 4 LL |     let ans = s("what");
2019-12-08T23:11:50.5736630Z -    |                 ^^^^^^ expected `isize`, found `&str`
2019-12-08T23:11:50.5736707Z +    |                 |
2019-12-08T23:11:50.5736744Z +    |                 expected `isize`, found `&str`
2019-12-08T23:11:50.5736744Z +    |                 expected `isize`, found `&str`
2019-12-08T23:11:50.5736799Z +    |                 help: consider dereferencing the borrow: `*"what"`
2019-12-08T23:11:50.5736874Z 7 error[E0057]: this function takes 1 parameter but 0 parameters were supplied
2019-12-08T23:11:50.5737078Z 8   --> $DIR/overloaded-calls-bad.rs:29:15
2019-12-08T23:11:50.5737104Z 
2019-12-08T23:11:50.5737124Z 
2019-12-08T23:11:50.5737124Z 
2019-12-08T23:11:50.5737160Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5737439Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/overloaded-calls-bad/overloaded-calls-bad.stderr
2019-12-08T23:11:50.5737638Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5737876Z To only update this specific test, also pass `--test-args mismatched_types/overloaded-calls-bad.rs`
2019-12-08T23:11:50.5737946Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5737984Z status: exit code: 1
2019-12-08T23:11:50.5737984Z status: exit code: 1
2019-12-08T23:11:50.5738756Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/overloaded-calls-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/overloaded-calls-bad" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/overloaded-calls-bad/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5739116Z ------------------------------------------
2019-12-08T23:11:50.5739145Z 
2019-12-08T23:11:50.5739335Z ------------------------------------------
2019-12-08T23:11:50.5739384Z stderr:
2019-12-08T23:11:50.5739384Z stderr:
2019-12-08T23:11:50.5739570Z ------------------------------------------
2019-12-08T23:11:50.5739610Z error[E0308]: mismatched types
2019-12-08T23:11:50.5739844Z   --> /checkout/src/test/ui/mismatched_types/overloaded-calls-bad.rs:28:17
2019-12-08T23:11:50.5739888Z    |
2019-12-08T23:11:50.5739931Z LL |     let ans = s("what");    //~ ERROR mismatched types
2019-12-08T23:11:50.5740027Z    |                 |
2019-12-08T23:11:50.5740067Z    |                 expected `isize`, found `&str`
2019-12-08T23:11:50.5740067Z    |                 expected `isize`, found `&str`
2019-12-08T23:11:50.5740110Z    |                 help: consider dereferencing the borrow: `*"what"`
2019-12-08T23:11:50.5740195Z error[E0057]: this function takes 1 parameter but 0 parameters were supplied
2019-12-08T23:11:50.5740421Z   --> /checkout/src/test/ui/mismatched_types/overloaded-calls-bad.rs:29:15
2019-12-08T23:11:50.5740477Z    |
2019-12-08T23:11:50.5740477Z    |
2019-12-08T23:11:50.5740513Z LL |     let ans = s();
2019-12-08T23:11:50.5740577Z 
2019-12-08T23:11:50.5740628Z error[E0057]: this function takes 1 parameter but 2 parameters were supplied
2019-12-08T23:11:50.5740858Z   --> /checkout/src/test/ui/mismatched_types/overloaded-calls-bad.rs:31:15
2019-12-08T23:11:50.5740898Z    |
2019-12-08T23:11:50.5740898Z    |
2019-12-08T23:11:50.5740945Z LL |     let ans = s("burma", "shave");
2019-12-08T23:11:50.5741019Z 
2019-12-08T23:11:50.5741055Z error: aborting due to 3 previous errors
2019-12-08T23:11:50.5741094Z 
2019-12-08T23:11:50.5741133Z Some errors have detailed explanations: E0057, E0308.
---
2019-12-08T23:11:50.5742116Z diff of stderr:
2019-12-08T23:11:50.5742139Z 
2019-12-08T23:11:50.5743093Z 2   --> $DIR/call-fn-never-arg-wrong-type.rs:8:9
2019-12-08T23:11:50.5743145Z 3    |
2019-12-08T23:11:50.5743211Z 4 LL |     foo("wow");
2019-12-08T23:11:50.5743434Z -    |         ^^^^^ expected `!`, found `&str`
2019-12-08T23:11:50.5743543Z +    |         |
2019-12-08T23:11:50.5743543Z +    |         |
2019-12-08T23:11:50.5743587Z +    |         expected `!`, found `&str`
2019-12-08T23:11:50.5743636Z +    |         help: consider dereferencing the borrow: `*"wow"`
2019-12-08T23:11:50.5743735Z 7    = note:   expected type `!`
2019-12-08T23:11:50.5743950Z 8            found reference `&'static str`
2019-12-08T23:11:50.5743983Z 
2019-12-08T23:11:50.5744019Z 
2019-12-08T23:11:50.5744019Z 
2019-12-08T23:11:50.5744064Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5744394Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/call-fn-never-arg-wrong-type/call-fn-never-arg-wrong-type.stderr
2019-12-08T23:11:50.5744653Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5745040Z To only update this specific test, also pass `--test-args never_type/call-fn-never-arg-wrong-type.rs`
2019-12-08T23:11:50.5745147Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5745247Z status: exit code: 1
2019-12-08T23:11:50.5745247Z status: exit code: 1
2019-12-08T23:11:50.5746622Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/call-fn-never-arg-wrong-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/call-fn-never-arg-wrong-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/call-fn-never-arg-wrong-type/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5746893Z ------------------------------------------
2019-12-08T23:11:50.5746920Z 
2019-12-08T23:11:50.5747111Z ------------------------------------------
2019-12-08T23:11:50.5747147Z stderr:
2019-12-08T23:11:50.5747147Z stderr:
2019-12-08T23:11:50.5747314Z ------------------------------------------
2019-12-08T23:11:50.5747371Z error[E0308]: mismatched types
2019-12-08T23:11:50.5747566Z   --> /checkout/src/test/ui/never_type/call-fn-never-arg-wrong-type.rs:8:9
2019-12-08T23:11:50.5747605Z    |
2019-12-08T23:11:50.5747654Z LL |     foo("wow"); //~ ERROR mismatched types
2019-12-08T23:11:50.5747723Z    |         |
2019-12-08T23:11:50.5747723Z    |         |
2019-12-08T23:11:50.5747767Z    |         expected `!`, found `&str`
2019-12-08T23:11:50.5747806Z    |         help: consider dereferencing the borrow: `*"wow"`
2019-12-08T23:11:50.5747875Z    = note:   expected type `!`
2019-12-08T23:11:50.5748059Z            found reference `&'static str`
2019-12-08T23:11:50.5748083Z 
2019-12-08T23:11:50.5748117Z error: aborting due to previous error
---
2019-12-08T23:11:50.5748827Z diff of stderr:
2019-12-08T23:11:50.5748848Z 
2019-12-08T23:11:50.5749029Z 2   --> $DIR/never-assign-wrong-type.rs:6:16
2019-12-08T23:11:50.5749065Z 3    |
2019-12-08T23:11:50.5749098Z 4 LL |     let x: ! = "hello";
2019-12-08T23:11:50.5749288Z -    |            -   ^^^^^^^ expected `!`, found `&str`
2019-12-08T23:11:50.5749599Z +    |            -   ^^^^^^^
2019-12-08T23:11:50.5749650Z +    |            |   |
2019-12-08T23:11:50.5749650Z +    |            |   |
2019-12-08T23:11:50.5749686Z +    |            |   expected `!`, found `&str`
2019-12-08T23:11:50.5749726Z +    |            |   help: consider dereferencing the borrow: `*"hello"`
2019-12-08T23:11:50.5749815Z 8    |
2019-12-08T23:11:50.5749849Z 9    = note:   expected type `!`
2019-12-08T23:11:50.5749877Z 
2019-12-08T23:11:50.5749897Z 
2019-12-08T23:11:50.5749897Z 
2019-12-08T23:11:50.5749941Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5750197Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never-assign-wrong-type/never-assign-wrong-type.stderr
2019-12-08T23:11:50.5750392Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5750620Z To only update this specific test, also pass `--test-args never_type/never-assign-wrong-type.rs`
2019-12-08T23:11:50.5750681Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5750731Z status: exit code: 1
2019-12-08T23:11:50.5750731Z status: exit code: 1
2019-12-08T23:11:50.5751433Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/never-assign-wrong-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never-assign-wrong-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never-assign-wrong-type/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5751933Z ------------------------------------------
2019-12-08T23:11:50.5752143Z 
2019-12-08T23:11:50.5752336Z ------------------------------------------
2019-12-08T23:11:50.5757410Z stderr:
2019-12-08T23:11:50.5757410Z stderr:
2019-12-08T23:11:50.5757669Z ------------------------------------------
2019-12-08T23:11:50.5757724Z error[E0308]: mismatched types
2019-12-08T23:11:50.5757931Z   --> /checkout/src/test/ui/never_type/never-assign-wrong-type.rs:6:16
2019-12-08T23:11:50.5757973Z    |
2019-12-08T23:11:50.5758015Z LL |     let x: ! = "hello"; //~ ERROR mismatched types
2019-12-08T23:11:50.5758216Z    |            -   ^^^^^^^
2019-12-08T23:11:50.5758257Z    |            |   |
2019-12-08T23:11:50.5758296Z    |            |   expected `!`, found `&str`
2019-12-08T23:11:50.5758356Z    |            |   help: consider dereferencing the borrow: `*"hello"`
2019-12-08T23:11:50.5758433Z    |
2019-12-08T23:11:50.5758482Z    = note:   expected type `!`
2019-12-08T23:11:50.5758669Z            found reference `&'static str`
2019-12-08T23:11:50.5758697Z 
---
2019-12-08T23:11:50.5759503Z diff of stderr:
2019-12-08T23:11:50.5759547Z 
2019-12-08T23:11:50.5759735Z 20   --> $DIR/fn-arg-doc-comment.rs:18:7
2019-12-08T23:11:50.5759774Z 21    |
2019-12-08T23:11:50.5759815Z 22 LL |     f("", "");
2019-12-08T23:11:50.5760019Z -    |       ^^ expected `u8`, found `&str`
2019-12-08T23:11:50.5760095Z +    |       |
2019-12-08T23:11:50.5760147Z +    |       expected `u8`, found `&str`
2019-12-08T23:11:50.5760147Z +    |       expected `u8`, found `&str`
2019-12-08T23:11:50.5760188Z +    |       help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.5760276Z 25 error[E0308]: mismatched types
2019-12-08T23:11:50.5760468Z 26   --> $DIR/fn-arg-doc-comment.rs:18:11
2019-12-08T23:11:50.5760494Z 
2019-12-08T23:11:50.5760528Z 27    |
2019-12-08T23:11:50.5760528Z 27    |
2019-12-08T23:11:50.5760575Z 28 LL |     f("", "");
2019-12-08T23:11:50.5760814Z -    |           ^^ expected `u8`, found `&str`
2019-12-08T23:11:50.5760899Z +    |           |
2019-12-08T23:11:50.5760945Z +    |           expected `u8`, found `&str`
2019-12-08T23:11:50.5760945Z +    |           expected `u8`, found `&str`
2019-12-08T23:11:50.5760987Z +    |           help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.5761082Z 31 error[E0308]: mismatched types
2019-12-08T23:11:50.5761450Z 32   --> $DIR/fn-arg-doc-comment.rs:23:9
2019-12-08T23:11:50.5761479Z 
2019-12-08T23:11:50.5761527Z 33    |
2019-12-08T23:11:50.5761527Z 33    |
2019-12-08T23:11:50.5761665Z 34 LL |     bar("");
2019-12-08T23:11:50.5762063Z -    |         ^^ expected `i32`, found `&str`
2019-12-08T23:11:50.5762170Z +    |         |
2019-12-08T23:11:50.5762170Z +    |         |
2019-12-08T23:11:50.5762216Z +    |         expected `i32`, found `&str`
2019-12-08T23:11:50.5762266Z +    |         help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.5762367Z 37 error: aborting due to 6 previous errors
2019-12-08T23:11:50.5762408Z 38 
2019-12-08T23:11:50.5762434Z 
2019-12-08T23:11:50.5762472Z 
2019-12-08T23:11:50.5762472Z 
2019-12-08T23:11:50.5762516Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5762954Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-arg-doc-comment/fn-arg-doc-comment.stderr
2019-12-08T23:11:50.5763312Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5763601Z To only update this specific test, also pass `--test-args parser/fn-arg-doc-comment.rs`
2019-12-08T23:11:50.5763683Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5763743Z status: exit code: 1
2019-12-08T23:11:50.5763743Z status: exit code: 1
2019-12-08T23:11:50.5764789Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/fn-arg-doc-comment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-arg-doc-comment" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-arg-doc-comment/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5765325Z ------------------------------------------
2019-12-08T23:11:50.5765355Z 
2019-12-08T23:11:50.5765558Z ------------------------------------------
2019-12-08T23:11:50.5765596Z stderr:
2019-12-08T23:11:50.5765596Z stderr:
2019-12-08T23:11:50.5765973Z ------------------------------------------
2019-12-08T23:11:50.5766261Z error: attributes cannot be applied to a function parameter's type
2019-12-08T23:11:50.5766467Z   --> /checkout/src/test/ui/parser/fn-arg-doc-comment.rs:12:12
2019-12-08T23:11:50.5766507Z    |
2019-12-08T23:11:50.5766560Z LL | fn bar(id: #[allow(dead_code)] i32) {}
2019-12-08T23:11:50.5766602Z    |            ^^^^^^^^^^^^^^^^^^^ attributes are not allowed here
2019-12-08T23:11:50.5766668Z error: documentation comments cannot be applied to function parameters
2019-12-08T23:11:50.5766886Z   --> /checkout/src/test/ui/parser/fn-arg-doc-comment.rs:2:5
2019-12-08T23:11:50.5766932Z    |
2019-12-08T23:11:50.5766967Z LL |     /// Comment
---
2019-12-08T23:11:50.5767822Z 
2019-12-08T23:11:50.5767856Z error[E0308]: mismatched types
2019-12-08T23:11:50.5768055Z   --> /checkout/src/test/ui/parser/fn-arg-doc-comment.rs:18:7
2019-12-08T23:11:50.5768107Z    |
2019-12-08T23:11:50.5768141Z LL |     f("", "");
2019-12-08T23:11:50.5768212Z    |       |
2019-12-08T23:11:50.5768261Z    |       expected `u8`, found `&str`
2019-12-08T23:11:50.5768261Z    |       expected `u8`, found `&str`
2019-12-08T23:11:50.5768308Z    |       help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.5768371Z error[E0308]: mismatched types
2019-12-08T23:11:50.5768613Z   --> /checkout/src/test/ui/parser/fn-arg-doc-comment.rs:18:11
2019-12-08T23:11:50.5768653Z    |
2019-12-08T23:11:50.5768653Z    |
2019-12-08T23:11:50.5768690Z LL |     f("", "");
2019-12-08T23:11:50.5768942Z    |           |
2019-12-08T23:11:50.5768978Z    |           expected `u8`, found `&str`
2019-12-08T23:11:50.5768978Z    |           expected `u8`, found `&str`
2019-12-08T23:11:50.5769030Z    |           help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.5769250Z error[E0308]: mismatched types
2019-12-08T23:11:50.5769438Z   --> /checkout/src/test/ui/parser/fn-arg-doc-comment.rs:23:9
2019-12-08T23:11:50.5769486Z    |
2019-12-08T23:11:50.5769518Z LL |     bar("");
2019-12-08T23:11:50.5769518Z LL |     bar("");
2019-12-08T23:11:50.5769550Z    |         ^^
2019-12-08T23:11:50.5769593Z    |         |
2019-12-08T23:11:50.5769628Z    |         expected `i32`, found `&str`
2019-12-08T23:11:50.5769770Z    |         help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.5769849Z error: aborting due to 6 previous errors
2019-12-08T23:11:50.5769920Z 
2019-12-08T23:11:50.5770138Z For more information about this error, try `rustc --explain E0308`.
2019-12-08T23:11:50.5770165Z 
---
2019-12-08T23:11:50.5771086Z -    |            -----   ^^ expected `usize`, found `&str`
2019-12-08T23:11:50.5771237Z -    |            |
2019-12-08T23:11:50.5771410Z +    |            -----   ^^
2019-12-08T23:11:50.5771447Z +    |            |       |
2019-12-08T23:11:50.5771491Z +    |            |       expected `usize`, found `&str`
2019-12-08T23:11:50.5771545Z +    |            |       help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.5771727Z 49 
2019-12-08T23:11:50.5772121Z 50 error[E0277]: can't compare `&str` with `char`
2019-12-08T23:11:50.5772169Z 
2019-12-08T23:11:50.5772194Z 
2019-12-08T23:11:50.5772194Z 
2019-12-08T23:11:50.5772239Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5772556Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bad-char-literals-6/lex-bad-char-literals-6.stderr
2019-12-08T23:11:50.5772819Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5773089Z To only update this specific test, also pass `--test-args parser/lex-bad-char-literals-6.rs`
2019-12-08T23:11:50.5773183Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5773235Z status: exit code: 1
2019-12-08T23:11:50.5773235Z status: exit code: 1
2019-12-08T23:11:50.5774015Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/lex-bad-char-literals-6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bad-char-literals-6" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bad-char-literals-6/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5774336Z ------------------------------------------
2019-12-08T23:11:50.5774382Z 
2019-12-08T23:11:50.5774591Z ------------------------------------------
2019-12-08T23:11:50.5774635Z stderr:
2019-12-08T23:11:50.5774635Z stderr:
2019-12-08T23:11:50.5774851Z ------------------------------------------
2019-12-08T23:11:50.5774901Z error: character literal may only contain one codepoint
2019-12-08T23:11:50.5775151Z   --> /checkout/src/test/ui/parser/lex-bad-char-literals-6.rs:2:19
2019-12-08T23:11:50.5775215Z    |
2019-12-08T23:11:50.5775561Z LL |     let x: &str = 'ab';
2019-12-08T23:11:50.5775632Z    |
2019-12-08T23:11:50.5775686Z help: if you meant to write a `str` literal, use double quotes
2019-12-08T23:11:50.5775722Z    |
2019-12-08T23:11:50.5775756Z LL |     let x: &str = "ab";
2019-12-08T23:11:50.5775756Z LL |     let x: &str = "ab";
2019-12-08T23:11:50.5775805Z    |                   ^^^^
2019-12-08T23:11:50.5775828Z 
2019-12-08T23:11:50.5775865Z error: character literal may only contain one codepoint
2019-12-08T23:11:50.5776067Z   --> /checkout/src/test/ui/parser/lex-bad-char-literals-6.rs:4:19
2019-12-08T23:11:50.5776116Z    |
2019-12-08T23:11:50.5776276Z LL |     let y: char = 'cd';
2019-12-08T23:11:50.5776356Z    |
2019-12-08T23:11:50.5776393Z help: if you meant to write a `str` literal, use double quotes
2019-12-08T23:11:50.5776498Z    |
2019-12-08T23:11:50.5776498Z    |
2019-12-08T23:11:50.5776552Z LL |     let y: char = "cd";
2019-12-08T23:11:50.5776653Z 
2019-12-08T23:11:50.5776689Z error: character literal may only contain one codepoint
2019-12-08T23:11:50.5776921Z   --> /checkout/src/test/ui/parser/lex-bad-char-literals-6.rs:6:13
2019-12-08T23:11:50.5776960Z    |
2019-12-08T23:11:50.5776960Z    |
2019-12-08T23:11:50.5777113Z LL |     let z = 'ef';
2019-12-08T23:11:50.5777199Z    |
2019-12-08T23:11:50.5777236Z help: if you meant to write a `str` literal, use double quotes
2019-12-08T23:11:50.5777271Z    |
2019-12-08T23:11:50.5777271Z    |
2019-12-08T23:11:50.5777321Z LL |     let z = "ef";
2019-12-08T23:11:50.5777377Z 
2019-12-08T23:11:50.5777574Z error[E0277]: can't compare `&str` with `char`
2019-12-08T23:11:50.5777773Z   --> /checkout/src/test/ui/parser/lex-bad-char-literals-6.rs:9:10
2019-12-08T23:11:50.5777810Z    |
---
2019-12-08T23:11:50.5778701Z    |
2019-12-08T23:11:50.5778738Z LL |     let a: usize = "";
2019-12-08T23:11:50.5778907Z    |            -----   ^^
2019-12-08T23:11:50.5778947Z    |            |       |
2019-12-08T23:11:50.5779000Z    |            |       expected `usize`, found `&str`
2019-12-08T23:11:50.5779044Z    |            |       help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.5779126Z 
2019-12-08T23:11:50.5779315Z error[E0277]: can't compare `&str` with `char`
2019-12-08T23:11:50.5779530Z   --> /checkout/src/test/ui/parser/lex-bad-char-literals-6.rs:12:10
2019-12-08T23:11:50.5779582Z    |
---
2019-12-08T23:11:50.5781133Z -    |            -----   ^^ expected `usize`, found `&str`
2019-12-08T23:11:50.5781298Z -    |            |
2019-12-08T23:11:50.5781471Z +    |            -----   ^^
2019-12-08T23:11:50.5781529Z +    |            |       |
2019-12-08T23:11:50.5781587Z +    |            |       expected `usize`, found `&str`
2019-12-08T23:11:50.5781717Z +    |            |       help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.5782170Z 20 
2019-12-08T23:11:50.5782214Z 21 error: aborting due to 3 previous errors
2019-12-08T23:11:50.5782243Z 
2019-12-08T23:11:50.5782281Z 
2019-12-08T23:11:50.5782281Z 
2019-12-08T23:11:50.5782324Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5782705Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/numeric-lifetime/numeric-lifetime.stderr
2019-12-08T23:11:50.5782996Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5783346Z To only update this specific test, also pass `--test-args parser/numeric-lifetime.rs`
2019-12-08T23:11:50.5783437Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5783481Z status: exit code: 1
2019-12-08T23:11:50.5783481Z status: exit code: 1
2019-12-08T23:11:50.5784236Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/numeric-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/numeric-lifetime" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/numeric-lifetime/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5784559Z ------------------------------------------
2019-12-08T23:11:50.5784608Z 
2019-12-08T23:11:50.5784820Z ------------------------------------------
2019-12-08T23:11:50.5784872Z stderr:
2019-12-08T23:11:50.5784872Z stderr:
2019-12-08T23:11:50.5785077Z ------------------------------------------
2019-12-08T23:11:50.5785143Z error: lifetimes cannot start with a number
2019-12-08T23:11:50.5785539Z   --> /checkout/src/test/ui/parser/numeric-lifetime.rs:1:10
2019-12-08T23:11:50.5785579Z    |
2019-12-08T23:11:50.5785929Z LL | struct S<'1> { s: &'1 usize }
2019-12-08T23:11:50.5786150Z 
2019-12-08T23:11:50.5786184Z error: lifetimes cannot start with a number
2019-12-08T23:11:50.5786383Z   --> /checkout/src/test/ui/parser/numeric-lifetime.rs:1:20
2019-12-08T23:11:50.5786420Z    |
2019-12-08T23:11:50.5786420Z    |
2019-12-08T23:11:50.5786578Z LL | struct S<'1> { s: &'1 usize }
2019-12-08T23:11:50.5786651Z 
2019-12-08T23:11:50.5786684Z error[E0308]: mismatched types
2019-12-08T23:11:50.5786872Z   --> /checkout/src/test/ui/parser/numeric-lifetime.rs:6:20
2019-12-08T23:11:50.5786922Z    |
2019-12-08T23:11:50.5786922Z    |
2019-12-08T23:11:50.5786960Z LL |     let x: usize = "";
2019-12-08T23:11:50.5787114Z    |            -----   ^^
2019-12-08T23:11:50.5787161Z    |            |       |
2019-12-08T23:11:50.5787197Z    |            |       expected `usize`, found `&str`
2019-12-08T23:11:50.5787237Z    |            |       help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.5787404Z 
2019-12-08T23:11:50.5787437Z error: aborting due to 3 previous errors
2019-12-08T23:11:50.5787459Z 
2019-12-08T23:11:50.5787663Z For more information about this error, try `rustc --explain E0308`.
---
2019-12-08T23:11:50.5788584Z -    |            -----   ^^ expected `usize`, found `&str`
2019-12-08T23:11:50.5788748Z -    |            |
2019-12-08T23:11:50.5788904Z +    |            -----   ^^
2019-12-08T23:11:50.5788939Z +    |            |       |
2019-12-08T23:11:50.5788988Z +    |            |       expected `usize`, found `&str`
2019-12-08T23:11:50.5789028Z +    |            |       help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.5789112Z 14 
2019-12-08T23:11:50.5789147Z 15 error: aborting due to 2 previous errors
2019-12-08T23:11:50.5789170Z 
2019-12-08T23:11:50.5789190Z 
2019-12-08T23:11:50.5789190Z 
2019-12-08T23:11:50.5789236Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5789533Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-tuple/recover-tuple.stderr
2019-12-08T23:11:50.5789756Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5790058Z To only update this specific test, also pass `--test-args parser/recover-tuple.rs`
2019-12-08T23:11:50.5790124Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5790161Z status: exit code: 1
2019-12-08T23:11:50.5790161Z status: exit code: 1
2019-12-08T23:11:50.5790781Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/recover-tuple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-tuple" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-tuple/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5791047Z ------------------------------------------
2019-12-08T23:11:50.5791081Z 
2019-12-08T23:11:50.5791265Z ------------------------------------------
2019-12-08T23:11:50.5791316Z stderr:
2019-12-08T23:11:50.5791316Z stderr:
2019-12-08T23:11:50.5791495Z ------------------------------------------
2019-12-08T23:11:50.5791536Z error: expected expression, found `.`
2019-12-08T23:11:50.5791745Z   --> /checkout/src/test/ui/parser/recover-tuple.rs:3:40
2019-12-08T23:11:50.5791786Z    |
2019-12-08T23:11:50.5791824Z LL |     let x: (usize, usize, usize) = (3, .=.);
2019-12-08T23:11:50.5791905Z 
2019-12-08T23:11:50.5791940Z error[E0308]: mismatched types
2019-12-08T23:11:50.5792172Z   --> /checkout/src/test/ui/parser/recover-tuple.rs:6:20
2019-12-08T23:11:50.5792226Z    |
2019-12-08T23:11:50.5792226Z    |
2019-12-08T23:11:50.5792263Z LL |     let y: usize = ""; //~ ERROR mismatched types
2019-12-08T23:11:50.5792722Z    |            -----   ^^
2019-12-08T23:11:50.5792831Z    |            |       expected `usize`, found `&str`
2019-12-08T23:11:50.5792831Z    |            |       expected `usize`, found `&str`
2019-12-08T23:11:50.5792890Z    |            |       help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.5792981Z 
2019-12-08T23:11:50.5793023Z error: aborting due to 2 previous errors
2019-12-08T23:11:50.5793052Z 
2019-12-08T23:11:50.5793306Z For more information about this error, try `rustc --explain E0308`.
---
2019-12-08T23:11:50.5793895Z diff of stderr:
2019-12-08T23:11:50.5793922Z 
2019-12-08T23:11:50.5794138Z 2   --> $DIR/attribute-spans-preserved.rs:7:23
2019-12-08T23:11:50.5794201Z 3    |
2019-12-08T23:11:50.5794253Z 4 LL | #[ foo ( let y: u32 = "z"; ) ]
2019-12-08T23:11:50.5794487Z -    |                 ---   ^^^ expected `u32`, found `&str`
2019-12-08T23:11:50.5794910Z +    |                 ---   ^^^
2019-12-08T23:11:50.5794956Z +    |                 |     |
2019-12-08T23:11:50.5795015Z +    |                 |     expected `u32`, found `&str`
2019-12-08T23:11:50.5795015Z +    |                 |     expected `u32`, found `&str`
2019-12-08T23:11:50.5795068Z +    |                 |     help: consider dereferencing the borrow: `*"z"`
2019-12-08T23:11:50.5795177Z 8 
2019-12-08T23:11:50.5795219Z 9 error[E0308]: mismatched types
2019-12-08T23:11:50.5795248Z 
2019-12-08T23:11:50.5795469Z 10   --> $DIR/attribute-spans-preserved.rs:8:23
2019-12-08T23:11:50.5795469Z 10   --> $DIR/attribute-spans-preserved.rs:8:23
2019-12-08T23:11:50.5795530Z 11    |
2019-12-08T23:11:50.5795574Z 12 LL | #[ bar { let x: u32 = "y"; } ]
2019-12-08T23:11:50.5795806Z -    |                 ---   ^^^ expected `u32`, found `&str`
2019-12-08T23:11:50.5796553Z +    |                 ---   ^^^
2019-12-08T23:11:50.5796598Z +    |                 |     |
2019-12-08T23:11:50.5796690Z +    |                 |     expected `u32`, found `&str`
2019-12-08T23:11:50.5796690Z +    |                 |     expected `u32`, found `&str`
2019-12-08T23:11:50.5796730Z +    |                 |     help: consider dereferencing the borrow: `*"y"`
2019-12-08T23:11:50.5796802Z 16 
2019-12-08T23:11:50.5796847Z 17 error: aborting due to 2 previous errors
2019-12-08T23:11:50.5796870Z 
2019-12-08T23:11:50.5796890Z 
2019-12-08T23:11:50.5796890Z 
2019-12-08T23:11:50.5796924Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5797212Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-spans-preserved/attribute-spans-preserved.stderr
2019-12-08T23:11:50.5797411Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5797643Z To only update this specific test, also pass `--test-args proc-macro/attribute-spans-preserved.rs`
2019-12-08T23:11:50.5797706Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5797746Z status: exit code: 1
2019-12-08T23:11:50.5797746Z status: exit code: 1
2019-12-08T23:11:50.5798384Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/attribute-spans-preserved.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-spans-preserved" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-spans-preserved/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5798641Z ------------------------------------------
2019-12-08T23:11:50.5798641Z ------------------------------------------
2019-12-08T23:11:50.5798682Z fn main () { let y : u32 = "z" ; { let x : u32 = "y" ; } }
2019-12-08T23:11:50.5798895Z ------------------------------------------
2019-12-08T23:11:50.5798931Z stderr:
2019-12-08T23:11:50.5799103Z ------------------------------------------
2019-12-08T23:11:50.5799150Z error[E0308]: mismatched types
2019-12-08T23:11:50.5799150Z error[E0308]: mismatched types
2019-12-08T23:11:50.5799344Z   --> /checkout/src/test/ui/proc-macro/attribute-spans-preserved.rs:7:23
2019-12-08T23:11:50.5799381Z    |
2019-12-08T23:11:50.5799428Z LL | #[ foo ( let y: u32 = "z"; ) ] //~ ERROR: mismatched types
2019-12-08T23:11:50.5799590Z    |                 ---   ^^^
2019-12-08T23:11:50.5799663Z    |                 |     expected `u32`, found `&str`
2019-12-08T23:11:50.5799663Z    |                 |     expected `u32`, found `&str`
2019-12-08T23:11:50.5799712Z    |                 |     help: consider dereferencing the borrow: `*"z"`
2019-12-08T23:11:50.5799773Z 
2019-12-08T23:11:50.5799813Z error[E0308]: mismatched types
2019-12-08T23:11:50.5800017Z   --> /checkout/src/test/ui/proc-macro/attribute-spans-preserved.rs:8:23
2019-12-08T23:11:50.5800055Z    |
2019-12-08T23:11:50.5800055Z    |
2019-12-08T23:11:50.5800105Z LL | #[ bar { let x: u32 = "y"; } ] //~ ERROR: mismatched types
2019-12-08T23:11:50.5800273Z    |                 ---   ^^^
2019-12-08T23:11:50.5800346Z    |                 |     expected `u32`, found `&str`
2019-12-08T23:11:50.5800346Z    |                 |     expected `u32`, found `&str`
2019-12-08T23:11:50.5800403Z    |                 |     help: consider dereferencing the borrow: `*"y"`
2019-12-08T23:11:50.5800463Z 
2019-12-08T23:11:50.5800512Z error: aborting due to 2 previous errors
2019-12-08T23:11:50.5800534Z 
2019-12-08T23:11:50.5800725Z For more information about this error, try `rustc --explain E0308`.
---
2019-12-08T23:11:50.5801264Z diff of stderr:
2019-12-08T23:11:50.5801293Z 
2019-12-08T23:11:50.5801481Z 2   --> $DIR/attribute-with-error.rs:10:18
2019-12-08T23:11:50.5801573Z 3    |
2019-12-08T23:11:50.5801620Z 4 LL |     let a: i32 = "foo";
2019-12-08T23:11:50.5802028Z -    |            ---   ^^^^^ expected `i32`, found `&str`
2019-12-08T23:11:50.5802369Z +    |            ---   ^^^^^
2019-12-08T23:11:50.5802455Z +    |            |     |
2019-12-08T23:11:50.5802455Z +    |            |     |
2019-12-08T23:11:50.5802495Z +    |            |     expected `i32`, found `&str`
2019-12-08T23:11:50.5802538Z +    |            |     help: consider dereferencing the borrow: `*"foo"`
2019-12-08T23:11:50.5802861Z 8 
2019-12-08T23:11:50.5802905Z 9 error[E0308]: mismatched types
2019-12-08T23:11:50.5803026Z 
2019-12-08T23:11:50.5803247Z 10   --> $DIR/attribute-with-error.rs:12:18
2019-12-08T23:11:50.5803247Z 10   --> $DIR/attribute-with-error.rs:12:18
2019-12-08T23:11:50.5803293Z 11    |
2019-12-08T23:11:50.5803491Z 12 LL |     let b: i32 = "f'oo";
2019-12-08T23:11:50.5803747Z -    |            ---   ^^^^^^ expected `i32`, found `&str`
2019-12-08T23:11:50.5804142Z +    |            ---   ^^^^^^
2019-12-08T23:11:50.5807315Z +    |            |     |
2019-12-08T23:11:50.5807315Z +    |            |     |
2019-12-08T23:11:50.5807363Z +    |            |     expected `i32`, found `&str`
2019-12-08T23:11:50.5807858Z +    |            |     help: consider dereferencing the borrow: `*"f'oo"`
2019-12-08T23:11:50.5807946Z 16 
2019-12-08T23:11:50.5807980Z 17 error[E0308]: mismatched types
2019-12-08T23:11:50.5808003Z 
2019-12-08T23:11:50.5808194Z 18   --> $DIR/attribute-with-error.rs:25:22
2019-12-08T23:11:50.5808194Z 18   --> $DIR/attribute-with-error.rs:25:22
2019-12-08T23:11:50.5808230Z 19    |
2019-12-08T23:11:50.5808262Z 20 LL |         let a: i32 = "foo";
2019-12-08T23:11:50.5809286Z -    |                ---   ^^^^^ expected `i32`, found `&str`
2019-12-08T23:11:50.5809650Z +    |                ---   ^^^^^
2019-12-08T23:11:50.5809720Z +    |                |     |
2019-12-08T23:11:50.5809720Z +    |                |     |
2019-12-08T23:11:50.5809761Z +    |                |     expected `i32`, found `&str`
2019-12-08T23:11:50.5809809Z +    |                |     help: consider dereferencing the borrow: `*"foo"`
2019-12-08T23:11:50.5809902Z 24 
2019-12-08T23:11:50.5809938Z 25 error[E0308]: mismatched types
2019-12-08T23:11:50.5809963Z 
2019-12-08T23:11:50.5810162Z 26   --> $DIR/attribute-with-error.rs:35:22
2019-12-08T23:11:50.5810162Z 26   --> $DIR/attribute-with-error.rs:35:22
2019-12-08T23:11:50.5810199Z 27    |
2019-12-08T23:11:50.5810234Z 28 LL |         let a: i32 = "foo";
2019-12-08T23:11:50.5810437Z -    |                ---   ^^^^^ expected `i32`, found `&str`
2019-12-08T23:11:50.5810939Z +    |                ---   ^^^^^
2019-12-08T23:11:50.5810977Z +    |                |     |
2019-12-08T23:11:50.5810977Z +    |                |     |
2019-12-08T23:11:50.5811032Z +    |                |     expected `i32`, found `&str`
2019-12-08T23:11:50.5811081Z +    |                |     help: consider dereferencing the borrow: `*"foo"`
2019-12-08T23:11:50.5811177Z 32 
2019-12-08T23:11:50.5811214Z 33 error: aborting due to 4 previous errors
2019-12-08T23:11:50.5811238Z 
2019-12-08T23:11:50.5811258Z 
2019-12-08T23:11:50.5811258Z 
2019-12-08T23:11:50.5811309Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5811570Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-with-error/attribute-with-error.stderr
2019-12-08T23:11:50.5811945Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5813103Z To only update this specific test, also pass `--test-args proc-macro/attribute-with-error.rs`
2019-12-08T23:11:50.5813193Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5813257Z status: exit code: 1
2019-12-08T23:11:50.5813257Z status: exit code: 1
2019-12-08T23:11:50.5814231Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/attribute-with-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-with-error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-with-error/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5814672Z ------------------------------------------
2019-12-08T23:11:50.5814707Z 
2019-12-08T23:11:50.5814934Z ------------------------------------------
2019-12-08T23:11:50.5814980Z stderr:
2019-12-08T23:11:50.5814980Z stderr:
2019-12-08T23:11:50.5815188Z ------------------------------------------
2019-12-08T23:11:50.5815235Z error[E0308]: mismatched types
2019-12-08T23:11:50.5815492Z   --> /checkout/src/test/ui/proc-macro/attribute-with-error.rs:10:18
2019-12-08T23:11:50.5815542Z    |
2019-12-08T23:11:50.5815596Z LL |     let a: i32 = "foo";
2019-12-08T23:11:50.5815967Z    |            ---   ^^^^^
2019-12-08T23:11:50.5816169Z    |            |     |
2019-12-08T23:11:50.5816214Z    |            |     expected `i32`, found `&str`
2019-12-08T23:11:50.5816267Z    |            |     help: consider dereferencing the borrow: `*"foo"`
2019-12-08T23:11:50.5816332Z 
2019-12-08T23:11:50.5816366Z error[E0308]: mismatched types
2019-12-08T23:11:50.5816585Z   --> /checkout/src/test/ui/proc-macro/attribute-with-error.rs:12:18
2019-12-08T23:11:50.5816624Z    |
2019-12-08T23:11:50.5816624Z    |
2019-12-08T23:11:50.5816788Z LL |     let b: i32 = "f'oo";
2019-12-08T23:11:50.5816967Z    |            ---   ^^^^^^
2019-12-08T23:11:50.5817005Z    |            |     |
2019-12-08T23:11:50.5817042Z    |            |     expected `i32`, found `&str`
2019-12-08T23:11:50.5817253Z    |            |     help: consider dereferencing the borrow: `*"f'oo"`
2019-12-08T23:11:50.5817326Z 
2019-12-08T23:11:50.5817361Z error[E0308]: mismatched types
2019-12-08T23:11:50.5817740Z   --> /checkout/src/test/ui/proc-macro/attribute-with-error.rs:25:22
2019-12-08T23:11:50.5817778Z    |
2019-12-08T23:11:50.5817778Z    |
2019-12-08T23:11:50.5817810Z LL |         let a: i32 = "foo";
2019-12-08T23:11:50.5817980Z    |                ---   ^^^^^
2019-12-08T23:11:50.5818190Z    |                |     |
2019-12-08T23:11:50.5818228Z    |                |     expected `i32`, found `&str`
2019-12-08T23:11:50.5818280Z    |                |     help: consider dereferencing the borrow: `*"foo"`
2019-12-08T23:11:50.5818344Z 
2019-12-08T23:11:50.5818377Z error[E0308]: mismatched types
2019-12-08T23:11:50.5818759Z   --> /checkout/src/test/ui/proc-macro/attribute-with-error.rs:35:22
2019-12-08T23:11:50.5818799Z    |
2019-12-08T23:11:50.5818799Z    |
2019-12-08T23:11:50.5818833Z LL |         let a: i32 = "foo";
2019-12-08T23:11:50.5819017Z    |                ---   ^^^^^
2019-12-08T23:11:50.5819063Z    |                |     |
2019-12-08T23:11:50.5819102Z    |                |     expected `i32`, found `&str`
2019-12-08T23:11:50.5819151Z    |                |     help: consider dereferencing the borrow: `*"foo"`
2019-12-08T23:11:50.5819232Z 
2019-12-08T23:11:50.5819268Z error: aborting due to 4 previous errors
2019-12-08T23:11:50.5819291Z 
2019-12-08T23:11:50.5819687Z For more information about this error, try `rustc --explain E0308`.
---
2019-12-08T23:11:50.5820500Z diff of stderr:
2019-12-08T23:11:50.5820523Z 
2019-12-08T23:11:50.5820708Z 2   --> $DIR/nested-item-spans.rs:9:22
2019-12-08T23:11:50.5820745Z 3    |
2019-12-08T23:11:50.5820852Z 4 LL |         let x: u32 = "x";
2019-12-08T23:11:50.5821086Z -    |                ---   ^^^ expected `u32`, found `&str`
2019-12-08T23:11:50.5821496Z +    |                ---   ^^^
2019-12-08T23:11:50.5821550Z +    |                |     |
2019-12-08T23:11:50.5821589Z +    |                |     expected `u32`, found `&str`
2019-12-08T23:11:50.5821589Z +    |                |     expected `u32`, found `&str`
2019-12-08T23:11:50.5821791Z +    |                |     help: consider dereferencing the borrow: `*"x"`
2019-12-08T23:11:50.5822581Z 8 
2019-12-08T23:11:50.5822625Z 9 error[E0308]: mismatched types
2019-12-08T23:11:50.5822654Z 
2019-12-08T23:11:50.5822920Z 10   --> $DIR/nested-item-spans.rs:18:22
2019-12-08T23:11:50.5822920Z 10   --> $DIR/nested-item-spans.rs:18:22
2019-12-08T23:11:50.5822966Z 11    |
2019-12-08T23:11:50.5823008Z 12 LL |         let x: u32 = "x";
2019-12-08T23:11:50.5823244Z -    |                ---   ^^^ expected `u32`, found `&str`
2019-12-08T23:11:50.5823661Z +    |                ---   ^^^
2019-12-08T23:11:50.5823707Z +    |                |     |
2019-12-08T23:11:50.5823768Z +    |                |     expected `u32`, found `&str`
2019-12-08T23:11:50.5823768Z +    |                |     expected `u32`, found `&str`
2019-12-08T23:11:50.5823827Z +    |                |     help: consider dereferencing the borrow: `*"x"`
2019-12-08T23:11:50.5823935Z 16 
2019-12-08T23:11:50.5823978Z 17 error: aborting due to 2 previous errors
2019-12-08T23:11:50.5824007Z 
2019-12-08T23:11:50.5824033Z 
2019-12-08T23:11:50.5824033Z 
2019-12-08T23:11:50.5824094Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5824406Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nested-item-spans/nested-item-spans.stderr
2019-12-08T23:11:50.5824651Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5824934Z To only update this specific test, also pass `--test-args proc-macro/nested-item-spans.rs`
2019-12-08T23:11:50.5825020Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5825077Z status: exit code: 1
2019-12-08T23:11:50.5825077Z status: exit code: 1
2019-12-08T23:11:50.5825985Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/nested-item-spans.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nested-item-spans" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nested-item-spans/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5826250Z ------------------------------------------
2019-12-08T23:11:50.5826277Z 
2019-12-08T23:11:50.5826452Z ------------------------------------------
2019-12-08T23:11:50.5826499Z stderr:
2019-12-08T23:11:50.5826499Z stderr:
2019-12-08T23:11:50.5826670Z ------------------------------------------
2019-12-08T23:11:50.5826714Z error[E0308]: mismatched types
2019-12-08T23:11:50.5826923Z   --> /checkout/src/test/ui/proc-macro/nested-item-spans.rs:9:22
2019-12-08T23:11:50.5827125Z    |
2019-12-08T23:11:50.5827163Z LL |         let x: u32 = "x"; //~ ERROR: mismatched types
2019-12-08T23:11:50.5827340Z    |                ---   ^^^
2019-12-08T23:11:50.5827414Z    |                |     expected `u32`, found `&str`
2019-12-08T23:11:50.5827414Z    |                |     expected `u32`, found `&str`
2019-12-08T23:11:50.5827471Z    |                |     help: consider dereferencing the borrow: `*"x"`
2019-12-08T23:11:50.5827532Z 
2019-12-08T23:11:50.5827565Z error[E0308]: mismatched types
2019-12-08T23:11:50.5827769Z   --> /checkout/src/test/ui/proc-macro/nested-item-spans.rs:18:22
2019-12-08T23:11:50.5827806Z    |
2019-12-08T23:11:50.5827806Z    |
2019-12-08T23:11:50.5827842Z LL |         let x: u32 = "x"; //~ ERROR: mismatched types
2019-12-08T23:11:50.5828015Z    |                ---   ^^^
2019-12-08T23:11:50.5828205Z    |                |     expected `u32`, found `&str`
2019-12-08T23:11:50.5828205Z    |                |     expected `u32`, found `&str`
2019-12-08T23:11:50.5828310Z    |                |     help: consider dereferencing the borrow: `*"x"`
2019-12-08T23:11:50.5828370Z 
2019-12-08T23:11:50.5828404Z error: aborting due to 2 previous errors
2019-12-08T23:11:50.5828442Z 
2019-12-08T23:11:50.5828657Z For more information about this error, try `rustc --explain E0308`.
---
2019-12-08T23:11:50.5829607Z -    |            -----   ^^^^^^^ expected `usize`, found `&str`
2019-12-08T23:11:50.5829759Z -    |            |
2019-12-08T23:11:50.5829927Z +    |            -----   ^^^^^^^
2019-12-08T23:11:50.5829978Z +    |            |       |
2019-12-08T23:11:50.5830015Z +    |            |       expected `usize`, found `&str`
2019-12-08T23:11:50.5830057Z +    |            |       help: consider dereferencing the borrow: `*"hello"`
2019-12-08T23:11:50.5830142Z 8 
2019-12-08T23:11:50.5830177Z 9 error[E0308]: mismatched types
2019-12-08T23:11:50.5830199Z 
2019-12-08T23:11:50.5830236Z 
2019-12-08T23:11:50.5830236Z 
2019-12-08T23:11:50.5830271Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5830516Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/span-preservation/span-preservation.stderr
2019-12-08T23:11:50.5830727Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5830944Z To only update this specific test, also pass `--test-args proc-macro/span-preservation.rs`
2019-12-08T23:11:50.5831013Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5831059Z status: exit code: 1
2019-12-08T23:11:50.5831059Z status: exit code: 1
2019-12-08T23:11:50.5831665Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/span-preservation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/span-preservation" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/span-preservation/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5831915Z ------------------------------------------
2019-12-08T23:11:50.5831941Z 
2019-12-08T23:11:50.5832513Z ------------------------------------------
2019-12-08T23:11:50.5832564Z stderr:
2019-12-08T23:11:50.5832564Z stderr:
2019-12-08T23:11:50.5832774Z ------------------------------------------
2019-12-08T23:11:50.5832844Z error[E0308]: mismatched types
2019-12-08T23:11:50.5833082Z   --> /checkout/src/test/ui/proc-macro/span-preservation.rs:11:20
2019-12-08T23:11:50.5833130Z    |
2019-12-08T23:11:50.5833194Z LL |     let x: usize = "hello"; //~ ERROR mismatched types
2019-12-08T23:11:50.5833397Z    |            -----   ^^^^^^^
2019-12-08T23:11:50.5833505Z    |            |       expected `usize`, found `&str`
2019-12-08T23:11:50.5833505Z    |            |       expected `usize`, found `&str`
2019-12-08T23:11:50.5833558Z    |            |       help: consider dereferencing the borrow: `*"hello"`
2019-12-08T23:11:50.5833635Z 
2019-12-08T23:11:50.5833693Z error[E0308]: mismatched types
2019-12-08T23:11:50.5833932Z   --> /checkout/src/test/ui/proc-macro/span-preservation.rs:17:29
2019-12-08T23:11:50.5834074Z    |
2019-12-08T23:11:50.5834074Z    |
2019-12-08T23:11:50.5834402Z LL | fn b(x: Option<isize>) -> usize {
2019-12-08T23:11:50.5834686Z    |                           ----- expected `usize` because of return type
2019-12-08T23:11:50.5834812Z LL |     match x {
2019-12-08T23:11:50.5834875Z LL |         Some(x) => { return x }, //~ ERROR mismatched types
2019-12-08T23:11:50.5834927Z    |                             ^ expected `usize`, found `isize`
2019-12-08T23:11:50.5835246Z help: you can convert an `isize` to `usize` and panic if the converted value wouldn't fit
2019-12-08T23:11:50.5835312Z    |
2019-12-08T23:11:50.5835312Z    |
2019-12-08T23:11:50.5835364Z LL |         Some(x) => { return x.try_into().unwrap() }, //~ ERROR mismatched types
2019-12-08T23:11:50.5835458Z 
2019-12-08T23:11:50.5835500Z error[E0308]: mismatched types
2019-12-08T23:11:50.5835899Z   --> /checkout/src/test/ui/proc-macro/span-preservation.rs:33:22
2019-12-08T23:11:50.5835951Z    |
2019-12-08T23:11:50.5835951Z    |
2019-12-08T23:11:50.5835995Z LL |     let x = Foo { a: 10isize }; //~ ERROR mismatched types
2019-12-08T23:11:50.5836037Z    |                      ^^^^^^^ expected `usize`, found `isize`
2019-12-08T23:11:50.5836068Z 
2019-12-08T23:11:50.5836118Z error[E0560]: struct `c::Foo` has no field named `b`
2019-12-08T23:11:50.5836355Z    |
2019-12-08T23:11:50.5836355Z    |
2019-12-08T23:11:50.5836404Z LL |     let y = Foo { a: 10, b: 10isize }; //~ ERROR has no field named `b`
2019-12-08T23:11:50.5836446Z    |                          ^ `c::Foo` does not have this field
2019-12-08T23:11:50.5836527Z    = note: available fields are: `a`
2019-12-08T23:11:50.5836550Z 
2019-12-08T23:11:50.5836583Z error[E0308]: mismatched types
2019-12-08T23:11:50.5836780Z   --> /checkout/src/test/ui/proc-macro/span-preservation.rs:39:5
---
2019-12-08T23:11:50.5837790Z 
2019-12-08T23:11:50.5837839Z error[E0308]: mismatched types
2019-12-08T23:11:50.5838033Z   --> /checkout/src/test/ui/proc-macro/span-preservation.rs:49:5
2019-12-08T23:11:50.5838070Z    |
2019-12-08T23:11:50.5838121Z LL | extern "Rust" fn rust_abi() {
2019-12-08T23:11:50.5838323Z    |                             - possibly return type missing here?
2019-12-08T23:11:50.5838419Z    |     ^ expected `()`, found integer
2019-12-08T23:11:50.5838442Z 
2019-12-08T23:11:50.5838474Z error[E0308]: mismatched types
2019-12-08T23:11:50.5838669Z   --> /checkout/src/test/ui/proc-macro/span-preservation.rs:54:5
2019-12-08T23:11:50.5838669Z   --> /checkout/src/test/ui/proc-macro/span-preservation.rs:54:5
2019-12-08T23:11:50.5838720Z    |
2019-12-08T23:11:50.5838755Z LL | extern "\x43" fn c_abi_escaped() {
2019-12-08T23:11:50.5838955Z    |                                  - possibly return type missing here?
2019-12-08T23:11:50.5839046Z    |     ^ expected `()`, found integer
2019-12-08T23:11:50.5839069Z 
2019-12-08T23:11:50.5839103Z error: aborting due to 8 previous errors
2019-12-08T23:11:50.5839141Z 
---
2019-12-08T23:11:50.5840067Z diff of stderr:
2019-12-08T23:11:50.5840090Z 
2019-12-08T23:11:50.5840260Z 13   --> $DIR/ptr-coercion.rs:13:25
2019-12-08T23:11:50.5840295Z 14    |
2019-12-08T23:11:50.5840344Z 15 LL |     let x: *mut isize = &42;
2019-12-08T23:11:50.5840533Z -    |            ----------   ^^^ types differ in mutability
2019-12-08T23:11:50.5840869Z +    |            ----------   ^^^
2019-12-08T23:11:50.5840907Z +    |            |            |
2019-12-08T23:11:50.5840946Z +    |            |            types differ in mutability
2019-12-08T23:11:50.5841003Z +    |            |            help: consider dereferencing the borrow: `*&42`
2019-12-08T23:11:50.5841003Z +    |            |            help: consider dereferencing the borrow: `*&42`
2019-12-08T23:11:50.5841042Z 18    |            expected due to this
2019-12-08T23:11:50.5841083Z 19    |
2019-12-08T23:11:50.5841136Z 20    = note: expected raw pointer `*mut isize`
2019-12-08T23:11:50.5841161Z 
2019-12-08T23:11:50.5841187Z 
2019-12-08T23:11:50.5841223Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5841477Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ptr-coercion/ptr-coercion.stderr
2019-12-08T23:11:50.5841678Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5841879Z To only update this specific test, also pass `--test-args ptr-coercion.rs`
2019-12-08T23:11:50.5841977Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5842013Z status: exit code: 1
2019-12-08T23:11:50.5842013Z status: exit code: 1
2019-12-08T23:11:50.5843084Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/ptr-coercion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ptr-coercion" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ptr-coercion/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5843421Z ------------------------------------------
2019-12-08T23:11:50.5843472Z 
2019-12-08T23:11:50.5843683Z ------------------------------------------
2019-12-08T23:11:50.5843727Z stderr:
2019-12-08T23:11:50.5843727Z stderr:
2019-12-08T23:11:50.5843950Z ------------------------------------------
2019-12-08T23:11:50.5843998Z error[E0308]: mismatched types
2019-12-08T23:11:50.5844220Z   --> /checkout/src/test/ui/ptr-coercion.rs:7:25
2019-12-08T23:11:50.5844265Z    |
2019-12-08T23:11:50.5844329Z LL |     let x: *mut isize = x; //~  ERROR mismatched types
2019-12-08T23:11:50.5844560Z    |            ----------   ^ types differ in mutability
2019-12-08T23:11:50.5844679Z    |            expected due to this
2019-12-08T23:11:50.5844721Z    |
2019-12-08T23:11:50.5844764Z    = note: expected raw pointer `*mut isize`
2019-12-08T23:11:50.5844764Z    = note: expected raw pointer `*mut isize`
2019-12-08T23:11:50.5844836Z               found raw pointer `*const isize`
2019-12-08T23:11:50.5844906Z error[E0308]: mismatched types
2019-12-08T23:11:50.5845132Z   --> /checkout/src/test/ui/ptr-coercion.rs:13:25
2019-12-08T23:11:50.5845194Z    |
2019-12-08T23:11:50.5845194Z    |
2019-12-08T23:11:50.5845240Z LL |     let x: *mut isize = &42; //~  ERROR mismatched types
2019-12-08T23:11:50.5845442Z    |            ----------   ^^^
2019-12-08T23:11:50.5845550Z    |            |            types differ in mutability
2019-12-08T23:11:50.5845601Z    |            |            help: consider dereferencing the borrow: `*&42`
2019-12-08T23:11:50.5845666Z    |            expected due to this
2019-12-08T23:11:50.5845707Z    |
2019-12-08T23:11:50.5845707Z    |
2019-12-08T23:11:50.5845908Z    = note: expected raw pointer `*mut isize`
2019-12-08T23:11:50.5846049Z                 found reference `&isize`
2019-12-08T23:11:50.5846082Z 
2019-12-08T23:11:50.5846116Z error[E0308]: mismatched types
2019-12-08T23:11:50.5846543Z   --> /checkout/src/test/ui/ptr-coercion.rs:19:25
2019-12-08T23:11:50.5846595Z    |
2019-12-08T23:11:50.5846631Z LL |     let x: *mut isize = x; //~  ERROR mismatched types
2019-12-08T23:11:50.5846815Z    |            ----------   ^ types differ in mutability
2019-12-08T23:11:50.5846904Z    |            expected due to this
2019-12-08T23:11:50.5846936Z    |
2019-12-08T23:11:50.5846970Z    = note: expected raw pointer `*mut isize`
2019-12-08T23:11:50.5846970Z    = note: expected raw pointer `*mut isize`
2019-12-08T23:11:50.5847024Z               found raw pointer `*const isize`
2019-12-08T23:11:50.5847080Z error: aborting due to 3 previous errors
2019-12-08T23:11:50.5847102Z 
2019-12-08T23:11:50.5847311Z For more information about this error, try `rustc --explain E0308`.
2019-12-08T23:11:50.5847338Z 
---
2019-12-08T23:11:50.5847818Z 
2019-12-08T23:11:50.5848015Z - error[E0109]: type arguments are not allowed for this type
2019-12-08T23:11:50.5848191Z -   --> $DIR/qualified-path-params-2.rs:18:26
2019-12-08T23:11:50.5848542Z -    |
2019-12-08T23:11:50.5848912Z - LL | type A = <S as Tr>::A::f<u8>;
2019-12-08T23:11:50.5849111Z -    |                          ^^ type argument not allowed
2019-12-08T23:11:50.5849467Z 7 error[E0223]: ambiguous associated type
2019-12-08T23:11:50.5849840Z 8   --> $DIR/qualified-path-params-2.rs:18:10
2019-12-08T23:11:50.5849877Z 9    |
2019-12-08T23:11:50.5849900Z 
2019-12-08T23:11:50.5849900Z 
2019-12-08T23:11:50.5849952Z 10 LL | type A = <S as Tr>::A::f<u8>;
2019-12-08T23:11:50.5850190Z 11    |          ^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<<S as Tr>::A as Trait>::f`
2019-12-08T23:11:50.5850431Z - error: aborting due to 2 previous errors
2019-12-08T23:11:50.5850478Z + error: aborting due to previous error
2019-12-08T23:11:50.5850513Z 14 
2019-12-08T23:11:50.5850704Z - Some errors have detailed explanations: E0109, E0223.
2019-12-08T23:11:50.5850704Z - Some errors have detailed explanations: E0109, E0223.
2019-12-08T23:11:50.5850927Z - For more information about an error, try `rustc --explain E0109`.
2019-12-08T23:11:50.5851131Z + For more information about this error, try `rustc --explain E0223`.
2019-12-08T23:11:50.5851330Z 17 
2019-12-08T23:11:50.5851368Z 
2019-12-08T23:11:50.5851389Z 
2019-12-08T23:11:50.5851425Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5851684Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/qualified/qualified-path-params-2/qualified-path-params-2.stderr
2019-12-08T23:11:50.5852094Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5853405Z To only update this specific test, also pass `--test-args qualified/qualified-path-params-2.rs`
2019-12-08T23:11:50.5853519Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5853570Z status: exit code: 1
2019-12-08T23:11:50.5853570Z status: exit code: 1
2019-12-08T23:11:50.5854411Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/qualified/qualified-path-params-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/qualified/qualified-path-params-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/qualified/qualified-path-params-2/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5854733Z ------------------------------------------
2019-12-08T23:11:50.5854785Z 
2019-12-08T23:11:50.5854999Z ------------------------------------------
2019-12-08T23:11:50.5855159Z stderr:
2019-12-08T23:11:50.5855159Z stderr:
2019-12-08T23:11:50.5855421Z ------------------------------------------
2019-12-08T23:11:50.5855549Z error[E0223]: ambiguous associated type
2019-12-08T23:11:50.5855811Z   --> /checkout/src/test/ui/qualified/qualified-path-params-2.rs:18:10
2019-12-08T23:11:50.5855879Z    |
2019-12-08T23:11:50.5855926Z LL | type A = <S as Tr>::A::f<u8>;
2019-12-08T23:11:50.5856195Z    |          ^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<<S as Tr>::A as Trait>::f`
2019-12-08T23:11:50.5856297Z error: aborting due to previous error
2019-12-08T23:11:50.5856491Z 
2019-12-08T23:11:50.5856687Z For more information about this error, try `rustc --explain E0223`.
2019-12-08T23:11:50.5856715Z 
---
2019-12-08T23:11:50.5857169Z diff of stderr:
2019-12-08T23:11:50.5857199Z 
2019-12-08T23:11:50.5857366Z 26   --> $DIR/repeat_count.rs:16:17
2019-12-08T23:11:50.5857419Z 27    |
2019-12-08T23:11:50.5857460Z 28 LL |     let e = [0; "foo"];
2019-12-08T23:11:50.5857650Z -    |                 ^^^^^ expected `usize`, found `&str`
2019-12-08T23:11:50.5857742Z +    |                 |
2019-12-08T23:11:50.5857780Z +    |                 expected `usize`, found `&str`
2019-12-08T23:11:50.5857780Z +    |                 expected `usize`, found `&str`
2019-12-08T23:11:50.5857822Z +    |                 help: consider dereferencing the borrow: `*"foo"`
2019-12-08T23:11:50.5857909Z 31 error[E0308]: mismatched types
2019-12-08T23:11:50.5858081Z 32   --> $DIR/repeat_count.rs:19:17
2019-12-08T23:11:50.5858106Z 
2019-12-08T23:11:50.5858143Z 
2019-12-08T23:11:50.5858143Z 
2019-12-08T23:11:50.5858180Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5858411Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repeat_count/repeat_count.stderr
2019-12-08T23:11:50.5858629Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5858833Z To only update this specific test, also pass `--test-args repeat_count.rs`
2019-12-08T23:11:50.5858902Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5858953Z status: exit code: 1
2019-12-08T23:11:50.5858953Z status: exit code: 1
2019-12-08T23:11:50.5859541Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/repeat_count.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repeat_count" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repeat_count/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5859801Z ------------------------------------------
2019-12-08T23:11:50.5859834Z 
2019-12-08T23:11:50.5860022Z ------------------------------------------
2019-12-08T23:11:50.5860058Z stderr:
2019-12-08T23:11:50.5860058Z stderr:
2019-12-08T23:11:50.5860232Z ------------------------------------------
2019-12-08T23:11:50.5860440Z error[E0435]: attempt to use a non-constant value in a constant
2019-12-08T23:11:50.5860628Z   --> /checkout/src/test/ui/repeat_count.rs:5:17
2019-12-08T23:11:50.5860666Z    |
2019-12-08T23:11:50.5860717Z LL |     let a = [0; n];
2019-12-08T23:11:50.5860889Z    |                 ^ non-constant value
2019-12-08T23:11:50.5860949Z error[E0308]: mismatched types
2019-12-08T23:11:50.5861143Z   --> /checkout/src/test/ui/repeat_count.rs:7:17
2019-12-08T23:11:50.5861180Z    |
2019-12-08T23:11:50.5861180Z    |
2019-12-08T23:11:50.5861214Z LL |     let b = [0; ()];
2019-12-08T23:11:50.5861268Z    |                 ^^ expected `usize`, found `()`
2019-12-08T23:11:50.5861326Z error[E0308]: mismatched types
2019-12-08T23:11:50.5861509Z   --> /checkout/src/test/ui/repeat_count.rs:10:17
2019-12-08T23:11:50.5862171Z    |
2019-12-08T23:11:50.5862171Z    |
2019-12-08T23:11:50.5862229Z LL |     let c = [0; true];
2019-12-08T23:11:50.5862276Z    |                 ^^^^ expected `usize`, found `bool`
2019-12-08T23:11:50.5862426Z error[E0308]: mismatched types
2019-12-08T23:11:50.5862710Z   --> /checkout/src/test/ui/repeat_count.rs:13:17
2019-12-08T23:11:50.5862756Z    |
2019-12-08T23:11:50.5862756Z    |
2019-12-08T23:11:50.5862816Z LL |     let d = [0; 0.5];
2019-12-08T23:11:50.5863058Z    |                 ^^^ expected `usize`, found floating-point number
2019-12-08T23:11:50.5863133Z error[E0308]: mismatched types
2019-12-08T23:11:50.5863375Z   --> /checkout/src/test/ui/repeat_count.rs:16:17
2019-12-08T23:11:50.5863420Z    |
2019-12-08T23:11:50.5863420Z    |
2019-12-08T23:11:50.5863461Z LL |     let e = [0; "foo"];
2019-12-08T23:11:50.5863565Z    |                 |
2019-12-08T23:11:50.5863611Z    |                 expected `usize`, found `&str`
2019-12-08T23:11:50.5863611Z    |                 expected `usize`, found `&str`
2019-12-08T23:11:50.5863688Z    |                 help: consider dereferencing the borrow: `*"foo"`
2019-12-08T23:11:50.5863761Z error[E0308]: mismatched types
2019-12-08T23:11:50.5863995Z   --> /checkout/src/test/ui/repeat_count.rs:19:17
2019-12-08T23:11:50.5864057Z    |
2019-12-08T23:11:50.5864057Z    |
2019-12-08T23:11:50.5864257Z LL |     let f = [0; -4_isize];
2019-12-08T23:11:50.5864307Z    |                 ^^^^^^^^ expected `usize`, found `isize`
2019-12-08T23:11:50.5864625Z help: you can convert an `isize` to `usize` and panic if the converted value wouldn't fit
2019-12-08T23:11:50.5864675Z    |
2019-12-08T23:11:50.5864675Z    |
2019-12-08T23:11:50.5864916Z LL |     let f = [0; (-4_isize).try_into().unwrap()];
2019-12-08T23:11:50.5864996Z 
2019-12-08T23:11:50.5865037Z error[E0308]: mismatched types
2019-12-08T23:11:50.5865431Z   --> /checkout/src/test/ui/repeat_count.rs:22:23
2019-12-08T23:11:50.5865468Z    |
2019-12-08T23:11:50.5865468Z    |
2019-12-08T23:11:50.5865641Z LL |     let f = [0_usize; -1_isize];
2019-12-08T23:11:50.5865700Z    |                       ^^^^^^^^ expected `usize`, found `isize`
2019-12-08T23:11:50.5865953Z help: you can convert an `isize` to `usize` and panic if the converted value wouldn't fit
2019-12-08T23:11:50.5865992Z    |
2019-12-08T23:11:50.5865992Z    |
2019-12-08T23:11:50.5866195Z LL |     let f = [0_usize; (-1_isize).try_into().unwrap()];
2019-12-08T23:11:50.5866262Z 
2019-12-08T23:11:50.5866312Z error[E0308]: mismatched types
2019-12-08T23:11:50.5866494Z   --> /checkout/src/test/ui/repeat_count.rs:28:17
2019-12-08T23:11:50.5866531Z    |
2019-12-08T23:11:50.5866531Z    |
2019-12-08T23:11:50.5866581Z LL |     let g = [0; G { g: () }];
2019-12-08T23:11:50.5866622Z    |                 ^^^^^^^^^^^ expected `usize`, found struct `main::G`
2019-12-08T23:11:50.5866682Z error: aborting due to 8 previous errors
2019-12-08T23:11:50.5866720Z 
2019-12-08T23:11:50.5866764Z Some errors have detailed explanations: E0308, E0435.
2019-12-08T23:11:50.5866965Z For more information about an error, try `rustc --explain E0308`.
2019-12-08T23:11:50.5866965Z For more information about an error, try `rustc --explain E0308`.
2019-12-08T23:11:50.5866999Z 
2019-12-08T23:11:50.5867185Z ------------------------------------------
2019-12-08T23:11:50.5867211Z 
2019-12-08T23:11:50.5867231Z 
2019-12-08T23:11:50.5867415Z ---- [ui] ui/return/return-from-diverging.rs stdout ----
2019-12-08T23:11:50.5867469Z diff of stderr:
2019-12-08T23:11:50.5867492Z 
2019-12-08T23:11:50.5867648Z 4 LL | fn fail() -> ! {
2019-12-08T23:11:50.5867835Z 5    |              - expected `!` because of return type
2019-12-08T23:11:50.5867890Z 6 LL |     return "wow";
2019-12-08T23:11:50.5868070Z -    |            ^^^^^ expected `!`, found `&str`
2019-12-08T23:11:50.5868158Z +    |            |
2019-12-08T23:11:50.5868158Z +    |            |
2019-12-08T23:11:50.5868195Z +    |            expected `!`, found `&str`
2019-12-08T23:11:50.5868235Z +    |            help: consider dereferencing the borrow: `*"wow"`
2019-12-08T23:11:50.5868396Z 9    = note:   expected type `!`
2019-12-08T23:11:50.5868591Z 10            found reference `&'static str`
2019-12-08T23:11:50.5868678Z 
2019-12-08T23:11:50.5868699Z 
2019-12-08T23:11:50.5868699Z 
2019-12-08T23:11:50.5868753Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5869025Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-from-diverging/return-from-diverging.stderr
2019-12-08T23:11:50.5869245Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5869489Z To only update this specific test, also pass `--test-args return/return-from-diverging.rs`
2019-12-08T23:11:50.5869558Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5869614Z status: exit code: 1
2019-12-08T23:11:50.5869614Z status: exit code: 1
2019-12-08T23:11:50.5870268Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/return/return-from-diverging.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-from-diverging" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-from-diverging/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5870557Z ------------------------------------------
2019-12-08T23:11:50.5870586Z 
2019-12-08T23:11:50.5870792Z ------------------------------------------
2019-12-08T23:11:50.5870830Z stderr:
2019-12-08T23:11:50.5870830Z stderr:
2019-12-08T23:11:50.5871017Z ------------------------------------------
2019-12-08T23:11:50.5871073Z error[E0308]: mismatched types
2019-12-08T23:11:50.5871284Z   --> /checkout/src/test/ui/return/return-from-diverging.rs:4:12
2019-12-08T23:11:50.5871325Z    |
2019-12-08T23:11:50.5871507Z LL | fn fail() -> ! {
2019-12-08T23:11:50.5871715Z    |              - expected `!` because of return type
2019-12-08T23:11:50.5871759Z LL |     return "wow"; //~ ERROR mismatched types
2019-12-08T23:11:50.5871858Z    |            |
2019-12-08T23:11:50.5871858Z    |            |
2019-12-08T23:11:50.5871895Z    |            expected `!`, found `&str`
2019-12-08T23:11:50.5872084Z    |            help: consider dereferencing the borrow: `*"wow"`
2019-12-08T23:11:50.5872446Z    = note:   expected type `!`
2019-12-08T23:11:50.5872704Z            found reference `&'static str`
2019-12-08T23:11:50.5872737Z 
2019-12-08T23:11:50.5872799Z error: aborting due to previous error
---
2019-12-08T23:11:50.5873696Z diff of stderr:
2019-12-08T23:11:50.5873724Z 
2019-12-08T23:11:50.5873938Z 708   --> $DIR/disallowed-positions.rs:86:44
2019-12-08T23:11:50.5874009Z 709    |
2019-12-08T23:11:50.5874057Z 710 LL |     if let Range { start: true, end } = t..&&false {}
2019-12-08T23:11:50.5874320Z -    |                                            ^^^^^^^ expected `bool`, found `&&bool`
2019-12-08T23:11:50.5874447Z +    |                                            |
2019-12-08T23:11:50.5874447Z +    |                                            |
2019-12-08T23:11:50.5874499Z +    |                                            expected `bool`, found `&&bool`
2019-12-08T23:11:50.5874574Z +    |                                            help: consider dereferencing the borrow: `*&&false`
2019-12-08T23:11:50.5874664Z 713 error[E0308]: mismatched types
2019-12-08T23:11:50.5874883Z 714   --> $DIR/disallowed-positions.rs:86:8
2019-12-08T23:11:50.5874931Z 
2019-12-08T23:11:50.5875245Z 896   --> $DIR/disallowed-positions.rs:150:47
2019-12-08T23:11:50.5875245Z 896   --> $DIR/disallowed-positions.rs:150:47
2019-12-08T23:11:50.5875300Z 897    |
2019-12-08T23:11:50.5875365Z 898 LL |     while let Range { start: true, end } = t..&&false {}
2019-12-08T23:11:50.5875872Z -    |                                               ^^^^^^^ expected `bool`, found `&&bool`
2019-12-08T23:11:50.5875975Z +    |                                               |
2019-12-08T23:11:50.5875975Z +    |                                               |
2019-12-08T23:11:50.5876018Z +    |                                               expected `bool`, found `&&bool`
2019-12-08T23:11:50.5876064Z +    |                                               help: consider dereferencing the borrow: `*&&false`
2019-12-08T23:11:50.5876153Z 901 error[E0308]: mismatched types
2019-12-08T23:11:50.5876337Z 902   --> $DIR/disallowed-positions.rs:150:11
2019-12-08T23:11:50.5876363Z 
2019-12-08T23:11:50.5876414Z 971 error[E0308]: mismatched types
2019-12-08T23:11:50.5876414Z 971 error[E0308]: mismatched types
2019-12-08T23:11:50.5876599Z 972   --> $DIR/disallowed-positions.rs:207:5
2019-12-08T23:11:50.5876636Z 973    |
2019-12-08T23:11:50.5876815Z - LL | fn outside_if_and_while_expr() {
2019-12-08T23:11:50.5877032Z -    |                                - help: try adding a return type: `-> &bool`
2019-12-08T23:11:50.5877178Z - ...
2019-12-08T23:11:50.5877215Z 977 LL |     &let 0 = 0
2019-12-08T23:11:50.5877267Z 978    |     ^^^^^^^^^^ expected `()`, found `&bool`
2019-12-08T23:11:50.5877337Z + help: try adding a return type
2019-12-08T23:11:50.5877387Z +    |
2019-12-08T23:11:50.5877387Z +    |
2019-12-08T23:11:50.5877569Z + LL | fn outside_if_and_while_expr() -> &bool {
2019-12-08T23:11:50.5877665Z + help: consider dereferencing the borrow
2019-12-08T23:11:50.5877700Z +    |
2019-12-08T23:11:50.5877700Z +    |
2019-12-08T23:11:50.5877733Z + LL |     *&let 0 = 0
2019-12-08T23:11:50.5877816Z 979 
2019-12-08T23:11:50.5877863Z 980 error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
2019-12-08T23:11:50.5878051Z 981   --> $DIR/disallowed-positions.rs:179:17
2019-12-08T23:11:50.5878100Z 
2019-12-08T23:11:50.5878100Z 
2019-12-08T23:11:50.5878120Z 
2019-12-08T23:11:50.5878156Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.5878426Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/disallowed-positions.stderr
2019-12-08T23:11:50.5878648Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.5878879Z To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/disallowed-positions.rs`
2019-12-08T23:11:50.5878961Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.5878996Z status: exit code: 1
2019-12-08T23:11:50.5878996Z status: exit code: 1
2019-12-08T23:11:50.5879911Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/auxiliary" "-A" "unused"
2019-12-08T23:11:50.5880195Z ------------------------------------------
2019-12-08T23:11:50.5880241Z 
2019-12-08T23:11:50.5880420Z ------------------------------------------
2019-12-08T23:11:50.5880458Z stderr:
2019-12-08T23:11:50.5880458Z stderr:
2019-12-08T23:11:50.5880649Z ------------------------------------------
2019-12-08T23:11:50.5880690Z error: expected one of `,` or `>`, found `&&`
2019-12-08T23:11:50.5880964Z    |
2019-12-08T23:11:50.5880964Z    |
2019-12-08T23:11:50.5881071Z LL |         true && let 1 = 1 //~ ERROR expected one of `,` or `>`, found `&&`
2019-12-08T23:11:50.5881121Z    |              ^^ expected one of `,` or `>`
2019-12-08T23:11:50.5881247Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5881483Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:32:9
2019-12-08T23:11:50.5881524Z    |
2019-12-08T23:11:50.5881524Z    |
2019-12-08T23:11:50.5881581Z LL |     if &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5881654Z    |
2019-12-08T23:11:50.5881654Z    |
2019-12-08T23:11:50.5882414Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5882702Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5882778Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5883087Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:35:9
2019-12-08T23:11:50.5883136Z    |
2019-12-08T23:11:50.5883136Z    |
2019-12-08T23:11:50.5883184Z LL |     if !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5883297Z    |
2019-12-08T23:11:50.5883297Z    |
2019-12-08T23:11:50.5883545Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5883619Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5883693Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5883946Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:36:9
2019-12-08T23:11:50.5884022Z    |
2019-12-08T23:11:50.5884022Z    |
2019-12-08T23:11:50.5884072Z LL |     if *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5884348Z    |
2019-12-08T23:11:50.5884348Z    |
2019-12-08T23:11:50.5884663Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5884733Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5884842Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5885120Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:38:9
2019-12-08T23:11:50.5885172Z    |
2019-12-08T23:11:50.5885172Z    |
2019-12-08T23:11:50.5885459Z LL |     if -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5885555Z    |
2019-12-08T23:11:50.5885555Z    |
2019-12-08T23:11:50.5885839Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5885899Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5886151Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5886367Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:46:9
2019-12-08T23:11:50.5886407Z    |
2019-12-08T23:11:50.5886407Z    |
2019-12-08T23:11:50.5886454Z LL |     if (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5886551Z    |
2019-12-08T23:11:50.5886551Z    |
2019-12-08T23:11:50.5886764Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5886829Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5886891Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5887125Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:50:16
2019-12-08T23:11:50.5887166Z    |
2019-12-08T23:11:50.5887166Z    |
2019-12-08T23:11:50.5887206Z LL |     if true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5887457Z    |
2019-12-08T23:11:50.5887457Z    |
2019-12-08T23:11:50.5887661Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5887807Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5887945Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5888179Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:51:17
2019-12-08T23:11:50.5888235Z    |
2019-12-08T23:11:50.5888235Z    |
2019-12-08T23:11:50.5888275Z LL |     if (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5888364Z    |
2019-12-08T23:11:50.5888364Z    |
2019-12-08T23:11:50.5888571Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5888616Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5888697Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5888905Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:52:25
2019-12-08T23:11:50.5888944Z    |
2019-12-08T23:11:50.5888944Z    |
2019-12-08T23:11:50.5889007Z LL |     if true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5889087Z    |
2019-12-08T23:11:50.5889087Z    |
2019-12-08T23:11:50.5889311Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5889356Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5889416Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5889642Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:53:25
2019-12-08T23:11:50.5889681Z    |
2019-12-08T23:11:50.5889681Z    |
2019-12-08T23:11:50.5889721Z LL |     if true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5889813Z    |
2019-12-08T23:11:50.5889813Z    |
2019-12-08T23:11:50.5890015Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5890083Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5890150Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5890361Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:56:12
2019-12-08T23:11:50.5890417Z    |
2019-12-08T23:11:50.5890417Z    |
2019-12-08T23:11:50.5890455Z LL |     if x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5890544Z    |
2019-12-08T23:11:50.5890544Z    |
2019-12-08T23:11:50.5890747Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5890793Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5890872Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5891079Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:59:15
2019-12-08T23:11:50.5891118Z    |
2019-12-08T23:11:50.5891118Z    |
2019-12-08T23:11:50.5891183Z LL |     if true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5891262Z    |
2019-12-08T23:11:50.5891262Z    |
2019-12-08T23:11:50.5891487Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5891533Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5891611Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5892204Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:61:11
2019-12-08T23:11:50.5892256Z    |
2019-12-08T23:11:50.5892256Z    |
2019-12-08T23:11:50.5892303Z LL |     if ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5892409Z    |
2019-12-08T23:11:50.5892409Z    |
2019-12-08T23:11:50.5892661Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5892833Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5892985Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5893284Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:63:9
2019-12-08T23:11:50.5893332Z    |
2019-12-08T23:11:50.5893332Z    |
2019-12-08T23:11:50.5893379Z LL |     if (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5893483Z    |
2019-12-08T23:11:50.5893483Z    |
2019-12-08T23:11:50.5893733Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5893787Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5893879Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5894133Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:67:8
2019-12-08T23:11:50.5894200Z    |
2019-12-08T23:11:50.5894200Z    |
2019-12-08T23:11:50.5894258Z LL |     if let Range { start: _, end: _ } = true..true && false {}
2019-12-08T23:11:50.5894379Z    |
2019-12-08T23:11:50.5894379Z    |
2019-12-08T23:11:50.5894654Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5894714Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5894811Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5895084Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:71:8
2019-12-08T23:11:50.5895134Z    |
2019-12-08T23:11:50.5895134Z    |
2019-12-08T23:11:50.5895200Z LL |     if let Range { start: _, end: _ } = true..true || false {}
2019-12-08T23:11:50.5895458Z    |
2019-12-08T23:11:50.5895458Z    |
2019-12-08T23:11:50.5895678Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5895730Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5895791Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5896024Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:78:8
2019-12-08T23:11:50.5896063Z    |
2019-12-08T23:11:50.5896063Z    |
2019-12-08T23:11:50.5896100Z LL |     if let Range { start: F, end } = F..|| true {}
2019-12-08T23:11:50.5896190Z    |
2019-12-08T23:11:50.5896190Z    |
2019-12-08T23:11:50.5896394Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5896456Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5896516Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5896722Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:86:8
2019-12-08T23:11:50.5896777Z    |
2019-12-08T23:11:50.5896777Z    |
2019-12-08T23:11:50.5896820Z LL |     if let Range { start: true, end } = t..&&false {}
2019-12-08T23:11:50.5896914Z    |
2019-12-08T23:11:50.5896914Z    |
2019-12-08T23:11:50.5897118Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5897164Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5897241Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5897450Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:92:19
2019-12-08T23:11:50.5897489Z    |
2019-12-08T23:11:50.5897546Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5897546Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5897588Z    |                   ^^^^^^^^^^^^^^^
2019-12-08T23:11:50.5897621Z    |
2019-12-08T23:11:50.5897840Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5897950Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5898034Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5898309Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:96:12
2019-12-08T23:11:50.5898348Z    |
2019-12-08T23:11:50.5898348Z    |
2019-12-08T23:11:50.5898386Z LL |     while &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5898475Z    |
2019-12-08T23:11:50.5898475Z    |
2019-12-08T23:11:50.5898680Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5898743Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5898804Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5899030Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:99:12
2019-12-08T23:11:50.5899070Z    |
2019-12-08T23:11:50.5899070Z    |
2019-12-08T23:11:50.5899115Z LL |     while !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5899209Z    |
2019-12-08T23:11:50.5899209Z    |
2019-12-08T23:11:50.5899417Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5899463Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5899541Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5899750Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:100:12
2019-12-08T23:11:50.5899805Z    |
2019-12-08T23:11:50.5899805Z    |
2019-12-08T23:11:50.5899844Z LL |     while *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5899915Z    |
2019-12-08T23:11:50.5899915Z    |
2019-12-08T23:11:50.5900136Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5900187Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5900264Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5900482Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:102:12
2019-12-08T23:11:50.5900521Z    |
2019-12-08T23:11:50.5900521Z    |
2019-12-08T23:11:50.5900740Z LL |     while -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5900812Z    |
2019-12-08T23:11:50.5900812Z    |
2019-12-08T23:11:50.5901016Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5901077Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5901137Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5901363Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:110:12
2019-12-08T23:11:50.5901434Z    |
2019-12-08T23:11:50.5901434Z    |
2019-12-08T23:11:50.5901480Z LL |     while (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5901568Z    |
2019-12-08T23:11:50.5901568Z    |
2019-12-08T23:11:50.5901965Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5902176Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5902429Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5902717Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:114:19
2019-12-08T23:11:50.5902783Z    |
2019-12-08T23:11:50.5902783Z    |
2019-12-08T23:11:50.5902833Z LL |     while true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5902940Z    |
2019-12-08T23:11:50.5902940Z    |
2019-12-08T23:11:50.5903190Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5903245Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5903444Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5903725Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:115:20
2019-12-08T23:11:50.5903844Z    |
2019-12-08T23:11:50.5903844Z    |
2019-12-08T23:11:50.5903912Z LL |     while (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5904001Z    |
2019-12-08T23:11:50.5904001Z    |
2019-12-08T23:11:50.5904286Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5904341Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5904415Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5904691Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:116:28
2019-12-08T23:11:50.5904737Z    |
2019-12-08T23:11:50.5904737Z    |
2019-12-08T23:11:50.5904794Z LL |     while true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5904908Z    |
2019-12-08T23:11:50.5904908Z    |
2019-12-08T23:11:50.5905160Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5905225Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5905299Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5905572Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:117:28
2019-12-08T23:11:50.5905619Z    |
2019-12-08T23:11:50.5905619Z    |
2019-12-08T23:11:50.5905670Z LL |     while true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5905936Z    |
2019-12-08T23:11:50.5905936Z    |
2019-12-08T23:11:50.5906141Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5906193Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5906273Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5906493Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:120:15
2019-12-08T23:11:50.5906545Z    |
2019-12-08T23:11:50.5906545Z    |
2019-12-08T23:11:50.5906584Z LL |     while x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5906667Z    |
2019-12-08T23:11:50.5906667Z    |
2019-12-08T23:11:50.5907032Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5907077Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5907148Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5907351Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:123:18
2019-12-08T23:11:50.5907388Z    |
2019-12-08T23:11:50.5907388Z    |
2019-12-08T23:11:50.5907446Z LL |     while true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5907522Z    |
2019-12-08T23:11:50.5907522Z    |
2019-12-08T23:11:50.5907738Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5907783Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5907842Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5908060Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:125:14
2019-12-08T23:11:50.5908097Z    |
2019-12-08T23:11:50.5908097Z    |
2019-12-08T23:11:50.5908135Z LL |     while ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5908219Z    |
2019-12-08T23:11:50.5908219Z    |
2019-12-08T23:11:50.5908415Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5908540Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5908606Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5908875Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:127:12
2019-12-08T23:11:50.5908928Z    |
2019-12-08T23:11:50.5908928Z    |
2019-12-08T23:11:50.5908966Z LL |     while (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5909050Z    |
2019-12-08T23:11:50.5909050Z    |
2019-12-08T23:11:50.5909249Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5909294Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5909367Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5909571Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:131:11
2019-12-08T23:11:50.5909608Z    |
2019-12-08T23:11:50.5909608Z    |
2019-12-08T23:11:50.5909665Z LL |     while let Range { start: _, end: _ } = true..true && false {}
2019-12-08T23:11:50.5909745Z    |
2019-12-08T23:11:50.5909745Z    |
2019-12-08T23:11:50.5909960Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5910004Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5910076Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5910281Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:135:11
2019-12-08T23:11:50.5910318Z    |
2019-12-08T23:11:50.5910318Z    |
2019-12-08T23:11:50.5910355Z LL |     while let Range { start: _, end: _ } = true..true || false {}
2019-12-08T23:11:50.5910442Z    |
2019-12-08T23:11:50.5910442Z    |
2019-12-08T23:11:50.5910641Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5910706Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5910765Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5910990Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:142:11
2019-12-08T23:11:50.5911027Z    |
2019-12-08T23:11:50.5911027Z    |
2019-12-08T23:11:50.5911063Z LL |     while let Range { start: F, end } = F..|| true {}
2019-12-08T23:11:50.5911148Z    |
2019-12-08T23:11:50.5911148Z    |
2019-12-08T23:11:50.5911347Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5911393Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5911466Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5911668Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:150:11
2019-12-08T23:11:50.5911892Z    |
2019-12-08T23:11:50.5911892Z    |
2019-12-08T23:11:50.5911937Z LL |     while let Range { start: true, end } = t..&&false {}
2019-12-08T23:11:50.5912368Z    |
2019-12-08T23:11:50.5912368Z    |
2019-12-08T23:11:50.5912655Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5912711Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5912802Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5913058Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:156:22
2019-12-08T23:11:50.5913104Z    |
2019-12-08T23:11:50.5913169Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5913169Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5913220Z    |                      ^^^^^^^^^^^^^^^
2019-12-08T23:11:50.5913262Z    |
2019-12-08T23:11:50.5913524Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5913675Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5913757Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5914111Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:170:6
2019-12-08T23:11:50.5914159Z    |
2019-12-08T23:11:50.5914159Z    |
2019-12-08T23:11:50.5914206Z LL |     &let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5914310Z    |
2019-12-08T23:11:50.5914310Z    |
2019-12-08T23:11:50.5914559Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5914629Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5914704Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5914956Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:172:6
2019-12-08T23:11:50.5940428Z    |
2019-12-08T23:11:50.5940428Z    |
2019-12-08T23:11:50.5940490Z LL |     !let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5940568Z    |
2019-12-08T23:11:50.5940568Z    |
2019-12-08T23:11:50.5940915Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5940963Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5941031Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5941239Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:173:6
2019-12-08T23:11:50.5941277Z    |
2019-12-08T23:11:50.5941277Z    |
2019-12-08T23:11:50.5941324Z LL |     *let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5941391Z    |
2019-12-08T23:11:50.5941391Z    |
2019-12-08T23:11:50.5941986Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5942050Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5942147Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5942434Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:175:6
2019-12-08T23:11:50.5942491Z    |
2019-12-08T23:11:50.5942491Z    |
2019-12-08T23:11:50.5942732Z LL |     -let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5942829Z    |
2019-12-08T23:11:50.5942829Z    |
2019-12-08T23:11:50.5943077Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5943140Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5943215Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5943475Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:183:6
2019-12-08T23:11:50.5943523Z    |
2019-12-08T23:11:50.5943523Z    |
2019-12-08T23:11:50.5943571Z LL |     (let 0 = 0)?; //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5943671Z    |
2019-12-08T23:11:50.5943671Z    |
2019-12-08T23:11:50.5943920Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5944395Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5944482Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5944766Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:187:13
2019-12-08T23:11:50.5944821Z    |
2019-12-08T23:11:50.5944821Z    |
2019-12-08T23:11:50.5944869Z LL |     true || let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5944963Z    |
2019-12-08T23:11:50.5944963Z    |
2019-12-08T23:11:50.5945508Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5945552Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5945920Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5946159Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:188:14
2019-12-08T23:11:50.5946289Z    |
2019-12-08T23:11:50.5946289Z    |
2019-12-08T23:11:50.5946332Z LL |     (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5946398Z    |
2019-12-08T23:11:50.5946398Z    |
2019-12-08T23:11:50.5946610Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5946652Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5946707Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5946908Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:189:22
2019-12-08T23:11:50.5946943Z    |
2019-12-08T23:11:50.5946943Z    |
2019-12-08T23:11:50.5946980Z LL |     true && (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5947063Z    |
2019-12-08T23:11:50.5947063Z    |
2019-12-08T23:11:50.5947254Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5947308Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5947364Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5947558Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:192:9
2019-12-08T23:11:50.5947600Z    |
2019-12-08T23:11:50.5947600Z    |
2019-12-08T23:11:50.5947635Z LL |     x = let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5947704Z    |
2019-12-08T23:11:50.5947704Z    |
2019-12-08T23:11:50.5947893Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5947934Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5947995Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5948193Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:194:12
2019-12-08T23:11:50.5948236Z    |
2019-12-08T23:11:50.5948236Z    |
2019-12-08T23:11:50.5948278Z LL |     true..(let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5948345Z    |
2019-12-08T23:11:50.5948345Z    |
2019-12-08T23:11:50.5948540Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5948581Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5948643Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5949015Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:195:8
2019-12-08T23:11:50.5949051Z    |
2019-12-08T23:11:50.5949051Z    |
2019-12-08T23:11:50.5949087Z LL |     ..(let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5949352Z    |
2019-12-08T23:11:50.5949352Z    |
2019-12-08T23:11:50.5949558Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5949608Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5949673Z error: `let` expressions are not supported here
2019-12-08T23:11:50.5949883Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:196:6
2019-12-08T23:11:50.5949921Z    |
2019-12-08T23:11:50.5949921Z    |
2019-12-08T23:11:50.5949958Z LL |     (let 0 = 0)..; //~ ERROR `let` expressions are not supported here
2019-12-08T23:11:50.5950031Z    |
2019-12-08T23:11:50.5950031Z    |
2019-12-08T23:11:50.5950229Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-12-08T23:11:50.5950273Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-12-08T23:11:50.5950337Z error: `let` expressions are not supported here
---
2019-12-08T23:11:50.6001490Z    |     ^^^^^^^^^^ expected `()`, found `&bool`
2019-12-08T23:11:50.6001525Z    |
2019-12-08T23:11:50.6001559Z help: try adding a return type
2019-12-08T23:11:50.6001599Z    |
2019-12-08T23:11:50.6001965Z LL | fn outside_if_and_while_expr() -> &bool {
2019-12-08T23:11:50.6002381Z help: consider dereferencing the borrow
2019-12-08T23:11:50.6002424Z    |
2019-12-08T23:11:50.6002424Z    |
2019-12-08T23:11:50.6002466Z LL |     *&let 0 = 0
2019-12-08T23:11:50.6002538Z 
2019-12-08T23:11:50.6002587Z error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
2019-12-08T23:11:50.6002882Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:179:17
2019-12-08T23:11:50.6002937Z    |
---
2019-12-08T23:11:50.6005278Z -    |            -----   ^^^^^^^^^^^^^ expected `usize`, found struct `std::string::String`
2019-12-08T23:11:50.6005482Z -    |            |
2019-12-08T23:11:50.6005688Z +    |            -----   ^^^^^^^^^^^^^
2019-12-08T23:11:50.6005736Z +    |            |       |
2019-12-08T23:11:50.6005791Z +    |            |       expected `usize`, found struct `std::string::String`
2019-12-08T23:11:50.6006123Z +    |            |       help: consider dereferencing the type: `*String::new()`
2019-12-08T23:11:50.6006483Z 8 
2019-12-08T23:11:50.6006525Z 9 error[E0308]: mismatched types
2019-12-08T23:11:50.6006553Z 
2019-12-08T23:11:50.6006578Z 
2019-12-08T23:11:50.6006578Z 
2019-12-08T23:11:50.6006625Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.6007082Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/coerce-suggestions/coerce-suggestions.stderr
2019-12-08T23:11:50.6007477Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.6007941Z To only update this specific test, also pass `--test-args span/coerce-suggestions.rs`
2019-12-08T23:11:50.6008009Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.6008047Z status: exit code: 1
2019-12-08T23:11:50.6008047Z status: exit code: 1
2019-12-08T23:11:50.6008899Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/coerce-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/coerce-suggestions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/coerce-suggestions/auxiliary" "-A" "unused"
2019-12-08T23:11:50.6009188Z ------------------------------------------
2019-12-08T23:11:50.6009227Z 
2019-12-08T23:11:50.6009427Z ------------------------------------------
2019-12-08T23:11:50.6009476Z stderr:
2019-12-08T23:11:50.6009476Z stderr:
2019-12-08T23:11:50.6009669Z ------------------------------------------
2019-12-08T23:11:50.6009712Z error[E0308]: mismatched types
2019-12-08T23:11:50.6009934Z   --> /checkout/src/test/ui/span/coerce-suggestions.rs:7:20
2019-12-08T23:11:50.6009981Z    |
2019-12-08T23:11:50.6010023Z LL |     let x: usize = String::new();
2019-12-08T23:11:50.6010216Z    |            -----   ^^^^^^^^^^^^^
2019-12-08T23:11:50.6010266Z    |            |       |
2019-12-08T23:11:50.6010311Z    |            |       expected `usize`, found struct `std::string::String`
2019-12-08T23:11:50.6010360Z    |            |       help: consider dereferencing the type: `*String::new()`
2019-12-08T23:11:50.6010454Z 
2019-12-08T23:11:50.6010492Z error[E0308]: mismatched types
2019-12-08T23:11:50.6010720Z   --> /checkout/src/test/ui/span/coerce-suggestions.rs:9:19
2019-12-08T23:11:50.6010929Z    |
---
2019-12-08T23:11:50.6013841Z    |
2019-12-08T23:11:50.6013881Z LL |     f = box f;
2019-12-08T23:11:50.6013941Z    |         ^^^^^
2019-12-08T23:11:50.6013983Z    |         |
2019-12-08T23:11:50.6014027Z    |         cyclic type of infinite size
2019-12-08T23:11:50.6014090Z    |         help: try using a conversion method: `(box f).to_string()`
2019-12-08T23:11:50.6014163Z error[E0308]: mismatched types
2019-12-08T23:11:50.6014397Z   --> /checkout/src/test/ui/span/coerce-suggestions.rs:21:9
2019-12-08T23:11:50.6014450Z    |
2019-12-08T23:11:50.6014491Z LL |     s = format!("foo");
---
2019-12-08T23:11:50.6016170Z diff of stderr:
2019-12-08T23:11:50.6016194Z 
2019-12-08T23:11:50.6016523Z 55   --> $DIR/issue-34264.rs:8:13
2019-12-08T23:11:50.6016559Z 56    |
2019-12-08T23:11:50.6016761Z 57 LL |     bar("", "");
2019-12-08T23:11:50.6016942Z -    |             ^^ expected `usize`, found `&str`
2019-12-08T23:11:50.6017018Z +    |             |
2019-12-08T23:11:50.6017055Z +    |             expected `usize`, found `&str`
2019-12-08T23:11:50.6017055Z +    |             expected `usize`, found `&str`
2019-12-08T23:11:50.6017094Z +    |             help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.6017173Z 60 error[E0061]: this function takes 2 parameters but 3 parameters were supplied
2019-12-08T23:11:50.6017348Z 61   --> $DIR/issue-34264.rs:10:5
2019-12-08T23:11:50.6017372Z 
2019-12-08T23:11:50.6017392Z 
2019-12-08T23:11:50.6017392Z 
2019-12-08T23:11:50.6017439Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.6017672Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-34264/issue-34264.stderr
2019-12-08T23:11:50.6017865Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.6018095Z To only update this specific test, also pass `--test-args span/issue-34264.rs`
2019-12-08T23:11:50.6018160Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.6018203Z status: exit code: 1
2019-12-08T23:11:50.6018203Z status: exit code: 1
2019-12-08T23:11:50.6018873Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-34264.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-34264" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-34264/auxiliary" "-A" "unused"
2019-12-08T23:11:50.6019223Z ------------------------------------------
2019-12-08T23:11:50.6019251Z 
2019-12-08T23:11:50.6019434Z ------------------------------------------
2019-12-08T23:11:50.6019479Z stderr:
2019-12-08T23:11:50.6019479Z stderr:
2019-12-08T23:11:50.6019661Z ------------------------------------------
2019-12-08T23:11:50.6019703Z error: expected one of `:`, `@`, or `|`, found `<`
2019-12-08T23:11:50.6019907Z   --> /checkout/src/test/ui/span/issue-34264.rs:1:14
2019-12-08T23:11:50.6019945Z    |
2019-12-08T23:11:50.6019984Z LL | fn foo(Option<i32>, String) {} //~ ERROR expected one of
2019-12-08T23:11:50.6020033Z    |              ^ expected one of `:`, `@`, or `|`
2019-12-08T23:11:50.6020115Z    = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
2019-12-08T23:11:50.6020163Z help: if this is a type, explicitly ignore the parameter name
2019-12-08T23:11:50.6020206Z    |
2019-12-08T23:11:50.6020206Z    |
2019-12-08T23:11:50.6020244Z LL | fn foo(_: Option<i32>, String) {} //~ ERROR expected one of
2019-12-08T23:11:50.6020312Z 
2019-12-08T23:11:50.6020312Z 
2019-12-08T23:11:50.6020348Z error: expected one of `:`, `@`, or `|`, found `)`
2019-12-08T23:11:50.6020553Z   --> /checkout/src/test/ui/span/issue-34264.rs:1:27
2019-12-08T23:11:50.6020597Z    |
2019-12-08T23:11:50.6020635Z LL | fn foo(Option<i32>, String) {} //~ ERROR expected one of
2019-12-08T23:11:50.6020678Z    |                           ^ expected one of `:`, `@`, or `|`
2019-12-08T23:11:50.6020760Z    = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
2019-12-08T23:11:50.6020801Z help: if this was a parameter name, give it a type
2019-12-08T23:11:50.6020843Z    |
2019-12-08T23:11:50.6020843Z    |
2019-12-08T23:11:50.6020889Z LL | fn foo(Option<i32>, String: TypeName) {} //~ ERROR expected one of
2019-12-08T23:11:50.6020975Z help: if this is a type, explicitly ignore the parameter name
2019-12-08T23:11:50.6021015Z    |
2019-12-08T23:11:50.6021015Z    |
2019-12-08T23:11:50.6021053Z LL | fn foo(Option<i32>, _: String) {} //~ ERROR expected one of
2019-12-08T23:11:50.6021124Z 
2019-12-08T23:11:50.6021124Z 
2019-12-08T23:11:50.6021159Z error: expected one of `:`, `@`, or `|`, found `,`
2019-12-08T23:11:50.6021363Z   --> /checkout/src/test/ui/span/issue-34264.rs:3:9
2019-12-08T23:11:50.6021402Z    |
2019-12-08T23:11:50.6021445Z LL | fn bar(x, y: usize) {} //~ ERROR expected one of
2019-12-08T23:11:50.6021484Z    |         ^ expected one of `:`, `@`, or `|`
2019-12-08T23:11:50.6021565Z    = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
2019-12-08T23:11:50.6021605Z help: if this was a parameter name, give it a type
2019-12-08T23:11:50.6021646Z    |
2019-12-08T23:11:50.6021646Z    |
2019-12-08T23:11:50.6021690Z LL | fn bar(x: TypeName, y: usize) {} //~ ERROR expected one of
2019-12-08T23:11:50.6021773Z help: if this is a type, explicitly ignore the parameter name
2019-12-08T23:11:50.6021817Z    |
2019-12-08T23:11:50.6021817Z    |
2019-12-08T23:11:50.6021854Z LL | fn bar(_: x, y: usize) {} //~ ERROR expected one of
2019-12-08T23:11:50.6022085Z 
2019-12-08T23:11:50.6022131Z error[E0061]: this function takes 2 parameters but 3 parameters were supplied
2019-12-08T23:11:50.6022668Z   --> /checkout/src/test/ui/span/issue-34264.rs:7:5
2019-12-08T23:11:50.6022715Z    |
2019-12-08T23:11:50.6022715Z    |
2019-12-08T23:11:50.6022769Z LL | fn foo(Option<i32>, String) {} //~ ERROR expected one of
2019-12-08T23:11:50.6023034Z ...
2019-12-08T23:11:50.6023034Z ...
2019-12-08T23:11:50.6023087Z LL |     foo(Some(42), 2, ""); //~ ERROR this function takes
2019-12-08T23:11:50.6023268Z 
2019-12-08T23:11:50.6023311Z error[E0308]: mismatched types
2019-12-08T23:11:50.6023625Z   --> /checkout/src/test/ui/span/issue-34264.rs:8:13
2019-12-08T23:11:50.6023674Z    |
2019-12-08T23:11:50.6023674Z    |
2019-12-08T23:11:50.6023719Z LL |     bar("", ""); //~ ERROR mismatched types
2019-12-08T23:11:50.6023815Z    |             |
2019-12-08T23:11:50.6023861Z    |             expected `usize`, found `&str`
2019-12-08T23:11:50.6023861Z    |             expected `usize`, found `&str`
2019-12-08T23:11:50.6023911Z    |             help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.6023997Z error[E0061]: this function takes 2 parameters but 3 parameters were supplied
2019-12-08T23:11:50.6024232Z   --> /checkout/src/test/ui/span/issue-34264.rs:10:5
2019-12-08T23:11:50.6024286Z    |
2019-12-08T23:11:50.6024286Z    |
2019-12-08T23:11:50.6024332Z LL | fn bar(x, y: usize) {} //~ ERROR expected one of
2019-12-08T23:11:50.6024586Z ...
2019-12-08T23:11:50.6024647Z LL |     bar(1, 2, 3); //~ ERROR this function takes
2019-12-08T23:11:50.6024694Z    |     ^^^^^^^^^^^^ expected 2 parameters
2019-12-08T23:11:50.6024731Z 
---
2019-12-08T23:11:50.6025402Z 
2019-12-08T23:11:50.6025637Z ---- [ui] ui/structs/struct-path-associated-type.rs stdout ----
2019-12-08T23:11:50.6025686Z diff of stderr:
2019-12-08T23:11:50.6025890Z 
2019-12-08T23:11:50.6025921Z 34 LL |     let s = S::A {};
2019-12-08T23:11:50.6026120Z 35    |             ^^^^ help: use fully-qualified syntax: `<S as Trait>::A`
2019-12-08T23:11:50.6026340Z - error[E0109]: type arguments are not allowed for this type
2019-12-08T23:11:50.6026519Z -   --> $DIR/struct-path-associated-type.rs:33:20
2019-12-08T23:11:50.6026666Z -    |
2019-12-08T23:11:50.6026666Z -    |
2019-12-08T23:11:50.6026822Z - LL |     let z = S::A::<u8> {};
2019-12-08T23:11:50.6026995Z -    |                    ^^ type argument not allowed
2019-12-08T23:11:50.6027172Z 43 error[E0223]: ambiguous associated type
2019-12-08T23:11:50.6027343Z 44   --> $DIR/struct-path-associated-type.rs:33:13
2019-12-08T23:11:50.6027385Z 45    |
2019-12-08T23:11:50.6027405Z 
2019-12-08T23:11:50.6027405Z 
2019-12-08T23:11:50.6027436Z 52 LL |         S::A {} => {}
2019-12-08T23:11:50.6027625Z 53    |         ^^^^ help: use fully-qualified syntax: `<S as Trait>::A`
2019-12-08T23:11:50.6027831Z - error: aborting due to 9 previous errors
2019-12-08T23:11:50.6027868Z + error: aborting due to 8 previous errors
2019-12-08T23:11:50.6027904Z 56 
2019-12-08T23:11:50.6027939Z 57 Some errors have detailed explanations: E0071, E0109, E0223.
2019-12-08T23:11:50.6027939Z 57 Some errors have detailed explanations: E0071, E0109, E0223.
2019-12-08T23:11:50.6028134Z 58 For more information about an error, try `rustc --explain E0071`.
2019-12-08T23:11:50.6028165Z 
2019-12-08T23:11:50.6028191Z 
2019-12-08T23:11:50.6028226Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.6028476Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-path-associated-type/struct-path-associated-type.stderr
2019-12-08T23:11:50.6028674Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.6028888Z To only update this specific test, also pass `--test-args structs/struct-path-associated-type.rs`
2019-12-08T23:11:50.6028954Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.6028988Z status: exit code: 1
2019-12-08T23:11:50.6028988Z status: exit code: 1
2019-12-08T23:11:50.6029662Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs/struct-path-associated-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-path-associated-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-path-associated-type/auxiliary" "-A" "unused"
2019-12-08T23:11:50.6029971Z ------------------------------------------
2019-12-08T23:11:50.6030003Z 
2019-12-08T23:11:50.6030167Z ------------------------------------------
2019-12-08T23:11:50.6030201Z stderr:
2019-12-08T23:11:50.6030201Z stderr:
2019-12-08T23:11:50.6030359Z ------------------------------------------
2019-12-08T23:11:50.6030407Z error[E0071]: expected struct, variant or union type, found associated type
2019-12-08T23:11:50.6030600Z   --> /checkout/src/test/ui/structs/struct-path-associated-type.rs:12:13
2019-12-08T23:11:50.6030643Z    |
2019-12-08T23:11:50.6030676Z LL |     let s = T::A {};
2019-12-08T23:11:50.6030737Z 
2019-12-08T23:11:50.6030777Z error[E0109]: type arguments are not allowed for this type
2019-12-08T23:11:50.6030977Z   --> /checkout/src/test/ui/structs/struct-path-associated-type.rs:14:20
2019-12-08T23:11:50.6031013Z    |
2019-12-08T23:11:50.6031013Z    |
2019-12-08T23:11:50.6031050Z LL |     let z = T::A::<u8> {};
2019-12-08T23:11:50.6031086Z    |                    ^^ type argument not allowed
2019-12-08T23:11:50.6031145Z error[E0071]: expected struct, variant or union type, found associated type
2019-12-08T23:11:50.6031344Z   --> /checkout/src/test/ui/structs/struct-path-associated-type.rs:14:13
2019-12-08T23:11:50.6031380Z    |
2019-12-08T23:11:50.6031380Z    |
2019-12-08T23:11:50.6031412Z LL |     let z = T::A::<u8> {};
2019-12-08T23:11:50.6031476Z 
2019-12-08T23:11:50.6031512Z error[E0071]: expected struct, variant or union type, found associated type
2019-12-08T23:11:50.6031711Z   --> /checkout/src/test/ui/structs/struct-path-associated-type.rs:18:9
2019-12-08T23:11:50.6031753Z    |
2019-12-08T23:11:50.6031753Z    |
2019-12-08T23:11:50.6031785Z LL |         T::A {} => {}
2019-12-08T23:11:50.6031847Z 
2019-12-08T23:11:50.6031886Z error[E0109]: type arguments are not allowed for this type
2019-12-08T23:11:50.6032732Z   --> /checkout/src/test/ui/structs/struct-path-associated-type.rs:25:20
2019-12-08T23:11:50.6032782Z    |
2019-12-08T23:11:50.6032782Z    |
2019-12-08T23:11:50.6032841Z LL |     let z = T::A::<u8> {}; //~ ERROR type arguments are not allowed for this type
2019-12-08T23:11:50.6032893Z    |                    ^^ type argument not allowed
2019-12-08T23:11:50.6032971Z error[E0223]: ambiguous associated type
2019-12-08T23:11:50.6033221Z   --> /checkout/src/test/ui/structs/struct-path-associated-type.rs:32:13
2019-12-08T23:11:50.6033268Z    |
2019-12-08T23:11:50.6033268Z    |
2019-12-08T23:11:50.6033314Z LL |     let s = S::A {}; //~ ERROR ambiguous associated type
2019-12-08T23:11:50.6033573Z    |             ^^^^ help: use fully-qualified syntax: `<S as Trait>::A`
2019-12-08T23:11:50.6033657Z error[E0223]: ambiguous associated type
2019-12-08T23:11:50.6033905Z   --> /checkout/src/test/ui/structs/struct-path-associated-type.rs:33:13
2019-12-08T23:11:50.6033953Z    |
2019-12-08T23:11:50.6033953Z    |
2019-12-08T23:11:50.6034000Z LL |     let z = S::A::<u8> {}; //~ ERROR ambiguous associated type
2019-12-08T23:11:50.6034259Z    |             ^^^^^^^^^^ help: use fully-qualified syntax: `<S as Trait>::A`
2019-12-08T23:11:50.6034335Z error[E0223]: ambiguous associated type
2019-12-08T23:11:50.6034577Z   --> /checkout/src/test/ui/structs/struct-path-associated-type.rs:36:9
2019-12-08T23:11:50.6034630Z    |
2019-12-08T23:11:50.6034630Z    |
2019-12-08T23:11:50.6034676Z LL |         S::A {} => {} //~ ERROR ambiguous associated type
2019-12-08T23:11:50.6034917Z    |         ^^^^ help: use fully-qualified syntax: `<S as Trait>::A`
2019-12-08T23:11:50.6035112Z error: aborting due to 8 previous errors
2019-12-08T23:11:50.6035149Z 
2019-12-08T23:11:50.6035196Z Some errors have detailed explanations: E0071, E0109, E0223.
2019-12-08T23:11:50.6035525Z For more information about an error, try `rustc --explain E0071`.
---
2019-12-08T23:11:50.6036175Z diff of stderr:
2019-12-08T23:11:50.6036197Z 
2019-12-08T23:11:50.6036365Z 34   --> $DIR/as-ref.rs:16:27
2019-12-08T23:11:50.6036400Z 35    |
2019-12-08T23:11:50.6036434Z 36 LL |   let y: Option<&usize> = x;
2019-12-08T23:11:50.6036597Z -    |          --------------   ^
2019-12-08T23:11:50.6036971Z -    |          |                expected enum `std::option::Option`, found reference
2019-12-08T23:11:50.6036971Z -    |          |                expected enum `std::option::Option`, found reference
2019-12-08T23:11:50.6037382Z -    |          |                help: you can convert from `&Option<T>` to `Option<&T>` using `.as_ref()`: `x.as_ref()`
2019-12-08T23:11:50.6037604Z +    |          --------------   ^ expected enum `std::option::Option`, found reference
2019-12-08T23:11:50.6037691Z 41    |          expected due to this
2019-12-08T23:11:50.6037724Z 42    |
2019-12-08T23:11:50.6037724Z 42    |
2019-12-08T23:11:50.6037759Z 43    = note:   expected enum `std::option::Option<&usize>`
2019-12-08T23:11:50.6037823Z 44            found reference `&std::option::Option<usize>`
2019-12-08T23:11:50.6037823Z 44            found reference `&std::option::Option<usize>`
2019-12-08T23:11:50.6037864Z + help: you can convert from `&Option<T>` to `Option<&T>` using `.as_ref()`
2019-12-08T23:11:50.6037899Z +    |
2019-12-08T23:11:50.6037939Z + LL |   let y: Option<&usize> = x.as_ref();
2019-12-08T23:11:50.6038011Z + help: consider dereferencing the borrow
2019-12-08T23:11:50.6038043Z +    |
2019-12-08T23:11:50.6038043Z +    |
2019-12-08T23:11:50.6038082Z + LL |   let y: Option<&usize> = *x;
2019-12-08T23:11:50.6038152Z 45 
2019-12-08T23:11:50.6038189Z 46 error[E0308]: mismatched types
2019-12-08T23:11:50.6038359Z 47   --> $DIR/as-ref.rs:19:35
2019-12-08T23:11:50.6038383Z 
2019-12-08T23:11:50.6038383Z 
2019-12-08T23:11:50.6038413Z 57    |
2019-12-08T23:11:50.6038453Z 58 LL |   let y: Result<&usize, &usize> = x.as_ref();
2019-12-08T23:11:50.6038524Z + help: consider dereferencing the borrow
2019-12-08T23:11:50.6038561Z +    |
2019-12-08T23:11:50.6038561Z +    |
2019-12-08T23:11:50.6038594Z + LL |   let y: Result<&usize, &usize> = *x;
2019-12-08T23:11:50.6038666Z 60 
2019-12-08T23:11:50.6038698Z 61 error[E0308]: mismatched types
2019-12-08T23:11:50.6038858Z 62   --> $DIR/as-ref.rs:23:34
2019-12-08T23:11:50.6038881Z 
2019-12-08T23:11:50.6038881Z 
2019-12-08T23:11:50.6038916Z 63    |
2019-12-08T23:11:50.6038949Z 64 LL |   let y: Result<&usize, usize> = x;
2019-12-08T23:11:50.6039160Z -    |          ---------------------   ^ expected enum `std::result::Result`, found reference
2019-12-08T23:11:50.6039483Z +    |          ---------------------   ^
2019-12-08T23:11:50.6039519Z +    |          |                       |
2019-12-08T23:11:50.6039557Z +    |          |                       expected enum `std::result::Result`, found reference
2019-12-08T23:11:50.6039557Z +    |          |                       expected enum `std::result::Result`, found reference
2019-12-08T23:11:50.6039607Z +    |          |                       help: consider dereferencing the borrow: `*x`
2019-12-08T23:11:50.6039676Z 68    |
2019-12-08T23:11:50.6039676Z 68    |
2019-12-08T23:11:50.6039716Z 69    = note:   expected enum `std::result::Result<&usize, usize>`
2019-12-08T23:11:50.6039759Z 
2019-12-08T23:11:50.6039793Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.6040021Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/as-ref/as-ref.stderr
2019-12-08T23:11:50.6040021Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/as-ref/as-ref.stderr
2019-12-08T23:11:50.6040272Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.6040512Z To only update this specific test, also pass `--test-args suggestions/as-ref.rs`
2019-12-08T23:11:50.6040640Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.6040676Z status: exit code: 1
2019-12-08T23:11:50.6040676Z status: exit code: 1
2019-12-08T23:11:50.6041672Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/as-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/as-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/as-ref/auxiliary" "-A" "unused"
2019-12-08T23:11:50.6042652Z ------------------------------------------
2019-12-08T23:11:50.6042702Z 
2019-12-08T23:11:50.6042929Z ------------------------------------------
2019-12-08T23:11:50.6042974Z stderr:
2019-12-08T23:11:50.6042974Z stderr:
2019-12-08T23:11:50.6043204Z ------------------------------------------
2019-12-08T23:11:50.6043251Z error[E0308]: mismatched types
2019-12-08T23:11:50.6043479Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:6:27
2019-12-08T23:11:50.6043533Z    |
2019-12-08T23:11:50.6043577Z LL |   opt.map(|arg| takes_ref(arg));
2019-12-08T23:11:50.6043818Z    |       ---                 ^^^ expected `&Foo`, found struct `Foo`
2019-12-08T23:11:50.6043925Z    |       help: consider using `as_ref` instead: `as_ref().map`
2019-12-08T23:11:50.6043957Z 
2019-12-08T23:11:50.6043998Z error[E0308]: mismatched types
2019-12-08T23:11:50.6044389Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:8:37
2019-12-08T23:11:50.6044389Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:8:37
2019-12-08T23:11:50.6044439Z    |
2019-12-08T23:11:50.6044483Z LL |   opt.and_then(|arg| Some(takes_ref(arg)));
2019-12-08T23:11:50.6044776Z    |       --------                      ^^^ expected `&Foo`, found struct `Foo`
2019-12-08T23:11:50.6044872Z    |       help: consider using `as_ref` instead: `as_ref().and_then`
2019-12-08T23:11:50.6044919Z 
2019-12-08T23:11:50.6044961Z error[E0308]: mismatched types
2019-12-08T23:11:50.6045192Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:11:27
2019-12-08T23:11:50.6045192Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:11:27
2019-12-08T23:11:50.6045238Z    |
2019-12-08T23:11:50.6045287Z LL |   opt.map(|arg| takes_ref(arg));
2019-12-08T23:11:50.6045527Z    |       ---                 ^^^ expected `&Foo`, found struct `Foo`
2019-12-08T23:11:50.6045779Z    |       help: consider using `as_ref` instead: `as_ref().map`
2019-12-08T23:11:50.6045987Z 
2019-12-08T23:11:50.6046019Z error[E0308]: mismatched types
2019-12-08T23:11:50.6046195Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:13:35
2019-12-08T23:11:50.6046195Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:13:35
2019-12-08T23:11:50.6046240Z    |
2019-12-08T23:11:50.6046275Z LL |   opt.and_then(|arg| Ok(takes_ref(arg)));
2019-12-08T23:11:50.6046476Z    |       --------                    ^^^ expected `&Foo`, found struct `Foo`
2019-12-08T23:11:50.6046561Z    |       help: consider using `as_ref` instead: `as_ref().and_then`
2019-12-08T23:11:50.6046585Z 
2019-12-08T23:11:50.6046623Z error[E0308]: mismatched types
2019-12-08T23:11:50.6046801Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:16:27
2019-12-08T23:11:50.6046801Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:16:27
2019-12-08T23:11:50.6046838Z    |
2019-12-08T23:11:50.6046871Z LL |   let y: Option<&usize> = x;
2019-12-08T23:11:50.6047074Z    |          --------------   ^ expected enum `std::option::Option`, found reference
2019-12-08T23:11:50.6047145Z    |          expected due to this
2019-12-08T23:11:50.6047183Z    |
2019-12-08T23:11:50.6047183Z    |
2019-12-08T23:11:50.6047218Z    = note:   expected enum `std::option::Option<&usize>`
2019-12-08T23:11:50.6047255Z            found reference `&std::option::Option<usize>`
2019-12-08T23:11:50.6047301Z help: you can convert from `&Option<T>` to `Option<&T>` using `.as_ref()`
2019-12-08T23:11:50.6047430Z    |
2019-12-08T23:11:50.6047473Z LL |   let y: Option<&usize> = x.as_ref();
2019-12-08T23:11:50.6047599Z help: consider dereferencing the borrow
2019-12-08T23:11:50.6047630Z    |
2019-12-08T23:11:50.6047630Z    |
2019-12-08T23:11:50.6047662Z LL |   let y: Option<&usize> = *x;
2019-12-08T23:11:50.6047723Z 
2019-12-08T23:11:50.6047755Z error[E0308]: mismatched types
2019-12-08T23:11:50.6047964Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:19:35
2019-12-08T23:11:50.6048006Z    |
2019-12-08T23:11:50.6048006Z    |
2019-12-08T23:11:50.6048039Z LL |   let y: Result<&usize, &usize> = x;
2019-12-08T23:11:50.6048244Z    |          ----------------------   ^ expected enum `std::result::Result`, found reference
2019-12-08T23:11:50.6048326Z    |          expected due to this
2019-12-08T23:11:50.6048357Z    |
2019-12-08T23:11:50.6048357Z    |
2019-12-08T23:11:50.6048405Z    = note:   expected enum `std::result::Result<&usize, &usize>`
2019-12-08T23:11:50.6048444Z            found reference `&std::result::Result<usize, usize>`
2019-12-08T23:11:50.6048490Z help: you can convert from `&Result<T, E>` to `Result<&T, &E>` using `.as_ref()`
2019-12-08T23:11:50.6048529Z    |
2019-12-08T23:11:50.6048563Z LL |   let y: Result<&usize, &usize> = x.as_ref();
2019-12-08T23:11:50.6048639Z help: consider dereferencing the borrow
2019-12-08T23:11:50.6048671Z    |
2019-12-08T23:11:50.6048671Z    |
2019-12-08T23:11:50.6048703Z LL |   let y: Result<&usize, &usize> = *x;
2019-12-08T23:11:50.6048765Z 
2019-12-08T23:11:50.6048796Z error[E0308]: mismatched types
2019-12-08T23:11:50.6048981Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:23:34
2019-12-08T23:11:50.6049023Z    |
2019-12-08T23:11:50.6049023Z    |
2019-12-08T23:11:50.6049057Z LL |   let y: Result<&usize, usize> = x;
2019-12-08T23:11:50.6049219Z    |          ---------------------   ^
2019-12-08T23:11:50.6049306Z    |          |                       expected enum `std::result::Result`, found reference
2019-12-08T23:11:50.6049306Z    |          |                       expected enum `std::result::Result`, found reference
2019-12-08T23:11:50.6049354Z    |          |                       help: consider dereferencing the borrow: `*x`
2019-12-08T23:11:50.6049427Z    |
2019-12-08T23:11:50.6049427Z    |
2019-12-08T23:11:50.6049461Z    = note:   expected enum `std::result::Result<&usize, usize>`
2019-12-08T23:11:50.6049499Z            found reference `&std::result::Result<usize, usize>`
2019-12-08T23:11:50.6049560Z error: aborting due to 7 previous errors
2019-12-08T23:11:50.6049581Z 
2019-12-08T23:11:50.6049768Z For more information about this error, try `rustc --explain E0308`.
2019-12-08T23:11:50.6049801Z 
2019-12-08T23:11:50.6049801Z 
2019-12-08T23:11:50.6049961Z ------------------------------------------
2019-12-08T23:11:50.6049985Z 
2019-12-08T23:11:50.6050004Z 
2019-12-08T23:11:50.6050193Z ---- [ui] ui/suggestions/let-binding-init-expr-as-ty.rs stdout ----
2019-12-08T23:11:50.6050229Z diff of stderr:
2019-12-08T23:11:50.6050250Z 
2019-12-08T23:11:50.6050289Z 15    |                   only `Fn` traits may use parentheses
2019-12-08T23:11:50.6050334Z 16    |                   help: use angle brackets instead: `from_be<num>`
2019-12-08T23:11:50.6050549Z - error[E0109]: type arguments are not allowed for this type
2019-12-08T23:11:50.6050727Z -   --> $DIR/let-binding-init-expr-as-ty.rs:2:27
2019-12-08T23:11:50.6050865Z -    |
2019-12-08T23:11:50.6050865Z -    |
2019-12-08T23:11:50.6051025Z - LL |     let foo: i32::from_be(num);
2019-12-08T23:11:50.6051210Z -    |                           ^^^ type argument not allowed
2019-12-08T23:11:50.6051379Z 24 error[E0223]: ambiguous associated type
2019-12-08T23:11:50.6051550Z 25   --> $DIR/let-binding-init-expr-as-ty.rs:2:14
2019-12-08T23:11:50.6051591Z 26    |
2019-12-08T23:11:50.6051610Z 
2019-12-08T23:11:50.6051610Z 
2019-12-08T23:11:50.6051643Z 27 LL |     let foo: i32::from_be(num);
2019-12-08T23:11:50.6052296Z 28    |              ^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<i32 as Trait>::from_be`
2019-12-08T23:11:50.6052995Z - error: aborting due to 4 previous errors
2019-12-08T23:11:50.6053044Z + error: aborting due to 3 previous errors
2019-12-08T23:11:50.6053096Z 31 
2019-12-08T23:11:50.6053346Z - Some errors have detailed explanations: E0109, E0214, E0223, E0573.
2019-12-08T23:11:50.6053346Z - Some errors have detailed explanations: E0109, E0214, E0223, E0573.
2019-12-08T23:11:50.6053588Z - For more information about an error, try `rustc --explain E0109`.
2019-12-08T23:11:50.6053650Z + Some errors have detailed explanations: E0214, E0223, E0573.
2019-12-08T23:11:50.6053891Z + For more information about an error, try `rustc --explain E0214`.
2019-12-08T23:11:50.6053936Z 34 
2019-12-08T23:11:50.6053969Z 
2019-12-08T23:11:50.6053995Z 
2019-12-08T23:11:50.6054039Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.6054374Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/let-binding-init-expr-as-ty/let-binding-init-expr-as-ty.stderr
2019-12-08T23:11:50.6054632Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.6054918Z To only update this specific test, also pass `--test-args suggestions/let-binding-init-expr-as-ty.rs`
2019-12-08T23:11:50.6055005Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.6055050Z status: exit code: 1
2019-12-08T23:11:50.6055050Z status: exit code: 1
2019-12-08T23:11:50.6055859Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/let-binding-init-expr-as-ty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/let-binding-init-expr-as-ty" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/let-binding-init-expr-as-ty/auxiliary" "-A" "unused"
2019-12-08T23:11:50.6056291Z ------------------------------------------
2019-12-08T23:11:50.6056329Z 
2019-12-08T23:11:50.6056492Z ------------------------------------------
2019-12-08T23:11:50.6056527Z stderr:
2019-12-08T23:11:50.6056527Z stderr:
2019-12-08T23:11:50.6056691Z ------------------------------------------
2019-12-08T23:11:50.6056730Z error[E0573]: expected type, found local variable `num`
2019-12-08T23:11:50.6056925Z   --> /checkout/src/test/ui/suggestions/let-binding-init-expr-as-ty.rs:2:27
2019-12-08T23:11:50.6056971Z    |
2019-12-08T23:11:50.6057005Z LL |     let foo: i32::from_be(num);
2019-12-08T23:11:50.6057216Z    |            |
2019-12-08T23:11:50.6057216Z    |            |
2019-12-08T23:11:50.6057253Z    |            help: use `=` if you meant to assign
2019-12-08T23:11:50.6057314Z error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
2019-12-08T23:11:50.6057517Z   --> /checkout/src/test/ui/suggestions/let-binding-init-expr-as-ty.rs:2:19
2019-12-08T23:11:50.6057555Z    |
2019-12-08T23:11:50.6057555Z    |
2019-12-08T23:11:50.6057594Z LL |     let foo: i32::from_be(num);
2019-12-08T23:11:50.6057668Z    |                   |
2019-12-08T23:11:50.6057668Z    |                   |
2019-12-08T23:11:50.6057705Z    |                   only `Fn` traits may use parentheses
2019-12-08T23:11:50.6057750Z    |                   help: use angle brackets instead: `from_be<num>`
2019-12-08T23:11:50.6057806Z error[E0223]: ambiguous associated type
2019-12-08T23:11:50.6058003Z   --> /checkout/src/test/ui/suggestions/let-binding-init-expr-as-ty.rs:2:14
2019-12-08T23:11:50.6058045Z    |
2019-12-08T23:11:50.6058045Z    |
2019-12-08T23:11:50.6058078Z LL |     let foo: i32::from_be(num);
2019-12-08T23:11:50.6058284Z    |              ^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<i32 as Trait>::from_be`
2019-12-08T23:11:50.6058351Z error: aborting due to 3 previous errors
2019-12-08T23:11:50.6058438Z 
2019-12-08T23:11:50.6058480Z Some errors have detailed explanations: E0214, E0223, E0573.
2019-12-08T23:11:50.6058749Z For more information about an error, try `rustc --explain E0214`.
---
2019-12-08T23:11:50.6059487Z 40 LL |     foo("");
2019-12-08T23:11:50.6059654Z -    |         ^^ expected `usize`, found `&str`
2019-12-08T23:11:50.6059696Z +    |         ^^
2019-12-08T23:11:50.6059729Z +    |         |
2019-12-08T23:11:50.6059763Z +    |         expected `usize`, found `&str`
2019-12-08T23:11:50.6059812Z +    |         help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.6059879Z 43 error: aborting due to 6 previous errors
2019-12-08T23:11:50.6059922Z 44 
2019-12-08T23:11:50.6059943Z 
2019-12-08T23:11:50.6059962Z 
2019-12-08T23:11:50.6059962Z 
2019-12-08T23:11:50.6059996Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.6060274Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/recover-from-semicolon-trailing-item/recover-from-semicolon-trailing-item.stderr
2019-12-08T23:11:50.6060470Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.6060691Z To only update this specific test, also pass `--test-args suggestions/recover-from-semicolon-trailing-item.rs`
2019-12-08T23:11:50.6060759Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.6060792Z status: exit code: 1
2019-12-08T23:11:50.6060792Z status: exit code: 1
2019-12-08T23:11:50.6061440Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/recover-from-semicolon-trailing-item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/recover-from-semicolon-trailing-item" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/recover-from-semicolon-trailing-item/auxiliary" "-A" "unused"
2019-12-08T23:11:50.6061690Z ------------------------------------------
2019-12-08T23:11:50.6061715Z 
2019-12-08T23:11:50.6062402Z ------------------------------------------
2019-12-08T23:11:50.6062454Z stderr:
2019-12-08T23:11:50.6062454Z stderr:
2019-12-08T23:11:50.6062692Z ------------------------------------------
2019-12-08T23:11:50.6062738Z error: expected item, found `;`
2019-12-08T23:11:50.6063004Z   --> /checkout/src/test/ui/suggestions/recover-from-semicolon-trailing-item.rs:2:9
2019-12-08T23:11:50.6063062Z    |
2019-12-08T23:11:50.6063105Z LL | mod M {};
2019-12-08T23:11:50.6063193Z 
2019-12-08T23:11:50.6063234Z error: expected item, found `;`
2019-12-08T23:11:50.6063493Z   --> /checkout/src/test/ui/suggestions/recover-from-semicolon-trailing-item.rs:4:12
2019-12-08T23:11:50.6063541Z    |
---
2019-12-08T23:11:50.6064336Z 
2019-12-08T23:11:50.6064379Z error[E0308]: mismatched types
2019-12-08T23:11:50.6064720Z   --> /checkout/src/test/ui/suggestions/recover-from-semicolon-trailing-item.rs:10:20
2019-12-08T23:11:50.6064775Z    |
2019-12-08T23:11:50.6064818Z LL |     let _: usize = S {};
2019-12-08T23:11:50.6065054Z    |            -----   ^^^^ expected `usize`, found struct `S`
2019-12-08T23:11:50.6065152Z    |            expected due to this
2019-12-08T23:11:50.6065181Z 
2019-12-08T23:11:50.6065221Z error[E0308]: mismatched types
2019-12-08T23:11:50.6065484Z   --> /checkout/src/test/ui/suggestions/recover-from-semicolon-trailing-item.rs:12:20
2019-12-08T23:11:50.6065484Z   --> /checkout/src/test/ui/suggestions/recover-from-semicolon-trailing-item.rs:12:20
2019-12-08T23:11:50.6065532Z    |
2019-12-08T23:11:50.6065748Z LL |     let _: usize = X {};
2019-12-08T23:11:50.6065927Z    |            -----   ^^^^ expected `usize`, found struct `main::X`
2019-12-08T23:11:50.6066007Z    |            expected due to this
2019-12-08T23:11:50.6066028Z 
2019-12-08T23:11:50.6066063Z error[E0308]: mismatched types
2019-12-08T23:11:50.6066265Z   --> /checkout/src/test/ui/suggestions/recover-from-semicolon-trailing-item.rs:14:9
2019-12-08T23:11:50.6066265Z   --> /checkout/src/test/ui/suggestions/recover-from-semicolon-trailing-item.rs:14:9
2019-12-08T23:11:50.6066301Z    |
2019-12-08T23:11:50.6066331Z LL |     foo("");
2019-12-08T23:11:50.6066366Z    |         ^^
2019-12-08T23:11:50.6066396Z    |         |
2019-12-08T23:11:50.6066429Z    |         expected `usize`, found `&str`
2019-12-08T23:11:50.6066471Z    |         help: consider dereferencing the borrow: `*""`
2019-12-08T23:11:50.6066525Z error: aborting due to 6 previous errors
2019-12-08T23:11:50.6066546Z 
2019-12-08T23:11:50.6066731Z For more information about this error, try `rustc --explain E0308`.
2019-12-08T23:11:50.6066755Z 
---
2019-12-08T23:11:50.6067164Z diff of stderr:
2019-12-08T23:11:50.6067184Z 
2019-12-08T23:11:50.6067342Z 2   --> $DIR/type-shadow.rs:6:20
2019-12-08T23:11:50.6067382Z 3    |
2019-12-08T23:11:50.6067414Z 4 LL |         let y: Y = "hello";
2019-12-08T23:11:50.6067590Z -    |                -   ^^^^^^^ expected `isize`, found `&str`
2019-12-08T23:11:50.6067895Z +    |                -   ^^^^^^^
2019-12-08T23:11:50.6067930Z +    |                |   |
2019-12-08T23:11:50.6067971Z +    |                |   expected `isize`, found `&str`
2019-12-08T23:11:50.6067971Z +    |                |   expected `isize`, found `&str`
2019-12-08T23:11:50.6068010Z +    |                |   help: consider dereferencing the borrow: `*"hello"`
2019-12-08T23:11:50.6068083Z 8 
2019-12-08T23:11:50.6068115Z 9 error: aborting due to previous error
2019-12-08T23:11:50.6068136Z 
2019-12-08T23:11:50.6068155Z 
2019-12-08T23:11:50.6068155Z 
2019-12-08T23:11:50.6068192Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.6068420Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-shadow/type-shadow.stderr
2019-12-08T23:11:50.6068612Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.6068810Z To only update this specific test, also pass `--test-args type/type-shadow.rs`
2019-12-08T23:11:50.6068868Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.6068905Z status: exit code: 1
2019-12-08T23:11:50.6068905Z status: exit code: 1
2019-12-08T23:11:50.6069512Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-shadow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-shadow" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-shadow/auxiliary" "-A" "unused"
2019-12-08T23:11:50.6069860Z ------------------------------------------
2019-12-08T23:11:50.6069956Z 
2019-12-08T23:11:50.6070315Z ------------------------------------------
2019-12-08T23:11:50.6070358Z stderr:
2019-12-08T23:11:50.6070358Z stderr:
2019-12-08T23:11:50.6070520Z ------------------------------------------
2019-12-08T23:11:50.6070557Z error[E0308]: mismatched types
2019-12-08T23:11:50.6070736Z   --> /checkout/src/test/ui/type/type-shadow.rs:6:20
2019-12-08T23:11:50.6070771Z    |
2019-12-08T23:11:50.6070807Z LL |         let y: Y = "hello"; //~ ERROR mismatched types
2019-12-08T23:11:50.6070983Z    |                -   ^^^^^^^
2019-12-08T23:11:50.6071060Z    |                |   expected `isize`, found `&str`
2019-12-08T23:11:50.6071060Z    |                |   expected `isize`, found `&str`
2019-12-08T23:11:50.6071107Z    |                |   help: consider dereferencing the borrow: `*"hello"`
2019-12-08T23:11:50.6071171Z 
2019-12-08T23:11:50.6071212Z error: aborting due to previous error
2019-12-08T23:11:50.6071240Z 
2019-12-08T23:11:50.6071444Z For more information about this error, try `rustc --explain E0308`.
---
2019-12-08T23:11:50.6073305Z 7 LL | |         None => &R,
2019-12-08T23:11:50.6073547Z -    | |                 ^^ expected struct `S`, found struct `R`
2019-12-08T23:11:50.6073597Z +    | |                 ^^
2019-12-08T23:11:50.6073641Z +    | |                 |
2019-12-08T23:11:50.6073695Z +    | |                 expected struct `S`, found struct `R`
2019-12-08T23:11:50.6073756Z +    | |                 help: consider dereferencing the borrow: `*&R`
2019-12-08T23:11:50.6074042Z 10    | |_____- `match` arms have incompatible types
2019-12-08T23:11:50.6074089Z 11    |
2019-12-08T23:11:50.6074115Z 
2019-12-08T23:11:50.6074141Z 
2019-12-08T23:11:50.6074141Z 
2019-12-08T23:11:50.6074192Z The actual stderr differed from the expected stderr.
2019-12-08T23:11:50.6074506Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-unsafe-trait-obj-match/wf-unsafe-trait-obj-match.stderr
2019-12-08T23:11:50.6074771Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T23:11:50.6075067Z To only update this specific test, also pass `--test-args wf/wf-unsafe-trait-obj-match.rs`
2019-12-08T23:11:50.6075149Z error: 1 errors occurred comparing output.
2019-12-08T23:11:50.6075201Z status: exit code: 1
2019-12-08T23:11:50.6075201Z status: exit code: 1
2019-12-08T23:11:50.6076127Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/wf-unsafe-trait-obj-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-unsafe-trait-obj-match" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-unsafe-trait-obj-match/auxiliary" "-A" "unused"
2019-12-08T23:11:50.6076553Z ------------------------------------------
2019-12-08T23:11:50.6076579Z 
2019-12-08T23:11:50.6076740Z ------------------------------------------
2019-12-08T23:11:50.6076781Z stderr:
2019-12-08T23:11:50.6076781Z stderr:
2019-12-08T23:11:50.6076942Z ------------------------------------------
2019-12-08T23:11:50.6076979Z error[E0308]: match arms have incompatible types
2019-12-08T23:11:50.6077171Z   --> /checkout/src/test/ui/wf/wf-unsafe-trait-obj-match.rs:23:17
2019-12-08T23:11:50.6077288Z    |
2019-12-08T23:11:50.6077329Z LL | /     match opt() {
2019-12-08T23:11:50.6077370Z LL | |         Some(()) => &S,
2019-12-08T23:11:50.6077624Z    | |                     -- this is found to be of type `&S`
2019-12-08T23:11:50.6077664Z LL | |         None => &R,  //~ ERROR E0308
2019-12-08T23:11:50.6077738Z    | |                 |
2019-12-08T23:11:50.6077774Z    | |                 expected struct `S`, found struct `R`
2019-12-08T23:11:50.6077774Z    | |                 expected struct `S`, found struct `R`
2019-12-08T23:11:50.6077818Z    | |                 help: consider dereferencing the borrow: `*&R`
2019-12-08T23:11:50.6078026Z    | |_____- `match` arms have incompatible types
2019-12-08T23:11:50.6078061Z    |
2019-12-08T23:11:50.6078099Z    = note:   expected type `&S`
2019-12-08T23:11:50.6078132Z            found reference `&R`
2019-12-08T23:11:50.6078132Z            found reference `&R`
2019-12-08T23:11:50.6078154Z 
2019-12-08T23:11:50.6078193Z error[E0038]: the trait `Trait` cannot be made into an object
2019-12-08T23:11:50.6078392Z   --> /checkout/src/test/ui/wf/wf-unsafe-trait-obj-match.rs:26:21
2019-12-08T23:11:50.6078428Z    |
2019-12-08T23:11:50.6078467Z LL |         Some(()) => &S, //~ ERROR E0038
2019-12-08T23:11:50.6078511Z    |                     ^^ the trait `Trait` cannot be made into an object
2019-12-08T23:11:50.6078580Z    = note: the trait cannot require that `Self : Sized`
2019-12-08T23:11:50.6078580Z    = note: the trait cannot require that `Self : Sized`
2019-12-08T23:11:50.6078628Z    = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<&dyn Trait>` for `&S`
2019-12-08T23:11:50.6078671Z    = note: required by cast to type `&dyn Trait`
2019-12-08T23:11:50.6078735Z error[E0038]: the trait `Trait` cannot be made into an object
2019-12-08T23:11:50.6078924Z   --> /checkout/src/test/ui/wf/wf-unsafe-trait-obj-match.rs:25:25
2019-12-08T23:11:50.6078960Z    |
2019-12-08T23:11:50.6078960Z    |
2019-12-08T23:11:50.6078999Z LL |     let t: &dyn Trait = match opt() { //~ ERROR E0038
2019-12-08T23:11:50.6079046Z    |                         ^^^^^^^^^^^ the trait `Trait` cannot be made into an object
2019-12-08T23:11:50.6079119Z    = note: the trait cannot require that `Self : Sized`
2019-12-08T23:11:50.6079119Z    = note: the trait cannot require that `Self : Sized`
2019-12-08T23:11:50.6079169Z    = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<&dyn Trait>` for `&R`
2019-12-08T23:11:50.6079209Z    = note: required by cast to type `&dyn Trait`
2019-12-08T23:11:50.6079268Z error: aborting due to 3 previous errors
2019-12-08T23:11:50.6079290Z 
2019-12-08T23:11:50.6079324Z Some errors have detailed explanations: E0038, E0308.
2019-12-08T23:11:50.6079514Z For more information about an error, try `rustc --explain E0038`.
---
2019-12-08T23:11:50.6087381Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-08T23:11:50.6087428Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-08T23:11:50.6087452Z 
2019-12-08T23:11:50.6087470Z 
2019-12-08T23:11:50.6088587Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-08T23:11:50.6088742Z 
2019-12-08T23:11:50.6088762Z 
2019-12-08T23:11:50.6088805Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-08T23:11:50.6088842Z Build completed unsuccessfully in 1:02:27
2019-12-08T23:11:50.6088842Z Build completed unsuccessfully in 1:02:27
2019-12-08T23:11:50.6088879Z == clock drift check ==
2019-12-08T23:11:50.6088917Z   local time: Sun Dec  8 23:11:50 UTC 2019
2019-12-08T23:11:50.8523588Z   network time: Sun, 08 Dec 2019 23:11:50 GMT
2019-12-08T23:11:50.8527554Z == end clock drift check ==
2019-12-08T23:11:51.6068142Z 
2019-12-08T23:11:51.6173998Z ##[error]Bash exited with code '1'.
2019-12-08T23:11:51.6207503Z ##[section]Starting: Checkout
2019-12-08T23:11:51.6209361Z ==============================================================================
2019-12-08T23:11:51.6209403Z Task         : Get sources
2019-12-08T23:11:51.6209454Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
