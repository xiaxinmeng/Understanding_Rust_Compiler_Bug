plain
2019-11-12T11:24:43.7726710Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T11:24:43.7930358Z ##[command]git config gc.auto 0
2019-11-12T11:24:43.7990806Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T11:24:43.8057486Z ##[command]git config --get-all http.proxy
2019-11-12T11:24:43.8212133Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66325/merge:refs/remotes/pull/66325/merge
---
2019-11-12T12:23:05.5491116Z .................................................................................................... 1400/9231
2019-11-12T12:23:12.0531995Z .................................................................................................... 1500/9231
2019-11-12T12:23:18.1531403Z .................................................................................................... 1600/9231
2019-11-12T12:23:27.2727522Z .................................................................................................... 1700/9231
2019-11-12T12:23:35.6308270Z ..i................................................................................................. 1800/9231
2019-11-12T12:23:42.3382372Z ......................................................................................iiiii......... 1900/9231
2019-11-12T12:24:03.3142266Z .................................................................................................... 2100/9231
2019-11-12T12:24:05.6873366Z .................................................................................................... 2200/9231
2019-11-12T12:24:08.1616462Z .................................................................................................... 2300/9231
2019-11-12T12:24:17.7123783Z .................................................................................................... 2400/9231
---
2019-11-12T12:27:10.1714385Z ..................................................................................i...............i. 4700/9231
2019-11-12T12:27:17.2946681Z .................................................................................................... 4800/9231
2019-11-12T12:27:26.3464437Z .................................................................................................... 4900/9231
2019-11-12T12:27:31.7432181Z .................................................................................................... 5000/9231
2019-11-12T12:27:43.0615944Z .....................................................................................ii.ii.......... 5100/9231
2019-11-12T12:27:46.8971491Z .i.................................................................................................. 5200/9231
2019-11-12T12:28:01.4880973Z .................................................................................................... 5400/9231
2019-11-12T12:28:08.5261844Z ...................................................................i..................F............. 5500/9231
2019-11-12T12:28:15.9022072Z .................................................................................................... 5600/9231
2019-11-12T12:28:23.6737510Z .................................................................................................... 5700/9231
2019-11-12T12:28:23.6737510Z .................................................................................................... 5700/9231
2019-11-12T12:28:32.5666517Z ....................................................ii...i...ii..........i.......................... 5800/9231
2019-11-12T12:28:55.0488881Z .................................................................................................... 6000/9231
2019-11-12T12:29:02.6249717Z .................................................................................................... 6100/9231
2019-11-12T12:29:02.6249717Z .................................................................................................... 6100/9231
2019-11-12T12:29:07.8444791Z .......................................................................i..ii........................ 6200/9231
2019-11-12T12:29:36.8291243Z .................................................................................................... 6400/9231
2019-11-12T12:29:39.0411245Z .......................................i............................................................ 6500/9231
2019-11-12T12:29:41.3503994Z .................................................................................................... 6600/9231
2019-11-12T12:29:43.7119079Z .......................i............................................................................ 6700/9231
---
2019-11-12T12:34:31.2627564Z 
2019-11-12T12:34:31.2627589Z 
2019-11-12T12:34:31.2627615Z 
2019-11-12T12:34:31.2627680Z The actual stderr differed from the expected stderr.
2019-11-12T12:34:31.2628051Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/label_break_value/label_break_value.stderr
2019-11-12T12:34:31.2628305Z To update references, rerun the tests and pass the `--bless` flag
2019-11-12T12:34:31.2628603Z To only update this specific test, also pass `--test-args for-loop-while/label_break_value.rs`
2019-11-12T12:34:31.2628825Z error: 1 errors occurred comparing output.
2019-11-12T12:34:31.2628892Z status: exit code: 0
2019-11-12T12:34:31.2628892Z status: exit code: 0
2019-11-12T12:34:31.2629707Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/for-loop-while/label_break_value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/label_break_value/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/label_break_value/auxiliary"
2019-11-12T12:34:31.2630342Z ------------------------------------------
2019-11-12T12:34:31.2630370Z 
2019-11-12T12:34:31.2630728Z ------------------------------------------
2019-11-12T12:34:31.2630771Z stderr:
---
2019-11-12T12:34:31.2632146Z normalized stderr:
2019-11-12T12:34:31.2632177Z warning: unused label
2019-11-12T12:34:31.2632348Z   --> $DIR/loop-label-shadowing.rs:8:5
2019-11-12T12:34:31.2632382Z    |
2019-11-12T12:34:31.2632537Z LL |     'foo: for i in &[1, 2, 3] {
2019-11-12T12:34:31.2632626Z    |
2019-11-12T12:34:31.2632659Z    = note: `#[warn(unused_labels)]` on by default
2019-11-12T12:34:31.2632682Z 
2019-11-12T12:34:31.2632702Z 
2019-11-12T12:34:31.2632702Z 
2019-11-12T12:34:31.2632738Z 
2019-11-12T12:34:31.2632756Z 
2019-11-12T12:34:31.2632796Z The actual stderr differed from the expected stderr.
2019-11-12T12:34:31.2633043Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/loop-label-shadowing/loop-label-shadowing.stderr
2019-11-12T12:34:31.2633255Z To update references, rerun the tests and pass the `--bless` flag
2019-11-12T12:34:31.2633458Z To only update this specific test, also pass `--test-args for-loop-while/loop-label-shadowing.rs`
2019-11-12T12:34:31.2633536Z error: 1 errors occurred comparing output.
2019-11-12T12:34:31.2633569Z status: exit code: 0
2019-11-12T12:34:31.2633569Z status: exit code: 0
2019-11-12T12:34:31.2634222Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/for-loop-while/loop-label-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/loop-label-shadowing/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/loop-label-shadowing/auxiliary"
2019-11-12T12:34:31.2634547Z ------------------------------------------
2019-11-12T12:34:31.2634592Z 
2019-11-12T12:34:31.2634770Z ------------------------------------------
2019-11-12T12:34:31.2634808Z stderr:
2019-11-12T12:34:31.2634808Z stderr:
2019-11-12T12:34:31.2634993Z ------------------------------------------
2019-11-12T12:34:31.2635032Z warning: unused label
2019-11-12T12:34:31.2635227Z   --> /checkout/src/test/ui/for-loop-while/loop-label-shadowing.rs:8:5
2019-11-12T12:34:31.2635284Z    |
2019-11-12T12:34:31.2635572Z LL |     'foo: for i in &[1, 2, 3] {
2019-11-12T12:34:31.2635643Z    |
2019-11-12T12:34:31.2635694Z    = note: `#[warn(unused_labels)]` on by default
2019-11-12T12:34:31.2635719Z 
2019-11-12T12:34:31.2635740Z 
---
2019-11-12T12:34:31.2636890Z 
2019-11-12T12:34:31.2636933Z + warning: unused label
2019-11-12T12:34:31.2637154Z +   --> $DIR/hygienic-labels-in-let.rs:15:9
2019-11-12T12:34:31.2637201Z +    |
2019-11-12T12:34:31.2637415Z + LL |         'x: loop { $e }
2019-11-12T12:34:31.2637506Z + ...
2019-11-12T12:34:31.2637506Z + ...
2019-11-12T12:34:31.2637724Z + LL |             loop_x!(break 'x);
2019-11-12T12:34:31.2638020Z +    |
2019-11-12T12:34:31.2638082Z +    = note: `#[warn(unused_labels)]` on by default
2019-11-12T12:34:31.2638127Z + 
2019-11-12T12:34:31.2638169Z + warning: unused label
2019-11-12T12:34:31.2638169Z + warning: unused label
2019-11-12T12:34:31.2638386Z +   --> $DIR/hygienic-labels-in-let.rs:15:9
2019-11-12T12:34:31.2638447Z +    |
2019-11-12T12:34:31.2638649Z + LL |         'x: loop { $e }
2019-11-12T12:34:31.2638754Z + ...
2019-11-12T12:34:31.2638754Z + ...
2019-11-12T12:34:31.2638958Z + LL |             loop_x!(break 'x);
2019-11-12T12:34:31.2639254Z + 
2019-11-12T12:34:31.2639297Z + warning: unused label
2019-11-12T12:34:31.2639511Z +   --> $DIR/hygienic-labels-in-let.rs:26:9
2019-11-12T12:34:31.2639558Z +    |
2019-11-12T12:34:31.2639558Z +    |
2019-11-12T12:34:31.2639786Z + LL |         'x: while 1 + 1 == 2 { $e }
2019-11-12T12:34:31.2639873Z + ...
2019-11-12T12:34:31.2639873Z + ...
2019-11-12T12:34:31.2640219Z + LL |             while_true!(break 'x);
2019-11-12T12:34:31.2640452Z + 
2019-11-12T12:34:31.2640486Z + warning: unused label
2019-11-12T12:34:31.2640667Z +   --> $DIR/hygienic-labels-in-let.rs:38:9
2019-11-12T12:34:31.2640704Z +    |
2019-11-12T12:34:31.2640704Z +    |
2019-11-12T12:34:31.2640875Z + LL |         'x: for _ in 0..1 { $e }
2019-11-12T12:34:31.2640960Z + ...
2019-11-12T12:34:31.2640960Z + ...
2019-11-12T12:34:31.2641126Z + LL |             run_once!(continue 'x);
2019-11-12T12:34:31.2641536Z + 
2019-11-12T12:34:31.2641536Z + 
2019-11-12T12:34:31.2641722Z 1 warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2641904Z 2   --> $DIR/hygienic-labels-in-let.rs:15:9
2019-11-12T12:34:31.2641960Z 
2019-11-12T12:34:31.2641980Z 
2019-11-12T12:34:31.2642015Z The actual stderr differed from the expected stderr.
2019-11-12T12:34:31.2642287Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-labels-in-let/hygienic-labels-in-let.stderr
2019-11-12T12:34:31.2642287Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-labels-in-let/hygienic-labels-in-let.stderr
2019-11-12T12:34:31.2642478Z To update references, rerun the tests and pass the `--bless` flag
2019-11-12T12:34:31.2642800Z To only update this specific test, also pass `--test-args hygiene/hygienic-labels-in-let.rs`
2019-11-12T12:34:31.2642876Z error: 1 errors occurred comparing output.
2019-11-12T12:34:31.2642909Z status: exit code: 0
2019-11-12T12:34:31.2642909Z status: exit code: 0
2019-11-12T12:34:31.2643543Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/hygienic-labels-in-let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-labels-in-let/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-labels-in-let/auxiliary"
2019-11-12T12:34:31.2644071Z ------------------------------------------
2019-11-12T12:34:31.2644098Z 
2019-11-12T12:34:31.2644258Z ------------------------------------------
2019-11-12T12:34:31.2644316Z stderr:
2019-11-12T12:34:31.2644316Z stderr:
2019-11-12T12:34:31.2644477Z ------------------------------------------
2019-11-12T12:34:31.2644513Z warning: unused label
2019-11-12T12:34:31.2644707Z   --> /checkout/src/test/ui/hygiene/hygienic-labels-in-let.rs:15:9
2019-11-12T12:34:31.2644745Z    |
2019-11-12T12:34:31.2644897Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2644978Z ...
2019-11-12T12:34:31.2644978Z ...
2019-11-12T12:34:31.2645131Z LL |             loop_x!(break 'x);
2019-11-12T12:34:31.2645355Z    |
2019-11-12T12:34:31.2645388Z    = note: `#[warn(unused_labels)]` on by default
2019-11-12T12:34:31.2645418Z 
2019-11-12T12:34:31.2645449Z warning: unused label
2019-11-12T12:34:31.2645449Z warning: unused label
2019-11-12T12:34:31.2645647Z   --> /checkout/src/test/ui/hygiene/hygienic-labels-in-let.rs:15:9
2019-11-12T12:34:31.2645683Z    |
2019-11-12T12:34:31.2646008Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2646091Z ...
2019-11-12T12:34:31.2646091Z ...
2019-11-12T12:34:31.2646666Z LL |             loop_x!(break 'x);
2019-11-12T12:34:31.2646966Z 
2019-11-12T12:34:31.2647009Z warning: unused label
2019-11-12T12:34:31.2647248Z   --> /checkout/src/test/ui/hygiene/hygienic-labels-in-let.rs:26:9
2019-11-12T12:34:31.2647312Z    |
2019-11-12T12:34:31.2647312Z    |
2019-11-12T12:34:31.2647519Z LL |         'x: while 1 + 1 == 2 { $e }
2019-11-12T12:34:31.2647623Z ...
2019-11-12T12:34:31.2647623Z ...
2019-11-12T12:34:31.2647829Z LL |             while_true!(break 'x);
2019-11-12T12:34:31.2648107Z 
2019-11-12T12:34:31.2648166Z warning: unused label
2019-11-12T12:34:31.2648407Z   --> /checkout/src/test/ui/hygiene/hygienic-labels-in-let.rs:38:9
2019-11-12T12:34:31.2648454Z    |
2019-11-12T12:34:31.2648454Z    |
2019-11-12T12:34:31.2648672Z LL |         'x: for _ in 0..1 { $e }
2019-11-12T12:34:31.2648759Z ...
2019-11-12T12:34:31.2648759Z ...
2019-11-12T12:34:31.2648976Z LL |             run_once!(continue 'x);
2019-11-12T12:34:31.2649265Z 
2019-11-12T12:34:31.2649265Z 
2019-11-12T12:34:31.2649501Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2649808Z    |
2019-11-12T12:34:31.2649808Z    |
2019-11-12T12:34:31.2650139Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2650319Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2650354Z ...
2019-11-12T12:34:31.2650500Z LL |         'x: loop {
2019-11-12T12:34:31.2650656Z    |         -- first declared here
2019-11-12T12:34:31.2650864Z LL |             // this 'x should refer to the outer loop, lexically
2019-11-12T12:34:31.2651194Z LL |             loop_x!(break 'x);
2019-11-12T12:34:31.2651406Z 
2019-11-12T12:34:31.2651406Z 
2019-11-12T12:34:31.2651676Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2651964Z    |
2019-11-12T12:34:31.2651964Z    |
2019-11-12T12:34:31.2652135Z LL |         'x: loop {
2019-11-12T12:34:31.2652302Z    |         -- first declared here
2019-11-12T12:34:31.2652338Z ...
2019-11-12T12:34:31.2652518Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2652695Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2652721Z 
2019-11-12T12:34:31.2652914Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2653247Z    |
2019-11-12T12:34:31.2653247Z    |
2019-11-12T12:34:31.2653436Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2653617Z    |         -- first declared here
2019-11-12T12:34:31.2653653Z ...
2019-11-12T12:34:31.2653812Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2654011Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2654039Z 
2019-11-12T12:34:31.2654233Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2654481Z    |
2019-11-12T12:34:31.2654481Z    |
2019-11-12T12:34:31.2654638Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2654811Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2654862Z ...
2019-11-12T12:34:31.2655017Z LL |         'x: loop {
2019-11-12T12:34:31.2655180Z    |         -- first declared here
2019-11-12T12:34:31.2655230Z ...
2019-11-12T12:34:31.2655391Z LL |             loop_x!(break 'x);
2019-11-12T12:34:31.2655611Z 
2019-11-12T12:34:31.2655611Z 
2019-11-12T12:34:31.2655818Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2656224Z    |
2019-11-12T12:34:31.2656224Z    |
2019-11-12T12:34:31.2656818Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2656912Z    |         |
2019-11-12T12:34:31.2656956Z    |         first declared here
2019-11-12T12:34:31.2656956Z    |         first declared here
2019-11-12T12:34:31.2657192Z    |         lifetime 'x already in scope
2019-11-12T12:34:31.2657238Z ...
2019-11-12T12:34:31.2657439Z LL |             loop_x!(break 'x);
2019-11-12T12:34:31.2657720Z 
2019-11-12T12:34:31.2657720Z 
2019-11-12T12:34:31.2657960Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2658295Z    |
2019-11-12T12:34:31.2658295Z    |
2019-11-12T12:34:31.2658506Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2658737Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2658802Z ...
2019-11-12T12:34:31.2659020Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2659238Z    |         -- first declared here
2019-11-12T12:34:31.2659301Z ...
2019-11-12T12:34:31.2659528Z LL |             loop_x!(break 'x);
2019-11-12T12:34:31.2659813Z 
2019-11-12T12:34:31.2659813Z 
2019-11-12T12:34:31.2660210Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2660446Z    |
2019-11-12T12:34:31.2660446Z    |
2019-11-12T12:34:31.2660615Z LL |         'x: loop {
2019-11-12T12:34:31.2660784Z    |         -- first declared here
2019-11-12T12:34:31.2660820Z ...
2019-11-12T12:34:31.2660982Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2661173Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2661208Z 
2019-11-12T12:34:31.2661403Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2661656Z    |
2019-11-12T12:34:31.2661656Z    |
2019-11-12T12:34:31.2661870Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2662164Z    |         -- first declared here
2019-11-12T12:34:31.2662213Z ...
2019-11-12T12:34:31.2662572Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2662743Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2662785Z 
2019-11-12T12:34:31.2663173Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2663463Z    |
2019-11-12T12:34:31.2663463Z    |
2019-11-12T12:34:31.2663627Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2663789Z    |         -- first declared here
2019-11-12T12:34:31.2663824Z ...
2019-11-12T12:34:31.2664149Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2664322Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2664349Z 
2019-11-12T12:34:31.2664552Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2664791Z    |
2019-11-12T12:34:31.2664791Z    |
2019-11-12T12:34:31.2664947Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2665125Z    |         -- first declared here
2019-11-12T12:34:31.2665160Z ...
2019-11-12T12:34:31.2665317Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2665507Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2665533Z 
2019-11-12T12:34:31.2665721Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2666142Z    |
2019-11-12T12:34:31.2666142Z    |
2019-11-12T12:34:31.2667061Z LL |         'x: while 1 + 1 == 2 { $e }
2019-11-12T12:34:31.2667299Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2667363Z ...
2019-11-12T12:34:31.2667553Z LL |         'x: loop {
2019-11-12T12:34:31.2667754Z    |         -- first declared here
2019-11-12T12:34:31.2667816Z ...
2019-11-12T12:34:31.2668023Z LL |             while_true!(break 'x);
2019-11-12T12:34:31.2668299Z 
2019-11-12T12:34:31.2668299Z 
2019-11-12T12:34:31.2668554Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2668891Z    |
2019-11-12T12:34:31.2668891Z    |
2019-11-12T12:34:31.2669103Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2669305Z    |         -- first declared here
2019-11-12T12:34:31.2669349Z ...
2019-11-12T12:34:31.2669569Z LL |         'x: while 1 + 1 == 2 { $e }
2019-11-12T12:34:31.2669786Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2669832Z ...
2019-11-12T12:34:31.2670192Z LL |             while_true!(break 'x);
2019-11-12T12:34:31.2670426Z 
2019-11-12T12:34:31.2670426Z 
2019-11-12T12:34:31.2670607Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2670850Z    |
2019-11-12T12:34:31.2670850Z    |
2019-11-12T12:34:31.2671179Z LL |         'x: while 1 + 1 == 2 { $e }
2019-11-12T12:34:31.2671349Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2671383Z ...
2019-11-12T12:34:31.2671530Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2671678Z    |         -- first declared here
2019-11-12T12:34:31.2671725Z ...
2019-11-12T12:34:31.2671877Z LL |             while_true!(break 'x);
2019-11-12T12:34:31.2672087Z 
2019-11-12T12:34:31.2672087Z 
2019-11-12T12:34:31.2672263Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2672481Z    |
2019-11-12T12:34:31.2672481Z    |
2019-11-12T12:34:31.2672638Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2672784Z    |         -- first declared here
2019-11-12T12:34:31.2672817Z ...
2019-11-12T12:34:31.2672983Z LL |         'x: while 1 + 1 == 2 { $e }
2019-11-12T12:34:31.2673236Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2673279Z ...
2019-11-12T12:34:31.2673479Z LL |             while_true!(break 'x);
2019-11-12T12:34:31.2673678Z 
2019-11-12T12:34:31.2673678Z 
2019-11-12T12:34:31.2673849Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2674075Z    |
2019-11-12T12:34:31.2674075Z    |
2019-11-12T12:34:31.2674227Z LL |         'x: while 1 + 1 == 2 { $e }
2019-11-12T12:34:31.2674396Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2674507Z ...
2019-11-12T12:34:31.2674681Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2674830Z    |         -- first declared here
2019-11-12T12:34:31.2674878Z ...
2019-11-12T12:34:31.2675029Z LL |             while_true!(break 'x);
2019-11-12T12:34:31.2675245Z 
2019-11-12T12:34:31.2675245Z 
2019-11-12T12:34:31.2675421Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2675662Z    |
2019-11-12T12:34:31.2675662Z    |
2019-11-12T12:34:31.2675813Z LL |         'x: loop {
2019-11-12T12:34:31.2676149Z    |         -- first declared here
2019-11-12T12:34:31.2676340Z ...
2019-11-12T12:34:31.2676798Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2677014Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2677047Z 
2019-11-12T12:34:31.2677300Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2677602Z    |
2019-11-12T12:34:31.2677602Z    |
2019-11-12T12:34:31.2677795Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2678013Z    |         -- first declared here
2019-11-12T12:34:31.2678057Z ...
2019-11-12T12:34:31.2680845Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2682631Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2682671Z 
2019-11-12T12:34:31.2684196Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2687524Z    |
2019-11-12T12:34:31.2687524Z    |
2019-11-12T12:34:31.2688824Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2690346Z    |         -- first declared here
2019-11-12T12:34:31.2690426Z ...
2019-11-12T12:34:31.2691204Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2691600Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2691629Z 
2019-11-12T12:34:31.2691859Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2692103Z    |
2019-11-12T12:34:31.2692103Z    |
2019-11-12T12:34:31.2692284Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2692454Z    |         -- first declared here
2019-11-12T12:34:31.2692497Z ...
2019-11-12T12:34:31.2692679Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2692864Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2692892Z 
2019-11-12T12:34:31.2693087Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2693343Z    |
2019-11-12T12:34:31.2693343Z    |
2019-11-12T12:34:31.2693508Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2693691Z    |         -- first declared here
2019-11-12T12:34:31.2693728Z ...
2019-11-12T12:34:31.2693895Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2694092Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2694127Z 
2019-11-12T12:34:31.2694326Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2694580Z    |
2019-11-12T12:34:31.2694580Z    |
2019-11-12T12:34:31.2694882Z LL |         'x: while 1 + 1 == 2 { $e }
2019-11-12T12:34:31.2695108Z    |         -- first declared here
2019-11-12T12:34:31.2695162Z ...
2019-11-12T12:34:31.2695330Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2695508Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2695536Z 
2019-11-12T12:34:31.2695747Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2696169Z    |
2019-11-12T12:34:31.2696169Z    |
2019-11-12T12:34:31.2696520Z LL |         'x: for _ in 0..1 { $e }
2019-11-12T12:34:31.2697115Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2697293Z ...
2019-11-12T12:34:31.2697612Z LL |         'x: loop {
2019-11-12T12:34:31.2697837Z    |         -- first declared here
2019-11-12T12:34:31.2697883Z ...
2019-11-12T12:34:31.2698088Z LL |             run_once!(continue 'x);
2019-11-12T12:34:31.2698389Z 
2019-11-12T12:34:31.2698389Z 
2019-11-12T12:34:31.2698629Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2698937Z    |
2019-11-12T12:34:31.2698937Z    |
2019-11-12T12:34:31.2699134Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2699338Z    |         -- first declared here
2019-11-12T12:34:31.2699399Z ...
2019-11-12T12:34:31.2699606Z LL |         'x: for _ in 0..1 { $e }
2019-11-12T12:34:31.2699820Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2699881Z ...
2019-11-12T12:34:31.2700088Z LL |             run_once!(continue 'x);
2019-11-12T12:34:31.2700516Z 
2019-11-12T12:34:31.2700516Z 
2019-11-12T12:34:31.2700891Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2701113Z    |
2019-11-12T12:34:31.2701113Z    |
2019-11-12T12:34:31.2701293Z LL |         'x: for _ in 0..1 { $e }
2019-11-12T12:34:31.2701798Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2701849Z ...
2019-11-12T12:34:31.2702110Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2702319Z    |         -- first declared here
2019-11-12T12:34:31.2702364Z ...
2019-11-12T12:34:31.2702569Z LL |             run_once!(continue 'x);
2019-11-12T12:34:31.2702855Z 
2019-11-12T12:34:31.2702855Z 
2019-11-12T12:34:31.2703347Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2703684Z    |
2019-11-12T12:34:31.2703684Z    |
2019-11-12T12:34:31.2703883Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2704089Z    |         -- first declared here
2019-11-12T12:34:31.2704151Z ...
2019-11-12T12:34:31.2704354Z LL |         'x: for _ in 0..1 { $e }
2019-11-12T12:34:31.2704576Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2704638Z ...
2019-11-12T12:34:31.2704847Z LL |             run_once!(continue 'x);
2019-11-12T12:34:31.2705114Z 
2019-11-12T12:34:31.2705114Z 
2019-11-12T12:34:31.2705486Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2705695Z    |
2019-11-12T12:34:31.2705695Z    |
2019-11-12T12:34:31.2706237Z LL |         'x: for _ in 0..1 { $e }
2019-11-12T12:34:31.2706439Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2706474Z ...
2019-11-12T12:34:31.2707080Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2707312Z    |         -- first declared here
2019-11-12T12:34:31.2707357Z ...
2019-11-12T12:34:31.2707579Z LL |             run_once!(continue 'x);
2019-11-12T12:34:31.2707851Z 
2019-11-12T12:34:31.2707851Z 
2019-11-12T12:34:31.2709163Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2709646Z    |
2019-11-12T12:34:31.2709646Z    |
2019-11-12T12:34:31.2709861Z LL |         'x: while 1 + 1 == 2 { $e }
2019-11-12T12:34:31.2711682Z    |         -- first declared here
2019-11-12T12:34:31.2711729Z ...
2019-11-12T12:34:31.2711891Z LL |         'x: for _ in 0..1 { $e }
2019-11-12T12:34:31.2712586Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2712659Z ...
2019-11-12T12:34:31.2713695Z LL |             run_once!(continue 'x);
2019-11-12T12:34:31.2714337Z 
2019-11-12T12:34:31.2714337Z 
2019-11-12T12:34:31.2714576Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2714809Z    |
2019-11-12T12:34:31.2714809Z    |
2019-11-12T12:34:31.2714969Z LL |         'x: for _ in 0..1 { $e }
2019-11-12T12:34:31.2715127Z    |         ^^ lifetime 'x already in scope
2019-11-12T12:34:31.2715161Z ...
2019-11-12T12:34:31.2715322Z LL |         'x: for _ in 0..1 {
2019-11-12T12:34:31.2715470Z    |         -- first declared here
2019-11-12T12:34:31.2715502Z ...
2019-11-12T12:34:31.2715666Z LL |             run_once!(continue 'x);
2019-11-12T12:34:31.2716042Z 
2019-11-12T12:34:31.2716063Z 
2019-11-12T12:34:31.2716239Z ------------------------------------------
2019-11-12T12:34:31.2716263Z 
2019-11-12T12:34:31.2716263Z 
2019-11-12T12:34:31.2716283Z 
2019-11-12T12:34:31.2716455Z ---- [ui] ui/hygiene/hygienic-labels.rs stdout ----
2019-11-12T12:34:31.2717065Z diff of stderr:
2019-11-12T12:34:31.2717099Z 
2019-11-12T12:34:31.2717142Z + warning: unused label
2019-11-12T12:34:31.2717402Z +   --> $DIR/hygienic-labels.rs:12:9
2019-11-12T12:34:31.2717465Z +    |
2019-11-12T12:34:31.2717664Z + LL |         'x: loop { $e }
2019-11-12T12:34:31.2717781Z + ...
2019-11-12T12:34:31.2717781Z + ...
2019-11-12T12:34:31.2717985Z + LL |         loop_x!(break 'x);
2019-11-12T12:34:31.2718263Z +    |
2019-11-12T12:34:31.2718326Z +    = note: `#[warn(unused_labels)]` on by default
2019-11-12T12:34:31.2718369Z + 
2019-11-12T12:34:31.2718412Z + warning: unused label
2019-11-12T12:34:31.2718412Z + warning: unused label
2019-11-12T12:34:31.2725404Z +   --> $DIR/hygienic-labels.rs:12:9
2019-11-12T12:34:31.2725691Z +    |
2019-11-12T12:34:31.2726861Z + LL |         'x: loop { $e }
2019-11-12T12:34:31.2727005Z + ...
2019-11-12T12:34:31.2727005Z + ...
2019-11-12T12:34:31.2727258Z + LL |         loop_x!(break 'x);
2019-11-12T12:34:31.2727565Z + 
2019-11-12T12:34:31.2727609Z + warning: unused label
2019-11-12T12:34:31.2727823Z +   --> $DIR/hygienic-labels.rs:37:9
2019-11-12T12:34:31.2727886Z +    |
2019-11-12T12:34:31.2727886Z +    |
2019-11-12T12:34:31.2728113Z + LL |         'x: while 1 + 1 == 2 { $e }
2019-11-12T12:34:31.2728204Z + ...
2019-11-12T12:34:31.2728204Z + ...
2019-11-12T12:34:31.2728426Z + LL |         while_x!(break 'x);
2019-11-12T12:34:31.2728703Z + 
2019-11-12T12:34:31.2728762Z + warning: unused label
2019-11-12T12:34:31.2728968Z +   --> $DIR/hygienic-labels.rs:23:9
2019-11-12T12:34:31.2729014Z +    |
2019-11-12T12:34:31.2729014Z +    |
2019-11-12T12:34:31.2729238Z + LL |         'x: for _ in 0..1 { $e }
2019-11-12T12:34:31.2729327Z + ...
2019-11-12T12:34:31.2729327Z + ...
2019-11-12T12:34:31.2729532Z + LL |         run_once!(continue 'x);
2019-11-12T12:34:31.2729841Z + 
2019-11-12T12:34:31.2729841Z + 
2019-11-12T12:34:31.2730282Z 1 warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2730802Z 2   --> $DIR/hygienic-labels.rs:12:9
2019-11-12T12:34:31.2731000Z 
2019-11-12T12:34:31.2731033Z 
2019-11-12T12:34:31.2731086Z The actual stderr differed from the expected stderr.
2019-11-12T12:34:31.2731361Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-labels/hygienic-labels.stderr
2019-11-12T12:34:31.2731361Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-labels/hygienic-labels.stderr
2019-11-12T12:34:31.2731555Z To update references, rerun the tests and pass the `--bless` flag
2019-11-12T12:34:31.2731782Z To only update this specific test, also pass `--test-args hygiene/hygienic-labels.rs`
2019-11-12T12:34:31.2731847Z error: 1 errors occurred comparing output.
2019-11-12T12:34:31.2731897Z status: exit code: 0
2019-11-12T12:34:31.2731897Z status: exit code: 0
2019-11-12T12:34:31.2732597Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/hygienic-labels.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-labels/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-labels/auxiliary"
2019-11-12T12:34:31.2732887Z ------------------------------------------
2019-11-12T12:34:31.2732916Z 
2019-11-12T12:34:31.2733099Z ------------------------------------------
2019-11-12T12:34:31.2733155Z stderr:
2019-11-12T12:34:31.2733155Z stderr:
2019-11-12T12:34:31.2733327Z ------------------------------------------
2019-11-12T12:34:31.2733368Z warning: unused label
2019-11-12T12:34:31.2733575Z   --> /checkout/src/test/ui/hygiene/hygienic-labels.rs:12:9
2019-11-12T12:34:31.2733624Z    |
2019-11-12T12:34:31.2733793Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2733880Z ...
2019-11-12T12:34:31.2733880Z ...
2019-11-12T12:34:31.2734045Z LL |         loop_x!(break 'x);
2019-11-12T12:34:31.2734287Z    |
2019-11-12T12:34:31.2734368Z    = note: `#[warn(unused_labels)]` on by default
2019-11-12T12:34:31.2734395Z 
2019-11-12T12:34:31.2734446Z warning: unused label
2019-11-12T12:34:31.2734446Z warning: unused label
2019-11-12T12:34:31.2734655Z   --> /checkout/src/test/ui/hygiene/hygienic-labels.rs:12:9
2019-11-12T12:34:31.2734694Z    |
2019-11-12T12:34:31.2734853Z LL |         'x: loop { $e }
2019-11-12T12:34:31.2734939Z ...
2019-11-12T12:34:31.2734939Z ...
2019-11-12T12:34:31.2735104Z LL |         loop_x!(break 'x);
2019-11-12T12:34:31.2735334Z 
2019-11-12T12:34:31.2735370Z warning: unused label
2019-11-12T12:34:31.2735561Z   --> /checkout/src/test/ui/hygiene/hygienic-labels.rs:37:9
2019-11-12T12:34:31.2735620Z    |
2019-11-12T12:34:31.2735620Z    |
2019-11-12T12:34:31.2735795Z LL |         'x: while 1 + 1 == 2 { $e }
2019-11-12T12:34:31.2736058Z ...
2019-11-12T12:34:31.2736058Z ...
2019-11-12T12:34:31.2736222Z LL |         while_x!(break 'x);
2019-11-12T12:34:31.2737027Z 
2019-11-12T12:34:31.2737096Z warning: unused label
2019-11-12T12:34:31.2737392Z   --> /checkout/src/test/ui/hygiene/hygienic-labels.rs:23:9
2019-11-12T12:34:31.2737443Z    |
2019-11-12T12:34:31.2737443Z    |
2019-11-12T12:34:31.2737668Z LL |         'x: for _ in 0..1 { $e }
2019-11-12T12:34:31.2737757Z ...
2019-11-12T12:34:31.2737757Z ...
2019-11-12T12:34:31.2737977Z LL |         run_once!(continue 'x);
2019-11-12T12:34:31.2738246Z 
2019-11-12T12:34:31.2738246Z 
2019-11-12T12:34:31.2738484Z warning: label name `'x` shadows a label name that is already in scope
2019-11-12T12:34:31.2738793Z    |
---
2019-11-12T12:34:31.2803700Z 
2019-11-12T12:34:31.2803730Z + warning: unused label
2019-11-12T12:34:31.2803900Z +   --> $DIR/macro-lifetime-used-with-labels.rs:21:9
2019-11-12T12:34:31.2803952Z +    |
2019-11-12T12:34:31.2804093Z + LL |         'b: loop {
2019-11-12T12:34:31.2804158Z + ...
2019-11-12T12:34:31.2804158Z + ...
2019-11-12T12:34:31.2804313Z + LL |         br2!('b);
2019-11-12T12:34:31.2804579Z +    |
2019-11-12T12:34:31.2804630Z +    = note: `#[warn(unused_labels)]` on by default
2019-11-12T12:34:31.2804662Z + 
2019-11-12T12:34:31.2804662Z + 
2019-11-12T12:34:31.2804866Z 1 warning: label name `'b` shadows a label name that is already in scope
2019-11-12T12:34:31.2805086Z 3    |
2019-11-12T12:34:31.2805112Z 
2019-11-12T12:34:31.2805131Z 
2019-11-12T12:34:31.2805180Z The actual stderr differed from the expected stderr.
2019-11-12T12:34:31.2805180Z The actual stderr differed from the expected stderr.
2019-11-12T12:34:31.2805424Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-lifetime-used-with-labels/macro-lifetime-used-with-labels.stderr
2019-11-12T12:34:31.2805604Z To update references, rerun the tests and pass the `--bless` flag
2019-11-12T12:34:31.2805823Z To only update this specific test, also pass `--test-args macros/macro-lifetime-used-with-labels.rs`
2019-11-12T12:34:31.2805883Z error: 1 errors occurred comparing output.
2019-11-12T12:34:31.2805915Z status: exit code: 0
2019-11-12T12:34:31.2805915Z status: exit code: 0
2019-11-12T12:34:31.2806919Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-lifetime-used-with-labels.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-lifetime-used-with-labels/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-lifetime-used-with-labels/auxiliary"
2019-11-12T12:34:31.2807291Z ------------------------------------------
2019-11-12T12:34:31.2807326Z 
2019-11-12T12:34:31.2807542Z ------------------------------------------
2019-11-12T12:34:31.2807607Z stderr:
2019-11-12T12:34:31.2807607Z stderr:
2019-11-12T12:34:31.2807817Z ------------------------------------------
2019-11-12T12:34:31.2807865Z warning: unused label
2019-11-12T12:34:31.2808136Z   --> /checkout/src/test/ui/macros/macro-lifetime-used-with-labels.rs:21:9
2019-11-12T12:34:31.2808187Z    |
2019-11-12T12:34:31.2808443Z LL |         'b: loop { //~ WARNING `'b` shadows a label name that is already in scope
2019-11-12T12:34:31.2808555Z ...
2019-11-12T12:34:31.2808555Z ...
2019-11-12T12:34:31.2808751Z LL |         br2!('b);
2019-11-12T12:34:31.2809034Z    |
2019-11-12T12:34:31.2809080Z    = note: `#[warn(unused_labels)]` on by default
2019-11-12T12:34:31.2809111Z 
2019-11-12T12:34:31.2809111Z 
2019-11-12T12:34:31.2809369Z warning: label name `'b` shadows a label name that is already in scope
2019-11-12T12:34:31.2809669Z    |
2019-11-12T12:34:31.2809669Z    |
2019-11-12T12:34:31.2809935Z LL |         'b: loop { //~ WARNING `'b` shadows a label name that is already in scope
2019-11-12T12:34:31.2810310Z    |         ^^ lifetime 'b already in scope
2019-11-12T12:34:31.2810483Z LL |     'b: loop {
2019-11-12T12:34:31.2810647Z    |     -- first declared here
2019-11-12T12:34:31.2810647Z    |     -- first declared here
2019-11-12T12:34:31.2810788Z LL |         br2!('b);
2019-11-12T12:34:31.2810984Z 
2019-11-12T12:34:31.2811003Z 
2019-11-12T12:34:31.2811245Z ------------------------------------------
2019-11-12T12:34:31.2811277Z 
---
2019-11-12T12:34:31.2815690Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-12T12:34:31.2820560Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-12T12:34:31.2820815Z 
2019-11-12T12:34:31.2820952Z 
2019-11-12T12:34:31.2822604Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-12T12:34:31.2823527Z 
2019-11-12T12:34:31.2832382Z 
2019-11-12T12:34:31.2832527Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-12T12:34:31.2832598Z Build completed unsuccessfully in 1:03:29
2019-11-12T12:34:31.2832598Z Build completed unsuccessfully in 1:03:29
2019-11-12T12:34:31.2832773Z == clock drift check ==
2019-11-12T12:34:31.2832898Z   local time: Tue Nov 12 12:34:31 UTC 2019
2019-11-12T12:34:31.5588031Z   network time: Tue, 12 Nov 2019 12:34:31 GMT
2019-11-12T12:34:32.1382034Z == end clock drift check ==
2019-11-12T12:34:32.4183456Z 
2019-11-12T12:34:32.4255197Z ##[error]Bash exited with code '1'.
2019-11-12T12:34:32.4338298Z ##[section]Starting: Checkout
2019-11-12T12:34:32.4340321Z ==============================================================================
2019-11-12T12:34:32.4340362Z Task         : Get sources
2019-11-12T12:34:32.4340418Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
