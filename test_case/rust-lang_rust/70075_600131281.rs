plain
2020-03-17T14:09:54.4897209Z ========================== Starting Command Output ===========================
2020-03-17T14:09:54.4901956Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1aa11ed4-ca4a-4c54-b8d8-31b339cabe39.sh
2020-03-17T14:09:54.4902456Z 
2020-03-17T14:09:54.4907579Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T14:09:54.4928224Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70075/merge to s
2020-03-17T14:09:54.4931718Z Task         : Get sources
2020-03-17T14:09:54.4932067Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T14:09:54.4932380Z Version      : 1.0.0
2020-03-17T14:09:54.4932597Z Author       : Microsoft
---
2020-03-17T14:09:55.4776765Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T14:09:55.4781933Z ##[command]git config gc.auto 0
2020-03-17T14:09:55.4785496Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T14:09:55.4788800Z ##[command]git config --get-all http.proxy
2020-03-17T14:09:55.4794745Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70075/merge:refs/remotes/pull/70075/merge
---
2020-03-17T15:12:12.6961646Z .................................................................................................... 1700/9795
2020-03-17T15:12:17.5956780Z .................................................................................................... 1800/9795
2020-03-17T15:12:29.6124865Z ..........................................................................i......................... 1900/9795
2020-03-17T15:12:36.3256673Z .................................................................................................... 2000/9795
2020-03-17T15:12:44.7144789Z ................................................................iiiii............................... 2100/9795
2020-03-17T15:13:03.3079347Z .................................................................................................... 2300/9795
2020-03-17T15:13:05.4889774Z .................................................................................................... 2400/9795
2020-03-17T15:13:08.3710351Z .................................................................................................... 2500/9795
2020-03-17T15:13:29.1581432Z .................................................................................................... 2600/9795
---
2020-03-17T15:16:14.3958069Z ....................................i...............i............................................... 5000/9795
2020-03-17T15:16:23.6997113Z .................................................................................................... 5100/9795
2020-03-17T15:16:30.6944827Z ...............................................................................i.................... 5200/9795
2020-03-17T15:16:36.6894751Z .................................................................................................... 5300/9795
2020-03-17T15:16:47.6004507Z ............................................................ii.ii........i...i...................... 5400/9795
2020-03-17T15:16:51.9388697Z ...................................................................................................i 5500/9795
2020-03-17T15:17:06.1394064Z .................................................................................................... 5700/9795
2020-03-17T15:17:12.3359033Z .....................................................i.............................................. 5800/9795
2020-03-17T15:17:18.7541876Z ...................................................FF............................................... 5900/9795
2020-03-17T15:17:28.4131576Z .................................................................................................... 6000/9795
2020-03-17T15:17:28.4131576Z .................................................................................................... 6000/9795
2020-03-17T15:17:34.6655754Z ...............................................ii...i..ii...........i............................... 6100/9795
2020-03-17T15:17:54.8983053Z .................................................................................................... 6300/9795
2020-03-17T15:18:02.0247275Z .................................................................................................... 6400/9795
2020-03-17T15:18:02.0247275Z .................................................................................................... 6400/9795
2020-03-17T15:18:10.7259688Z .............................................................................i..ii.................. 6500/9795
2020-03-17T15:18:38.1536266Z .................................................................................................... 6700/9795
2020-03-17T15:18:47.5542725Z ...........................................................................i........................ 6800/9795
2020-03-17T15:18:49.5857840Z .................................................................................................... 6900/9795
2020-03-17T15:18:51.7185267Z .................................................................................................... 7000/9795
---
2020-03-17T15:20:36.2235618Z .................................................................................................... 7800/9795
2020-03-17T15:20:41.5634990Z .................................................................................................... 7900/9795
2020-03-17T15:20:47.7871450Z ............................................................i....................................... 8000/9795
2020-03-17T15:20:57.7365031Z .................................................................................................... 8100/9795
2020-03-17T15:21:03.3731305Z .........iiiiiiiiii.i............................................................................... 8200/9795
2020-03-17T15:21:17.2460173Z .................................................................................................... 8400/9795
2020-03-17T15:21:25.2136595Z .................................................................................................... 8500/9795
2020-03-17T15:21:39.0311779Z .................................................................................................... 8600/9795
2020-03-17T15:21:45.6207374Z .................................................................................................... 8700/9795
---
2020-03-17T15:23:41.0188220Z diff of stderr:
2020-03-17T15:23:41.0188481Z 
2020-03-17T15:23:41.0188894Z 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-17T15:23:41.0189333Z 6    |
2020-03-17T15:23:41.0189692Z 7    = note: expanding `println! { "Hello, World!" }`
2020-03-17T15:23:41.0190768Z -    = note: to `{ $crate :: io :: _print ($crate :: format_args_nl ! ("Hello, World!")) ; }`
2020-03-17T15:23:41.0191654Z +    = note: to `{ $crate :: io :: _print($crate :: format_args_nl !("Hello, World!")) ; }`
2020-03-17T15:23:41.0192554Z 10 
2020-03-17T15:23:41.0192766Z 
2020-03-17T15:23:41.0193173Z 
2020-03-17T15:23:41.0193548Z The actual stderr differed from the expected stderr.
2020-03-17T15:23:41.0193548Z The actual stderr differed from the expected stderr.
2020-03-17T15:23:41.0194768Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace-macro/trace-macro.stderr
2020-03-17T15:23:41.0195831Z To update references, rerun the tests and pass the `--bless` flag
2020-03-17T15:23:41.0197004Z To only update this specific test, also pass `--test-args macros/trace-macro.rs`
2020-03-17T15:23:41.0197952Z error: 1 errors occurred comparing output.
2020-03-17T15:23:41.0198330Z status: exit code: 0
2020-03-17T15:23:41.0198330Z status: exit code: 0
2020-03-17T15:23:41.0201042Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/trace-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace-macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "trace-macros" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace-macro/auxiliary"
2020-03-17T15:23:41.0204039Z ------------------------------------------
2020-03-17T15:23:41.0204395Z 
2020-03-17T15:23:41.0205131Z ------------------------------------------
2020-03-17T15:23:41.0205709Z stderr:
2020-03-17T15:23:41.0205709Z stderr:
2020-03-17T15:23:41.0206617Z ------------------------------------------
2020-03-17T15:23:41.0207218Z note: trace_macro
2020-03-17T15:23:41.0208111Z   --> /checkout/src/test/ui/macros/trace-macro.rs:5:5
2020-03-17T15:23:41.0208522Z    |
2020-03-17T15:23:41.0209022Z LL |     println!("Hello, World!");
2020-03-17T15:23:41.0209544Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-17T15:23:41.0209891Z    |
2020-03-17T15:23:41.0210366Z    = note: expanding `println! { "Hello, World!" }`
2020-03-17T15:23:41.0211081Z    = note: to `{ $crate :: io :: _print($crate :: format_args_nl !("Hello, World!")) ; }`
2020-03-17T15:23:41.0211935Z 
2020-03-17T15:23:41.0212726Z ------------------------------------------
2020-03-17T15:23:41.0213085Z 
2020-03-17T15:23:41.0213574Z 
2020-03-17T15:23:41.0213574Z 
2020-03-17T15:23:41.0214845Z ---- [ui] ui/macros/trace_faulty_macros.rs stdout ----
2020-03-17T15:23:41.0216229Z diff of stderr:
2020-03-17T15:23:41.0250309Z 
2020-03-17T15:23:41.0251061Z 19    |     ^^^^^^^^^^^^^^^^^^^
2020-03-17T15:23:41.0251486Z 20    |
2020-03-17T15:23:41.0251926Z 21    = note: expanding `my_faulty_macro! {  }`
2020-03-17T15:23:41.0252907Z -    = note: to `my_faulty_macro ! (bcd) ;`
2020-03-17T15:23:41.0253452Z +    = note: to `my_faulty_macro !(bcd) ;`
2020-03-17T15:23:41.0253965Z 23    = note: expanding `my_faulty_macro! { bcd }`
2020-03-17T15:23:41.0254811Z 25 error: recursion limit reached while expanding `my_recursive_macro!`
2020-03-17T15:23:41.0255209Z 
2020-03-17T15:23:41.0255561Z 41    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-17T15:23:41.0255917Z 42    |
2020-03-17T15:23:41.0255917Z 42    |
2020-03-17T15:23:41.0256356Z 43    = note: expanding `my_recursive_macro! {  }`
2020-03-17T15:23:41.0257136Z -    = note: to `my_recursive_macro ! () ;`
2020-03-17T15:23:41.0257673Z +    = note: to `my_recursive_macro !() ;`
2020-03-17T15:23:41.0258152Z 45    = note: expanding `my_recursive_macro! {  }`
2020-03-17T15:23:41.0258902Z -    = note: to `my_recursive_macro ! () ;`
2020-03-17T15:23:41.0259421Z +    = note: to `my_recursive_macro !() ;`
2020-03-17T15:23:41.0259890Z 47    = note: expanding `my_recursive_macro! {  }`
2020-03-17T15:23:41.0260632Z -    = note: to `my_recursive_macro ! () ;`
2020-03-17T15:23:41.0261153Z +    = note: to `my_recursive_macro !() ;`
2020-03-17T15:23:41.0261626Z 49    = note: expanding `my_recursive_macro! {  }`
2020-03-17T15:23:41.0262383Z -    = note: to `my_recursive_macro ! () ;`
2020-03-17T15:23:41.0262895Z +    = note: to `my_recursive_macro !() ;`
2020-03-17T15:23:41.0263666Z 52 error: aborting due to 2 previous errors
2020-03-17T15:23:41.0264042Z 53 
2020-03-17T15:23:41.0264317Z 
2020-03-17T15:23:41.0264573Z 
2020-03-17T15:23:41.0264573Z 
2020-03-17T15:23:41.0264982Z The actual stderr differed from the expected stderr.
2020-03-17T15:23:41.0265974Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros/trace_faulty_macros.stderr
2020-03-17T15:23:41.0266976Z To update references, rerun the tests and pass the `--bless` flag
2020-03-17T15:23:41.0267921Z To only update this specific test, also pass `--test-args macros/trace_faulty_macros.rs`
2020-03-17T15:23:41.0268778Z error: 1 errors occurred comparing output.
2020-03-17T15:23:41.0269193Z status: exit code: 1
2020-03-17T15:23:41.0269193Z status: exit code: 1
2020-03-17T15:23:41.0271704Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/trace_faulty_macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "trace-macros" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros/auxiliary"
2020-03-17T15:23:41.0274275Z ------------------------------------------
2020-03-17T15:23:41.0274655Z 
2020-03-17T15:23:41.0275424Z ------------------------------------------
2020-03-17T15:23:41.0275910Z stderr:
2020-03-17T15:23:41.0275910Z stderr:
2020-03-17T15:23:41.0276572Z ------------------------------------------
2020-03-17T15:23:41.0277104Z error: no rules expected the token `bcd`
2020-03-17T15:23:41.0278123Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:7:26
2020-03-17T15:23:41.0278615Z    |
2020-03-17T15:23:41.0278981Z LL | macro_rules! my_faulty_macro {
2020-03-17T15:23:41.0279726Z    | ---------------------------- when calling this macro
2020-03-17T15:23:41.0280252Z LL |     () => {
2020-03-17T15:23:41.0280701Z LL |         my_faulty_macro!(bcd); //~ ERROR no rules
2020-03-17T15:23:41.0281689Z ...
2020-03-17T15:23:41.0281689Z ...
2020-03-17T15:23:41.0282057Z LL |     my_faulty_macro!();
2020-03-17T15:23:41.0286779Z    |
2020-03-17T15:23:41.0287490Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-17T15:23:41.0287830Z 
2020-03-17T15:23:41.0288028Z note: trace_macro
2020-03-17T15:23:41.0288028Z note: trace_macro
2020-03-17T15:23:41.0288595Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:33:5
2020-03-17T15:23:41.0288878Z    |
2020-03-17T15:23:41.0289094Z LL |     my_faulty_macro!();
2020-03-17T15:23:41.0289528Z    |
2020-03-17T15:23:41.0289528Z    |
2020-03-17T15:23:41.0289779Z    = note: expanding `my_faulty_macro! {  }`
2020-03-17T15:23:41.0290089Z    = note: to `my_faulty_macro !(bcd) ;`
2020-03-17T15:23:41.0290412Z    = note: expanding `my_faulty_macro! { bcd }`
2020-03-17T15:23:41.0290910Z error: recursion limit reached while expanding `my_recursive_macro!`
2020-03-17T15:23:41.0291568Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:22:9
2020-03-17T15:23:41.0291848Z    |
2020-03-17T15:23:41.0291848Z    |
2020-03-17T15:23:41.0292132Z LL |         my_recursive_macro!(); //~ ERROR recursion limit
2020-03-17T15:23:41.0292665Z ...
2020-03-17T15:23:41.0292665Z ...
2020-03-17T15:23:41.0292884Z LL |     my_recursive_macro!();
2020-03-17T15:23:41.0293684Z    |
2020-03-17T15:23:41.0293684Z    |
2020-03-17T15:23:41.0294051Z    = help: consider adding a `#![recursion_limit="8"]` attribute to your crate (`trace_faulty_macros`)
2020-03-17T15:23:41.0295214Z 
2020-03-17T15:23:41.0295389Z note: trace_macro
2020-03-17T15:23:41.0295960Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:34:5
2020-03-17T15:23:41.0296240Z    |
2020-03-17T15:23:41.0296240Z    |
2020-03-17T15:23:41.0296443Z LL |     my_recursive_macro!();
2020-03-17T15:23:41.0296913Z    |
2020-03-17T15:23:41.0296913Z    |
2020-03-17T15:23:41.0297151Z    = note: expanding `my_recursive_macro! {  }`
2020-03-17T15:23:41.0297483Z    = note: to `my_recursive_macro !() ;`
2020-03-17T15:23:41.0297803Z    = note: expanding `my_recursive_macro! {  }`
2020-03-17T15:23:41.0298117Z    = note: to `my_recursive_macro !() ;`
2020-03-17T15:23:41.0298448Z    = note: expanding `my_recursive_macro! {  }`
2020-03-17T15:23:41.0298761Z    = note: to `my_recursive_macro !() ;`
2020-03-17T15:23:41.0299074Z    = note: expanding `my_recursive_macro! {  }`
2020-03-17T15:23:41.0299404Z    = note: to `my_recursive_macro !() ;`
2020-03-17T15:23:41.0299812Z error: aborting due to 2 previous errors
2020-03-17T15:23:41.0300161Z 
2020-03-17T15:23:41.0300271Z 
2020-03-17T15:23:41.0300766Z ------------------------------------------
2020-03-17T15:23:41.0300766Z ------------------------------------------
2020-03-17T15:23:41.0300966Z 
2020-03-17T15:23:41.0301077Z 
2020-03-17T15:23:41.0301571Z ---- [ui] ui/proc-macro/attribute-spans-preserved.rs stdout ----
2020-03-17T15:23:41.0301889Z diff of stdout:
2020-03-17T15:23:41.0302028Z 
2020-03-17T15:23:41.0302545Z - fn main () { let y : u32 = "z" ; { let x : u32 = "y" ; } }
2020-03-17T15:23:41.0303075Z + fn main() { let y : u32 = "z" ; { let x : u32 = "y" ; } }
2020-03-17T15:23:41.0303481Z 
2020-03-17T15:23:41.0303590Z 
2020-03-17T15:23:41.0303841Z The actual stdout differed from the expected stdout.
2020-03-17T15:23:41.0304712Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-spans-preserved/attribute-spans-preserved.stdout
2020-03-17T15:23:41.0304712Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-spans-preserved/attribute-spans-preserved.stdout
2020-03-17T15:23:41.0305504Z To update references, rerun the tests and pass the `--bless` flag
2020-03-17T15:23:41.0306260Z To only update this specific test, also pass `--test-args proc-macro/attribute-spans-preserved.rs`
2020-03-17T15:23:41.0307622Z error: 1 errors occurred comparing output.
2020-03-17T15:23:41.0307919Z status: exit code: 1
2020-03-17T15:23:41.0307919Z status: exit code: 1
2020-03-17T15:23:41.0319024Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/attribute-spans-preserved.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-spans-preserved" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-spans-preserved/auxiliary"
2020-03-17T15:23:41.0321902Z ------------------------------------------
2020-03-17T15:23:41.0321902Z ------------------------------------------
2020-03-17T15:23:41.0322277Z fn main() { let y : u32 = "z" ; { let x : u32 = "y" ; } }
2020-03-17T15:23:41.0323342Z ------------------------------------------
2020-03-17T15:23:41.0323589Z stderr:
2020-03-17T15:23:41.0324095Z ------------------------------------------
2020-03-17T15:23:41.0324383Z error[E0308]: mismatched types
2020-03-17T15:23:41.0324383Z error[E0308]: mismatched types
2020-03-17T15:23:41.0324995Z   --> /checkout/src/test/ui/proc-macro/attribute-spans-preserved.rs:7:23
2020-03-17T15:23:41.0325330Z    |
2020-03-17T15:23:41.0325635Z LL | #[ foo ( let y: u32 = "z"; ) ] //~ ERROR: mismatched types
2020-03-17T15:23:41.0326270Z    |                 ---   ^^^ expected `u32`, found `&str`
2020-03-17T15:23:41.0327670Z    |                 expected due to this
2020-03-17T15:23:41.0327871Z 
2020-03-17T15:23:41.0328074Z error[E0308]: mismatched types
2020-03-17T15:23:41.0328797Z   --> /checkout/src/test/ui/proc-macro/attribute-spans-preserved.rs:8:23
2020-03-17T15:23:41.0328797Z   --> /checkout/src/test/ui/proc-macro/attribute-spans-preserved.rs:8:23
2020-03-17T15:23:41.0329095Z    |
2020-03-17T15:23:41.0329406Z LL | #[ bar { let x: u32 = "y"; } ] //~ ERROR: mismatched types
2020-03-17T15:23:41.0330061Z    |                 ---   ^^^ expected `u32`, found `&str`
2020-03-17T15:23:41.0331719Z    |                 expected due to this
2020-03-17T15:23:41.0331964Z 
2020-03-17T15:23:41.0332188Z error: aborting due to 2 previous errors
2020-03-17T15:23:41.0332580Z 
---
2020-03-17T15:23:41.0335711Z 
2020-03-17T15:23:41.0336595Z ---- [ui] ui/proc-macro/dollar-crate-issue-57089.rs stdout ----
2020-03-17T15:23:41.0337504Z diff of stdout:
2020-03-17T15:23:41.0337654Z 
2020-03-17T15:23:41.0338260Z - PRINT-BANG INPUT (DISPLAY): struct M ($crate :: S) ;
2020-03-17T15:23:41.0345334Z + PRINT-BANG INPUT (DISPLAY): struct M($crate :: S) ;
2020-03-17T15:23:41.0347679Z 2 PRINT-BANG INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0349600Z 4         ident: "struct",
2020-03-17T15:23:41.0349777Z 
2020-03-17T15:23:41.0349925Z 39     },
2020-03-17T15:23:41.0350081Z 40 ]
2020-03-17T15:23:41.0350081Z 40 ]
2020-03-17T15:23:41.0350735Z 41 PRINT-ATTR INPUT (DISPLAY): struct A(crate::S);
2020-03-17T15:23:41.0351394Z - PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ($crate :: S) ;
2020-03-17T15:23:41.0353559Z + PRINT-ATTR RE-COLLECTED (DISPLAY): struct A($crate :: S) ;
2020-03-17T15:23:41.0354249Z 43 PRINT-ATTR INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0361904Z 45         ident: "struct",
2020-03-17T15:23:41.0362095Z 
2020-03-17T15:23:41.0362235Z 
2020-03-17T15:23:41.0362461Z The actual stdout differed from the expected stdout.
2020-03-17T15:23:41.0362461Z The actual stdout differed from the expected stdout.
2020-03-17T15:23:41.0364507Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-57089/dollar-crate-issue-57089.stdout
2020-03-17T15:23:41.0366065Z To update references, rerun the tests and pass the `--bless` flag
2020-03-17T15:23:41.0366807Z To only update this specific test, also pass `--test-args proc-macro/dollar-crate-issue-57089.rs`
2020-03-17T15:23:41.0367359Z error: 1 errors occurred comparing output.
2020-03-17T15:23:41.0367628Z status: exit code: 0
2020-03-17T15:23:41.0367628Z status: exit code: 0
2020-03-17T15:23:41.0370545Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/dollar-crate-issue-57089.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-57089" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-57089/auxiliary"
2020-03-17T15:23:41.0373022Z ------------------------------------------
2020-03-17T15:23:41.0373022Z ------------------------------------------
2020-03-17T15:23:41.0373596Z PRINT-BANG INPUT (DISPLAY): struct M($crate :: S) ;
2020-03-17T15:23:41.0375558Z PRINT-BANG INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0376061Z         ident: "struct",
2020-03-17T15:23:41.0376061Z         ident: "struct",
2020-03-17T15:23:41.0376323Z         span: #3 bytes(399..405),
2020-03-17T15:23:41.0376733Z     Ident {
2020-03-17T15:23:41.0376931Z         ident: "M",
2020-03-17T15:23:41.0376931Z         ident: "M",
2020-03-17T15:23:41.0377180Z         span: #3 bytes(406..407),
2020-03-17T15:23:41.0377572Z     Group {
2020-03-17T15:23:41.0378387Z         delimiter: Parenthesis,
2020-03-17T15:23:41.0378883Z         stream: TokenStream [
2020-03-17T15:23:41.0379112Z             Ident {
2020-03-17T15:23:41.0379112Z             Ident {
2020-03-17T15:23:41.0379354Z                 ident: "$crate",
2020-03-17T15:23:41.0379672Z                 span: #3 bytes(408..414),
2020-03-17T15:23:41.0380129Z             Punct {
2020-03-17T15:23:41.0380129Z             Punct {
2020-03-17T15:23:41.0380665Z                 ch: ':',
2020-03-17T15:23:41.0380954Z                 spacing: Joint,
2020-03-17T15:23:41.0381253Z                 span: #3 bytes(414..416),
2020-03-17T15:23:41.0381714Z             Punct {
2020-03-17T15:23:41.0381714Z             Punct {
2020-03-17T15:23:41.0382456Z                 ch: ':',
2020-03-17T15:23:41.0382952Z                 spacing: Alone,
2020-03-17T15:23:41.0388001Z                 span: #3 bytes(414..416),
2020-03-17T15:23:41.0388464Z             Ident {
2020-03-17T15:23:41.0388715Z                 ident: "S",
2020-03-17T15:23:41.0388715Z                 ident: "S",
2020-03-17T15:23:41.0389006Z                 span: #3 bytes(416..417),
2020-03-17T15:23:41.0389433Z         ],
2020-03-17T15:23:41.0389433Z         ],
2020-03-17T15:23:41.0389679Z         span: #3 bytes(407..418),
2020-03-17T15:23:41.0390059Z     Punct {
2020-03-17T15:23:41.0390059Z     Punct {
2020-03-17T15:23:41.0391142Z         ch: ';',
2020-03-17T15:23:41.0391384Z         spacing: Alone,
2020-03-17T15:23:41.0391643Z         span: #3 bytes(418..419),
2020-03-17T15:23:41.0392228Z ]
2020-03-17T15:23:41.0392228Z ]
2020-03-17T15:23:41.0393036Z PRINT-ATTR INPUT (DISPLAY): struct A(crate::S);
2020-03-17T15:23:41.0393768Z PRINT-ATTR RE-COLLECTED (DISPLAY): struct A($crate :: S) ;
2020-03-17T15:23:41.0394909Z PRINT-ATTR INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0395576Z         ident: "struct",
2020-03-17T15:23:41.0395576Z         ident: "struct",
2020-03-17T15:23:41.0395840Z         span: #3 bytes(461..467),
2020-03-17T15:23:41.0396219Z     Ident {
2020-03-17T15:23:41.0396438Z         ident: "A",
2020-03-17T15:23:41.0396438Z         ident: "A",
2020-03-17T15:23:41.0396689Z         span: #3 bytes(468..469),
2020-03-17T15:23:41.0397566Z     Group {
2020-03-17T15:23:41.0397793Z         delimiter: Parenthesis,
2020-03-17T15:23:41.0398049Z         stream: TokenStream [
2020-03-17T15:23:41.0398308Z             Ident {
2020-03-17T15:23:41.0398308Z             Ident {
2020-03-17T15:23:41.0398553Z                 ident: "$crate",
2020-03-17T15:23:41.0398853Z                 span: #3 bytes(470..476),
2020-03-17T15:23:41.0399313Z             Punct {
2020-03-17T15:23:41.0399313Z             Punct {
2020-03-17T15:23:41.0400293Z                 ch: ':',
2020-03-17T15:23:41.0400566Z                 spacing: Joint,
2020-03-17T15:23:41.0400893Z                 span: #3 bytes(476..478),
2020-03-17T15:23:41.0401349Z             Punct {
2020-03-17T15:23:41.0401349Z             Punct {
2020-03-17T15:23:41.0401813Z                 ch: ':',
2020-03-17T15:23:41.0402086Z                 spacing: Alone,
2020-03-17T15:23:41.0402384Z                 span: #3 bytes(476..478),
2020-03-17T15:23:41.0403102Z             Ident {
2020-03-17T15:23:41.0403361Z                 ident: "S",
2020-03-17T15:23:41.0403361Z                 ident: "S",
2020-03-17T15:23:41.0403671Z                 span: #3 bytes(478..479),
2020-03-17T15:23:41.0404100Z         ],
2020-03-17T15:23:41.0404100Z         ],
2020-03-17T15:23:41.0404334Z         span: #3 bytes(469..480),
2020-03-17T15:23:41.0404732Z     Punct {
2020-03-17T15:23:41.0404732Z     Punct {
2020-03-17T15:23:41.0405170Z         ch: ';',
2020-03-17T15:23:41.0405422Z         spacing: Alone,
2020-03-17T15:23:41.0405681Z         span: #3 bytes(480..481),
2020-03-17T15:23:41.0406055Z ]
2020-03-17T15:23:41.0406172Z 
2020-03-17T15:23:41.0407085Z ------------------------------------------
2020-03-17T15:23:41.0407341Z stderr:
---
2020-03-17T15:23:41.0408840Z 
2020-03-17T15:23:41.0409342Z ---- [ui] ui/proc-macro/dollar-crate-issue-62325.rs stdout ----
2020-03-17T15:23:41.0409634Z diff of stdout:
2020-03-17T15:23:41.0409774Z 
2020-03-17T15:23:41.0410317Z 1 PRINT-ATTR INPUT (DISPLAY): struct A(identity!(crate :: S));
2020-03-17T15:23:41.0411054Z - PRINT-ATTR RE-COLLECTED (DISPLAY): struct A (identity ! ($crate :: S)) ;
2020-03-17T15:23:41.0411797Z + PRINT-ATTR RE-COLLECTED (DISPLAY): struct A(identity !($crate :: S)) ;
2020-03-17T15:23:41.0412444Z 3 PRINT-ATTR INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0412927Z 5         ident: "struct",
2020-03-17T15:23:41.0413119Z 
2020-03-17T15:23:41.0413268Z 55     },
2020-03-17T15:23:41.0413425Z 56 ]
2020-03-17T15:23:41.0413425Z 56 ]
2020-03-17T15:23:41.0414033Z 57 PRINT-ATTR INPUT (DISPLAY): struct B(identity!(::dollar_crate_external :: S));
2020-03-17T15:23:41.0414832Z - PRINT-ATTR RE-COLLECTED (DISPLAY): struct B (identity ! ($crate :: S)) ;
2020-03-17T15:23:41.0415578Z + PRINT-ATTR RE-COLLECTED (DISPLAY): struct B(identity !($crate :: S)) ;
2020-03-17T15:23:41.0416227Z 59 PRINT-ATTR INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0416714Z 61         ident: "struct",
2020-03-17T15:23:41.0416889Z 
2020-03-17T15:23:41.0417015Z 
2020-03-17T15:23:41.0417406Z The actual stdout differed from the expected stdout.
2020-03-17T15:23:41.0417406Z The actual stdout differed from the expected stdout.
2020-03-17T15:23:41.0418286Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-62325/dollar-crate-issue-62325.stdout
2020-03-17T15:23:41.0419098Z To update references, rerun the tests and pass the `--bless` flag
2020-03-17T15:23:41.0419834Z To only update this specific test, also pass `--test-args proc-macro/dollar-crate-issue-62325.rs`
2020-03-17T15:23:41.0420486Z error: 1 errors occurred comparing output.
2020-03-17T15:23:41.0420770Z status: exit code: 0
2020-03-17T15:23:41.0420770Z status: exit code: 0
2020-03-17T15:23:41.0423166Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/dollar-crate-issue-62325.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-62325" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-62325/auxiliary"
2020-03-17T15:23:41.0425118Z ------------------------------------------
2020-03-17T15:23:41.0425118Z ------------------------------------------
2020-03-17T15:23:41.0426076Z PRINT-ATTR INPUT (DISPLAY): struct A(identity!(crate :: S));
2020-03-17T15:23:41.0427318Z PRINT-ATTR RE-COLLECTED (DISPLAY): struct A(identity !($crate :: S)) ;
2020-03-17T15:23:41.0427958Z PRINT-ATTR INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0428447Z         ident: "struct",
2020-03-17T15:23:41.0428447Z         ident: "struct",
2020-03-17T15:23:41.0428710Z         span: #3 bytes(457..463),
2020-03-17T15:23:41.0429107Z     Ident {
2020-03-17T15:23:41.0429305Z         ident: "A",
2020-03-17T15:23:41.0429305Z         ident: "A",
2020-03-17T15:23:41.0429574Z         span: #3 bytes(464..465),
2020-03-17T15:23:41.0429961Z     Group {
2020-03-17T15:23:41.0430202Z         delimiter: Parenthesis,
2020-03-17T15:23:41.0430471Z         stream: TokenStream [
2020-03-17T15:23:41.0430705Z             Ident {
2020-03-17T15:23:41.0430705Z             Ident {
2020-03-17T15:23:41.0430953Z                 ident: "identity",
2020-03-17T15:23:41.0431280Z                 span: #3 bytes(466..474),
2020-03-17T15:23:41.0431729Z             Punct {
2020-03-17T15:23:41.0431729Z             Punct {
2020-03-17T15:23:41.0432188Z                 ch: '!',
2020-03-17T15:23:41.0432460Z                 spacing: Alone,
2020-03-17T15:23:41.0432757Z                 span: #3 bytes(474..475),
2020-03-17T15:23:41.0433223Z             Group {
2020-03-17T15:23:41.0433480Z                 delimiter: Parenthesis,
2020-03-17T15:23:41.0433803Z                 stream: TokenStream [
2020-03-17T15:23:41.0434075Z                     Ident {
2020-03-17T15:23:41.0434075Z                     Ident {
2020-03-17T15:23:41.0434353Z                         ident: "$crate",
2020-03-17T15:23:41.0434710Z                         span: #3 bytes(476..482),
2020-03-17T15:23:41.0435228Z                     Punct {
2020-03-17T15:23:41.0435228Z                     Punct {
2020-03-17T15:23:41.0435710Z                         ch: ':',
2020-03-17T15:23:41.0436043Z                         spacing: Joint,
2020-03-17T15:23:41.0436382Z                         span: #3 bytes(482..484),
2020-03-17T15:23:41.0436913Z                     Punct {
2020-03-17T15:23:41.0436913Z                     Punct {
2020-03-17T15:23:41.0437388Z                         ch: ':',
2020-03-17T15:23:41.0437700Z                         spacing: Alone,
2020-03-17T15:23:41.0438058Z                         span: #3 bytes(482..484),
2020-03-17T15:23:41.0438571Z                     Ident {
2020-03-17T15:23:41.0438858Z                         ident: "S",
2020-03-17T15:23:41.0438858Z                         ident: "S",
2020-03-17T15:23:41.0439187Z                         span: #3 bytes(484..485),
2020-03-17T15:23:41.0439696Z                 ],
2020-03-17T15:23:41.0439696Z                 ],
2020-03-17T15:23:41.0440094Z                 span: #3 bytes(475..486),
2020-03-17T15:23:41.0440522Z         ],
2020-03-17T15:23:41.0440522Z         ],
2020-03-17T15:23:41.0440765Z         span: #3 bytes(465..487),
2020-03-17T15:23:41.0441142Z     Punct {
2020-03-17T15:23:41.0441142Z     Punct {
2020-03-17T15:23:41.0441578Z         ch: ';',
2020-03-17T15:23:41.0441811Z         spacing: Alone,
2020-03-17T15:23:41.0442070Z         span: #3 bytes(487..488),
2020-03-17T15:23:41.0442539Z ]
2020-03-17T15:23:41.0442539Z ]
2020-03-17T15:23:41.0443430Z PRINT-ATTR INPUT (DISPLAY): struct B(identity!(::dollar_crate_external :: S));
2020-03-17T15:23:41.0444243Z PRINT-ATTR RE-COLLECTED (DISPLAY): struct B(identity !($crate :: S)) ;
2020-03-17T15:23:41.0444861Z PRINT-ATTR INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0445339Z         ident: "struct",
2020-03-17T15:23:41.0445339Z         ident: "struct",
2020-03-17T15:23:41.0445621Z         span: #10 bytes(6794126..6794132),
2020-03-17T15:23:41.0446025Z     Ident {
2020-03-17T15:23:41.0446239Z         ident: "B",
2020-03-17T15:23:41.0446239Z         ident: "B",
2020-03-17T15:23:41.0446507Z         span: #10 bytes(6794133..6794134),
2020-03-17T15:23:41.0446921Z     Group {
2020-03-17T15:23:41.0447144Z         delimiter: Parenthesis,
2020-03-17T15:23:41.0447412Z         stream: TokenStream [
2020-03-17T15:23:41.0447662Z             Ident {
2020-03-17T15:23:41.0447662Z             Ident {
2020-03-17T15:23:41.0447911Z                 ident: "identity",
2020-03-17T15:23:41.0448237Z                 span: #10 bytes(6794135..6794143),
2020-03-17T15:23:41.0448718Z             Punct {
2020-03-17T15:23:41.0448718Z             Punct {
2020-03-17T15:23:41.0449157Z                 ch: '!',
2020-03-17T15:23:41.0449429Z                 spacing: Alone,
2020-03-17T15:23:41.0449767Z                 span: #10 bytes(6794143..6794144),
2020-03-17T15:23:41.0450231Z             Group {
2020-03-17T15:23:41.0450507Z                 delimiter: Parenthesis,
2020-03-17T15:23:41.0450814Z                 stream: TokenStream [
2020-03-17T15:23:41.0451090Z                     Ident {
2020-03-17T15:23:41.0451090Z                     Ident {
2020-03-17T15:23:41.0451387Z                         ident: "$crate",
2020-03-17T15:23:41.0451749Z                         span: #10 bytes(6794145..6794151),
2020-03-17T15:23:41.0452294Z                     Punct {
2020-03-17T15:23:41.0452294Z                     Punct {
2020-03-17T15:23:41.0452772Z                         ch: ':',
2020-03-17T15:23:41.0453090Z                         spacing: Joint,
2020-03-17T15:23:41.0453466Z                         span: #10 bytes(6794151..6794153),
2020-03-17T15:23:41.0454041Z                     Punct {
2020-03-17T15:23:41.0454041Z                     Punct {
2020-03-17T15:23:41.0454554Z                         ch: ':',
2020-03-17T15:23:41.0454858Z                         spacing: Alone,
2020-03-17T15:23:41.0455214Z                         span: #10 bytes(6794151..6794153),
2020-03-17T15:23:41.0455759Z                     Ident {
2020-03-17T15:23:41.0456036Z                         ident: "S",
2020-03-17T15:23:41.0456036Z                         ident: "S",
2020-03-17T15:23:41.0456383Z                         span: #10 bytes(6794153..6794154),
2020-03-17T15:23:41.0456910Z                 ],
2020-03-17T15:23:41.0456910Z                 ],
2020-03-17T15:23:41.0457186Z                 span: #10 bytes(6794144..6794155),
2020-03-17T15:23:41.0457647Z         ],
2020-03-17T15:23:41.0457647Z         ],
2020-03-17T15:23:41.0457889Z         span: #10 bytes(6794134..6794156),
2020-03-17T15:23:41.0458306Z     Punct {
2020-03-17T15:23:41.0458306Z     Punct {
2020-03-17T15:23:41.0458695Z         ch: ';',
2020-03-17T15:23:41.0458929Z         spacing: Alone,
2020-03-17T15:23:41.0459227Z         span: #10 bytes(6794156..6794157),
2020-03-17T15:23:41.0459600Z ]
2020-03-17T15:23:41.0459732Z 
2020-03-17T15:23:41.0460162Z ------------------------------------------
2020-03-17T15:23:41.0460395Z stderr:
---
2020-03-17T15:23:41.0461965Z 
2020-03-17T15:23:41.0462441Z ---- [ui] ui/proc-macro/dollar-crate.rs stdout ----
2020-03-17T15:23:41.0466020Z diff of stdout:
2020-03-17T15:23:41.0466176Z 
2020-03-17T15:23:41.0466821Z - PRINT-BANG INPUT (DISPLAY): struct M ($crate :: S) ;
2020-03-17T15:23:41.0467498Z + PRINT-BANG INPUT (DISPLAY): struct M($crate :: S) ;
2020-03-17T15:23:41.0470569Z 2 PRINT-BANG INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0471103Z 4         ident: "struct",
2020-03-17T15:23:41.0471279Z 
2020-03-17T15:23:41.0471427Z 39     },
2020-03-17T15:23:41.0471605Z 40 ]
2020-03-17T15:23:41.0471605Z 40 ]
2020-03-17T15:23:41.0472160Z 41 PRINT-ATTR INPUT (DISPLAY): struct A(crate::S);
2020-03-17T15:23:41.0472821Z - PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ($crate :: S) ;
2020-03-17T15:23:41.0473513Z + PRINT-ATTR RE-COLLECTED (DISPLAY): struct A($crate :: S) ;
2020-03-17T15:23:41.0474132Z 43 PRINT-ATTR INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0474624Z 45         ident: "struct",
2020-03-17T15:23:41.0474820Z 
2020-03-17T15:23:41.0474968Z 80     },
2020-03-17T15:23:41.0475123Z 81 ]
2020-03-17T15:23:41.0475123Z 81 ]
2020-03-17T15:23:41.0475645Z 82 PRINT-DERIVE INPUT (DISPLAY): struct D(crate::S);
2020-03-17T15:23:41.0476310Z - PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D ($crate :: S) ;
2020-03-17T15:23:41.0477005Z + PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D($crate :: S) ;
2020-03-17T15:23:41.0477642Z 84 PRINT-DERIVE INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0478137Z 86         ident: "struct",
2020-03-17T15:23:41.0478314Z 
2020-03-17T15:23:41.0478314Z 
2020-03-17T15:23:41.0478548Z 120         span: #3 bytes(LO..HI),
2020-03-17T15:23:41.0478940Z 122 ]
2020-03-17T15:23:41.0478940Z 122 ]
2020-03-17T15:23:41.0479468Z - PRINT-BANG INPUT (DISPLAY): struct M ($crate :: S) ;
2020-03-17T15:23:41.0480099Z + PRINT-BANG INPUT (DISPLAY): struct M($crate :: S) ;
2020-03-17T15:23:41.0480697Z 124 PRINT-BANG INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0481220Z 126         ident: "struct",
2020-03-17T15:23:41.0481398Z 
2020-03-17T15:23:41.0481548Z 161     },
2020-03-17T15:23:41.0481725Z 162 ]
2020-03-17T15:23:41.0481725Z 162 ]
2020-03-17T15:23:41.0482290Z 163 PRINT-ATTR INPUT (DISPLAY): struct A(::dollar_crate_external::S);
2020-03-17T15:23:41.0483589Z - PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ($crate :: S) ;
2020-03-17T15:23:41.0484420Z + PRINT-ATTR RE-COLLECTED (DISPLAY): struct A($crate :: S) ;
2020-03-17T15:23:41.0485041Z 165 PRINT-ATTR INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0485562Z 167         ident: "struct",
2020-03-17T15:23:41.0485742Z 
2020-03-17T15:23:41.0485891Z 202     },
2020-03-17T15:23:41.0486051Z 203 ]
2020-03-17T15:23:41.0486051Z 203 ]
2020-03-17T15:23:41.0486637Z 204 PRINT-DERIVE INPUT (DISPLAY): struct D(::dollar_crate_external::S);
2020-03-17T15:23:41.0487342Z - PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D ($crate :: S) ;
2020-03-17T15:23:41.0488037Z + PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D($crate :: S) ;
2020-03-17T15:23:41.0488676Z 206 PRINT-DERIVE INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0489179Z 208         ident: "struct",
2020-03-17T15:23:41.0489357Z 
2020-03-17T15:23:41.0489488Z 
2020-03-17T15:23:41.0489723Z The actual stdout differed from the expected stdout.
2020-03-17T15:23:41.0489723Z The actual stdout differed from the expected stdout.
2020-03-17T15:23:41.0490494Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate/dollar-crate.stdout
2020-03-17T15:23:41.0491260Z To update references, rerun the tests and pass the `--bless` flag
2020-03-17T15:23:41.0491953Z To only update this specific test, also pass `--test-args proc-macro/dollar-crate.rs`
2020-03-17T15:23:41.0492478Z error: 1 errors occurred comparing output.
2020-03-17T15:23:41.0492746Z status: exit code: 1
2020-03-17T15:23:41.0492746Z status: exit code: 1
2020-03-17T15:23:41.0495139Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/dollar-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate/auxiliary"
2020-03-17T15:23:41.0497127Z ------------------------------------------
2020-03-17T15:23:41.0497127Z ------------------------------------------
2020-03-17T15:23:41.0497732Z PRINT-BANG INPUT (DISPLAY): struct M($crate :: S) ;
2020-03-17T15:23:41.0498303Z PRINT-BANG INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0498786Z         ident: "struct",
2020-03-17T15:23:41.0498786Z         ident: "struct",
2020-03-17T15:23:41.0499052Z         span: #3 bytes(491..497),
2020-03-17T15:23:41.0499462Z     Ident {
2020-03-17T15:23:41.0499662Z         ident: "M",
2020-03-17T15:23:41.0499662Z         ident: "M",
2020-03-17T15:23:41.0499914Z         span: #3 bytes(498..499),
2020-03-17T15:23:41.0500310Z     Group {
2020-03-17T15:23:41.0500536Z         delimiter: Parenthesis,
2020-03-17T15:23:41.0500820Z         stream: TokenStream [
2020-03-17T15:23:41.0501055Z             Ident {
2020-03-17T15:23:41.0501055Z             Ident {
2020-03-17T15:23:41.0501301Z                 ident: "$crate",
2020-03-17T15:23:41.0501606Z                 span: #3 bytes(500..506),
2020-03-17T15:23:41.0502073Z             Punct {
2020-03-17T15:23:41.0502073Z             Punct {
2020-03-17T15:23:41.0502514Z                 ch: ':',
2020-03-17T15:23:41.0502809Z                 spacing: Joint,
2020-03-17T15:23:41.0503109Z                 span: #3 bytes(506..508),
2020-03-17T15:23:41.0503573Z             Punct {
2020-03-17T15:23:41.0503573Z             Punct {
2020-03-17T15:23:41.0504003Z                 ch: ':',
2020-03-17T15:23:41.0504275Z                 spacing: Alone,
2020-03-17T15:23:41.0504600Z                 span: #3 bytes(506..508),
2020-03-17T15:23:41.0505048Z             Ident {
2020-03-17T15:23:41.0505283Z                 ident: "S",
2020-03-17T15:23:41.0505283Z                 ident: "S",
2020-03-17T15:23:41.0505594Z                 span: #3 bytes(508..509),
2020-03-17T15:23:41.0506021Z         ],
2020-03-17T15:23:41.0506021Z         ],
2020-03-17T15:23:41.0506263Z         span: #3 bytes(499..510),
2020-03-17T15:23:41.0506647Z     Punct {
2020-03-17T15:23:41.0506647Z     Punct {
2020-03-17T15:23:41.0507054Z         ch: ';',
2020-03-17T15:23:41.0507288Z         spacing: Alone,
2020-03-17T15:23:41.0507549Z         span: #3 bytes(510..511),
2020-03-17T15:23:41.0507927Z ]
2020-03-17T15:23:41.0507927Z ]
2020-03-17T15:23:41.0508403Z PRINT-ATTR INPUT (DISPLAY): struct A(crate::S);
2020-03-17T15:23:41.0509033Z PRINT-ATTR RE-COLLECTED (DISPLAY): struct A($crate :: S) ;
2020-03-17T15:23:41.0509649Z PRINT-ATTR INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0512931Z         ident: "struct",
2020-03-17T15:23:41.0512931Z         ident: "struct",
2020-03-17T15:23:41.0513237Z         span: #3 bytes(565..571),
2020-03-17T15:23:41.0513616Z     Ident {
2020-03-17T15:23:41.0513837Z         ident: "A",
2020-03-17T15:23:41.0513837Z         ident: "A",
2020-03-17T15:23:41.0514311Z         span: #3 bytes(572..573),
2020-03-17T15:23:41.0514714Z     Group {
2020-03-17T15:23:41.0514938Z         delimiter: Parenthesis,
2020-03-17T15:23:41.0515219Z         stream: TokenStream [
2020-03-17T15:23:41.0515471Z             Ident {
2020-03-17T15:23:41.0515471Z             Ident {
2020-03-17T15:23:41.0515715Z                 ident: "$crate",
2020-03-17T15:23:41.0516013Z                 span: #3 bytes(574..580),
2020-03-17T15:23:41.0516477Z             Punct {
2020-03-17T15:23:41.0516477Z             Punct {
2020-03-17T15:23:41.0517066Z                 ch: ':',
2020-03-17T15:23:41.0517333Z                 spacing: Joint,
2020-03-17T15:23:41.0517646Z                 span: #3 bytes(580..582),
2020-03-17T15:23:41.0518252Z             Punct {
2020-03-17T15:23:41.0518252Z             Punct {
2020-03-17T15:23:41.0518872Z                 ch: ':',
2020-03-17T15:23:41.0519142Z                 spacing: Alone,
2020-03-17T15:23:41.0519432Z                 span: #3 bytes(580..582),
2020-03-17T15:23:41.0519893Z             Ident {
2020-03-17T15:23:41.0520127Z                 ident: "S",
2020-03-17T15:23:41.0520127Z                 ident: "S",
2020-03-17T15:23:41.0520414Z                 span: #3 bytes(582..583),
2020-03-17T15:23:41.0520970Z         ],
2020-03-17T15:23:41.0520970Z         ],
2020-03-17T15:23:41.0521194Z         span: #3 bytes(573..584),
2020-03-17T15:23:41.0521594Z     Punct {
2020-03-17T15:23:41.0521594Z     Punct {
2020-03-17T15:23:41.0522027Z         ch: ';',
2020-03-17T15:23:41.0522272Z         spacing: Alone,
2020-03-17T15:23:41.0522523Z         span: #3 bytes(584..585),
2020-03-17T15:23:41.0523068Z ]
2020-03-17T15:23:41.0523068Z ]
2020-03-17T15:23:41.0523608Z PRINT-DERIVE INPUT (DISPLAY): struct D(crate::S);
2020-03-17T15:23:41.0524254Z PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D($crate :: S) ;
2020-03-17T15:23:41.0524869Z PRINT-DERIVE INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0525357Z         ident: "struct",
2020-03-17T15:23:41.0525357Z         ident: "struct",
2020-03-17T15:23:41.0525619Z         span: #3 bytes(628..634),
2020-03-17T15:23:41.0526016Z     Ident {
2020-03-17T15:23:41.0526213Z         ident: "D",
2020-03-17T15:23:41.0526213Z         ident: "D",
2020-03-17T15:23:41.0526480Z         span: #3 bytes(635..636),
2020-03-17T15:23:41.0526865Z     Group {
2020-03-17T15:23:41.0527106Z         delimiter: Parenthesis,
2020-03-17T15:23:41.0527373Z         stream: TokenStream [
2020-03-17T15:23:41.0527607Z             Ident {
2020-03-17T15:23:41.0527607Z             Ident {
2020-03-17T15:23:41.0527852Z                 ident: "$crate",
2020-03-17T15:23:41.0528174Z                 span: #3 bytes(637..643),
2020-03-17T15:23:41.0528618Z             Punct {
2020-03-17T15:23:41.0528618Z             Punct {
2020-03-17T15:23:41.0529077Z                 ch: ':',
2020-03-17T15:23:41.0529351Z                 spacing: Joint,
2020-03-17T15:23:41.0529656Z                 span: #3 bytes(643..645),
2020-03-17T15:23:41.0530120Z             Punct {
2020-03-17T15:23:41.0530120Z             Punct {
2020-03-17T15:23:41.0530547Z                 ch: ':',
2020-03-17T15:23:41.0530836Z                 spacing: Alone,
2020-03-17T15:23:41.0531134Z                 span: #3 bytes(643..645),
2020-03-17T15:23:41.0531576Z             Ident {
2020-03-17T15:23:41.0531834Z                 ident: "S",
2020-03-17T15:23:41.0531834Z                 ident: "S",
2020-03-17T15:23:41.0532123Z                 span: #3 bytes(645..646),
2020-03-17T15:23:41.0532564Z         ],
2020-03-17T15:23:41.0532564Z         ],
2020-03-17T15:23:41.0532787Z         span: #3 bytes(636..647),
2020-03-17T15:23:41.0533182Z     Punct {
2020-03-17T15:23:41.0533182Z     Punct {
2020-03-17T15:23:41.0533569Z         ch: ';',
2020-03-17T15:23:41.0533802Z         spacing: Alone,
2020-03-17T15:23:41.0534080Z         span: #3 bytes(647..648),
2020-03-17T15:23:41.0534446Z ]
2020-03-17T15:23:41.0534446Z ]
2020-03-17T15:23:41.0534940Z PRINT-BANG INPUT (DISPLAY): struct M($crate :: S) ;
2020-03-17T15:23:41.0535531Z PRINT-BANG INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0535992Z         ident: "struct",
2020-03-17T15:23:41.0535992Z         ident: "struct",
2020-03-17T15:23:41.0536293Z         span: #13 bytes(6794251..6794257),
2020-03-17T15:23:41.0536690Z     Ident {
2020-03-17T15:23:41.0536907Z         ident: "M",
2020-03-17T15:23:41.0536907Z         ident: "M",
2020-03-17T15:23:41.0537183Z         span: #13 bytes(6794258..6794259),
2020-03-17T15:23:41.0537596Z     Group {
2020-03-17T15:23:41.0537819Z         delimiter: Parenthesis,
2020-03-17T15:23:41.0538087Z         stream: TokenStream [
2020-03-17T15:23:41.0538320Z             Ident {
2020-03-17T15:23:41.0538320Z             Ident {
2020-03-17T15:23:41.0538584Z                 ident: "$crate",
2020-03-17T15:23:41.0538903Z                 span: #13 bytes(6794260..6794266),
2020-03-17T15:23:41.0540735Z             Punct {
2020-03-17T15:23:41.0540735Z             Punct {
2020-03-17T15:23:41.0541482Z                 ch: ':',
2020-03-17T15:23:41.0542301Z                 spacing: Joint,
2020-03-17T15:23:41.0542652Z                 span: #13 bytes(6794266..6794268),
2020-03-17T15:23:41.0543112Z             Punct {
2020-03-17T15:23:41.0543112Z             Punct {
2020-03-17T15:23:41.0543627Z                 ch: ':',
2020-03-17T15:23:41.0543901Z                 spacing: Alone,
2020-03-17T15:23:41.0544216Z                 span: #13 bytes(6794266..6794268),
2020-03-17T15:23:41.0544841Z             Ident {
2020-03-17T15:23:41.0545073Z                 ident: "S",
2020-03-17T15:23:41.0545073Z                 ident: "S",
2020-03-17T15:23:41.0545382Z                 span: #13 bytes(6794268..6794269),
2020-03-17T15:23:41.0545845Z         ],
2020-03-17T15:23:41.0545845Z         ],
2020-03-17T15:23:41.0546087Z         span: #13 bytes(6794259..6794270),
2020-03-17T15:23:41.0546497Z     Punct {
2020-03-17T15:23:41.0546497Z     Punct {
2020-03-17T15:23:41.0546919Z         ch: ';',
2020-03-17T15:23:41.0547174Z         spacing: Alone,
2020-03-17T15:23:41.0547461Z         span: #13 bytes(6794270..6794271),
2020-03-17T15:23:41.0547852Z ]
2020-03-17T15:23:41.0547852Z ]
2020-03-17T15:23:41.0548395Z PRINT-ATTR INPUT (DISPLAY): struct A(::dollar_crate_external::S);
2020-03-17T15:23:41.0549072Z PRINT-ATTR RE-COLLECTED (DISPLAY): struct A($crate :: S) ;
2020-03-17T15:23:41.0549658Z PRINT-ATTR INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0550145Z         ident: "struct",
2020-03-17T15:23:41.0550145Z         ident: "struct",
2020-03-17T15:23:41.0550423Z         span: #13 bytes(6794313..6794319),
2020-03-17T15:23:41.0550837Z     Ident {
2020-03-17T15:23:41.0551034Z         ident: "A",
2020-03-17T15:23:41.0551034Z         ident: "A",
2020-03-17T15:23:41.0551317Z         span: #13 bytes(6794320..6794321),
2020-03-17T15:23:41.0551709Z     Group {
2020-03-17T15:23:41.0551949Z         delimiter: Parenthesis,
2020-03-17T15:23:41.0552215Z         stream: TokenStream [
2020-03-17T15:23:41.0552447Z             Ident {
2020-03-17T15:23:41.0552447Z             Ident {
2020-03-17T15:23:41.0552696Z                 ident: "$crate",
2020-03-17T15:23:41.0553031Z                 span: #13 bytes(6794322..6794328),
2020-03-17T15:23:41.0553489Z             Punct {
2020-03-17T15:23:41.0553489Z             Punct {
2020-03-17T15:23:41.0553944Z                 ch: ':',
2020-03-17T15:23:41.0554216Z                 spacing: Joint,
2020-03-17T15:23:41.0554533Z                 span: #13 bytes(6794328..6794330),
2020-03-17T15:23:41.0555023Z             Punct {
2020-03-17T15:23:41.0555023Z             Punct {
2020-03-17T15:23:41.0555446Z                 ch: ':',
2020-03-17T15:23:41.0555737Z                 spacing: Alone,
2020-03-17T15:23:41.0556111Z                 span: #13 bytes(6794328..6794330),
2020-03-17T15:23:41.0556594Z             Ident {
2020-03-17T15:23:41.0556827Z                 ident: "S",
2020-03-17T15:23:41.0556827Z                 ident: "S",
2020-03-17T15:23:41.0557135Z                 span: #13 bytes(6794330..6794331),
2020-03-17T15:23:41.0557594Z         ],
2020-03-17T15:23:41.0557594Z         ],
2020-03-17T15:23:41.0557842Z         span: #13 bytes(6794321..6794332),
2020-03-17T15:23:41.0558255Z     Punct {
2020-03-17T15:23:41.0558255Z     Punct {
2020-03-17T15:23:41.0558663Z         ch: ';',
2020-03-17T15:23:41.0558897Z         spacing: Alone,
2020-03-17T15:23:41.0559193Z         span: #13 bytes(6794332..6794333),
2020-03-17T15:23:41.0559568Z ]
2020-03-17T15:23:41.0559568Z ]
2020-03-17T15:23:41.0560130Z PRINT-DERIVE INPUT (DISPLAY): struct D(::dollar_crate_external::S);
2020-03-17T15:23:41.0560825Z PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D($crate :: S) ;
2020-03-17T15:23:41.0561424Z PRINT-DERIVE INPUT (DEBUG): TokenStream [
2020-03-17T15:23:41.0561911Z         ident: "struct",
2020-03-17T15:23:41.0561911Z         ident: "struct",
2020-03-17T15:23:41.0562188Z         span: #13 bytes(6794368..6794374),
2020-03-17T15:23:41.0562600Z     Ident {
2020-03-17T15:23:41.0562797Z         ident: "D",
2020-03-17T15:23:41.0562797Z         ident: "D",
2020-03-17T15:23:41.0563273Z         span: #13 bytes(6794375..6794376),
2020-03-17T15:23:41.0563825Z     Group {
2020-03-17T15:23:41.0564045Z         delimiter: Parenthesis,
2020-03-17T15:23:41.0564330Z         stream: TokenStream [
2020-03-17T15:23:41.0564565Z             Ident {
2020-03-17T15:23:41.0564565Z             Ident {
2020-03-17T15:23:41.0564808Z                 ident: "$crate",
2020-03-17T15:23:41.0565124Z                 span: #13 bytes(6794377..6794383),
2020-03-17T15:23:41.0565602Z             Punct {
2020-03-17T15:23:41.0565602Z             Punct {
2020-03-17T15:23:41.0566190Z                 ch: ':',
2020-03-17T15:23:41.0566495Z                 spacing: Joint,
2020-03-17T15:23:41.0566812Z                 span: #13 bytes(6794383..6794385),
2020-03-17T15:23:41.0567289Z             Punct {
2020-03-17T15:23:41.0567289Z             Punct {
2020-03-17T15:23:41.0567740Z                 ch: ':',
2020-03-17T15:23:41.0568015Z                 spacing: Alone,
2020-03-17T15:23:41.0568351Z                 span: #13 bytes(6794383..6794385),
2020-03-17T15:23:41.0568822Z             Ident {
2020-03-17T15:23:41.0569073Z                 ident: "S",
2020-03-17T15:23:41.0569073Z                 ident: "S",
2020-03-17T15:23:41.0569379Z                 span: #13 bytes(6794385..6794386),
2020-03-17T15:23:41.0569822Z         ],
2020-03-17T15:23:41.0569822Z         ],
2020-03-17T15:23:41.0570084Z         span: #13 bytes(6794376..6794387),
2020-03-17T15:23:41.0570480Z     Punct {
2020-03-17T15:23:41.0570480Z     Punct {
2020-03-17T15:23:41.0570888Z         ch: ';',
2020-03-17T15:23:41.0571127Z         spacing: Alone,
2020-03-17T15:23:41.0571407Z         span: #13 bytes(6794387..6794388),
2020-03-17T15:23:41.0571802Z ]
2020-03-17T15:23:41.0571918Z 
2020-03-17T15:23:41.0572354Z ------------------------------------------
2020-03-17T15:23:41.0572605Z stderr:
2020-03-17T15:23:41.0572605Z stderr:
2020-03-17T15:23:41.0573044Z ------------------------------------------
2020-03-17T15:23:41.0573376Z error[E0428]: the name `D` is defined multiple times
2020-03-17T15:23:41.0574011Z   --> /checkout/src/test/ui/proc-macro/dollar-crate.rs:26:13
2020-03-17T15:23:41.0574294Z    |
2020-03-17T15:23:41.0574627Z LL |             struct D($crate::S); //~ ERROR the name `D` is defined multiple times
2020-03-17T15:23:41.0575292Z    |             |
2020-03-17T15:23:41.0575530Z    |             `D` redefined here
2020-03-17T15:23:41.0575530Z    |             `D` redefined here
2020-03-17T15:23:41.0575863Z    |             previous definition of the type `D` here
2020-03-17T15:23:41.0576284Z LL |     local!();
2020-03-17T15:23:41.0576786Z    |     --------- in this macro invocation
2020-03-17T15:23:41.0577017Z    |
2020-03-17T15:23:41.0577315Z    = note: `D` must be defined only once in the type namespace of this module
2020-03-17T15:23:41.0577315Z    = note: `D` must be defined only once in the type namespace of this module
2020-03-17T15:23:41.0578122Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-17T15:23:41.0578453Z 
2020-03-17T15:23:41.0578695Z error[E0428]: the name `D` is defined multiple times
2020-03-17T15:23:41.0579305Z   --> /checkout/src/test/ui/proc-macro/dollar-crate.rs:36:5
2020-03-17T15:23:41.0579605Z    |
2020-03-17T15:23:41.0579947Z LL |     dollar_crate_external::external!(); //~ ERROR the name `D` is defined multiple times
2020-03-17T15:23:41.0580618Z    |     |
2020-03-17T15:23:41.0580823Z    |     `D` redefined here
2020-03-17T15:23:41.0580823Z    |     `D` redefined here
2020-03-17T15:23:41.0581105Z    |     previous definition of the type `D` here
2020-03-17T15:23:41.0581658Z    = note: `D` must be defined only once in the type namespace of this module
2020-03-17T15:23:41.0582450Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-17T15:23:41.0582799Z 
2020-03-17T15:23:41.0583014Z error: aborting due to 2 previous errors
---
2020-03-17T15:23:41.0590715Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-17T15:23:41.0591192Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-17T15:23:41.0591511Z 
2020-03-17T15:23:41.0591640Z 
2020-03-17T15:23:41.0595896Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-17T15:23:41.0598980Z 
2020-03-17T15:23:41.0599093Z 
2020-03-17T15:23:41.0599364Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-17T15:23:41.0599735Z Build completed unsuccessfully in 1:07:51
2020-03-17T15:23:41.0599735Z Build completed unsuccessfully in 1:07:51
2020-03-17T15:23:41.0600035Z == clock drift check ==
2020-03-17T15:23:41.0600313Z   local time: Tue Mar 17 15:23:41 UTC 2020
2020-03-17T15:23:41.1995949Z   network time: Tue, 17 Mar 2020 15:23:41 GMT
2020-03-17T15:23:41.2000176Z == end clock drift check ==
2020-03-17T15:23:41.6982860Z 
2020-03-17T15:23:41.7061668Z ##[error]Bash exited with code '1'.
2020-03-17T15:23:41.7075761Z ##[section]Finishing: Run build
2020-03-17T15:23:41.7148938Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70075/merge to s
2020-03-17T15:23:41.7153965Z Task         : Get sources
2020-03-17T15:23:41.7154324Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T15:23:41.7154661Z Version      : 1.0.0
2020-03-17T15:23:41.7154889Z Author       : Microsoft
2020-03-17T15:23:41.7154889Z Author       : Microsoft
2020-03-17T15:23:41.7155249Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T15:23:41.7155676Z ==============================================================================
2020-03-17T15:23:42.0576359Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-17T15:23:42.0626664Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70075/merge to s
2020-03-17T15:23:42.0740446Z Cleaning up task key
2020-03-17T15:23:42.0742121Z Start cleaning up orphan processes.
2020-03-17T15:23:42.0934299Z Terminate orphan process: pid (5120) (python)
2020-03-17T15:23:42.1089011Z ##[section]Finishing: Finalize Job
