plain
2019-11-11T14:38:00.9124786Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T14:38:00.9334675Z ##[command]git config gc.auto 0
2019-11-11T14:38:00.9415612Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T14:38:00.9479280Z ##[command]git config --get-all http.proxy
2019-11-11T14:38:00.9621373Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66294/merge:refs/remotes/pull/66294/merge
---
2019-11-11T15:38:12.2153516Z .................................................................................................... 1400/9228
2019-11-11T15:38:18.6368475Z .................................................................................................... 1500/9228
2019-11-11T15:38:24.7475845Z ..................................F................................................................. 1600/9228
2019-11-11T15:38:33.9951580Z ....................................................................F............................... 1700/9228
2019-11-11T15:38:42.5181127Z ..i................................................................................................. 1800/9228
2019-11-11T15:38:49.5112675Z ......................................................................................iiiii......... 1900/9228
2019-11-11T15:39:11.0005174Z .................................................................................................... 2100/9228
2019-11-11T15:39:13.3625620Z .................................................................................................... 2200/9228
2019-11-11T15:39:16.3709915Z .................................................................................................... 2300/9228
2019-11-11T15:39:26.2193253Z .................................................................................................... 2400/9228
---
2019-11-11T15:42:19.4663757Z ..................................................................................i...............i. 4700/9228
2019-11-11T15:42:26.4144128Z .................................................................................................... 4800/9228
2019-11-11T15:42:35.6922243Z .................................................................................................... 4900/9228
2019-11-11T15:42:40.9710589Z .................................................................................................... 5000/9228
2019-11-11T15:42:52.5323474Z .....................................................................................ii.ii.......... 5100/9228
2019-11-11T15:42:56.2484369Z .i.................................................................................................. 5200/9228
2019-11-11T15:43:10.9477182Z .................................................................................................... 5400/9228
2019-11-11T15:43:17.9755307Z ...................................................................i................................ 5500/9228
2019-11-11T15:43:25.3837449Z .................................................................................................... 5600/9228
2019-11-11T15:43:33.2114753Z .................................................................................................... 5700/9228
2019-11-11T15:43:33.2114753Z .................................................................................................... 5700/9228
2019-11-11T15:43:42.3649339Z ....................................................ii...i..ii...........i.......................... 5800/9228
2019-11-11T15:44:05.2088149Z .................................................................................................... 6000/9228
2019-11-11T15:44:13.5442262Z .................................................................................................... 6100/9228
2019-11-11T15:44:13.5442262Z .................................................................................................... 6100/9228
2019-11-11T15:44:18.5742879Z .......................................................................i..ii........................ 6200/9228
2019-11-11T15:44:47.8929435Z .................................................................................................... 6400/9228
2019-11-11T15:44:50.0915527Z .......................................i............................................................ 6500/9228
2019-11-11T15:44:52.3625454Z .................................................................................................... 6600/9228
2019-11-11T15:44:54.7731281Z .......................i............................................................................ 6700/9228
---
2019-11-11T15:49:43.7304139Z 
2019-11-11T15:49:43.7304942Z ---- [ui] ui/consts/const-size_of-cycle.rs stdout ----
2019-11-11T15:49:43.7305241Z diff of stderr:
2019-11-11T15:49:43.7305423Z 
2019-11-11T15:49:43.7305640Z 10 LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-11-11T15:49:43.7305832Z 11    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-11T15:49:43.7306391Z 12 note: ...which requires const-evaluating `Foo::bytes::{{constant}}#0`...
2019-11-11T15:49:43.7306958Z +   --> $DIR/const-size_of-cycle.rs:5:17
2019-11-11T15:49:43.7307415Z +    |
2019-11-11T15:49:43.7307506Z + LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-11-11T15:49:43.7307590Z +    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-11T15:49:43.7309840Z + note: ...which requires const-evaluating `std::mem::size_of`...
2019-11-11T15:49:43.7310638Z 13   --> $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-11-11T15:49:43.7310872Z 15 LL |     intrinsics::size_of::<T>()
2019-11-11T15:49:43.7310922Z 
2019-11-11T15:49:43.7310950Z 
2019-11-11T15:49:43.7310995Z The actual stderr differed from the expected stderr.
2019-11-11T15:49:43.7310995Z The actual stderr differed from the expected stderr.
2019-11-11T15:49:43.7311355Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
2019-11-11T15:49:43.7311637Z To update references, rerun the tests and pass the `--bless` flag
2019-11-11T15:49:43.7311906Z To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`
2019-11-11T15:49:43.7312005Z error: 1 errors occurred comparing output.
2019-11-11T15:49:43.7312050Z status: exit code: 1
2019-11-11T15:49:43.7312050Z status: exit code: 1
2019-11-11T15:49:43.7312825Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary" "-A" "unused"
2019-11-11T15:49:43.7313171Z ------------------------------------------
2019-11-11T15:49:43.7313221Z 
2019-11-11T15:49:43.7313760Z ------------------------------------------
2019-11-11T15:49:43.7313815Z stderr:
2019-11-11T15:49:43.7313815Z stderr:
2019-11-11T15:49:43.7314046Z ------------------------------------------
2019-11-11T15:49:43.7314318Z error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}#0`
2019-11-11T15:49:43.7314651Z    |
2019-11-11T15:49:43.7314651Z    |
2019-11-11T15:49:43.7314697Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-11-11T15:49:43.7314801Z    |
2019-11-11T15:49:43.7314801Z    |
2019-11-11T15:49:43.7315062Z note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}#0`...
2019-11-11T15:49:43.7315350Z    |
2019-11-11T15:49:43.7315350Z    |
2019-11-11T15:49:43.7315409Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-11-11T15:49:43.7315456Z    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-11T15:49:43.7315707Z note: ...which requires const-evaluating `Foo::bytes::{{constant}}#0`...
2019-11-11T15:49:43.7316009Z    |
2019-11-11T15:49:43.7316009Z    |
2019-11-11T15:49:43.7316052Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-11-11T15:49:43.7316114Z    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-11T15:49:43.7316670Z note: ...which requires const-evaluating `std::mem::size_of`...
2019-11-11T15:49:43.7316989Z    |
2019-11-11T15:49:43.7317032Z LL |     intrinsics::size_of::<T>()
2019-11-11T15:49:43.7317077Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-11T15:49:43.7317077Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-11T15:49:43.7317349Z note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
2019-11-11T15:49:43.7317624Z    |
2019-11-11T15:49:43.7317624Z    |
2019-11-11T15:49:43.7317828Z LL |     pub fn size_of<T>() -> usize;
2019-11-11T15:49:43.7317892Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-11T15:49:43.7317941Z    = note: ...which requires computing layout of `Foo`...
2019-11-11T15:49:43.7318001Z    = note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [u8; _] }`...
2019-11-11T15:49:43.7318333Z    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}#0`, completing the cycle
2019-11-11T15:49:43.7318399Z note: cycle used when processing `Foo`
2019-11-11T15:49:43.7318698Z    |
2019-11-11T15:49:43.7318739Z LL | struct Foo {
2019-11-11T15:49:43.7318781Z    | ^^^^^^^^^^
2019-11-11T15:49:43.7318827Z 
---
2019-11-11T15:49:43.7319456Z 
2019-11-11T15:49:43.7319703Z ---- [ui] ui/consts/uninhabited-const-issue-61744.rs stdout ----
2019-11-11T15:49:43.7319752Z diff of stderr:
2019-11-11T15:49:43.7319779Z 
2019-11-11T15:49:43.7320000Z - error: any use of this value will cause an error
2019-11-11T15:49:43.7320286Z + error[E0391]: cycle detected when const-evaluating `hint_unreachable`
2019-11-11T15:49:43.7320515Z +   --> $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7320618Z + LL |     fake_type()
2019-11-11T15:49:43.7320661Z +    |     ^^^^^^^^^^^
2019-11-11T15:49:43.7320702Z +    |
2019-11-11T15:49:43.7320702Z +    |
2019-11-11T15:49:43.7320931Z + note: ...which requires const-evaluating `fake_type`...
2019-11-11T15:49:43.7321177Z 2   --> $DIR/uninhabited-const-issue-61744.rs:4:5
2019-11-11T15:49:43.7321264Z 4 LL |     hint_unreachable()
2019-11-11T15:49:43.7321308Z 
2019-11-11T15:49:43.7321350Z 5    |     ^^^^^^^^^^^^^^^^^^
2019-11-11T15:49:43.7321536Z -    |     |
2019-11-11T15:49:43.7321536Z -    |     |
2019-11-11T15:49:43.7321770Z -    |     reached the configured maximum number of stack frames
2019-11-11T15:49:43.7322057Z -    |     inside call to `hint_unreachable` at $DIR/uninhabited-const-issue-61744.rs:4:5
2019-11-11T15:49:43.7322321Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7322603Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7322889Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7323149Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7323737Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7324456Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7324787Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7325069Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7325338Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7325732Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7326120Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7326388Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7326650Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7326926Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7327193Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7327454Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7327731Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7327995Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7328267Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7328562Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7328824Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7329085Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7329368Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7329629Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7329889Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7330171Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7330433Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7330716Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7331002Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7331263Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7331525Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7331807Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7332068Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7332329Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7332611Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7332873Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7333173Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7333759Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7334030Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7334312Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7334579Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7334839Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7335116Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7335381Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7335748Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7336119Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7336386Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7336647Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7336925Z -    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7337198Z -    |     inside call to `fake_type::<i32>` at $DIR/uninhabited-const-issue-61744.rs:12:36
2019-11-11T15:49:43.7337379Z - ...
2019-11-11T15:49:43.7337778Z - LL |     const CONSTANT: i32 = unsafe { fake_type() };
2019-11-11T15:49:43.7338010Z -    |     ---------------------------------------------
2019-11-11T15:49:43.7338276Z +    = note: ...which again requires const-evaluating `hint_unreachable`, completing the cycle
2019-11-11T15:49:43.7338524Z + note: cycle used when const-evaluating `fake_type`
2019-11-11T15:49:43.7338770Z +   --> $DIR/uninhabited-const-issue-61744.rs:4:5
2019-11-11T15:49:43.7339055Z -    = note: `#[deny(const_err)]` on by default
2019-11-11T15:49:43.7339104Z + LL |     hint_unreachable()
2019-11-11T15:49:43.7339147Z +    |     ^^^^^^^^^^^^^^^^^^
2019-11-11T15:49:43.7339187Z 64 
2019-11-11T15:49:43.7339187Z 64 
2019-11-11T15:49:43.7339419Z - error[E0080]: erroneous constant used
2019-11-11T15:49:43.7339643Z -   --> $DIR/uninhabited-const-issue-61744.rs:18:10
2019-11-11T15:49:43.7339858Z -    |
2019-11-11T15:49:43.7340104Z - LL |     dbg!(i32::CONSTANT);
2019-11-11T15:49:43.7340359Z -    |          ^^^^^^^^^^^^^ referenced constant has errors
2019-11-11T15:49:43.7340472Z 70 
2019-11-11T15:49:43.7340711Z - error: aborting due to 2 previous errors
2019-11-11T15:49:43.7340903Z - 
2019-11-11T15:49:43.7341164Z - For more information about this error, try `rustc --explain E0080`.
2019-11-11T15:49:43.7341164Z - For more information about this error, try `rustc --explain E0080`.
2019-11-11T15:49:43.7341474Z + For more information about this error, try `rustc --explain E0391`.
2019-11-11T15:49:43.7341524Z 74 
2019-11-11T15:49:43.7341554Z 
2019-11-11T15:49:43.7341582Z 
2019-11-11T15:49:43.7341646Z The actual stderr differed from the expected stderr.
2019-11-11T15:49:43.7342002Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/uninhabited-const-issue-61744.stderr
2019-11-11T15:49:43.7342269Z To update references, rerun the tests and pass the `--bless` flag
2019-11-11T15:49:43.7342588Z To only update this specific test, also pass `--test-args consts/uninhabited-const-issue-61744.rs`
2019-11-11T15:49:43.7342671Z error: 1 errors occurred comparing output.
2019-11-11T15:49:43.7342734Z status: exit code: 1
2019-11-11T15:49:43.7342734Z status: exit code: 1
2019-11-11T15:49:43.7343812Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/auxiliary" "-A" "unused"
2019-11-11T15:49:43.7344217Z ------------------------------------------
2019-11-11T15:49:43.7344254Z 
2019-11-11T15:49:43.7344489Z ------------------------------------------
2019-11-11T15:49:43.7344534Z stderr:
2019-11-11T15:49:43.7344534Z stderr:
2019-11-11T15:49:43.7344746Z ------------------------------------------
2019-11-11T15:49:43.7345004Z error[E0391]: cycle detected when const-evaluating `hint_unreachable`
2019-11-11T15:49:43.7345261Z   --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-11T15:49:43.7345368Z LL |     fake_type()
2019-11-11T15:49:43.7345583Z    |     ^^^^^^^^^^^
2019-11-11T15:49:43.7345628Z    |
2019-11-11T15:49:43.7345628Z    |
2019-11-11T15:49:43.7345890Z note: ...which requires const-evaluating `fake_type`...
2019-11-11T15:49:43.7346159Z   --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2019-11-11T15:49:43.7346206Z    |
2019-11-11T15:49:43.7346254Z LL |     hint_unreachable() //~ ERROR any use of this value will cause an error
2019-11-11T15:49:43.7346319Z    |     ^^^^^^^^^^^^^^^^^^
2019-11-11T15:49:43.7346584Z    = note: ...which again requires const-evaluating `hint_unreachable`, completing the cycle
2019-11-11T15:49:43.7346815Z note: cycle used when const-evaluating `fake_type`
2019-11-11T15:49:43.7347086Z   --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2019-11-11T15:49:43.7347133Z    |
2019-11-11T15:49:43.7347181Z LL |     hint_unreachable() //~ ERROR any use of this value will cause an error
2019-11-11T15:49:43.7348854Z 
2019-11-11T15:49:43.7349121Z error: aborting due to previous error
2019-11-11T15:49:43.7349156Z 
2019-11-11T15:49:43.7351900Z For more information about this error, try `rustc --explain E0391`.
---
2019-11-11T15:49:43.7359161Z - error[E0080]: evaluation of constant value failed
2019-11-11T15:49:43.7359733Z + error[E0391]: cycle detected when const-evaluating `a`
2019-11-11T15:49:43.7359975Z 2   --> $DIR/infinite-recursion-const-fn.rs:3:25
2019-11-11T15:49:43.7360044Z 3    |
2019-11-11T15:49:43.7360259Z 4 LL | const fn a() -> usize { b() }
2019-11-11T15:49:43.7360334Z 5    |                         ^^^
2019-11-11T15:49:43.7360552Z -    |                         |
2019-11-11T15:49:43.7360842Z -    |                         reached the configured maximum number of stack frames
2019-11-11T15:49:43.7360842Z -    |                         reached the configured maximum number of stack frames
2019-11-11T15:49:43.7361115Z -    |                         inside call to `b` at $DIR/infinite-recursion-const-fn.rs:3:25
2019-11-11T15:49:43.7361179Z +    |
2019-11-11T15:49:43.7361403Z + note: ...which requires const-evaluating `b`...
2019-11-11T15:49:43.7361625Z +   --> $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-11T15:49:43.7361669Z +    |
2019-11-11T15:49:43.7361896Z 9 LL | const fn b() -> usize { a() }
2019-11-11T15:49:43.7362304Z -    |                         |
2019-11-11T15:49:43.7362587Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-11T15:49:43.7362857Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-11T15:49:43.7363128Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
---
2019-11-11T15:49:43.7385102Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-11T15:49:43.7385373Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-11T15:49:43.7385639Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-11T15:49:43.7385712Z +    |                         ^^^
2019-11-11T15:49:43.7385965Z +    = note: ...which again requires const-evaluating `a`, completing the cycle
2019-11-11T15:49:43.7386204Z + note: cycle used when const-evaluating `ARR::{{constant}}#0`
2019-11-11T15:49:43.7386452Z +   --> $DIR/infinite-recursion-const-fn.rs:5:18
2019-11-11T15:49:43.7386499Z +    |
2019-11-11T15:49:43.7386543Z 61 LL | const ARR: [i32; a()] = [5; 6];
2019-11-11T15:49:43.7386834Z -    |                  --- inside call to `a` at $DIR/infinite-recursion-const-fn.rs:5:18
2019-11-11T15:49:43.7386935Z 63 
2019-11-11T15:49:43.7386994Z 64 error: aborting due to previous error
2019-11-11T15:49:43.7387036Z 65 
2019-11-11T15:49:43.7387064Z 
2019-11-11T15:49:43.7387064Z 
2019-11-11T15:49:43.7387313Z - For more information about this error, try `rustc --explain E0080`.
2019-11-11T15:49:43.7387576Z + For more information about this error, try `rustc --explain E0391`.
2019-11-11T15:49:43.7387622Z 67 
2019-11-11T15:49:43.7387647Z 
2019-11-11T15:49:43.7387673Z 
2019-11-11T15:49:43.7387734Z The actual stderr differed from the expected stderr.
2019-11-11T15:49:43.7388058Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn/infinite-recursion-const-fn.stderr
2019-11-11T15:49:43.7388307Z To update references, rerun the tests and pass the `--bless` flag
2019-11-11T15:49:43.7388613Z To only update this specific test, also pass `--test-args infinite/infinite-recursion-const-fn.rs`
2019-11-11T15:49:43.7388700Z error: 1 errors occurred comparing output.
2019-11-11T15:49:43.7388761Z status: exit code: 1
2019-11-11T15:49:43.7388761Z status: exit code: 1
2019-11-11T15:49:43.7390575Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn/auxiliary" "-A" "unused"
2019-11-11T15:49:43.7390946Z ------------------------------------------
2019-11-11T15:49:43.7390982Z 
2019-11-11T15:49:43.7391214Z ------------------------------------------
2019-11-11T15:49:43.7391282Z stderr:
2019-11-11T15:49:43.7391282Z stderr:
2019-11-11T15:49:43.7391499Z ------------------------------------------
2019-11-11T15:49:43.7391728Z error[E0391]: cycle detected when const-evaluating `a`
2019-11-11T15:49:43.7391996Z   --> /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:3:25
2019-11-11T15:49:43.7392046Z    |
2019-11-11T15:49:43.7392295Z LL | const fn a() -> usize { b() } //~ ERROR evaluation of constant value failed
2019-11-11T15:49:43.7392405Z    |
2019-11-11T15:49:43.7392405Z    |
2019-11-11T15:49:43.7392625Z note: ...which requires const-evaluating `b`...
2019-11-11T15:49:43.7392936Z    |
2019-11-11T15:49:43.7392936Z    |
2019-11-11T15:49:43.7393137Z LL | const fn b() -> usize { a() }
2019-11-11T15:49:43.7393201Z    |                         ^^^
2019-11-11T15:49:43.7393838Z    = note: ...which again requires const-evaluating `a`, completing the cycle
2019-11-11T15:49:43.7394343Z note: cycle used when const-evaluating `ARR::{{constant}}#0`
2019-11-11T15:49:43.7394690Z    |
2019-11-11T15:49:43.7394690Z    |
2019-11-11T15:49:43.7394733Z LL | const ARR: [i32; a()] = [5; 6];
2019-11-11T15:49:43.7394824Z 
2019-11-11T15:49:43.7394865Z error: aborting due to previous error
2019-11-11T15:49:43.7394894Z 
2019-11-11T15:49:43.7395137Z For more information about this error, try `rustc --explain E0391`.
---
2019-11-11T15:49:43.7395691Z ---- [ui] ui/invalid_const_promotion.rs stdout ----
2019-11-11T15:49:43.7395723Z 
2019-11-11T15:49:43.7395947Z error: test compilation failed although it shouldn't!
2019-11-11T15:49:43.7395995Z status: exit code: 1
2019-11-11T15:49:43.7396769Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/invalid_const_promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid_const_promotion/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid_const_promotion/auxiliary"
2019-11-11T15:49:43.7397102Z ------------------------------------------
2019-11-11T15:49:43.7397135Z 
2019-11-11T15:49:43.7397349Z ------------------------------------------
2019-11-11T15:49:43.7397409Z stderr:
2019-11-11T15:49:43.7397409Z stderr:
2019-11-11T15:49:43.7397621Z ------------------------------------------
2019-11-11T15:49:43.7397670Z error[E0080]: evaluation of constant value failed
2019-11-11T15:49:43.7397918Z   --> /checkout/src/test/ui/invalid_const_promotion.rs:22:27
2019-11-11T15:49:43.7397977Z    |
2019-11-11T15:49:43.7398205Z LL | const fn bar() -> usize { 0 - 1 }
2019-11-11T15:49:43.7398274Z    |                           ^^^^^ attempt to subtract with overflow
2019-11-11T15:49:43.7398349Z error[E0080]: evaluation of constant expression failed
2019-11-11T15:49:43.7398588Z   --> /checkout/src/test/ui/invalid_const_promotion.rs:25:25
2019-11-11T15:49:43.7398651Z    |
2019-11-11T15:49:43.7398855Z LL |     let _: &'static _ = &bar();
---
2019-11-11T15:49:43.7400079Z ---- [ui] ui/union/union-nodrop.rs stdout ----
2019-11-11T15:49:43.7400111Z 
2019-11-11T15:49:43.7401122Z error: test compilation failed although it shouldn't!
2019-11-11T15:49:43.7401185Z status: exit code: 101
2019-11-11T15:49:43.7401935Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-nodrop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-nodrop/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-nodrop/auxiliary"
2019-11-11T15:49:43.7402265Z ------------------------------------------
2019-11-11T15:49:43.7402429Z 
2019-11-11T15:49:43.7402780Z ------------------------------------------
2019-11-11T15:49:43.7402833Z stderr:
2019-11-11T15:49:43.7402833Z stderr:
2019-11-11T15:49:43.7403066Z ------------------------------------------
2019-11-11T15:49:43.7403602Z thread 'rustc' panicked at 'assertion failed: body.arg_count == 0', src/librustc_mir/const_eval.rs:149:5
2019-11-11T15:49:43.7403708Z 
2019-11-11T15:49:43.7403770Z error: internal compiler error: unexpected panic
2019-11-11T15:49:43.7403799Z 
2019-11-11T15:49:43.7403844Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-11T15:49:43.7403844Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-11T15:49:43.7403873Z 
2019-11-11T15:49:43.7404531Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-11-11T15:49:43.7404826Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-11-11T15:49:43.7404873Z 
2019-11-11T15:49:43.7404873Z 
2019-11-11T15:49:43.7405166Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-11-11T15:49:43.7405237Z 
2019-11-11T15:49:43.7405463Z ------------------------------------------
2019-11-11T15:49:43.7405495Z 
2019-11-11T15:49:43.7405520Z 
---
2019-11-11T15:49:43.7407190Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-11T15:49:43.7407257Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-11T15:49:43.7407312Z 
2019-11-11T15:49:43.7407337Z 
2019-11-11T15:49:43.7408843Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-11T15:49:43.7409089Z 
2019-11-11T15:49:43.7409117Z 
2019-11-11T15:49:43.7409162Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-11T15:49:43.7409227Z Build completed unsuccessfully in 1:05:05
2019-11-11T15:49:43.7409227Z Build completed unsuccessfully in 1:05:05
2019-11-11T15:49:43.7449149Z == clock drift check ==
2019-11-11T15:49:43.7468434Z   local time: Mon Nov 11 15:49:43 UTC 2019
2019-11-11T15:49:44.0390355Z   network time: Mon, 11 Nov 2019 15:49:44 GMT
2019-11-11T15:49:44.0392023Z == end clock drift check ==
2019-11-11T15:49:44.7347468Z 
2019-11-11T15:49:44.7468409Z ##[error]Bash exited with code '1'.
2019-11-11T15:49:44.7529159Z ##[section]Starting: Checkout
2019-11-11T15:49:44.7531309Z ==============================================================================
2019-11-11T15:49:44.7531376Z Task         : Get sources
2019-11-11T15:49:44.7531447Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
