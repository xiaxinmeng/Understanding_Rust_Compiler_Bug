plain
2019-10-25T01:12:08.2247197Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T01:12:08.2431390Z ##[command]git config gc.auto 0
2019-10-25T01:12:08.2513993Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T01:12:08.2591695Z ##[command]git config --get-all http.proxy
2019-10-25T01:12:08.2715265Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65752/merge:refs/remotes/pull/65752/merge
---
2019-10-25T02:16:16.7001435Z .................................................................................................... 1600/9242
2019-10-25T02:16:22.2140885Z .................................................................................................... 1700/9242
2019-10-25T02:16:35.1423123Z .....................................................i...............i.............................. 1800/9242
2019-10-25T02:16:43.2633286Z .................................................................................................... 1900/9242
2019-10-25T02:16:58.0352383Z ...........................................iiiii.................................................... 2000/9242
2019-10-25T02:17:09.0975229Z .................................................................................................... 2200/9242
2019-10-25T02:17:11.7512695Z .................................................................................................... 2300/9242
2019-10-25T02:17:15.8821129Z .................................................................................................... 2400/9242
2019-10-25T02:17:39.8883970Z .................................................................................................... 2500/9242
---
2019-10-25T02:20:36.7777118Z ...............................................i...............i.................................... 4800/9242
2019-10-25T02:20:45.8632339Z .................................................................................................... 4900/9242
2019-10-25T02:20:54.8507683Z .................................................................................................... 5000/9242
2019-10-25T02:21:01.8157676Z .................................................................................................... 5100/9242
2019-10-25T02:21:11.9322973Z ...............................................ii.ii................................................ 5200/9242
2019-10-25T02:21:22.0444229Z .................................................................................................... 5400/9242
2019-10-25T02:21:31.9324416Z .................................................................................................... 5500/9242
2019-10-25T02:21:39.3723458Z ..............i..................................................................................... 5600/9242
2019-10-25T02:21:44.9575390Z .................................................................................................... 5700/9242
2019-10-25T02:21:44.9575390Z .................................................................................................... 5700/9242
2019-10-25T02:21:57.1362491Z .................................................................................................... 5800/9242
2019-10-25T02:22:09.5907948Z .....F.....ii...i..ii...........i................................................................... 5900/9242
2019-10-25T02:22:31.1095275Z .................................................................................................... 6100/9242
2019-10-25T02:22:39.9159773Z .................................................................................................... 6200/9242
2019-10-25T02:22:39.9159773Z .................................................................................................... 6200/9242
2019-10-25T02:22:57.4683109Z .................................i..ii.............................................................. 6300/9242
2019-10-25T02:23:18.8482656Z ...................................................................................................i 6500/9242
2019-10-25T02:23:21.1239783Z .................................................................................................... 6600/9242
2019-10-25T02:23:23.4598154Z ..........................................................................i......................... 6700/9242
2019-10-25T02:23:26.2311204Z .................................................................................................... 6800/9242
---
2019-10-25T02:27:37.0025541Z 28    |
2019-10-25T02:27:37.0025811Z 29 LL | impl std::fmt::Display for MyType4 {}
2019-10-25T02:27:37.0026117Z 30    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `fmt` in implementation
2019-10-25T02:27:37.0026486Z -    |
2019-10-25T02:27:37.0027071Z 32    = help: implement the missing item: `fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { unimplemented!() }`
2019-10-25T02:27:37.0027406Z 34 error: aborting due to 4 previous errors
2019-10-25T02:27:37.0027522Z 
2019-10-25T02:27:37.0027648Z 
2019-10-25T02:27:37.0027778Z The actual stderr differed from the expected stderr.
2019-10-25T02:27:37.0027778Z The actual stderr differed from the expected stderr.
2019-10-25T02:27:37.0028178Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/trait_type/trait_type.stderr
2019-10-25T02:27:37.0029300Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T02:27:37.0029849Z To only update this specific test, also pass `--test-args impl-trait/trait_type.rs`
2019-10-25T02:27:37.0030217Z error: 1 errors occurred comparing output.
2019-10-25T02:27:37.0030380Z status: exit code: 1
2019-10-25T02:27:37.0030380Z status: exit code: 1
2019-10-25T02:27:37.0031268Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/trait_type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/trait_type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/trait_type/auxiliary" "-A" "unused"
2019-10-25T02:27:37.0032547Z ------------------------------------------
2019-10-25T02:27:37.0032712Z 
2019-10-25T02:27:37.0033039Z ------------------------------------------
2019-10-25T02:27:37.0033221Z stderr:
2019-10-25T02:27:37.0033221Z stderr:
2019-10-25T02:27:37.0033544Z ------------------------------------------
2019-10-25T02:27:37.0033852Z error[E0053]: method `fmt` has an incompatible type for trait
2019-10-25T02:27:37.0034341Z   --> /checkout/src/test/ui/impl-trait/trait_type.rs:7:4
2019-10-25T02:27:37.0036451Z    |
2019-10-25T02:27:37.0036899Z LL |    fn fmt(&self, x: &str) -> () { }
2019-10-25T02:27:37.0036969Z    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
2019-10-25T02:27:37.0037011Z    |
2019-10-25T02:27:37.0037280Z    = note: expected type `fn(&MyType, &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error>`
2019-10-25T02:27:37.0037353Z               found type `fn(&MyType, &str)`
2019-10-25T02:27:37.0037433Z error[E0050]: method `fmt` has 1 parameter but the declaration in trait `std::fmt::Display::fmt` has 2
2019-10-25T02:27:37.0037799Z   --> /checkout/src/test/ui/impl-trait/trait_type.rs:12:11
2019-10-25T02:27:37.0037841Z    |
2019-10-25T02:27:37.0037841Z    |
2019-10-25T02:27:37.0038024Z LL |    fn fmt(&self) -> () { }
2019-10-25T02:27:37.0038122Z    |
2019-10-25T02:27:37.0038122Z    |
2019-10-25T02:27:37.0039071Z    = note: `fmt` from trait: `fn(&Self, &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error>`
2019-10-25T02:27:37.0039149Z 
2019-10-25T02:27:37.0039227Z error[E0186]: method `fmt` has a `&self` declaration in the trait, but not in the impl
2019-10-25T02:27:37.0039632Z    |
2019-10-25T02:27:37.0039632Z    |
2019-10-25T02:27:37.0039854Z LL |    fn fmt() -> () { }
2019-10-25T02:27:37.0039904Z    |    ^^^^^^^^^^^^^^ expected `&self` in impl
2019-10-25T02:27:37.0039946Z    |
2019-10-25T02:27:37.0040246Z    = note: `fmt` from trait: `fn(&Self, &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error>`
2019-10-25T02:27:37.0040343Z error[E0046]: not all trait items implemented, missing: `fmt`
2019-10-25T02:27:37.0040585Z   --> /checkout/src/test/ui/impl-trait/trait_type.rs:21:1
2019-10-25T02:27:37.0040649Z    |
2019-10-25T02:27:37.0040693Z LL | impl std::fmt::Display for MyType4 {}
2019-10-25T02:27:37.0040693Z LL | impl std::fmt::Display for MyType4 {}
2019-10-25T02:27:37.0040750Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `fmt` in implementation
2019-10-25T02:27:37.0041097Z    = help: implement the missing item: `fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { unimplemented!() }`
2019-10-25T02:27:37.0041189Z error: aborting due to 4 previous errors
2019-10-25T02:27:37.0041234Z 
2019-10-25T02:27:37.0041280Z Some errors have detailed explanations: E0046, E0050, E0053, E0186.
2019-10-25T02:27:37.0041535Z For more information about an error, try `rustc --explain E0046`.
---
2019-10-25T02:27:37.0042466Z 3    |
2019-10-25T02:27:37.0042507Z 4 LL | impl PartialOrd for Thing {
2019-10-25T02:27:37.0042577Z 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^ missing `partial_cmp` in implementation
2019-10-25T02:27:37.0042758Z -    |
2019-10-25T02:27:37.0043049Z 7    = help: implement the missing item: `fn partial_cmp(&self, _: &Rhs) -> std::option::Option<std::cmp::Ordering> { unimplemented!() }`
2019-10-25T02:27:37.0043155Z 9 error: aborting due to previous error
2019-10-25T02:27:37.0043181Z 
2019-10-25T02:27:37.0043204Z 
2019-10-25T02:27:37.0043262Z The actual stderr differed from the expected stderr.
2019-10-25T02:27:37.0043262Z The actual stderr differed from the expected stderr.
2019-10-25T02:27:37.0043536Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3344/issue-3344.stderr
2019-10-25T02:27:37.0043907Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T02:27:37.0044171Z To only update this specific test, also pass `--test-args issues/issue-3344.rs`
2019-10-25T02:27:37.0044244Z error: 1 errors occurred comparing output.
2019-10-25T02:27:37.0044294Z status: exit code: 1
2019-10-25T02:27:37.0044294Z status: exit code: 1
2019-10-25T02:27:37.0045192Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3344.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3344" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3344/auxiliary" "-A" "unused"
2019-10-25T02:27:37.0045611Z ------------------------------------------
2019-10-25T02:27:37.0045651Z 
2019-10-25T02:27:37.0045869Z ------------------------------------------
2019-10-25T02:27:37.0045930Z stderr:
2019-10-25T02:27:37.0045930Z stderr:
2019-10-25T02:27:37.0046141Z ------------------------------------------
2019-10-25T02:27:37.0046191Z error[E0046]: not all trait items implemented, missing: `partial_cmp`
2019-10-25T02:27:37.0046525Z   --> /checkout/src/test/ui/issues/issue-3344.rs:3:1
2019-10-25T02:27:37.0046584Z    |
2019-10-25T02:27:37.0046633Z LL | impl PartialOrd for Thing { //~ ERROR not all trait items implemented, missing: `partial_cmp`
2019-10-25T02:27:37.0046706Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^ missing `partial_cmp` in implementation
2019-10-25T02:27:37.0047042Z    = help: implement the missing item: `fn partial_cmp(&self, _: &Rhs) -> std::option::Option<std::cmp::Ordering> { unimplemented!() }`
2019-10-25T02:27:37.0047144Z error: aborting due to previous error
2019-10-25T02:27:37.0047172Z 
2019-10-25T02:27:37.0047412Z For more information about this error, try `rustc --explain E0046`.
2019-10-25T02:27:37.0047454Z 
2019-10-25T02:27:37.0047454Z 
2019-10-25T02:27:37.0047681Z ------------------------------------------
2019-10-25T02:27:37.0047711Z 
2019-10-25T02:27:37.0047735Z 
2019-10-25T02:27:37.0048081Z ---- [ui] ui/missing/missing-items/m2.rs stdout ----
2019-10-25T02:27:37.0048143Z diff of stderr:
2019-10-25T02:27:37.0048177Z 
2019-10-25T02:27:37.0048328Z 3    |
2019-10-25T02:27:37.0048371Z 4 LL | impl m1::X for X {
2019-10-25T02:27:37.0048435Z 5    | ^^^^^^^^^^^^^^^^ missing `CONSTANT`, `Type`, `method` in implementation
2019-10-25T02:27:37.0049113Z 7    = help: implement the missing item: `const CONSTANT: u32 = 42;`
2019-10-25T02:27:37.0049187Z 8    = help: implement the missing item: `type Type = Type;`
2019-10-25T02:27:37.0049187Z 8    = help: implement the missing item: `type Type = Type;`
2019-10-25T02:27:37.0049507Z 9    = help: implement the missing item: `fn method(&self, _: std::string::String) -> <Self as m1::X>::Type { unimplemented!() }`
2019-10-25T02:27:37.0049571Z 
2019-10-25T02:27:37.0049646Z The actual stderr differed from the expected stderr.
2019-10-25T02:27:37.0049646Z The actual stderr differed from the expected stderr.
2019-10-25T02:27:37.0049938Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-items/m2/m2.stderr
2019-10-25T02:27:37.0050184Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T02:27:37.0050471Z To only update this specific test, also pass `--test-args missing/missing-items/m2.rs`
2019-10-25T02:27:37.0050549Z error: 1 errors occurred comparing output.
2019-10-25T02:27:37.0050592Z status: exit code: 1
2019-10-25T02:27:37.0050592Z status: exit code: 1
2019-10-25T02:27:37.0051327Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing/missing-items/m2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-items/m2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-items/m2/auxiliary" "-A" "unused"
2019-10-25T02:27:37.0051967Z ------------------------------------------
2019-10-25T02:27:37.0052000Z 
2019-10-25T02:27:37.0052197Z ------------------------------------------
2019-10-25T02:27:37.0052262Z stderr:
2019-10-25T02:27:37.0052262Z stderr:
2019-10-25T02:27:37.0052452Z ------------------------------------------
2019-10-25T02:27:37.0052500Z error[E0046]: not all trait items implemented, missing: `CONSTANT`, `Type`, `method`
2019-10-25T02:27:37.0052772Z    |
2019-10-25T02:27:37.0052772Z    |
2019-10-25T02:27:37.0052813Z LL | impl m1::X for X { //~ ERROR not all trait items implemented
2019-10-25T02:27:37.0052875Z    | ^^^^^^^^^^^^^^^^ missing `CONSTANT`, `Type`, `method` in implementation
2019-10-25T02:27:37.0052922Z    = help: implement the missing item: `const CONSTANT: u32 = 42;`
2019-10-25T02:27:37.0052977Z    = help: implement the missing item: `type Type = Type;`
2019-10-25T02:27:37.0053288Z    = help: implement the missing item: `fn method(&self, _: std::string::String) -> <Self as m1::X>::Type { unimplemented!() }`
2019-10-25T02:27:37.0053370Z error: aborting due to previous error
2019-10-25T02:27:37.0053397Z 
2019-10-25T02:27:37.0053731Z For more information about this error, try `rustc --explain E0046`.
2019-10-25T02:27:37.0053771Z 
2019-10-25T02:27:37.0053771Z 
2019-10-25T02:27:37.0054013Z ------------------------------------------
2019-10-25T02:27:37.0054044Z 
2019-10-25T02:27:37.0054086Z 
2019-10-25T02:27:37.0054311Z ---- [ui] ui/span/impl-wrong-item-for-trait.rs stdout ----
2019-10-25T02:27:37.0054355Z diff of stderr:
2019-10-25T02:27:37.0054381Z 
2019-10-25T02:27:37.0054433Z 63    |
2019-10-25T02:27:37.0054474Z 64 LL | impl Debug for FooTypeForMethod {
2019-10-25T02:27:37.0054518Z 65    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `fmt` in implementation
2019-10-25T02:27:37.0054717Z -    |
2019-10-25T02:27:37.0055031Z 67    = help: implement the missing item: `fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { unimplemented!() }`
2019-10-25T02:27:37.0055137Z 69 error: aborting due to 8 previous errors
2019-10-25T02:27:37.0055165Z 
2019-10-25T02:27:37.0055188Z 
2019-10-25T02:27:37.0055235Z The actual stderr differed from the expected stderr.
2019-10-25T02:27:37.0055235Z The actual stderr differed from the expected stderr.
2019-10-25T02:27:37.0055547Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/impl-wrong-item-for-trait/impl-wrong-item-for-trait.stderr
2019-10-25T02:27:37.0055899Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T02:27:37.0056280Z To only update this specific test, also pass `--test-args span/impl-wrong-item-for-trait.rs`
2019-10-25T02:27:37.0056374Z error: 1 errors occurred comparing output.
2019-10-25T02:27:37.0056416Z status: exit code: 1
2019-10-25T02:27:37.0056416Z status: exit code: 1
2019-10-25T02:27:37.0057282Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/impl-wrong-item-for-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/impl-wrong-item-for-trait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/impl-wrong-item-for-trait/auxiliary" "-A" "unused"
2019-10-25T02:27:37.0057618Z ------------------------------------------
2019-10-25T02:27:37.0057651Z 
2019-10-25T02:27:37.0057873Z ------------------------------------------
2019-10-25T02:27:37.0057917Z stderr:
2019-10-25T02:27:37.0057917Z stderr:
2019-10-25T02:27:37.0058148Z ------------------------------------------
2019-10-25T02:27:37.0058199Z error[E0437]: type `bar` is not a member of trait `Foo`
2019-10-25T02:27:37.0058446Z   --> /checkout/src/test/ui/span/impl-wrong-item-for-trait.rs:30:5
2019-10-25T02:27:37.0059558Z    |
2019-10-25T02:27:37.0059601Z LL |     type bar = u64;
2019-10-25T02:27:37.0059646Z    |     ^^^^^^^^^^^^^^^ not a member of trait `Foo`
2019-10-25T02:27:37.0059677Z 
2019-10-25T02:27:37.0060101Z error[E0323]: item `bar` is an associated const, which doesn't match its trait `Foo`
2019-10-25T02:27:37.0060425Z    |
2019-10-25T02:27:37.0060485Z LL |     fn bar(&self);
2019-10-25T02:27:37.0060709Z    |     -------------- item in trait
2019-10-25T02:27:37.0060754Z ...
---
2019-10-25T02:27:37.0061255Z    |
2019-10-25T02:27:37.0061296Z LL |     fn bar(&self);
2019-10-25T02:27:37.0061533Z    |     -------------- `bar` from trait
2019-10-25T02:27:37.0061577Z ...
2019-10-25T02:27:37.0061619Z LL | impl Foo for FooConstForMethod {
2019-10-25T02:27:37.0061684Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `bar` in implementation
2019-10-25T02:27:37.0061715Z 
2019-10-25T02:27:37.0062195Z error[E0324]: item `MY_CONST` is an associated method, which doesn't match its trait `Foo`
2019-10-25T02:27:37.0062643Z    |
2019-10-25T02:27:37.0062643Z    |
2019-10-25T02:27:37.0062683Z LL |     const MY_CONST: u32;
2019-10-25T02:27:37.0062893Z    |     -------------------- item in trait
2019-10-25T02:27:37.0062951Z ...
2019-10-25T02:27:37.0062991Z LL |     fn MY_CONST() {}
2019-10-25T02:27:37.0063034Z    |     ^^^^^^^^^^^^^^^^ does not match trait
2019-10-25T02:27:37.0063120Z error[E0046]: not all trait items implemented, missing: `MY_CONST`
2019-10-25T02:27:37.0063363Z   --> /checkout/src/test/ui/span/impl-wrong-item-for-trait.rs:19:1
2019-10-25T02:27:37.0063406Z    |
2019-10-25T02:27:37.0063406Z    |
2019-10-25T02:27:37.0063462Z LL |     const MY_CONST: u32;
2019-10-25T02:27:37.0063676Z    |     -------------------- `MY_CONST` from trait
2019-10-25T02:27:37.0063718Z ...
2019-10-25T02:27:37.0063773Z LL | impl Foo for FooMethodForConst {
2019-10-25T02:27:37.0063826Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `MY_CONST` in implementation
2019-10-25T02:27:37.0063855Z 
2019-10-25T02:27:37.0064095Z error[E0325]: item `bar` is an associated type, which doesn't match its trait `Foo`
2019-10-25T02:27:37.0064410Z    |
2019-10-25T02:27:37.0064450Z LL |     fn bar(&self);
2019-10-25T02:27:37.0064681Z    |     -------------- item in trait
2019-10-25T02:27:37.0064725Z ...
---
2019-10-25T02:27:37.0065222Z    |
2019-10-25T02:27:37.0065263Z LL |     fn bar(&self);
2019-10-25T02:27:37.0065490Z    |     -------------- `bar` from trait
2019-10-25T02:27:37.0065534Z ...
2019-10-25T02:27:37.0065593Z LL | impl Foo for FooTypeForMethod {
2019-10-25T02:27:37.0065743Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `bar` in implementation
2019-10-25T02:27:37.0065825Z error[E0046]: not all trait items implemented, missing: `fmt`
2019-10-25T02:27:37.0066045Z   --> /checkout/src/test/ui/span/impl-wrong-item-for-trait.rs:36:1
2019-10-25T02:27:37.0066086Z    |
2019-10-25T02:27:37.0066086Z    |
2019-10-25T02:27:37.0066139Z LL | impl Debug for FooTypeForMethod {
2019-10-25T02:27:37.0066183Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `fmt` in implementation
2019-10-25T02:27:37.0066464Z    = help: implement the missing item: `fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { unimplemented!() }`
2019-10-25T02:27:37.0066666Z error: aborting due to 8 previous errors
2019-10-25T02:27:37.0066692Z 
2019-10-25T02:27:37.0066739Z Some errors have detailed explanations: E0046, E0323, E0324, E0325, E0437.
2019-10-25T02:27:37.0067012Z For more information about an error, try `rustc --explain E0046`.
---
2019-10-25T02:27:37.0068524Z 
2019-10-25T02:27:37.0068792Z 
2019-10-25T02:27:37.0068853Z The actual stderr differed from the expected stderr.
2019-10-25T02:27:37.0069349Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-23729/issue-23729.stderr
2019-10-25T02:27:37.0069684Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T02:27:37.0069966Z To only update this specific test, also pass `--test-args span/issue-23729.rs`
2019-10-25T02:27:37.0070065Z error: 1 errors occurred comparing output.
2019-10-25T02:27:37.0070112Z status: exit code: 1
2019-10-25T02:27:37.0070112Z status: exit code: 1
2019-10-25T02:27:37.0070843Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-23729.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-23729" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-23729/auxiliary" "-A" "unused"
2019-10-25T02:27:37.0071202Z ------------------------------------------
2019-10-25T02:27:37.0071256Z 
2019-10-25T02:27:37.0071501Z ------------------------------------------
2019-10-25T02:27:37.0071549Z stderr:
---
2019-10-25T02:27:37.0073384Z ---- [ui] ui/span/issue-23827.rs stdout ----
2019-10-25T02:27:37.0073443Z diff of stderr:
2019-10-25T02:27:37.0073469Z 
2019-10-25T02:27:37.0073504Z 3    |
2019-10-25T02:27:37.0073562Z 4 LL | impl<C: Component> FnOnce<(C,)> for Prototype {
2019-10-25T02:27:37.0073612Z 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `Output` in implementation
2019-10-25T02:27:37.0080816Z 7    = help: implement the missing item: `type Output = Type;`
2019-10-25T02:27:37.0081073Z 8 
2019-10-25T02:27:37.0081117Z 9 error: aborting due to previous error
2019-10-25T02:27:37.0081149Z 
2019-10-25T02:27:37.0081149Z 
2019-10-25T02:27:37.0081194Z 
2019-10-25T02:27:37.0081240Z The actual stderr differed from the expected stderr.
2019-10-25T02:27:37.0081650Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-23827/issue-23827.stderr
2019-10-25T02:27:37.0081934Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T02:27:37.0082317Z To only update this specific test, also pass `--test-args span/issue-23827.rs`
2019-10-25T02:27:37.0082390Z error: 1 errors occurred comparing output.
2019-10-25T02:27:37.0082452Z status: exit code: 1
2019-10-25T02:27:37.0082452Z status: exit code: 1
2019-10-25T02:27:37.0083196Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-23827.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-23827" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-23827/auxiliary" "-A" "unused"
2019-10-25T02:27:37.0083595Z ------------------------------------------
2019-10-25T02:27:37.0083635Z 
2019-10-25T02:27:37.0083890Z ------------------------------------------
2019-10-25T02:27:37.0083931Z stderr:
2019-10-25T02:27:37.0083931Z stderr:
2019-10-25T02:27:37.0084120Z ------------------------------------------
2019-10-25T02:27:37.0084182Z error[E0046]: not all trait items implemented, missing: `Output`
2019-10-25T02:27:37.0084392Z   --> /checkout/src/test/ui/span/issue-23827.rs:26:1
2019-10-25T02:27:37.0084435Z    |
2019-10-25T02:27:37.0084492Z LL | impl<C: Component> FnOnce<(C,)> for Prototype {
2019-10-25T02:27:37.0084539Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `Output` in implementation
2019-10-25T02:27:37.0084594Z    = help: implement the missing item: `type Output = Type;`
2019-10-25T02:27:37.0084676Z error: aborting due to previous error
2019-10-25T02:27:37.0084701Z 
2019-10-25T02:27:37.0084918Z For more information about this error, try `rustc --explain E0046`.
2019-10-25T02:27:37.0084955Z 
---
2019-10-25T02:27:37.0085943Z 
2019-10-25T02:27:37.0085964Z 
2019-10-25T02:27:37.0086019Z The actual stderr differed from the expected stderr.
2019-10-25T02:27:37.0086277Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24356/issue-24356.stderr
2019-10-25T02:27:37.0086502Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T02:27:37.0086748Z To only update this specific test, also pass `--test-args span/issue-24356.rs`
2019-10-25T02:27:37.0086814Z error: 1 errors occurred comparing output.
2019-10-25T02:27:37.0086871Z status: exit code: 1
2019-10-25T02:27:37.0086871Z status: exit code: 1
2019-10-25T02:27:37.0087476Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-24356.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24356" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24356/auxiliary" "-A" "unused"
2019-10-25T02:27:37.0087886Z ------------------------------------------
2019-10-25T02:27:37.0087925Z 
2019-10-25T02:27:37.0088137Z ------------------------------------------
2019-10-25T02:27:37.0088177Z stderr:
---
2019-10-25T02:27:37.0092329Z test result: FAILED. 9195 passed; 7 failed; 40 ignored; 0 measured; 0 filtered out
2019-10-25T02:27:37.0092379Z 
2019-10-25T02:27:37.0096251Z 
2019-10-25T02:27:37.0096326Z 
2019-10-25T02:27:37.0098116Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-25T02:27:37.0099753Z 
2019-10-25T02:27:37.0099783Z 
2019-10-25T02:27:37.0100198Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-25T02:27:37.0100282Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-25T02:27:37.0100282Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-25T02:27:37.0100338Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-25T02:27:37.0100390Z Build completed unsuccessfully in 1:08:47
2019-10-25T02:27:37.0135302Z == clock drift check ==
2019-10-25T02:27:37.0154378Z   local time: Fri Oct 25 02:27:37 UTC 2019
2019-10-25T02:27:37.9684237Z   network time: Fri, 25 Oct 2019 02:27:37 GMT
2019-10-25T02:27:37.9685093Z == end clock drift check ==
2019-10-25T02:27:38.2179439Z 
2019-10-25T02:27:38.2296307Z ##[error]Bash exited with code '1'.
2019-10-25T02:27:38.2335703Z ##[section]Starting: Checkout
2019-10-25T02:27:38.2337438Z ==============================================================================
2019-10-25T02:27:38.2337489Z Task         : Get sources
2019-10-25T02:27:38.2337531Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
