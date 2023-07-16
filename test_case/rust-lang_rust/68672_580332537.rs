plain
2020-01-30T15:12:08.2975543Z ========================== Starting Command Output ===========================
2020-01-30T15:12:08.2977355Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/49ab6373-6d6f-4120-bec0-a44ba6db128a.sh
2020-01-30T15:12:08.2977392Z 
2020-01-30T15:12:08.2980068Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-30T15:12:08.2987655Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68672/merge to s
2020-01-30T15:12:08.2989523Z Task         : Get sources
2020-01-30T15:12:08.2989562Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-30T15:12:08.2989645Z Version      : 1.0.0
2020-01-30T15:12:08.2989682Z Author       : Microsoft
---
2020-01-30T15:12:09.1957185Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-30T15:12:09.2048356Z ##[command]git config gc.auto 0
2020-01-30T15:12:09.2132056Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-30T15:12:09.2191809Z ##[command]git config --get-all http.proxy
2020-01-30T15:12:09.2350389Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68672/merge:refs/remotes/pull/68672/merge
---
2020-01-30T16:08:11.6386376Z .................................................................................................... 1700/9558
2020-01-30T16:08:16.6515684Z .................................................................................................... 1800/9558
2020-01-30T16:08:29.4846811Z .........................i.......................................................................... 1900/9558
2020-01-30T16:08:36.6159334Z .................................................................................................... 2000/9558
2020-01-30T16:08:51.3998089Z ...............iiiii................................................................................ 2100/9558
2020-01-30T16:09:01.5139739Z .................................................................................................... 2300/9558
2020-01-30T16:09:03.9917570Z .................................................................................................... 2400/9558
2020-01-30T16:09:09.1742432Z .................................................................................................... 2500/9558
2020-01-30T16:09:30.1972380Z .................................................................................................... 2600/9558
---
2020-01-30T16:12:07.7129288Z .................................................................................................... 4800/9558
2020-01-30T16:12:12.8624742Z ..........................................................i...............i......................... 4900/9558
2020-01-30T16:12:20.9045932Z .................................................................................................... 5000/9558
2020-01-30T16:12:28.9562408Z .................................................................................................... 5100/9558
2020-01-30T16:12:33.9962395Z .i.................................................................................................. 5200/9558
2020-01-30T16:12:44.9798618Z ..........................................................................ii.ii........i...i........ 5300/9558
2020-01-30T16:12:53.5643969Z ............i....................................................................................... 5500/9558
2020-01-30T16:13:03.6117039Z .................................................................................................... 5600/9558
2020-01-30T16:13:10.0125826Z .............................................................i...................................... 5700/9558
2020-01-30T16:13:17.2600260Z .................................................................................................... 5800/9558
2020-01-30T16:13:17.2600260Z .................................................................................................... 5800/9558
2020-01-30T16:13:25.3879656Z .................................................................................................... 5900/9558
2020-01-30T16:13:34.1480808Z ....................................................ii...i..ii...........i.......................... 6000/9558
2020-01-30T16:13:56.3329409Z .................................................................................................... 6200/9558
2020-01-30T16:14:01.8862678Z .................................................................................................... 6300/9558
2020-01-30T16:14:01.8862678Z .................................................................................................... 6300/9558
2020-01-30T16:14:06.4525289Z ................................................................................i..ii............... 6400/9558
2020-01-30T16:14:33.4050912Z .................................................................................................... 6600/9558
2020-01-30T16:14:38.9058309Z ........................................................i........................................... 6700/9558
2020-01-30T16:14:41.1196043Z .................................................................................................... 6800/9558
2020-01-30T16:14:43.4197358Z .......................................................i............................................ 6900/9558
---
2020-01-30T16:16:25.4910227Z .................................................................................................... 7600/9558
2020-01-30T16:16:30.9228457Z .................................................................................................... 7700/9558
2020-01-30T16:16:37.6787888Z .................................................................................................... 7800/9558
2020-01-30T16:16:48.5160393Z .................................................................................................... 7900/9558
2020-01-30T16:16:54.7174492Z ...........iiiiiii.i................................................................................ 8000/9558
2020-01-30T16:17:09.2740061Z .................................................................................................... 8200/9558
2020-01-30T16:17:20.1794488Z .................................................................................................... 8300/9558
2020-01-30T16:17:33.5859004Z .................................................................................................... 8400/9558
2020-01-30T16:17:40.3412886Z .................................................................................................... 8500/9558
---
2020-01-30T16:19:36.7383727Z 
2020-01-30T16:19:36.7383868Z 20 LL |     assert_sync(|| {
2020-01-30T16:19:36.7384018Z 21    |     ^^^^^^^^^^^ future returned by `main` is not `Sync`
2020-01-30T16:19:36.7384181Z 22    |
2020-01-30T16:19:36.7384729Z -    = help: within `[generator@$DIR/not-send-sync.rs:9:17: 13:6 {std::cell::Cell<i32>, (), ()}]`, the trait `std::marker::Sync` is not implemented for `std::cell::Cell<i32>`
2020-01-30T16:19:36.7385280Z +    = help: within `[generator@$DIR/not-send-sync.rs:9:17: 13:6 {std::cell::Cell<i32>, ()}]`, the trait `std::marker::Sync` is not implemented for `std::cell::Cell<i32>`
2020-01-30T16:19:36.7385458Z 24 note: future is not `Sync` as this value is used across an yield
2020-01-30T16:19:36.7385980Z 26    |
2020-01-30T16:19:36.7386097Z 
2020-01-30T16:19:36.7386214Z 
2020-01-30T16:19:36.7386369Z The actual stderr differed from the expected stderr.
2020-01-30T16:19:36.7386369Z The actual stderr differed from the expected stderr.
2020-01-30T16:19:36.7386811Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/not-send-sync/not-send-sync.stderr
2020-01-30T16:19:36.7387216Z To update references, rerun the tests and pass the `--bless` flag
2020-01-30T16:19:36.7387624Z To only update this specific test, also pass `--test-args generator/not-send-sync.rs`
2020-01-30T16:19:36.7387927Z error: 1 errors occurred comparing output.
2020-01-30T16:19:36.7388076Z status: exit code: 1
2020-01-30T16:19:36.7388076Z status: exit code: 1
2020-01-30T16:19:36.7389583Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/not-send-sync.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/not-send-sync" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/not-send-sync/auxiliary" "-A" "unused"
2020-01-30T16:19:36.7390027Z ------------------------------------------
2020-01-30T16:19:36.7390068Z 
2020-01-30T16:19:36.7390308Z ------------------------------------------
2020-01-30T16:19:36.7390380Z stderr:
2020-01-30T16:19:36.7390380Z stderr:
2020-01-30T16:19:36.7390626Z ------------------------------------------
2020-01-30T16:19:36.7390696Z error[E0277]: `std::cell::Cell<i32>` cannot be shared between threads safely
2020-01-30T16:19:36.7390960Z   --> /checkout/src/test/ui/generator/not-send-sync.rs:16:5
2020-01-30T16:19:36.7391030Z    |
2020-01-30T16:19:36.7391079Z LL |     fn assert_send<T: Send>(_: T) {}
2020-01-30T16:19:36.7391417Z ...
2020-01-30T16:19:36.7391465Z LL |     assert_send(|| {
2020-01-30T16:19:36.7391465Z LL |     assert_send(|| {
2020-01-30T16:19:36.7391969Z    |     ^^^^^^^^^^^ `std::cell::Cell<i32>` cannot be shared between threads safely
2020-01-30T16:19:36.7392090Z    = help: the trait `std::marker::Sync` is not implemented for `std::cell::Cell<i32>`
2020-01-30T16:19:36.7392305Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::cell::Cell<i32>`
2020-01-30T16:19:36.7392305Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::cell::Cell<i32>`
2020-01-30T16:19:36.7392857Z    = note: required because it appears within the type `[generator@/checkout/src/test/ui/generator/not-send-sync.rs:16:17: 20:6 a:&std::cell::Cell<i32> _]`
2020-01-30T16:19:36.7393055Z error: future cannot be shared between threads safely
2020-01-30T16:19:36.7393325Z   --> /checkout/src/test/ui/generator/not-send-sync.rs:9:5
2020-01-30T16:19:36.7393376Z    |
2020-01-30T16:19:36.7393376Z    |
2020-01-30T16:19:36.7393440Z LL |     fn assert_sync<T: Sync>(_: T) {}
2020-01-30T16:19:36.7393710Z    |        -----------    ---- required by this bound in `main::assert_sync`
2020-01-30T16:19:36.7393824Z LL |     assert_sync(|| {
2020-01-30T16:19:36.7393877Z    |     ^^^^^^^^^^^ future returned by `main` is not `Sync`
2020-01-30T16:19:36.7393926Z    |
2020-01-30T16:19:36.7393926Z    |
2020-01-30T16:19:36.7394338Z    = help: within `[generator@/checkout/src/test/ui/generator/not-send-sync.rs:9:17: 13:6 {std::cell::Cell<i32>, ()}]`, the trait `std::marker::Sync` is not implemented for `std::cell::Cell<i32>`
2020-01-30T16:19:36.7394419Z note: future is not `Sync` as this value is used across an yield
2020-01-30T16:19:36.7394756Z    |
2020-01-30T16:19:36.7394805Z LL |         let a = Cell::new(2);
2020-01-30T16:19:36.7395052Z    |             - has type `std::cell::Cell<i32>`
2020-01-30T16:19:36.7395119Z LL |         yield;
2020-01-30T16:19:36.7395119Z LL |         yield;
2020-01-30T16:19:36.7395173Z    |         ^^^^^ yield occurs here, with `a` maybe used later
2020-01-30T16:19:36.7395465Z    |     - `a` is later dropped here
2020-01-30T16:19:36.7395502Z 
2020-01-30T16:19:36.7395551Z error: aborting due to 2 previous errors
2020-01-30T16:19:36.7395583Z 
---
2020-01-30T16:19:36.7396566Z 
2020-01-30T16:19:36.7396815Z 76 LL | fn generator_capture() -> impl Sized {
2020-01-30T16:19:36.7396872Z 77    |                           ^^^^^^^^^^ expands to a recursive type
2020-01-30T16:19:36.7396921Z 78    |
2020-01-30T16:19:36.7397256Z -    = note: expanded type is `[generator@$DIR/recursive-impl-trait-type-indirect.rs:50:5: 50:26 x:impl Sized {(), ()}]`
2020-01-30T16:19:36.7397585Z +    = note: expanded type is `[generator@$DIR/recursive-impl-trait-type-indirect.rs:50:5: 50:26 x:impl Sized {()}]`
2020-01-30T16:19:36.7397708Z 81 error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7397963Z 82   --> $DIR/recursive-impl-trait-type-indirect.rs:53:26
2020-01-30T16:19:36.7397998Z 
2020-01-30T16:19:36.7397998Z 
2020-01-30T16:19:36.7398229Z 92 LL | fn generator_hold() -> impl Sized {
2020-01-30T16:19:36.7398357Z 94    |
2020-01-30T16:19:36.7398357Z 94    |
2020-01-30T16:19:36.7398672Z -    = note: expanded type is `[generator@$DIR/recursive-impl-trait-type-indirect.rs:58:5: 62:6 {impl Sized, (), ()}]`
2020-01-30T16:19:36.7399013Z +    = note: expanded type is `[generator@$DIR/recursive-impl-trait-type-indirect.rs:58:5: 62:6 {(), impl Sized}]`
2020-01-30T16:19:36.7399116Z 97 error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7399383Z 98   --> $DIR/recursive-impl-trait-type-indirect.rs:69:26
2020-01-30T16:19:36.7399418Z 
2020-01-30T16:19:36.7399445Z 
2020-01-30T16:19:36.7399445Z 
2020-01-30T16:19:36.7399495Z The actual stderr differed from the expected stderr.
2020-01-30T16:19:36.7399871Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-indirect/recursive-impl-trait-type-indirect.stderr
2020-01-30T16:19:36.7400142Z To update references, rerun the tests and pass the `--bless` flag
2020-01-30T16:19:36.7400522Z To only update this specific test, also pass `--test-args impl-trait/recursive-impl-trait-type-indirect.rs`
2020-01-30T16:19:36.7400663Z error: 1 errors occurred comparing output.
2020-01-30T16:19:36.7400712Z status: exit code: 1
2020-01-30T16:19:36.7400712Z status: exit code: 1
2020-01-30T16:19:36.7402537Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-indirect" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-indirect/auxiliary" "-A" "unused"
2020-01-30T16:19:36.7402910Z ------------------------------------------
2020-01-30T16:19:36.7402947Z 
2020-01-30T16:19:36.7403201Z ------------------------------------------
2020-01-30T16:19:36.7403539Z stderr:
2020-01-30T16:19:36.7403539Z stderr:
2020-01-30T16:19:36.7403783Z ------------------------------------------
2020-01-30T16:19:36.7403837Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7404131Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:6:22
2020-01-30T16:19:36.7404186Z    |
2020-01-30T16:19:36.7404433Z LL | fn option(i: i32) -> impl Sized { //~ ERROR
2020-01-30T16:19:36.7404556Z    |
2020-01-30T16:19:36.7404556Z    |
2020-01-30T16:19:36.7404608Z    = note: expanded type is `std::option::Option<(impl Sized, i32)>`
2020-01-30T16:19:36.7404705Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7404983Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:14:15
2020-01-30T16:19:36.7405043Z    |
2020-01-30T16:19:36.7405043Z    |
2020-01-30T16:19:36.7405287Z LL | fn tuple() -> impl Sized { //~ ERROR
2020-01-30T16:19:36.7405400Z    |
2020-01-30T16:19:36.7405400Z    |
2020-01-30T16:19:36.7405463Z    = note: expanded type is `(impl Sized,)`
2020-01-30T16:19:36.7405545Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7405835Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:18:15
2020-01-30T16:19:36.7405887Z    |
2020-01-30T16:19:36.7405887Z    |
2020-01-30T16:19:36.7406115Z LL | fn array() -> impl Sized { //~ ERROR
2020-01-30T16:19:36.7406230Z    |
2020-01-30T16:19:36.7406230Z    |
2020-01-30T16:19:36.7406280Z    = note: expanded type is `[impl Sized; 1]`
2020-01-30T16:19:36.7406375Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7406660Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:22:13
2020-01-30T16:19:36.7406713Z    |
2020-01-30T16:19:36.7406713Z    |
2020-01-30T16:19:36.7406938Z LL | fn ptr() -> impl Sized { //~ ERROR
2020-01-30T16:19:36.7407061Z    |
2020-01-30T16:19:36.7407061Z    |
2020-01-30T16:19:36.7407111Z    = note: expanded type is `*const impl Sized`
2020-01-30T16:19:36.7407207Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7407482Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:26:16
2020-01-30T16:19:36.7407547Z    |
2020-01-30T16:19:36.7407547Z    |
2020-01-30T16:19:36.7407777Z LL | fn fn_ptr() -> impl Sized { //~ ERROR
2020-01-30T16:19:36.7407882Z    |
2020-01-30T16:19:36.7408131Z    = note: expanded type is `fn() -> impl Sized`
2020-01-30T16:19:36.7408165Z 
2020-01-30T16:19:36.7408214Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7408214Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7408581Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:30:25
2020-01-30T16:19:36.7408634Z    |
2020-01-30T16:19:36.7408938Z LL | fn closure_capture() -> impl Sized { //~ ERROR
2020-01-30T16:19:36.7408996Z    |                         ^^^^^^^^^^ expands to a recursive type
2020-01-30T16:19:36.7409058Z    |
2020-01-30T16:19:36.7409390Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:32:5: 32:19 x:impl Sized]`
2020-01-30T16:19:36.7409499Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7409778Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:35:29
2020-01-30T16:19:36.7409830Z    |
2020-01-30T16:19:36.7409830Z    |
2020-01-30T16:19:36.7410086Z LL | fn closure_ref_capture() -> impl Sized { //~ ERROR
2020-01-30T16:19:36.7410191Z    |
2020-01-30T16:19:36.7410191Z    |
2020-01-30T16:19:36.7410541Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:37:5: 37:20 x:impl Sized]`
2020-01-30T16:19:36.7410639Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7410914Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:40:21
2020-01-30T16:19:36.7410978Z    |
2020-01-30T16:19:36.7410978Z    |
2020-01-30T16:19:36.7411214Z LL | fn closure_sig() -> impl Sized { //~ ERROR
2020-01-30T16:19:36.7411334Z    |
2020-01-30T16:19:36.7411334Z    |
2020-01-30T16:19:36.7412554Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:41:5: 41:21]`
2020-01-30T16:19:36.7412659Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7412936Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:44:23
2020-01-30T16:19:36.7412988Z    |
2020-01-30T16:19:36.7412988Z    |
2020-01-30T16:19:36.7413239Z LL | fn generator_sig() -> impl Sized { //~ ERROR
2020-01-30T16:19:36.7413363Z    |
2020-01-30T16:19:36.7413363Z    |
2020-01-30T16:19:36.7413682Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:45:5: 45:23]`
2020-01-30T16:19:36.7413787Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7414063Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:48:27
2020-01-30T16:19:36.7414127Z    |
2020-01-30T16:19:36.7414127Z    |
2020-01-30T16:19:36.7414367Z LL | fn generator_capture() -> impl Sized { //~ ERROR
2020-01-30T16:19:36.7414489Z    |
2020-01-30T16:19:36.7414489Z    |
2020-01-30T16:19:36.7414829Z    = note: expanded type is `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:50:5: 50:26 x:impl Sized {()}]`
2020-01-30T16:19:36.7414930Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7415220Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:53:26
2020-01-30T16:19:36.7415279Z    |
2020-01-30T16:19:36.7415279Z    |
2020-01-30T16:19:36.7415547Z LL | fn substs_change<T>() -> impl Sized { //~ ERROR
2020-01-30T16:19:36.7415672Z    |
2020-01-30T16:19:36.7415672Z    |
2020-01-30T16:19:36.7415722Z    = note: expanded type is `(impl Sized,)`
2020-01-30T16:19:36.7415818Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7416099Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:57:24
2020-01-30T16:19:36.7416149Z    |
2020-01-30T16:19:36.7416149Z    |
2020-01-30T16:19:36.7416402Z LL | fn generator_hold() -> impl Sized { //~ ERROR
2020-01-30T16:19:36.7416510Z    |
2020-01-30T16:19:36.7416510Z    |
2020-01-30T16:19:36.7416942Z    = note: expanded type is `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:58:5: 62:6 {(), impl Sized}]`
2020-01-30T16:19:36.7417079Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7417378Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:69:26
2020-01-30T16:19:36.7417429Z    |
2020-01-30T16:19:36.7419783Z LL | fn mutual_recursion() -> impl Sync { //~ ERROR
2020-01-30T16:19:36.7419783Z LL | fn mutual_recursion() -> impl Sync { //~ ERROR
2020-01-30T16:19:36.7419861Z    |                          ^^^^^^^^^ expands to a recursive type
2020-01-30T16:19:36.7420321Z    |
2020-01-30T16:19:36.7420374Z    = note: type resolves to itself
2020-01-30T16:19:36.7420407Z 
2020-01-30T16:19:36.7420473Z error[E0720]: opaque type expands to a recursive type
2020-01-30T16:19:36.7421298Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:73:28
2020-01-30T16:19:36.7421359Z    |
2020-01-30T16:19:36.7423177Z LL | fn mutual_recursion_b() -> impl Sized { //~ ERROR
2020-01-30T16:19:36.7423338Z    |
2020-01-30T16:19:36.7423396Z    = note: type resolves to itself
2020-01-30T16:19:36.7423443Z 
2020-01-30T16:19:36.7423491Z error: aborting due to 14 previous errors
---
2020-01-30T16:19:36.7428405Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-01-30T16:19:36.7428504Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-30T16:19:36.7456592Z 
2020-01-30T16:19:36.7456699Z 
2020-01-30T16:19:36.7459949Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-30T16:19:36.7460223Z 
2020-01-30T16:19:36.7460258Z 
2020-01-30T16:19:36.7460361Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-30T16:19:36.7460419Z Build completed unsuccessfully in 1:01:39
2020-01-30T16:19:36.7460419Z Build completed unsuccessfully in 1:01:39
2020-01-30T16:19:36.7524538Z == clock drift check ==
2020-01-30T16:19:36.7542455Z   local time: Thu Jan 30 16:19:36 UTC 2020
2020-01-30T16:19:37.0520678Z   network time: Thu, 30 Jan 2020 16:19:37 GMT
2020-01-30T16:19:37.0520813Z == end clock drift check ==
2020-01-30T16:19:37.5114377Z 
2020-01-30T16:19:37.5196712Z ##[error]Bash exited with code '1'.
2020-01-30T16:19:37.5228012Z ##[section]Finishing: Run build
2020-01-30T16:19:37.5262934Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68672/merge to s
2020-01-30T16:19:37.5264875Z Task         : Get sources
2020-01-30T16:19:37.5264928Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-30T16:19:37.5264995Z Version      : 1.0.0
2020-01-30T16:19:37.5265043Z Author       : Microsoft
2020-01-30T16:19:37.5265043Z Author       : Microsoft
2020-01-30T16:19:37.5265096Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-30T16:19:37.5265165Z ==============================================================================
2020-01-30T16:19:37.9900867Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-30T16:19:37.9949157Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68672/merge to s
2020-01-30T16:19:38.0076316Z Cleaning up task key
2020-01-30T16:19:38.0077312Z Start cleaning up orphan processes.
2020-01-30T16:19:38.0186844Z Terminate orphan process: pid (4547) (python)
2020-01-30T16:19:38.0475836Z ##[section]Finishing: Finalize Job
