plain
2019-11-25T02:39:13.6970927Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-25T02:39:13.7151558Z ##[command]git config gc.auto 0
2019-11-25T02:39:13.7230609Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-25T02:39:13.7284271Z ##[command]git config --get-all http.proxy
2019-11-25T02:39:13.7427600Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66726/merge:refs/remotes/pull/66726/merge
---
2019-11-25T03:39:32.6553859Z .................................................................................................... 1600/9283
2019-11-25T03:39:37.6099729Z ...............................................................................................F.... 1700/9283
2019-11-25T03:39:51.5467238Z .............................i...................................................................... 1800/9283
2019-11-25T03:39:58.4202450Z .................................................................................................... 1900/9283
2019-11-25T03:40:12.8048362Z ..............iiiii................................................................................. 2000/9283
2019-11-25T03:40:22.7739806Z .................................................................................................... 2200/9283
2019-11-25T03:40:25.3762802Z .................................................................................................... 2300/9283
2019-11-25T03:40:30.7033833Z .................................................................................................... 2400/9283
2019-11-25T03:40:52.2115988Z .................................................................................................... 2500/9283
---
2019-11-25T03:43:35.9989357Z ..............i...............i..................................................................... 4800/9283
2019-11-25T03:43:46.5320628Z .................................................................................................... 4900/9283
2019-11-25T03:43:52.4086298Z .................................................................................................... 5000/9283
2019-11-25T03:44:02.1745649Z .................................................................................................... 5100/9283
2019-11-25T03:44:08.4617166Z ...................ii.ii...........i................................................................ 5200/9283
2019-11-25T03:44:17.9110337Z .................................................................................................... 5400/9283
2019-11-25T03:44:29.1844479Z .................................................................................................... 5500/9283
2019-11-25T03:44:37.2655370Z .i.................................................................................................. 5600/9283
2019-11-25T03:44:42.8689768Z .................................................................................................... 5700/9283
2019-11-25T03:44:42.8689768Z .................................................................................................... 5700/9283
2019-11-25T03:44:53.5323344Z .......................................................................................ii...i..ii... 5800/9283
2019-11-25T03:45:16.9330728Z .................................................................................................... 6000/9283
2019-11-25T03:45:25.4002002Z .................................................................................................... 6100/9283
2019-11-25T03:45:30.8975236Z .................................................................................................... 6200/9283
2019-11-25T03:45:30.8975236Z .................................................................................................... 6200/9283
2019-11-25T03:45:43.9767031Z ..........i..ii..................................................................................... 6300/9283
2019-11-25T03:46:03.5209744Z ..............................................................................i..................... 6500/9283
2019-11-25T03:46:05.9462912Z .................................................................................................... 6600/9283
2019-11-25T03:46:08.3575463Z .....................................................................i.............................. 6700/9283
2019-11-25T03:46:11.4221751Z .................................................................................................... 6800/9283
---
2019-11-25T03:51:04.2577039Z 
2019-11-25T03:51:04.2577734Z ---- [ui] ui/consts/uninhabited-const-issue-61744.rs stdout ----
2019-11-25T03:51:04.2577931Z diff of stderr:
2019-11-25T03:51:04.2578049Z 
2019-11-25T03:51:04.2578424Z 55    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2578788Z 56    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2579164Z 57    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2580039Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2580545Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2581407Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2581902Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2582342Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2582802Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2583526Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2586502Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2589800Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2590094Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2590395Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2590683Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2590953Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2591223Z +    |     inside call to `fake_type::<!>` at $DIR/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2591520Z 58    |     inside call to `fake_type::<i32>` at $DIR/uninhabited-const-issue-61744.rs:12:36
2019-11-25T03:51:04.2591574Z 59 ...
2019-11-25T03:51:04.2591625Z 60 LL |     const CONSTANT: i32 = unsafe { fake_type() };
2019-11-25T03:51:04.2591704Z 
2019-11-25T03:51:04.2591751Z The actual stderr differed from the expected stderr.
2019-11-25T03:51:04.2592099Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/uninhabited-const-issue-61744.stderr
2019-11-25T03:51:04.2592099Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/uninhabited-const-issue-61744.stderr
2019-11-25T03:51:04.2592396Z To update references, rerun the tests and pass the `--bless` flag
2019-11-25T03:51:04.2592849Z To only update this specific test, also pass `--test-args consts/uninhabited-const-issue-61744.rs`
2019-11-25T03:51:04.2592945Z error: 1 errors occurred comparing output.
2019-11-25T03:51:04.2593153Z status: exit code: 1
2019-11-25T03:51:04.2593153Z status: exit code: 1
2019-11-25T03:51:04.2594034Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/auxiliary" "-A" "unused"
2019-11-25T03:51:04.2594354Z ------------------------------------------
2019-11-25T03:51:04.2594400Z 
2019-11-25T03:51:04.2594597Z ------------------------------------------
2019-11-25T03:51:04.2594637Z stderr:
2019-11-25T03:51:04.2594637Z stderr:
2019-11-25T03:51:04.2594824Z ------------------------------------------
2019-11-25T03:51:04.2594885Z error: any use of this value will cause an error
2019-11-25T03:51:04.2595105Z   --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2019-11-25T03:51:04.2595149Z    |
2019-11-25T03:51:04.2595214Z LL |     hint_unreachable() //~ ERROR any use of this value will cause an error
2019-11-25T03:51:04.2595296Z    |     |
2019-11-25T03:51:04.2595529Z    |     reached the configured maximum number of stack frames
2019-11-25T03:51:04.2595805Z    |     inside call to `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2019-11-25T03:51:04.2595805Z    |     inside call to `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
2019-11-25T03:51:04.2596076Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2597013Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2597308Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2597591Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2597860Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2598126Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2598583Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2599020Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2599725Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2600052Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2600346Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2601182Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2601519Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2601815Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2602109Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2602448Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2602746Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2603056Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2603351Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2604139Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2604567Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2604801Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2605034Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2605300Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2605534Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2605766Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2606021Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2606621Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2606884Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2607135Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2607797Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2608102Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2608596Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2608861Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2613042Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2613323Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2613573Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2613862Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2614124Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2616317Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2616701Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2616946Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2617204Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2617450Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2617690Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2617973Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2618218Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2618456Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2618714Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2619762Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2620097Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2620412Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2620732Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2621045Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2621340Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2621633Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2621945Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2622241Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2622532Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2622980Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2623625Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2623881Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2624134Z    |     inside call to `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
2019-11-25T03:51:04.2624558Z    |     inside call to `fake_type::<i32>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:12:36
2019-11-25T03:51:04.2624608Z ...
2019-11-25T03:51:04.2624666Z LL |     const CONSTANT: i32 = unsafe { fake_type() };
2019-11-25T03:51:04.2624936Z    |
2019-11-25T03:51:04.2624993Z    = note: `#[deny(const_err)]` on by default
2019-11-25T03:51:04.2625021Z 
2019-11-25T03:51:04.2625059Z error[E0080]: erroneous constant used
2019-11-25T03:51:04.2625059Z error[E0080]: erroneous constant used
2019-11-25T03:51:04.2625313Z   --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:18:10
2019-11-25T03:51:04.2625373Z    |
2019-11-25T03:51:04.2625413Z LL |     dbg!(i32::CONSTANT); //~ ERROR erroneous constant used
2019-11-25T03:51:04.2625457Z    |          ^^^^^^^^^^^^^ referenced constant has errors
2019-11-25T03:51:04.2625537Z error: aborting due to 2 previous errors
2019-11-25T03:51:04.2625563Z 
2019-11-25T03:51:04.2625786Z For more information about this error, try `rustc --explain E0080`.
2019-11-25T03:51:04.2625831Z 
2019-11-25T03:51:04.2625831Z 
2019-11-25T03:51:04.2626032Z ------------------------------------------
2019-11-25T03:51:04.2626059Z 
2019-11-25T03:51:04.2626081Z 
2019-11-25T03:51:04.2626308Z ---- [ui] ui/infinite/infinite-recursion-const-fn.rs stdout ----
2019-11-25T03:51:04.2626352Z diff of stderr:
2019-11-25T03:51:04.2626376Z 
2019-11-25T03:51:04.2626616Z 58    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2626894Z 59    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2627138Z 60    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2627380Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2627641Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2627883Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2628126Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2628545Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2628766Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2629617Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2630238Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2630515Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2630806Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2631083Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2631357Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2631646Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2631922Z +    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2631975Z 61 LL | const ARR: [i32; a()] = [5; 6];
2019-11-25T03:51:04.2632491Z 62    |                  --- inside call to `a` at $DIR/infinite-recursion-const-fn.rs:5:18
2019-11-25T03:51:04.2632764Z 
2019-11-25T03:51:04.2632786Z 
2019-11-25T03:51:04.2632840Z The actual stderr differed from the expected stderr.
2019-11-25T03:51:04.2633702Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn/infinite-recursion-const-fn.stderr
2019-11-25T03:51:04.2633702Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn/infinite-recursion-const-fn.stderr
2019-11-25T03:51:04.2633917Z To update references, rerun the tests and pass the `--bless` flag
2019-11-25T03:51:04.2634172Z To only update this specific test, also pass `--test-args infinite/infinite-recursion-const-fn.rs`
2019-11-25T03:51:04.2634239Z error: 1 errors occurred comparing output.
2019-11-25T03:51:04.2634276Z status: exit code: 1
2019-11-25T03:51:04.2634276Z status: exit code: 1
2019-11-25T03:51:04.2634957Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn/auxiliary" "-A" "unused"
2019-11-25T03:51:04.2635242Z ------------------------------------------
2019-11-25T03:51:04.2635270Z 
2019-11-25T03:51:04.2635449Z ------------------------------------------
2019-11-25T03:51:04.2635505Z stderr:
2019-11-25T03:51:04.2635505Z stderr:
2019-11-25T03:51:04.2635684Z ------------------------------------------
2019-11-25T03:51:04.2635725Z error[E0080]: evaluation of constant value failed
2019-11-25T03:51:04.2635948Z   --> /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:3:25
2019-11-25T03:51:04.2635988Z    |
2019-11-25T03:51:04.2636364Z LL | const fn a() -> usize { b() } //~ ERROR evaluation of constant value failed
2019-11-25T03:51:04.2636651Z    |                         |
2019-11-25T03:51:04.2636692Z    |                         reached the configured maximum number of stack frames
2019-11-25T03:51:04.2636960Z    |                         inside call to `b` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:3:25
2019-11-25T03:51:04.2636960Z    |                         inside call to `b` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:3:25
2019-11-25T03:51:04.2637146Z LL | const fn b() -> usize { a() }
2019-11-25T03:51:04.2637556Z    |                         |
2019-11-25T03:51:04.2637807Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2638241Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2638703Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2638703Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2640255Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2640572Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2640896Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2641197Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2641515Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2641817Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2642116Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2642657Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2643300Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2643709Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2643968Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2644206Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2644443Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2644701Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2644955Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2645209Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2645448Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2645684Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2645939Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2646176Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2646589Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2647056Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2648908Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2649988Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2650310Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2650612Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2650932Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2651234Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2651565Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2651888Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2652187Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2652486Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2652959Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2653381Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2653652Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2654157Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2654594Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2654857Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2655103Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2655348Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2655609Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2655860Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2656297Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2656913Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2657347Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2658009Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2658311Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2658610Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2659742Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2660180Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2660482Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2660804Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2661104Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2661404Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2661724Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2662032Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2662359Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2662819Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2663098Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2663396Z    |                         inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:4:25
2019-11-25T03:51:04.2663449Z LL | const ARR: [i32; a()] = [5; 6];
2019-11-25T03:51:04.2663880Z    |                  --- inside call to `a` at /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:5:18
2019-11-25T03:51:04.2663972Z error: aborting due to previous error
2019-11-25T03:51:04.2663999Z 
2019-11-25T03:51:04.2664769Z For more information about this error, try `rustc --explain E0080`.
2019-11-25T03:51:04.2664988Z 
---
2019-11-25T03:51:04.2666271Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-25T03:51:04.2666337Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-25T03:51:04.2668009Z 
2019-11-25T03:51:04.2668055Z 
2019-11-25T03:51:04.2670265Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-25T03:51:04.2670517Z 
2019-11-25T03:51:04.2670548Z 
2019-11-25T03:51:04.2670611Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-25T03:51:04.2670662Z Build completed unsuccessfully in 1:05:40
2019-11-25T03:51:04.2670662Z Build completed unsuccessfully in 1:05:40
2019-11-25T03:51:04.2676406Z == clock drift check ==
2019-11-25T03:51:04.2693747Z   local time: Mon Nov 25 03:51:04 UTC 2019
2019-11-25T03:51:04.5477431Z   network time: Mon, 25 Nov 2019 03:51:04 GMT
2019-11-25T03:51:04.5477575Z == end clock drift check ==
2019-11-25T03:51:05.3193293Z 
2019-11-25T03:51:05.3318713Z ##[error]Bash exited with code '1'.
2019-11-25T03:51:05.3355767Z ##[section]Starting: Checkout
2019-11-25T03:51:05.3357434Z ==============================================================================
2019-11-25T03:51:05.3357500Z Task         : Get sources
2019-11-25T03:51:05.3357544Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
