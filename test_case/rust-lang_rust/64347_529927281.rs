plain
2019-09-10T11:51:05.1396202Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-10T11:51:05.1582884Z ##[command]git config gc.auto 0
2019-09-10T11:51:05.1671845Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-10T11:51:05.1736116Z ##[command]git config --get-all http.proxy
2019-09-10T11:51:05.1899428Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64347/merge:refs/remotes/pull/64347/merge
---
2019-09-10T12:58:59.8892065Z .................................................................................................... 1500/9006
2019-09-10T12:59:06.2258450Z .................................................................................................... 1600/9006
2019-09-10T12:59:20.0222516Z .......................................................i...............i............................ 1700/9006
2019-09-10T12:59:28.5727048Z .................................................................................................... 1800/9006
2019-09-10T12:59:44.8716575Z ..............................................iiiii................................................. 1900/9006
2019-09-10T12:59:57.1899377Z .................................................................................................... 2100/9006
2019-09-10T13:00:00.0285349Z .................................................................................................... 2200/9006
2019-09-10T13:00:04.3226354Z .................................................................................................... 2300/9006
2019-09-10T13:00:13.0848271Z .................................................................................................... 2400/9006
---
2019-09-10T13:03:28.2647936Z .................................i...............i.................................................. 4700/9006
2019-09-10T13:03:40.7144850Z .................................................F.................................................. 4800/9006
2019-09-10T13:03:47.3656717Z .................................................................................................... 4900/9006
2019-09-10T13:03:58.7998556Z .................................................................................................... 5000/9006
2019-09-10T13:04:05.3462896Z ................ii.ii............................................................................... 5100/9006
2019-09-10T13:04:16.8391463Z .................................................................................................... 5300/9006
2019-09-10T13:04:27.7284323Z ........FF.....................................................................i.................... 5400/9006
2019-09-10T13:04:36.0371724Z .................................................................................................... 5500/9006
2019-09-10T13:04:42.3661024Z .................................................................................................... 5600/9006
2019-09-10T13:04:42.3661024Z .................................................................................................... 5600/9006
2019-09-10T13:04:53.6602344Z .........................................................................ii...i..ii...........i..... 5700/9006
2019-09-10T13:05:20.4710046Z .....................................................................................F.............. 5900/9006
2019-09-10T13:05:30.7458233Z .F.................................................................................................. 6000/9006
2019-09-10T13:05:30.7458233Z .F.................................................................................................. 6000/9006
2019-09-10T13:05:41.6986197Z ..FFF......................................................................i..ii.................... 6100/9006
2019-09-10T13:06:14.5751411Z .................................................................................................... 6300/9006
2019-09-10T13:06:16.8942492Z ..................................i................................................................. 6400/9006
2019-09-10T13:06:19.2627715Z .................................................................................................... 6500/9006
2019-09-10T13:06:22.1411196Z ......i............................................................................................. 6600/9006
---
2019-09-10T13:10:47.0525560Z 
2019-09-10T13:10:47.0525750Z 
2019-09-10T13:10:47.0525986Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0526541Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10291/issue-10291.stderr
2019-09-10T13:10:47.0527151Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0528384Z To only update this specific test, also pass `--test-args issues/issue-10291.rs`
2019-09-10T13:10:47.0529251Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0529430Z status: exit code: 1
2019-09-10T13:10:47.0529430Z status: exit code: 1
2019-09-10T13:10:47.0530408Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-10291.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10291" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10291/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0531078Z ------------------------------------------
2019-09-10T13:10:47.0531280Z 
2019-09-10T13:10:47.0531651Z ------------------------------------------
2019-09-10T13:10:47.0531826Z stderr:
2019-09-10T13:10:47.0531826Z stderr:
2019-09-10T13:10:47.0532225Z ------------------------------------------
2019-09-10T13:10:47.0532415Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0532817Z   --> /checkout/src/test/ui/issues/issue-10291.rs:3:9
2019-09-10T13:10:47.0533136Z LL |         x //~ ERROR E0312
2019-09-10T13:10:47.0533283Z    |         ^
2019-09-10T13:10:47.0533416Z    |
2019-09-10T13:10:47.0533416Z    |
2019-09-10T13:10:47.0533558Z note: ...the reference is valid for the anonymous lifetime #2 defined on the body at 2:69...
2019-09-10T13:10:47.0533960Z   --> /checkout/src/test/ui/issues/issue-10291.rs:2:69
2019-09-10T13:10:47.0534147Z    |
2019-09-10T13:10:47.0536035Z LL |       drop::<Box<dyn for<'z> FnMut(&'z isize) -> &'z isize>>(Box::new(|z| {
2019-09-10T13:10:47.0536521Z LL | |         x //~ ERROR E0312
2019-09-10T13:10:47.0536677Z LL | |     }));
2019-09-10T13:10:47.0536824Z    | |_____^
2019-09-10T13:10:47.0536824Z    | |_____^
2019-09-10T13:10:47.0537323Z note: ...but the borrowed content is only valid for the lifetime 'x as defined on the function body at 1:9
2019-09-10T13:10:47.0538186Z   --> /checkout/src/test/ui/issues/issue-10291.rs:1:9
2019-09-10T13:10:47.0538453Z    |
2019-09-10T13:10:47.0538813Z LL | fn test<'x>(x: &'x isize) {
2019-09-10T13:10:47.0539133Z 
2019-09-10T13:10:47.0539266Z error: aborting due to previous error
2019-09-10T13:10:47.0539399Z 
2019-09-10T13:10:47.0539786Z For more information about this error, try `rustc --explain E0312`.
---
2019-09-10T13:10:47.0544305Z 
2019-09-10T13:10:47.0544422Z 
2019-09-10T13:10:47.0545772Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0546667Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52533/issue-52533.stderr
2019-09-10T13:10:47.0548276Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0548903Z To only update this specific test, also pass `--test-args issues/issue-52533.rs`
2019-09-10T13:10:47.0549985Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0550155Z status: exit code: 1
2019-09-10T13:10:47.0550155Z status: exit code: 1
2019-09-10T13:10:47.0551321Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52533.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52533" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52533/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0552103Z ------------------------------------------
2019-09-10T13:10:47.0552279Z 
2019-09-10T13:10:47.0552667Z ------------------------------------------
2019-09-10T13:10:47.0552877Z stderr:
2019-09-10T13:10:47.0552877Z stderr:
2019-09-10T13:10:47.0553250Z ------------------------------------------
2019-09-10T13:10:47.0554114Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0556565Z   --> /checkout/src/test/ui/issues/issue-52533.rs:5:16
2019-09-10T13:10:47.0557043Z LL |     foo(|a, b| b)
2019-09-10T13:10:47.0557181Z    |                ^
2019-09-10T13:10:47.0557313Z    |
2019-09-10T13:10:47.0557313Z    |
2019-09-10T13:10:47.0557490Z note: ...the reference is valid for the anonymous lifetime #2 defined on the body at 5:9...
2019-09-10T13:10:47.0558323Z   --> /checkout/src/test/ui/issues/issue-52533.rs:5:9
2019-09-10T13:10:47.0558667Z LL |     foo(|a, b| b)
2019-09-10T13:10:47.0558825Z    |         ^^^^^^^^
2019-09-10T13:10:47.0558825Z    |         ^^^^^^^^
2019-09-10T13:10:47.0558971Z note: ...but the borrowed content is only valid for the anonymous lifetime #3 defined on the body at 5:9
2019-09-10T13:10:47.0559379Z   --> /checkout/src/test/ui/issues/issue-52533.rs:5:9
2019-09-10T13:10:47.0559720Z LL |     foo(|a, b| b)
2019-09-10T13:10:47.0559874Z    |         ^^^^^^^^
2019-09-10T13:10:47.0559992Z 
2019-09-10T13:10:47.0560127Z error: aborting due to previous error
---
2019-09-10T13:10:47.0563499Z 29 
2019-09-10T13:10:47.0563616Z 
2019-09-10T13:10:47.0563729Z 
2019-09-10T13:10:47.0563881Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0564543Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-if/lub-if.stderr
2019-09-10T13:10:47.0565005Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0565463Z To only update this specific test, also pass `--test-args lub-if.rs`
2019-09-10T13:10:47.0565830Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0565970Z status: exit code: 1
2019-09-10T13:10:47.0565970Z status: exit code: 1
2019-09-10T13:10:47.0566786Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lub-if.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-if" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-if/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0567414Z ------------------------------------------
2019-09-10T13:10:47.0567942Z 
2019-09-10T13:10:47.0568426Z ------------------------------------------
2019-09-10T13:10:47.0568905Z stderr:
2019-09-10T13:10:47.0568905Z stderr:
2019-09-10T13:10:47.0569753Z ------------------------------------------
2019-09-10T13:10:47.0570146Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0574242Z    |
2019-09-10T13:10:47.0574563Z LL |         s  //~ ERROR E0312
2019-09-10T13:10:47.0574741Z    |         ^
2019-09-10T13:10:47.0574886Z    |
2019-09-10T13:10:47.0574886Z    |
2019-09-10T13:10:47.0575029Z    = note: ...the reference is valid for the static lifetime...
2019-09-10T13:10:47.0575579Z note: ...but the borrowed content is only valid for the lifetime 'a as defined on the function body at 23:17
2019-09-10T13:10:47.0576295Z    |
2019-09-10T13:10:47.0576295Z    |
2019-09-10T13:10:47.0576706Z LL | pub fn opt_str2<'a>(maybestr: &'a Option<String>) -> &'static str {
2019-09-10T13:10:47.0584622Z 
2019-09-10T13:10:47.0584622Z 
2019-09-10T13:10:47.0584946Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0590087Z    |
2019-09-10T13:10:47.0590197Z LL |         s  //~ ERROR E0312
2019-09-10T13:10:47.0590251Z    |         ^
2019-09-10T13:10:47.0590298Z    |
2019-09-10T13:10:47.0590298Z    |
2019-09-10T13:10:47.0590366Z    = note: ...the reference is valid for the static lifetime...
2019-09-10T13:10:47.0590886Z note: ...but the borrowed content is only valid for the lifetime 'a as defined on the function body at 32:17
2019-09-10T13:10:47.0591226Z    |
2019-09-10T13:10:47.0591226Z    |
2019-09-10T13:10:47.0591509Z LL | pub fn opt_str3<'a>(maybestr: &'a Option<String>) -> &'static str {
2019-09-10T13:10:47.0591612Z 
2019-09-10T13:10:47.0591678Z error: aborting due to 2 previous errors
2019-09-10T13:10:47.0591710Z 
2019-09-10T13:10:47.0591985Z For more information about this error, try `rustc --explain E0312`.
---
2019-09-10T13:10:47.0593173Z 29 
2019-09-10T13:10:47.0593202Z 
2019-09-10T13:10:47.0593247Z 
2019-09-10T13:10:47.0593298Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0593610Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-match/lub-match.stderr
2019-09-10T13:10:47.0594186Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0594470Z To only update this specific test, also pass `--test-args lub-match.rs`
2019-09-10T13:10:47.0594556Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0594623Z status: exit code: 1
2019-09-10T13:10:47.0594623Z status: exit code: 1
2019-09-10T13:10:47.0595395Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lub-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-match" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-match/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0595771Z ------------------------------------------
2019-09-10T13:10:47.0595809Z 
2019-09-10T13:10:47.0596073Z ------------------------------------------
2019-09-10T13:10:47.0596125Z stderr:
2019-09-10T13:10:47.0596125Z stderr:
2019-09-10T13:10:47.0596366Z ------------------------------------------
2019-09-10T13:10:47.0596442Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0596877Z    |
2019-09-10T13:10:47.0596954Z LL |             s //~ ERROR E0312
2019-09-10T13:10:47.0597005Z    |             ^
2019-09-10T13:10:47.0597049Z    |
2019-09-10T13:10:47.0597049Z    |
2019-09-10T13:10:47.0597101Z    = note: ...the reference is valid for the static lifetime...
2019-09-10T13:10:47.0597478Z note: ...but the borrowed content is only valid for the lifetime 'a as defined on the function body at 25:17
2019-09-10T13:10:47.0598202Z    |
2019-09-10T13:10:47.0598202Z    |
2019-09-10T13:10:47.0598514Z LL | pub fn opt_str2<'a>(maybestr: &'a Option<String>) -> &'static str {
2019-09-10T13:10:47.0598618Z 
2019-09-10T13:10:47.0598618Z 
2019-09-10T13:10:47.0598687Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0598997Z    |
2019-09-10T13:10:47.0599044Z LL |             s //~ ERROR E0312
2019-09-10T13:10:47.0599118Z    |             ^
2019-09-10T13:10:47.0599163Z    |
2019-09-10T13:10:47.0599163Z    |
2019-09-10T13:10:47.0599213Z    = note: ...the reference is valid for the static lifetime...
2019-09-10T13:10:47.0599552Z note: ...but the borrowed content is only valid for the lifetime 'a as defined on the function body at 35:17
2019-09-10T13:10:47.0599858Z    |
2019-09-10T13:10:47.0599858Z    |
2019-09-10T13:10:47.0600147Z LL | pub fn opt_str3<'a>(maybestr: &'a Option<String>) -> &'static str {
2019-09-10T13:10:47.0600233Z 
2019-09-10T13:10:47.0600289Z error: aborting due to 2 previous errors
2019-09-10T13:10:47.0600336Z 
2019-09-10T13:10:47.0600609Z For more information about this error, try `rustc --explain E0312`.
---
2019-09-10T13:10:47.0601897Z 
2019-09-10T13:10:47.0601925Z 
2019-09-10T13:10:47.0601993Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0602320Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742/issue-52742.stderr
2019-09-10T13:10:47.0602800Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0603111Z To only update this specific test, also pass `--test-args nll/issue-52742.rs`
2019-09-10T13:10:47.0603199Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0603268Z status: exit code: 1
2019-09-10T13:10:47.0603268Z status: exit code: 1
2019-09-10T13:10:47.0604060Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0604425Z ------------------------------------------
2019-09-10T13:10:47.0604472Z 
2019-09-10T13:10:47.0604719Z ------------------------------------------
2019-09-10T13:10:47.0604788Z stderr:
2019-09-10T13:10:47.0604788Z stderr:
2019-09-10T13:10:47.0605028Z ------------------------------------------
2019-09-10T13:10:47.0605086Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0605368Z   --> /checkout/src/test/ui/nll/issue-52742.rs:14:18
2019-09-10T13:10:47.0605521Z    |
2019-09-10T13:10:47.0605577Z LL |         self.y = b.z
2019-09-10T13:10:47.0605689Z    |
2019-09-10T13:10:47.0605689Z    |
2019-09-10T13:10:47.0606020Z note: ...the reference is valid for the lifetime '_ as defined on the impl at 12:10...
2019-09-10T13:10:47.0606303Z   --> /checkout/src/test/ui/nll/issue-52742.rs:12:10
2019-09-10T13:10:47.0606357Z    |
2019-09-10T13:10:47.0606577Z LL | impl Foo<'_, '_> {
2019-09-10T13:10:47.0606626Z    |          ^^
2019-09-10T13:10:47.0606702Z note: ...but the borrowed content is only valid for the anonymous lifetime #2 defined on the method body at 13:5
2019-09-10T13:10:47.0606986Z   --> /checkout/src/test/ui/nll/issue-52742.rs:13:5
2019-09-10T13:10:47.0607057Z    |
2019-09-10T13:10:47.0607306Z LL | /     fn take_bar(&mut self, b: Bar<'_>) {
2019-09-10T13:10:47.0607361Z LL | |         self.y = b.z
2019-09-10T13:10:47.0607409Z LL | |         //~^ ERROR
2019-09-10T13:10:47.0607527Z    | |_____^
2019-09-10T13:10:47.0607557Z 
2019-09-10T13:10:47.0607605Z error: aborting due to previous error
2019-09-10T13:10:47.0607926Z 
---
2019-09-10T13:10:47.0610022Z 
2019-09-10T13:10:47.0610051Z 
2019-09-10T13:10:47.0610117Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0610452Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55401/issue-55401.stderr
2019-09-10T13:10:47.0610730Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0611075Z To only update this specific test, also pass `--test-args nll/issue-55401.rs`
2019-09-10T13:10:47.0611170Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0611238Z status: exit code: 1
2019-09-10T13:10:47.0611238Z status: exit code: 1
2019-09-10T13:10:47.0612047Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-55401.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55401" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55401/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0612687Z ------------------------------------------
2019-09-10T13:10:47.0612727Z 
2019-09-10T13:10:47.0613001Z ------------------------------------------
2019-09-10T13:10:47.0613071Z stderr:
2019-09-10T13:10:47.0613071Z stderr:
2019-09-10T13:10:47.0613337Z ------------------------------------------
2019-09-10T13:10:47.0613398Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0613705Z   --> /checkout/src/test/ui/nll/issue-55401.rs:3:5
2019-09-10T13:10:47.0613763Z    |
2019-09-10T13:10:47.0613811Z LL |     *y //~ ERROR
2019-09-10T13:10:47.0613935Z    |
2019-09-10T13:10:47.0613935Z    |
2019-09-10T13:10:47.0613989Z    = note: ...the reference is valid for the static lifetime...
2019-09-10T13:10:47.0614363Z note: ...but the borrowed content is only valid for the lifetime 'a as defined on the function body at 1:47
2019-09-10T13:10:47.0614661Z   --> /checkout/src/test/ui/nll/issue-55401.rs:1:47
2019-09-10T13:10:47.0614721Z    |
2019-09-10T13:10:47.0615161Z LL | fn static_to_a_to_static_through_ref_in_tuple<'a>(x: &'a u32) -> &'static u32 {
2019-09-10T13:10:47.0615276Z 
2019-09-10T13:10:47.0615326Z error: aborting due to previous error
2019-09-10T13:10:47.0615378Z 
2019-09-10T13:10:47.0615708Z For more information about this error, try `rustc --explain E0312`.
---
2019-09-10T13:10:47.0617027Z 16 
2019-09-10T13:10:47.0617058Z 
2019-09-10T13:10:47.0617098Z 
2019-09-10T13:10:47.0617168Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0617564Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-normalize/constant-in-expr-normalize.stderr
2019-09-10T13:10:47.0618372Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0618749Z To only update this specific test, also pass `--test-args nll/user-annotations/constant-in-expr-normalize.rs`
2019-09-10T13:10:47.0618861Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0618928Z status: exit code: 1
2019-09-10T13:10:47.0618928Z status: exit code: 1
2019-09-10T13:10:47.0619824Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/constant-in-expr-normalize.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-normalize" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-normalize/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0620201Z ------------------------------------------
2019-09-10T13:10:47.0620239Z 
2019-09-10T13:10:47.0620503Z ------------------------------------------
2019-09-10T13:10:47.0620555Z stderr:
2019-09-10T13:10:47.0620555Z stderr:
2019-09-10T13:10:47.0621000Z ------------------------------------------
2019-09-10T13:10:47.0621082Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0621442Z    |
2019-09-10T13:10:47.0621442Z    |
2019-09-10T13:10:47.0621698Z LL |     <() as Foo<'a>>::C //~ ERROR
2019-09-10T13:10:47.0621811Z    |
2019-09-10T13:10:47.0621811Z    |
2019-09-10T13:10:47.0621879Z    = note: ...the reference is valid for the static lifetime...
2019-09-10T13:10:47.0622203Z note: ...but the borrowed content is only valid for the lifetime 'a as defined on the function body at 17:8
2019-09-10T13:10:47.0622582Z    |
2019-09-10T13:10:47.0622582Z    |
2019-09-10T13:10:47.0622834Z LL | fn foo<'a>(_: &'a u32) -> &'static u32 {
2019-09-10T13:10:47.0622918Z 
2019-09-10T13:10:47.0622991Z error: aborting due to previous error
2019-09-10T13:10:47.0623024Z 
2019-09-10T13:10:47.0623296Z For more information about this error, try `rustc --explain E0312`.
---
2019-09-10T13:10:47.0624665Z 16 
2019-09-10T13:10:47.0624695Z 
2019-09-10T13:10:47.0624739Z 
2019-09-10T13:10:47.0624789Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0625162Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-1/constant-in-expr-trait-item-1.stderr
2019-09-10T13:10:47.0625477Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0625830Z To only update this specific test, also pass `--test-args nll/user-annotations/constant-in-expr-trait-item-1.rs`
2019-09-10T13:10:47.0625944Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0626006Z status: exit code: 1
2019-09-10T13:10:47.0626006Z status: exit code: 1
2019-09-10T13:10:47.0626913Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-1/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0627319Z ------------------------------------------
2019-09-10T13:10:47.0627374Z 
2019-09-10T13:10:47.0628009Z ------------------------------------------
2019-09-10T13:10:47.0628075Z stderr:
2019-09-10T13:10:47.0628075Z stderr:
2019-09-10T13:10:47.0628414Z ------------------------------------------
2019-09-10T13:10:47.0628476Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0628783Z   --> /checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-1.rs:10:5
2019-09-10T13:10:47.0628859Z    |
2019-09-10T13:10:47.0629104Z LL |     <() as Foo<'a>>::C //~ ERROR
2019-09-10T13:10:47.0629202Z    |
2019-09-10T13:10:47.0629202Z    |
2019-09-10T13:10:47.0629272Z    = note: ...the reference is valid for the static lifetime...
2019-09-10T13:10:47.0629590Z note: ...but the borrowed content is only valid for the lifetime 'a as defined on the function body at 9:8
2019-09-10T13:10:47.0630112Z   --> /checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-1.rs:9:8
2019-09-10T13:10:47.0630174Z    |
2019-09-10T13:10:47.0630427Z LL | fn foo<'a>(_: &'a u32) -> &'static u32 {
2019-09-10T13:10:47.0630532Z 
2019-09-10T13:10:47.0630579Z error: aborting due to previous error
2019-09-10T13:10:47.0631017Z 
2019-09-10T13:10:47.0631407Z For more information about this error, try `rustc --explain E0312`.
---
2019-09-10T13:10:47.0633039Z 16 
2019-09-10T13:10:47.0633070Z 
2019-09-10T13:10:47.0633099Z 
2019-09-10T13:10:47.0633148Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0633700Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-2/constant-in-expr-trait-item-2.stderr
2019-09-10T13:10:47.0634043Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0634400Z To only update this specific test, also pass `--test-args nll/user-annotations/constant-in-expr-trait-item-2.rs`
2019-09-10T13:10:47.0634516Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0634570Z status: exit code: 1
2019-09-10T13:10:47.0634570Z status: exit code: 1
2019-09-10T13:10:47.0635501Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-2/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0635906Z ------------------------------------------
2019-09-10T13:10:47.0635945Z 
2019-09-10T13:10:47.0636213Z ------------------------------------------
2019-09-10T13:10:47.0636283Z stderr:
2019-09-10T13:10:47.0636283Z stderr:
2019-09-10T13:10:47.0636548Z ------------------------------------------
2019-09-10T13:10:47.0636611Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0636932Z   --> /checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-2.rs:10:5
2019-09-10T13:10:47.0637022Z    |
2019-09-10T13:10:47.0637283Z LL |     <T as Foo<'a>>::C //~ ERROR
2019-09-10T13:10:47.0637405Z    |
2019-09-10T13:10:47.0637405Z    |
2019-09-10T13:10:47.0637458Z    = note: ...the reference is valid for the static lifetime...
2019-09-10T13:10:47.0638241Z note: ...but the borrowed content is only valid for the lifetime 'a as defined on the function body at 9:8
2019-09-10T13:10:47.0638602Z   --> /checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-2.rs:9:8
2019-09-10T13:10:47.0638664Z    |
2019-09-10T13:10:47.0638915Z LL | fn foo<'a, T: Foo<'a>>() -> &'static u32 {
2019-09-10T13:10:47.0639019Z 
2019-09-10T13:10:47.0639067Z error: aborting due to previous error
2019-09-10T13:10:47.0639099Z 
2019-09-10T13:10:47.0639390Z For more information about this error, try `rustc --explain E0312`.
---
2019-09-10T13:10:47.0641188Z 20 
2019-09-10T13:10:47.0641217Z 
2019-09-10T13:10:47.0641245Z 
2019-09-10T13:10:47.0641294Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0641685Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-early-bound-error-method/regions-early-bound-error-method.stderr
2019-09-10T13:10:47.0641972Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0642307Z To only update this specific test, also pass `--test-args regions/regions-early-bound-error-method.rs`
2019-09-10T13:10:47.0642406Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0642454Z status: exit code: 1
2019-09-10T13:10:47.0642454Z status: exit code: 1
2019-09-10T13:10:47.0643701Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-early-bound-error-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-early-bound-error-method" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-early-bound-error-method/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0644264Z ------------------------------------------
2019-09-10T13:10:47.0645515Z 
2019-09-10T13:10:47.0645980Z ------------------------------------------
2019-09-10T13:10:47.0646065Z stderr:
2019-09-10T13:10:47.0646065Z stderr:
2019-09-10T13:10:47.0646308Z ------------------------------------------
2019-09-10T13:10:47.0646366Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0646740Z    |
2019-09-10T13:10:47.0646796Z LL |         g2.get()
2019-09-10T13:10:47.0646861Z    |         ^^^^^^^^
2019-09-10T13:10:47.0646907Z    |
2019-09-10T13:10:47.0646907Z    |
2019-09-10T13:10:47.0647204Z note: ...the reference is valid for the lifetime 'a as defined on the impl at 18:6...
2019-09-10T13:10:47.0647566Z    |
2019-09-10T13:10:47.0647566Z    |
2019-09-10T13:10:47.0647787Z LL | impl<'a> Box<'a> {
2019-09-10T13:10:47.0647838Z    |      ^^
2019-09-10T13:10:47.0649394Z note: ...but the borrowed content is only valid for the lifetime 'b as defined on the method body at 19:11
2019-09-10T13:10:47.0649972Z    |
2019-09-10T13:10:47.0649972Z    |
2019-09-10T13:10:47.0650244Z LL |     fn or<'b,G:GetRef<'b>>(&self, g2: G) -> &'a isize {
2019-09-10T13:10:47.0650331Z 
2019-09-10T13:10:47.0650396Z error: aborting due to previous error
2019-09-10T13:10:47.0650430Z 
2019-09-10T13:10:47.0650712Z For more information about this error, try `rustc --explain E0312`.
---
2019-09-10T13:10:47.0652420Z 20 
2019-09-10T13:10:47.0652450Z 
2019-09-10T13:10:47.0652503Z 
2019-09-10T13:10:47.0652554Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0653028Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-early-bound-error/regions-early-bound-error.stderr
2019-09-10T13:10:47.0653346Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0653982Z To only update this specific test, also pass `--test-args regions/regions-early-bound-error.rs`
2019-09-10T13:10:47.0654102Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0654152Z status: exit code: 1
2019-09-10T13:10:47.0654152Z status: exit code: 1
2019-09-10T13:10:47.0655236Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-early-bound-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-early-bound-error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-early-bound-error/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0656029Z ------------------------------------------
2019-09-10T13:10:47.0656132Z 
2019-09-10T13:10:47.0656529Z ------------------------------------------
2019-09-10T13:10:47.0656652Z stderr:
2019-09-10T13:10:47.0656652Z stderr:
2019-09-10T13:10:47.0657256Z ------------------------------------------
2019-09-10T13:10:47.0657357Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0658971Z    |
2019-09-10T13:10:47.0659048Z LL |     g1.get()
2019-09-10T13:10:47.0659094Z    |     ^^^^^^^^
2019-09-10T13:10:47.0659156Z    |
2019-09-10T13:10:47.0659156Z    |
2019-09-10T13:10:47.0659637Z note: ...the reference is valid for the lifetime 'b as defined on the function body at 18:11...
2019-09-10T13:10:47.0659980Z    |
2019-09-10T13:10:47.0659980Z    |
2019-09-10T13:10:47.0660268Z LL | fn get<'a,'b,G:GetRef<'a, isize>>(g1: G, b: &'b isize) -> &'b isize {
2019-09-10T13:10:47.0660325Z    |           ^^
2019-09-10T13:10:47.0660646Z note: ...but the borrowed content is only valid for the lifetime 'a as defined on the function body at 18:8
2019-09-10T13:10:47.0661006Z    |
2019-09-10T13:10:47.0661006Z    |
2019-09-10T13:10:47.0661285Z LL | fn get<'a,'b,G:GetRef<'a, isize>>(g1: G, b: &'b isize) -> &'b isize {
2019-09-10T13:10:47.0661390Z 
2019-09-10T13:10:47.0661438Z error: aborting due to previous error
2019-09-10T13:10:47.0661470Z 
2019-09-10T13:10:47.0661756Z For more information about this error, try `rustc --explain E0312`.
---
2019-09-10T13:10:47.0663743Z 60 
2019-09-10T13:10:47.0663774Z 
2019-09-10T13:10:47.0663803Z 
2019-09-10T13:10:47.0663854Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0664218Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns/regions-nested-fns.stderr
2019-09-10T13:10:47.0664495Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0665039Z To only update this specific test, also pass `--test-args regions/regions-nested-fns.rs`
2019-09-10T13:10:47.0665148Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0665196Z status: exit code: 1
2019-09-10T13:10:47.0665196Z status: exit code: 1
2019-09-10T13:10:47.0666030Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-nested-fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0666398Z ------------------------------------------
2019-09-10T13:10:47.0666446Z 
2019-09-10T13:10:47.0666697Z ------------------------------------------
2019-09-10T13:10:47.0666747Z stderr:
2019-09-10T13:10:47.0666747Z stderr:
2019-09-10T13:10:47.0667002Z ------------------------------------------
2019-09-10T13:10:47.0667063Z error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
2019-09-10T13:10:47.0667340Z   --> /checkout/src/test/ui/regions/regions-nested-fns.rs:5:18
2019-09-10T13:10:47.0667415Z    |
2019-09-10T13:10:47.0667810Z LL |     let mut ay = &y; //~ ERROR E0495
2019-09-10T13:10:47.0667972Z    |
2019-09-10T13:10:47.0667972Z    |
2019-09-10T13:10:47.0668029Z note: first, the lifetime cannot outlive the anonymous lifetime #2 defined on the body at 7:58...
2019-09-10T13:10:47.0668482Z    |
2019-09-10T13:10:47.0668482Z    |
2019-09-10T13:10:47.0668754Z LL |       ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
2019-09-10T13:10:47.0668815Z    |  __________________________________________________________^
2019-09-10T13:10:47.0668895Z LL | |         ay = x;
2019-09-10T13:10:47.0668943Z LL | |         ay = &y;
2019-09-10T13:10:47.0668988Z LL | |         ay = z;
2019-09-10T13:10:47.0669034Z LL | |     }));
2019-09-10T13:10:47.0669149Z note: ...so that reference does not outlive borrowed content
2019-09-10T13:10:47.0669424Z   --> /checkout/src/test/ui/regions/regions-nested-fns.rs:10:14
2019-09-10T13:10:47.0669504Z    |
2019-09-10T13:10:47.0669504Z    |
2019-09-10T13:10:47.0669551Z LL |         ay = z;
2019-09-10T13:10:47.0669598Z    |              ^
2019-09-10T13:10:47.0669672Z note: but, the lifetime must be valid for the anonymous lifetime #2 defined on the body at 13:72...
2019-09-10T13:10:47.0670000Z    |
2019-09-10T13:10:47.0670000Z    |
2019-09-10T13:10:47.0670298Z LL |       ignore::< Box<dyn for<'z> FnMut(&'z isize) -> &'z isize>>(Box::new(|z| {
2019-09-10T13:10:47.0670362Z    |  ________________________________________________________________________^
2019-09-10T13:10:47.0670427Z LL | |         if false { return x; } //~ ERROR E0312
2019-09-10T13:10:47.0670494Z LL | |         if false { return ay; }
2019-09-10T13:10:47.0670544Z LL | |         return z;
2019-09-10T13:10:47.0670590Z LL | |     }));
2019-09-10T13:10:47.0670703Z    = note: ...so that the types are compatible:
2019-09-10T13:10:47.0670703Z    = note: ...so that the types are compatible:
2019-09-10T13:10:47.0670761Z            expected &isize
2019-09-10T13:10:47.0670810Z               found &isize
2019-09-10T13:10:47.0670841Z 
2019-09-10T13:10:47.0670909Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0671242Z    |
2019-09-10T13:10:47.0671242Z    |
2019-09-10T13:10:47.0671309Z LL |         if false { return x; } //~ ERROR E0312
2019-09-10T13:10:47.0671406Z    |
2019-09-10T13:10:47.0671406Z    |
2019-09-10T13:10:47.0671478Z note: ...the reference is valid for the anonymous lifetime #2 defined on the body at 13:72...
2019-09-10T13:10:47.0671999Z    |
2019-09-10T13:10:47.0671999Z    |
2019-09-10T13:10:47.0672298Z LL |       ignore::< Box<dyn for<'z> FnMut(&'z isize) -> &'z isize>>(Box::new(|z| {
2019-09-10T13:10:47.0672361Z    |  ________________________________________________________________________^
2019-09-10T13:10:47.0672427Z LL | |         if false { return x; } //~ ERROR E0312
2019-09-10T13:10:47.0672496Z LL | |         if false { return ay; }
2019-09-10T13:10:47.0672546Z LL | |         return z;
2019-09-10T13:10:47.0672592Z LL | |     }));
2019-09-10T13:10:47.0672637Z    | |_____^
2019-09-10T13:10:47.0672966Z note: ...but the borrowed content is only valid for the lifetime 'x as defined on the function body at 3:11
2019-09-10T13:10:47.0673318Z    |
2019-09-10T13:10:47.0673318Z    |
2019-09-10T13:10:47.0673556Z LL | fn nested<'x>(x: &'x isize) {
2019-09-10T13:10:47.0673650Z 
2019-09-10T13:10:47.0673715Z error: aborting due to 2 previous errors
2019-09-10T13:10:47.0673747Z 
2019-09-10T13:10:47.0674019Z For more information about this error, try `rustc --explain E0312`.
---
2019-09-10T13:10:47.0675725Z 34 
2019-09-10T13:10:47.0675753Z 
2019-09-10T13:10:47.0675781Z 
2019-09-10T13:10:47.0675845Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0676207Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound.migrate/regions-static-bound.migrate.stderr
2019-09-10T13:10:47.0676485Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0676842Z To only update this specific test, also pass `--test-args regions/regions-static-bound.rs`
2019-09-10T13:10:47.0676887Z 
2019-09-10T13:10:47.0676941Z error in revision `migrate`: 1 errors occurred comparing output.
2019-09-10T13:10:47.0676994Z status: exit code: 1
2019-09-10T13:10:47.0677885Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-static-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound.migrate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound.migrate/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0678296Z ------------------------------------------
2019-09-10T13:10:47.0678337Z 
2019-09-10T13:10:47.0678968Z ------------------------------------------
2019-09-10T13:10:47.0679066Z stderr:
2019-09-10T13:10:47.0679066Z stderr:
2019-09-10T13:10:47.0679382Z ------------------------------------------
2019-09-10T13:10:47.0679443Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0679797Z    |
2019-09-10T13:10:47.0679797Z    |
2019-09-10T13:10:47.0679845Z LL |     t //[migrate]~ ERROR E0312
2019-09-10T13:10:47.0680117Z    |
2019-09-10T13:10:47.0680117Z    |
2019-09-10T13:10:47.0680169Z    = note: ...the reference is valid for the static lifetime...
2019-09-10T13:10:47.0680552Z note: ...but the borrowed content is only valid for the lifetime 'a as defined on the function body at 8:24
2019-09-10T13:10:47.0680899Z    |
2019-09-10T13:10:47.0680899Z    |
2019-09-10T13:10:47.0681202Z LL | fn static_id_wrong_way<'a>(t: &'a ()) -> &'static () where 'static: 'a {
2019-09-10T13:10:47.0681293Z 
2019-09-10T13:10:47.0681293Z 
2019-09-10T13:10:47.0681343Z error[E0621]: explicit lifetime required in the type of `u`
2019-09-10T13:10:47.0681690Z    |
2019-09-10T13:10:47.0681690Z    |
2019-09-10T13:10:47.0681737Z LL | fn error(u: &(), v: &()) {
2019-09-10T13:10:47.0682050Z    |             --- help: add explicit lifetime `'static` to the type of `u`: `&'static ()`
2019-09-10T13:10:47.0682117Z LL |     static_id(&u); //[migrate]~ ERROR explicit lifetime required in the type of `u` [E0621]
2019-09-10T13:10:47.0682380Z    |     ^^^^^^^^^ lifetime `'static` required
2019-09-10T13:10:47.0682434Z 
2019-09-10T13:10:47.0682486Z error[E0621]: explicit lifetime required in the type of `v`
2019-09-10T13:10:47.0682822Z    |
2019-09-10T13:10:47.0682822Z    |
2019-09-10T13:10:47.0682966Z LL | fn error(u: &(), v: &()) {
2019-09-10T13:10:47.0683316Z    |                     --- help: add explicit lifetime `'static` to the type of `v`: `&'static ()`
2019-09-10T13:10:47.0683372Z ...
2019-09-10T13:10:47.0683446Z LL |     static_id_indirect(&v); //[migrate]~ ERROR explicit lifetime required in the type of `v` [E0621]
2019-09-10T13:10:47.0683709Z    |     ^^^^^^^^^^^^^^^^^^ lifetime `'static` required
2019-09-10T13:10:47.0683812Z error: aborting due to 3 previous errors
2019-09-10T13:10:47.0683844Z 
2019-09-10T13:10:47.0683894Z Some errors have detailed explanations: E0312, E0621.
2019-09-10T13:10:47.0684178Z For more information about an error, try `rustc --explain E0312`.
---
2019-09-10T13:10:47.0685473Z 
2019-09-10T13:10:47.0685502Z 
2019-09-10T13:10:47.0685551Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0686001Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-infer-argument-types-two-region-pointers/unboxed-closures-infer-argument-types-two-region-pointers.stderr
2019-09-10T13:10:47.0686304Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0686653Z To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-infer-argument-types-two-region-pointers.rs`
2019-09-10T13:10:47.0686770Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0686819Z status: exit code: 1
2019-09-10T13:10:47.0686819Z status: exit code: 1
2019-09-10T13:10:47.0688511Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-infer-argument-types-two-region-pointers.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-infer-argument-types-two-region-pointers" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-infer-argument-types-two-region-pointers/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0696072Z ------------------------------------------
2019-09-10T13:10:47.0696114Z 
2019-09-10T13:10:47.0696368Z ------------------------------------------
2019-09-10T13:10:47.0696443Z stderr:
2019-09-10T13:10:47.0696443Z stderr:
2019-09-10T13:10:47.0696664Z ------------------------------------------
2019-09-10T13:10:47.0696719Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0697086Z    |
2019-09-10T13:10:47.0697086Z    |
2019-09-10T13:10:47.0697133Z LL |         x.set(y); //~ ERROR E0312
2019-09-10T13:10:47.0697236Z    |
2019-09-10T13:10:47.0697236Z    |
2019-09-10T13:10:47.0697294Z note: ...the reference is valid for the anonymous lifetime #3 defined on the body at 16:14...
2019-09-10T13:10:47.0698139Z    |
2019-09-10T13:10:47.0698139Z    |
2019-09-10T13:10:47.0698180Z LL |       doit(0, &|x, y| {
2019-09-10T13:10:47.0698241Z    |  ______________^
2019-09-10T13:10:47.0698456Z LL | |         x.set(y); //~ ERROR E0312
2019-09-10T13:10:47.0698552Z    | |_____^
2019-09-10T13:10:47.0698552Z    | |_____^
2019-09-10T13:10:47.0698620Z note: ...but the borrowed content is only valid for the anonymous lifetime #4 defined on the body at 16:14
2019-09-10T13:10:47.0699066Z    |
2019-09-10T13:10:47.0699066Z    |
2019-09-10T13:10:47.0699124Z LL |       doit(0, &|x, y| {
2019-09-10T13:10:47.0699168Z    |  ______________^
2019-09-10T13:10:47.0699211Z LL | |         x.set(y); //~ ERROR E0312
2019-09-10T13:10:47.0699322Z    | |_____^
2019-09-10T13:10:47.0699351Z 
2019-09-10T13:10:47.0699392Z error: aborting due to previous error
2019-09-10T13:10:47.0699436Z 
---
2019-09-10T13:10:47.0701080Z 109 
2019-09-10T13:10:47.0701106Z 
2019-09-10T13:10:47.0701131Z 
2019-09-10T13:10:47.0701175Z The actual stderr differed from the expected stderr.
2019-09-10T13:10:47.0701487Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-static-method/wf-static-method.stderr
2019-09-10T13:10:47.0701743Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T13:10:47.0701999Z To only update this specific test, also pass `--test-args wf/wf-static-method.rs`
2019-09-10T13:10:47.0702092Z error: 1 errors occurred comparing output.
2019-09-10T13:10:47.0702135Z status: exit code: 1
2019-09-10T13:10:47.0702135Z status: exit code: 1
2019-09-10T13:10:47.0702850Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/wf-static-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-static-method" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-static-method/auxiliary" "-A" "unused"
2019-09-10T13:10:47.0703324Z ------------------------------------------
2019-09-10T13:10:47.0703360Z 
2019-09-10T13:10:47.0703802Z ------------------------------------------
2019-09-10T13:10:47.0703855Z stderr:
2019-09-10T13:10:47.0703855Z stderr:
2019-09-10T13:10:47.0704094Z ------------------------------------------
2019-09-10T13:10:47.0704147Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0704385Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:17:9
2019-09-10T13:10:47.0704494Z LL |         u //~ ERROR E0312
2019-09-10T13:10:47.0704536Z    |         ^
2019-09-10T13:10:47.0704592Z    |
2019-09-10T13:10:47.0704592Z    |
2019-09-10T13:10:47.0704855Z note: ...the reference is valid for the lifetime 'a as defined on the impl at 14:6...
2019-09-10T13:10:47.0705099Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:14:6
2019-09-10T13:10:47.0705144Z    |
2019-09-10T13:10:47.0705383Z LL | impl<'a, 'b> Foo<'a, 'b, Evil<'a, 'b>> for () {
2019-09-10T13:10:47.0705431Z    |      ^^
2019-09-10T13:10:47.0705699Z note: ...but the borrowed content is only valid for the lifetime 'b as defined on the impl at 14:10
2019-09-10T13:10:47.0706069Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:14:10
2019-09-10T13:10:47.0706134Z    |
2019-09-10T13:10:47.0706389Z LL | impl<'a, 'b> Foo<'a, 'b, Evil<'a, 'b>> for () {
2019-09-10T13:10:47.0706488Z 
2019-09-10T13:10:47.0706531Z error[E0478]: lifetime bound not satisfied
2019-09-10T13:10:47.0706779Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:26:18
2019-09-10T13:10:47.0706825Z    |
2019-09-10T13:10:47.0706825Z    |
2019-09-10T13:10:47.0706873Z LL |         let me = Self::make_me(); //~ ERROR lifetime bound not satisfied
2019-09-10T13:10:47.0706986Z    |
2019-09-10T13:10:47.0706986Z    |
2019-09-10T13:10:47.0707257Z note: lifetime parameter instantiated with the lifetime 'b as defined on the impl at 23:10
2019-09-10T13:10:47.0707495Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:23:10
2019-09-10T13:10:47.0707914Z    |
2019-09-10T13:10:47.0708281Z LL | impl<'a, 'b> Foo<'a, 'b, ()> for IndirectEvil<'a, 'b> {
2019-09-10T13:10:47.0708345Z    |          ^^
2019-09-10T13:10:47.0708629Z note: but lifetime parameter must outlive the lifetime 'a as defined on the impl at 23:6
2019-09-10T13:10:47.0708871Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:23:6
2019-09-10T13:10:47.0708916Z    |
2019-09-10T13:10:47.0709159Z LL | impl<'a, 'b> Foo<'a, 'b, ()> for IndirectEvil<'a, 'b> {
2019-09-10T13:10:47.0709234Z 
2019-09-10T13:10:47.0709234Z 
2019-09-10T13:10:47.0709279Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-10T13:10:47.0709528Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:33:9
2019-09-10T13:10:47.0709622Z LL |         u //~ ERROR E0312
2019-09-10T13:10:47.0709681Z    |         ^
2019-09-10T13:10:47.0709720Z    |
2019-09-10T13:10:47.0709720Z    |
2019-09-10T13:10:47.0709980Z note: ...the reference is valid for the lifetime 'a as defined on the impl at 31:6...
2019-09-10T13:10:47.0710231Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:31:6
2019-09-10T13:10:47.0710278Z    |
2019-09-10T13:10:47.0710489Z LL | impl<'a, 'b> Evil<'a, 'b> {
2019-09-10T13:10:47.0710534Z    |      ^^
2019-09-10T13:10:47.0710823Z note: ...but the borrowed content is only valid for the lifetime 'b as defined on the impl at 31:10
2019-09-10T13:10:47.0711059Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:31:10
2019-09-10T13:10:47.0711104Z    |
2019-09-10T13:10:47.0711322Z LL | impl<'a, 'b> Evil<'a, 'b> {
2019-09-10T13:10:47.0711395Z 
2019-09-10T13:10:47.0711395Z 
2019-09-10T13:10:47.0711670Z error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements
2019-09-10T13:10:47.0712112Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:41:5
2019-09-10T13:10:47.0712159Z    |
2019-09-10T13:10:47.0712205Z LL |     <()>::static_evil(b) //~ ERROR cannot infer an appropriate lifetime
2019-09-10T13:10:47.0712309Z    |
2019-09-10T13:10:47.0712309Z    |
2019-09-10T13:10:47.0712591Z note: first, the lifetime cannot outlive the lifetime 'b as defined on the function body at 40:13...
2019-09-10T13:10:47.0712846Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:40:13
2019-09-10T13:10:47.0712892Z    |
2019-09-10T13:10:47.0713109Z LL | fn evil<'a, 'b>(b: &'b u32) -> &'a u32 {
2019-09-10T13:10:47.0713219Z note: ...so that reference does not outlive borrowed content
2019-09-10T13:10:47.0713449Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:41:23
2019-09-10T13:10:47.0713510Z    |
2019-09-10T13:10:47.0713510Z    |
2019-09-10T13:10:47.0713557Z LL |     <()>::static_evil(b) //~ ERROR cannot infer an appropriate lifetime
2019-09-10T13:10:47.0713611Z    |                       ^
2019-09-10T13:10:47.0713884Z note: but, the lifetime must be valid for the lifetime 'a as defined on the function body at 40:9...
2019-09-10T13:10:47.0714135Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:40:9
2019-09-10T13:10:47.0714181Z    |
2019-09-10T13:10:47.0714396Z LL | fn evil<'a, 'b>(b: &'b u32) -> &'a u32 {
2019-09-10T13:10:47.0714617Z note: ...so that reference does not outlive borrowed content
2019-09-10T13:10:47.0714883Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:41:5
2019-09-10T13:10:47.0714944Z    |
2019-09-10T13:10:47.0714944Z    |
2019-09-10T13:10:47.0714991Z LL |     <()>::static_evil(b) //~ ERROR cannot infer an appropriate lifetime
2019-09-10T13:10:47.0715066Z 
2019-09-10T13:10:47.0715066Z 
2019-09-10T13:10:47.0715361Z error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements
2019-09-10T13:10:47.0715607Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:45:5
2019-09-10T13:10:47.0715651Z    |
2019-09-10T13:10:47.0715710Z LL |     <IndirectEvil>::static_evil(b)
2019-09-10T13:10:47.0715795Z    |
2019-09-10T13:10:47.0715795Z    |
2019-09-10T13:10:47.0716082Z note: first, the lifetime cannot outlive the lifetime 'b as defined on the function body at 44:22...
2019-09-10T13:10:47.0716329Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:44:22
2019-09-10T13:10:47.0716374Z    |
2019-09-10T13:10:47.0716615Z LL | fn indirect_evil<'a, 'b>(b: &'b u32) -> &'a u32 {
2019-09-10T13:10:47.0716709Z note: ...so that reference does not outlive borrowed content
2019-09-10T13:10:47.0716940Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:45:33
2019-09-10T13:10:47.0717001Z    |
2019-09-10T13:10:47.0717001Z    |
2019-09-10T13:10:47.0717045Z LL |     <IndirectEvil>::static_evil(b)
2019-09-10T13:10:47.0717089Z    |                                 ^
2019-09-10T13:10:47.0717386Z note: but, the lifetime must be valid for the lifetime 'a as defined on the function body at 44:18...
2019-09-10T13:10:47.0717921Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:44:18
2019-09-10T13:10:47.0717980Z    |
2019-09-10T13:10:47.0718240Z LL | fn indirect_evil<'a, 'b>(b: &'b u32) -> &'a u32 {
2019-09-10T13:10:47.0718346Z note: ...so that reference does not outlive borrowed content
2019-09-10T13:10:47.0718595Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:45:5
2019-09-10T13:10:47.0718641Z    |
2019-09-10T13:10:47.0718641Z    |
2019-09-10T13:10:47.0718683Z LL |     <IndirectEvil>::static_evil(b)
2019-09-10T13:10:47.0718772Z 
2019-09-10T13:10:47.0718813Z error: aborting due to 5 previous errors
2019-09-10T13:10:47.0718840Z 
2019-09-10T13:10:47.0718884Z Some errors have detailed explanations: E0312, E0478.
---
2019-09-10T13:10:47.0723863Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-10T13:10:47.0723921Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-10T13:10:47.0723953Z 
2019-09-10T13:10:47.0723996Z 
2019-09-10T13:10:47.0725434Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-10T13:10:47.0725714Z 
2019-09-10T13:10:47.0725967Z 
2019-09-10T13:10:47.0726036Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-10T13:10:47.0726089Z Build completed unsuccessfully in 1:12:02
2019-09-10T13:10:47.0726089Z Build completed unsuccessfully in 1:12:02
2019-09-10T13:10:47.0726209Z == clock drift check ==
2019-09-10T13:10:47.0726266Z   local time: Tue Sep 10 13:10:47 UTC 2019
2019-09-10T13:10:47.1640665Z   network time: Tue, 10 Sep 2019 13:10:47 GMT
2019-09-10T13:10:47.1644473Z == end clock drift check ==
2019-09-10T13:10:47.9969862Z ##[error]Bash exited with code '1'.
2019-09-10T13:10:48.0007290Z ##[section]Starting: Checkout
2019-09-10T13:10:48.0009957Z ==============================================================================
2019-09-10T13:10:48.0010016Z Task         : Get sources
2019-09-10T13:10:48.0010080Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
