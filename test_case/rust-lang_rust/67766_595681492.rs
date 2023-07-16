plain
2020-03-06T08:12:49.3922183Z ========================== Starting Command Output ===========================
2020-03-06T08:12:49.3927290Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1e5e015e-c378-4ae9-ab1b-b4112f02133e.sh
2020-03-06T08:12:49.3927786Z 
2020-03-06T08:12:49.3932117Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-06T08:12:49.3952418Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67766/merge to s
2020-03-06T08:12:49.3956189Z Task         : Get sources
2020-03-06T08:12:49.3956479Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-06T08:12:49.3956759Z Version      : 1.0.0
2020-03-06T08:12:49.3956966Z Author       : Microsoft
---
2020-03-06T08:12:50.3887265Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-06T08:12:50.3892797Z ##[command]git config gc.auto 0
2020-03-06T08:12:50.3897044Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-06T08:12:50.3901135Z ##[command]git config --get-all http.proxy
2020-03-06T08:12:50.3909751Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67766/merge:refs/remotes/pull/67766/merge
---
2020-03-06T09:17:51.7287702Z .................................................................................................... 1700/9732
2020-03-06T09:17:56.2973170Z .................................................................................................... 1800/9732
2020-03-06T09:18:08.1250992Z ..........................................................i......................................... 1900/9732
2020-03-06T09:18:15.8074240Z .................................................................................................... 2000/9732
2020-03-06T09:18:30.1174087Z ................................................iiiii............................................... 2100/9732
2020-03-06T09:18:40.4646822Z .................................................................................................... 2300/9732
2020-03-06T09:18:42.7804784Z .................................................................................................... 2400/9732
2020-03-06T09:18:46.3087012Z .................................................................................................... 2500/9732
2020-03-06T09:19:08.7049599Z .................................................................................................... 2600/9732
---
2020-03-06T09:21:51.9911710Z .........i...............i.......................................................................... 5000/9732
2020-03-06T09:22:02.4723263Z .................................................................................................... 5100/9732
2020-03-06T09:22:07.4409403Z ....................................................i............................................... 5200/9732
2020-03-06T09:22:16.1936802Z .................................................................................................... 5300/9732
2020-03-06T09:22:24.1068375Z ...............................ii.ii........i...i................................................... 5400/9732
2020-03-06T09:22:33.1792482Z ...F................................................................................................ 5600/9732
2020-03-06T09:22:43.5284923Z ..............F........F............................................................................ 5700/9732
2020-03-06T09:22:51.2790917Z .......................i............................................................................ 5800/9732
2020-03-06T09:22:57.3433928Z .................................................................................................... 5900/9732
2020-03-06T09:22:57.3433928Z .................................................................................................... 5900/9732
2020-03-06T09:23:09.0952361Z .................................................................................................... 6000/9732
2020-03-06T09:23:20.1243659Z ...............ii...i..ii...........i............................................................... 6100/9732
2020-03-06T09:23:37.8963464Z .................................................................................................... 6300/9732
2020-03-06T09:23:45.3625763Z .................................................................................................... 6400/9732
2020-03-06T09:23:45.3625763Z .................................................................................................... 6400/9732
2020-03-06T09:24:10.8245260Z ..............................................i..ii................................................. 6500/9732
2020-03-06T09:24:32.7435047Z .................................................................................................... 6700/9732
2020-03-06T09:24:34.8994465Z ......................................i............................................................. 6800/9732
2020-03-06T09:24:37.1369295Z .................................................................................................... 6900/9732
2020-03-06T09:24:39.4790433Z ....................................................................i............................... 7000/9732
---
2020-03-06T09:26:26.0132998Z .................................................................................................... 7700/9732
2020-03-06T09:26:31.6654873Z .................................................................................................... 7800/9732
2020-03-06T09:26:36.8496450Z .................................................................................................... 7900/9732
2020-03-06T09:26:45.8835149Z ..............i..................................................................................... 8000/9732
2020-03-06T09:26:55.6430534Z ...............................................................iiiiiiiii.i.......................... 8100/9732
2020-03-06T09:27:12.3880558Z ......i......i...................................................................................... 8300/9732
2020-03-06T09:27:18.3365408Z .................................................................................................... 8400/9732
2020-03-06T09:27:33.3955738Z .................................................................................................... 8500/9732
2020-03-06T09:27:43.9535513Z .................................................................................................... 8600/9732
---
2020-03-06T09:29:50.0263305Z 
2020-03-06T09:29:50.0263974Z 17     use MyEnum::*;
2020-03-06T09:29:50.0264300Z 18 
2020-03-06T09:29:50.0264608Z 19     match x {
2020-03-06T09:29:50.0265443Z -         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-03-06T09:29:50.0266005Z +         A { i, j } | B { i, j } => { //~ ERROR unused variable
2020-03-06T09:29:50.0266794Z 22         }
2020-03-06T09:29:50.0267074Z 23     }
2020-03-06T09:29:50.0267328Z 
2020-03-06T09:29:50.0267621Z 27     use MyEnum::*;
2020-03-06T09:29:50.0267621Z 27     use MyEnum::*;
2020-03-06T09:29:50.0267911Z 28 
2020-03-06T09:29:50.0268203Z 29     match x {
2020-03-06T09:29:50.0268898Z -         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-03-06T09:29:50.0269456Z +         A { i, ref j } | B { i, ref j } => { //~ ERROR unused variable
2020-03-06T09:29:50.0270270Z 32         }
2020-03-06T09:29:50.0270565Z 33     }
2020-03-06T09:29:50.0270858Z 
2020-03-06T09:29:50.0271274Z 37     use MyEnum::*;
2020-03-06T09:29:50.0271274Z 37     use MyEnum::*;
2020-03-06T09:29:50.0274172Z 38 
2020-03-06T09:29:50.0274343Z 39     match x {
2020-03-06T09:29:50.0280279Z -         Some(A { i, j: _ } | B { i, j: _ }) => { //~ ERROR unused variable
2020-03-06T09:29:50.0280714Z +         Some(A { i, j } | B { i, j }) => { //~ ERROR unused variable
2020-03-06T09:29:50.0281256Z 42         }
2020-03-06T09:29:50.0281399Z 43 
2020-03-06T09:29:50.0281526Z 
2020-03-06T09:29:50.0281694Z 49     use MyEnum::*;
2020-03-06T09:29:50.0281694Z 49     use MyEnum::*;
2020-03-06T09:29:50.0281866Z 50 
2020-03-06T09:29:50.0282033Z 51     match x {
2020-03-06T09:29:50.0282669Z -         Some(A { i, j: _ } | B { i, j: _ }) => { //~ ERROR unused variable
2020-03-06T09:29:50.0283094Z +         Some(A { i, ref j } | B { i, ref j }) => { //~ ERROR unused variable
2020-03-06T09:29:50.0283619Z 54         }
2020-03-06T09:29:50.0283760Z 55 
2020-03-06T09:29:50.0283866Z 
2020-03-06T09:29:50.0284004Z 59 
2020-03-06T09:29:50.0284004Z 59 
2020-03-06T09:29:50.0284209Z 60 pub fn mixed_no_ref(x: MixedEnum) {
2020-03-06T09:29:50.0284433Z 61     match x {
2020-03-06T09:29:50.0284917Z -         MixedEnum::A { i: _ } | MixedEnum::B(_) => {
2020-03-06T09:29:50.0285268Z +         MixedEnum::A { i } | MixedEnum::B(i) => {
2020-03-06T09:29:50.0285552Z 63             println!("match");
2020-03-06T09:29:50.0285923Z 65     }
2020-03-06T09:29:50.0286037Z 
2020-03-06T09:29:50.0286156Z 67 
2020-03-06T09:29:50.0286156Z 67 
2020-03-06T09:29:50.0291163Z 68 pub fn mixed_with_ref(x: MixedEnum) {
2020-03-06T09:29:50.0291404Z 69     match x {
2020-03-06T09:29:50.0291975Z -         MixedEnum::A { i: _ } | MixedEnum::B(_) => {
2020-03-06T09:29:50.0292867Z +         MixedEnum::A { ref i } | MixedEnum::B(ref i) => {
2020-03-06T09:29:50.0293196Z 71             println!("match");
2020-03-06T09:29:50.0293541Z 73     }
2020-03-06T09:29:50.0293672Z 
2020-03-06T09:29:50.0293771Z 
2020-03-06T09:29:50.0293978Z The actual fixed differed from the expected fixed.
2020-03-06T09:29:50.0293978Z The actual fixed differed from the expected fixed.
2020-03-06T09:29:50.0294844Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-field-in-or-pattern/issue-67691-unused-field-in-or-pattern.fixed
2020-03-06T09:29:50.0295590Z To update references, rerun the tests and pass the `--bless` flag
2020-03-06T09:29:50.0296394Z To only update this specific test, also pass `--test-args lint/issue-67691-unused-field-in-or-pattern.rs`
2020-03-06T09:29:50.0296918Z error: 1 errors occurred comparing output.
2020-03-06T09:29:50.0297156Z status: exit code: 1
2020-03-06T09:29:50.0297156Z status: exit code: 1
2020-03-06T09:29:50.0301896Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-field-in-or-pattern" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-field-in-or-pattern/auxiliary"
2020-03-06T09:29:50.0303788Z ------------------------------------------
2020-03-06T09:29:50.0303977Z 
2020-03-06T09:29:50.0306934Z ------------------------------------------
2020-03-06T09:29:50.0308332Z stderr:
2020-03-06T09:29:50.0308332Z stderr:
2020-03-06T09:29:50.0310464Z ------------------------------------------
2020-03-06T09:29:50.0310744Z error: unused variable: `j`
2020-03-06T09:29:50.0312594Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:20:16
2020-03-06T09:29:50.0321028Z    |
2020-03-06T09:29:50.0321298Z LL |         A { i, j } | B { i, j } => { //~ ERROR unused variable
2020-03-06T09:29:50.0321810Z    |
2020-03-06T09:29:50.0321999Z note: the lint level is defined here
2020-03-06T09:29:50.0322738Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:4:9
2020-03-06T09:29:50.0323046Z    |
2020-03-06T09:29:50.0323046Z    |
2020-03-06T09:29:50.0325696Z LL | #![deny(unused)]
2020-03-06T09:29:50.0325914Z    |         ^^^^^^
2020-03-06T09:29:50.0326219Z    = note: `#[deny(unused_variables)]` implied by `#[deny(unused)]`
2020-03-06T09:29:50.0326523Z help: try ignoring the field
2020-03-06T09:29:50.0326701Z    |
2020-03-06T09:29:50.0326993Z LL |         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-03-06T09:29:50.0327506Z 
2020-03-06T09:29:50.0327682Z error: unused variable: `j`
2020-03-06T09:29:50.0330934Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:30:16
2020-03-06T09:29:50.0331246Z    |
2020-03-06T09:29:50.0331246Z    |
2020-03-06T09:29:50.0331505Z LL |         A { i, ref j } | B { i, ref j } => { //~ ERROR unused variable
2020-03-06T09:29:50.0332052Z    |
2020-03-06T09:29:50.0332225Z help: try ignoring the field
2020-03-06T09:29:50.0332417Z    |
2020-03-06T09:29:50.0332417Z    |
2020-03-06T09:29:50.0332688Z LL |         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-03-06T09:29:50.0333247Z 
2020-03-06T09:29:50.0333437Z error: unused variable: `j`
2020-03-06T09:29:50.0334036Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:40:21
2020-03-06T09:29:50.0334324Z    |
2020-03-06T09:29:50.0334324Z    |
2020-03-06T09:29:50.0334598Z LL |         Some(A { i, j } | B { i, j }) => { //~ ERROR unused variable
2020-03-06T09:29:50.0335326Z    |
2020-03-06T09:29:50.0335523Z help: try ignoring the field
2020-03-06T09:29:50.0335697Z    |
2020-03-06T09:29:50.0335697Z    |
2020-03-06T09:29:50.0335981Z LL |         Some(A { i, j: _ } | B { i, j: _ }) => { //~ ERROR unused variable
2020-03-06T09:29:50.0336532Z 
2020-03-06T09:29:50.0336706Z error: unused variable: `j`
2020-03-06T09:29:50.0337328Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:52:21
2020-03-06T09:29:50.0337615Z    |
2020-03-06T09:29:50.0337615Z    |
2020-03-06T09:29:50.0337882Z LL |         Some(A { i, ref j } | B { i, ref j }) => { //~ ERROR unused variable
2020-03-06T09:29:50.0338579Z    |
2020-03-06T09:29:50.0339016Z help: try ignoring the field
2020-03-06T09:29:50.0339210Z    |
2020-03-06T09:29:50.0339210Z    |
2020-03-06T09:29:50.0339519Z LL |         Some(A { i, j: _ } | B { i, j: _ }) => { //~ ERROR unused variable
2020-03-06T09:29:50.0340221Z 
2020-03-06T09:29:50.0340424Z error: unused variable: `i`
2020-03-06T09:29:50.0341083Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:62:24
2020-03-06T09:29:50.0341372Z    |
2020-03-06T09:29:50.0341372Z    |
2020-03-06T09:29:50.0344258Z LL |         MixedEnum::A { i } | MixedEnum::B(i) => {
2020-03-06T09:29:50.0345036Z    |
2020-03-06T09:29:50.0345217Z help: try ignoring the field
2020-03-06T09:29:50.0345414Z    |
2020-03-06T09:29:50.0345414Z    |
2020-03-06T09:29:50.0345663Z LL |         MixedEnum::A { i: _ } | MixedEnum::B(_) => {
2020-03-06T09:29:50.0346212Z 
2020-03-06T09:29:50.0346388Z error: unused variable: `i`
2020-03-06T09:29:50.0347099Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:70:24
2020-03-06T09:29:50.0347408Z    |
2020-03-06T09:29:50.0347408Z    |
2020-03-06T09:29:50.0347675Z LL |         MixedEnum::A { ref i } | MixedEnum::B(ref i) => {
2020-03-06T09:29:50.0348260Z    |
2020-03-06T09:29:50.0348437Z help: try ignoring the field
2020-03-06T09:29:50.0348609Z    |
2020-03-06T09:29:50.0348609Z    |
2020-03-06T09:29:50.0348857Z LL |         MixedEnum::A { i: _ } | MixedEnum::B(_) => {
2020-03-06T09:29:50.0349700Z 
2020-03-06T09:29:50.0349889Z error: aborting due to 6 previous errors
2020-03-06T09:29:50.0350060Z 
2020-03-06T09:29:50.0351019Z 
---
2020-03-06T09:29:50.0382002Z 29 
2020-03-06T09:29:50.0382153Z 
2020-03-06T09:29:50.0382253Z 
2020-03-06T09:29:50.0382499Z The actual stderr differed from the expected stderr.
2020-03-06T09:29:50.0383311Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-dead/liveness-dead.stderr
2020-03-06T09:29:50.0383985Z To update references, rerun the tests and pass the `--bless` flag
2020-03-06T09:29:50.0385047Z To only update this specific test, also pass `--test-args liveness/liveness-dead.rs`
2020-03-06T09:29:50.0390979Z error: 1 errors occurred comparing output.
2020-03-06T09:29:50.0391885Z status: exit code: 1
2020-03-06T09:29:50.0391885Z status: exit code: 1
2020-03-06T09:29:50.0394113Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-dead.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-dead" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-dead/auxiliary"
2020-03-06T09:29:50.0395776Z ------------------------------------------
2020-03-06T09:29:50.0395957Z 
2020-03-06T09:29:50.0396359Z ------------------------------------------
2020-03-06T09:29:50.0397199Z stderr:
---
2020-03-06T09:29:50.0403874Z 
2020-03-06T09:29:50.0404065Z error: value passed to `x` is never read
2020-03-06T09:29:50.0404664Z   --> /checkout/src/test/ui/liveness/liveness-dead.rs:20:7
2020-03-06T09:29:50.0404904Z    |
2020-03-06T09:29:50.0405184Z LL | fn f4(mut x: i32) { //~ ERROR: value passed to `x` is never read
2020-03-06T09:29:50.0405653Z    |
2020-03-06T09:29:50.0405875Z    = help: maybe it is overwritten before being read?
2020-03-06T09:29:50.0406072Z 
2020-03-06T09:29:50.0406283Z error: value assigned to `x` is never read
---
2020-03-06T09:29:50.0414117Z 53 
2020-03-06T09:29:50.0414222Z 
2020-03-06T09:29:50.0414452Z 65    = help: maybe it is overwritten before being read?
2020-03-06T09:29:50.0414676Z 66 
2020-03-06T09:29:50.0414912Z 67 error: variable `z` is assigned to, but never used
2020-03-06T09:29:50.0416141Z +   --> $DIR/liveness-unused.rs:37:9
2020-03-06T09:29:50.0416365Z 69    |
2020-03-06T09:29:50.0416365Z 69    |
2020-03-06T09:29:50.0416542Z 70 LL |     let mut z = 3;
2020-03-06T09:29:50.0417105Z +    |         ^^^^^
2020-03-06T09:29:50.0417270Z 72    |
2020-03-06T09:29:50.0417477Z 73    = note: consider using `_z` instead
2020-03-06T09:29:50.0417676Z 74 
2020-03-06T09:29:50.0417676Z 74 
2020-03-06T09:29:50.0417798Z 
2020-03-06T09:29:50.0417894Z 
2020-03-06T09:29:50.0418111Z The actual stderr differed from the expected stderr.
2020-03-06T09:29:50.0418791Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-unused/liveness-unused.stderr
2020-03-06T09:29:50.0419463Z To update references, rerun the tests and pass the `--bless` flag
2020-03-06T09:29:50.0420069Z To only update this specific test, also pass `--test-args liveness/liveness-unused.rs`
2020-03-06T09:29:50.0420532Z error: 1 errors occurred comparing output.
2020-03-06T09:29:50.0420769Z status: exit code: 1
2020-03-06T09:29:50.0420769Z status: exit code: 1
2020-03-06T09:29:50.0429558Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-unused/auxiliary"
2020-03-06T09:29:50.0431651Z ------------------------------------------
2020-03-06T09:29:50.0431848Z 
2020-03-06T09:29:50.0432237Z ------------------------------------------
2020-03-06T09:29:50.0432445Z stderr:
2020-03-06T09:29:50.0432445Z stderr:
2020-03-06T09:29:50.0432845Z ------------------------------------------
2020-03-06T09:29:50.0433095Z warning: unreachable statement
2020-03-06T09:29:50.0433768Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:92:9
2020-03-06T09:29:50.0434053Z    |
2020-03-06T09:29:50.0434222Z LL |         continue;
2020-03-06T09:29:50.0434750Z    |         -------- any code following this expression is unreachable
2020-03-06T09:29:50.0435301Z LL |         drop(*x as i32); //~ WARNING unreachable statement
2020-03-06T09:29:50.0435619Z    |         ^^^^^^^^^^^^^^^^ unreachable statement
2020-03-06T09:29:50.0436047Z note: the lint level is defined here
2020-03-06T09:29:50.0436584Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:1:9
2020-03-06T09:29:50.0436825Z    |
2020-03-06T09:29:50.0437003Z LL | #![warn(unused)]
2020-03-06T09:29:50.0437003Z LL | #![warn(unused)]
2020-03-06T09:29:50.0437408Z    |         ^^^^^^
2020-03-06T09:29:50.0437685Z    = note: `#[warn(unreachable_code)]` implied by `#[warn(unused)]`
2020-03-06T09:29:50.0437909Z 
2020-03-06T09:29:50.0438104Z error: unused variable: `x`
2020-03-06T09:29:50.0438883Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:8:7
2020-03-06T09:29:50.0439134Z    |
2020-03-06T09:29:50.0439328Z LL | fn f1(x: isize) {
2020-03-06T09:29:50.0439629Z    |       ^ help: consider prefixing with an underscore: `_x`
2020-03-06T09:29:50.0440092Z note: the lint level is defined here
2020-03-06T09:29:50.0440606Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:2:9
2020-03-06T09:29:50.0440845Z    |
2020-03-06T09:29:50.0441175Z LL | #![deny(unused_variables)]
2020-03-06T09:29:50.0441175Z LL | #![deny(unused_variables)]
2020-03-06T09:29:50.0441420Z    |         ^^^^^^^^^^^^^^^^
2020-03-06T09:29:50.0441742Z 
2020-03-06T09:29:50.0441920Z error: unused variable: `x`
2020-03-06T09:29:50.0442501Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:12:8
2020-03-06T09:29:50.0443173Z    |
2020-03-06T09:29:50.0443360Z LL | fn f1b(x: &mut isize) {
2020-03-06T09:29:50.0443698Z    |        ^ help: consider prefixing with an underscore: `_x`
2020-03-06T09:29:50.0444109Z error: unused variable: `x`
2020-03-06T09:29:50.0444679Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:20:9
2020-03-06T09:29:50.0444940Z    |
2020-03-06T09:29:50.0445112Z LL |     let x: isize;
---
2020-03-06T09:29:50.0453010Z LL | #![deny(unused_assignments)]
2020-03-06T09:29:50.0453238Z    |         ^^^^^^^^^^^^^^^^^^
2020-03-06T09:29:50.0453507Z    = help: maybe it is overwritten before being read?
2020-03-06T09:29:50.0453725Z 
2020-03-06T09:29:50.0453932Z error: variable `z` is assigned to, but never used
2020-03-06T09:29:50.0462733Z    |
2020-03-06T09:29:50.0462733Z    |
2020-03-06T09:29:50.0462905Z LL |     let mut z = 3;
2020-03-06T09:29:50.0463251Z    |
2020-03-06T09:29:50.0463467Z    = note: consider using `_z` instead
2020-03-06T09:29:50.0463640Z 
2020-03-06T09:29:50.0463817Z error: unused variable: `i`
---
2020-03-06T09:29:50.0479614Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-06T09:29:50.0480038Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-06T09:29:50.0482188Z 
2020-03-06T09:29:50.0482540Z 
2020-03-06T09:29:50.0486590Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-06T09:29:50.0489865Z 
2020-03-06T09:29:50.0490008Z 
2020-03-06T09:29:50.0490269Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-06T09:29:50.0490740Z Build completed unsuccessfully in 1:10:53
2020-03-06T09:29:50.0490740Z Build completed unsuccessfully in 1:10:53
2020-03-06T09:29:50.0491105Z == clock drift check ==
2020-03-06T09:29:50.0491403Z   local time: Fri Mar  6 09:29:50 UTC 2020
2020-03-06T09:29:50.3250554Z   network time: Fri, 06 Mar 2020 09:29:50 GMT
2020-03-06T09:29:50.3251418Z == end clock drift check ==
2020-03-06T09:29:50.7977993Z 
2020-03-06T09:29:50.8087610Z ##[error]Bash exited with code '1'.
2020-03-06T09:29:50.8102642Z ##[section]Finishing: Run build
2020-03-06T09:29:50.8183551Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67766/merge to s
2020-03-06T09:29:50.8189359Z Task         : Get sources
2020-03-06T09:29:50.8189794Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-06T09:29:50.8190201Z Version      : 1.0.0
2020-03-06T09:29:50.8190484Z Author       : Microsoft
2020-03-06T09:29:50.8190484Z Author       : Microsoft
2020-03-06T09:29:50.8190918Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-06T09:29:50.8191438Z ==============================================================================
2020-03-06T09:29:51.1969121Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-06T09:29:51.2025526Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67766/merge to s
2020-03-06T09:29:51.2123144Z Cleaning up task key
2020-03-06T09:29:51.2124559Z Start cleaning up orphan processes.
2020-03-06T09:29:51.2350869Z Terminate orphan process: pid (4538) (python)
2020-03-06T09:29:51.2610159Z ##[section]Finishing: Finalize Job
