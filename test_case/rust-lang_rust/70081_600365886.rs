plain
2020-03-17T23:30:08.3022611Z ========================== Starting Command Output ===========================
2020-03-17T23:30:08.3025196Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5d1d75ba-4457-4fdf-8891-ef7bdf78f8a6.sh
2020-03-17T23:30:08.3025492Z 
2020-03-17T23:30:08.3029257Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T23:30:08.3049342Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-17T23:30:08.3052530Z Task         : Get sources
2020-03-17T23:30:08.3052849Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T23:30:08.3053149Z Version      : 1.0.0
2020-03-17T23:30:08.3053348Z Author       : Microsoft
---
2020-03-17T23:30:09.2986494Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T23:30:09.2994374Z ##[command]git config gc.auto 0
2020-03-17T23:30:09.2999150Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T23:30:09.3004885Z ##[command]git config --get-all http.proxy
2020-03-17T23:30:09.3013519Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70081/merge:refs/remotes/pull/70081/merge
---
2020-03-18T00:24:32.5322458Z .................................................................................................... 1700/9800
2020-03-18T00:24:36.3332611Z .................................................................................................... 1800/9800
2020-03-18T00:24:46.4171121Z ............................................................................i....................... 1900/9800
2020-03-18T00:24:52.1481966Z .................................................................................................... 2000/9800
2020-03-18T00:24:59.2587898Z ..................................................................iiiii............................. 2100/9800
2020-03-18T00:25:15.5371461Z .................................................................................................... 2300/9800
2020-03-18T00:25:17.5555260Z .................................................................................................... 2400/9800
2020-03-18T00:25:20.0973256Z .................................................................................................... 2500/9800
2020-03-18T00:25:38.3282826Z .................................................................................................... 2600/9800
---
2020-03-18T00:27:55.6057469Z ......................................i...............i............................................. 5000/9800
2020-03-18T00:28:03.1334873Z .................................................................................................... 5100/9800
2020-03-18T00:28:08.6935811Z .................................................................................i.................. 5200/9800
2020-03-18T00:28:13.5072042Z .................................................................................................... 5300/9800
2020-03-18T00:28:22.2929339Z ..............................................................ii.ii........i...i.................... 5400/9800
2020-03-18T00:28:29.0258899Z .i.................................................................................................. 5600/9800
2020-03-18T00:28:36.8984855Z ..................................F................................................................. 5700/9800
2020-03-18T00:28:42.0932553Z .........................................................i.......................................... 5800/9800
2020-03-18T00:28:47.5395146Z .................................................................................................... 5900/9800
2020-03-18T00:28:47.5395146Z .................................................................................................... 5900/9800
2020-03-18T00:28:53.9626213Z .................................................................................................... 6000/9800
2020-03-18T00:29:00.4265406Z ...................................................ii...i..ii...........i........................... 6100/9800
2020-03-18T00:29:17.7863938Z .................................................................................................... 6300/9800
2020-03-18T00:29:21.2026317Z .................................................................................................... 6400/9800
2020-03-18T00:29:21.2026317Z .................................................................................................... 6400/9800
2020-03-18T00:29:24.3893908Z .................................................................................i..ii.............. 6500/9800
2020-03-18T00:29:44.2805875Z .................................................................................................... 6700/9800
2020-03-18T00:29:52.2978218Z ...............................................................................i.................... 6800/9800
2020-03-18T00:29:54.0968093Z .................................................................................................... 6900/9800
2020-03-18T00:29:55.8762201Z .................................................................................................... 7000/9800
---
2020-03-18T00:31:22.1331700Z .................................................................................................... 7800/9800
2020-03-18T00:31:26.6162200Z .................................................................................................... 7900/9800
2020-03-18T00:31:31.8534671Z .................................................................i.................................. 8000/9800
2020-03-18T00:31:40.3938646Z .................................................................................................... 8100/9800
2020-03-18T00:31:44.8261915Z ..............iiiiiiiiii.i.......................................................................... 8200/9800
2020-03-18T00:31:56.5794522Z .................................................................................................... 8400/9800
2020-03-18T00:32:03.1540404Z .................................................................................................... 8500/9800
2020-03-18T00:32:14.4584868Z .................................................................................................... 8600/9800
2020-03-18T00:32:19.8383542Z .................................................................................................... 8700/9800
---
2020-03-18T00:33:54.3771181Z normalized stderr:
2020-03-18T00:33:54.3771562Z warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3772303Z   --> $DIR/vec-fixed-length.rs:12:31
2020-03-18T00:33:54.3772671Z    |
2020-03-18T00:33:54.3773107Z LL |     assert_eq!(size_of::<[u8; (1 << 32)]>(), (1 << 32));
2020-03-18T00:33:54.3774098Z    |
2020-03-18T00:33:54.3774600Z    = note: `#[warn(unused_parens)]` on by default
2020-03-18T00:33:54.3775039Z 
2020-03-18T00:33:54.3775254Z 
2020-03-18T00:33:54.3775254Z 
2020-03-18T00:33:54.3775478Z 
2020-03-18T00:33:54.3775917Z 
2020-03-18T00:33:54.3776221Z The actual stderr differed from the expected stderr.
2020-03-18T00:33:54.3777100Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vec-fixed-length/vec-fixed-length.stderr
2020-03-18T00:33:54.3778112Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T00:33:54.3779546Z To only update this specific test, also pass `--test-args array-slice-vec/vec-fixed-length.rs`
2020-03-18T00:33:54.3782065Z error: 1 errors occurred comparing output.
2020-03-18T00:33:54.3782319Z status: exit code: 0
2020-03-18T00:33:54.3782319Z status: exit code: 0
2020-03-18T00:33:54.3784287Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/vec-fixed-length.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vec-fixed-length/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vec-fixed-length/auxiliary"
2020-03-18T00:33:54.3785850Z ------------------------------------------
2020-03-18T00:33:54.3786025Z 
2020-03-18T00:33:54.3786399Z ------------------------------------------
2020-03-18T00:33:54.3786600Z stderr:
2020-03-18T00:33:54.3786600Z stderr:
2020-03-18T00:33:54.3786967Z ------------------------------------------
2020-03-18T00:33:54.3787277Z warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3787832Z   --> /checkout/src/test/ui/array-slice-vec/vec-fixed-length.rs:12:31
2020-03-18T00:33:54.3788188Z    |
2020-03-18T00:33:54.3788429Z LL |     assert_eq!(size_of::<[u8; (1 << 32)]>(), (1 << 32));
2020-03-18T00:33:54.3789042Z    |
2020-03-18T00:33:54.3789253Z    = note: `#[warn(unused_parens)]` on by default
2020-03-18T00:33:54.3789427Z 
2020-03-18T00:33:54.3789521Z 
---
2020-03-18T00:33:54.3791438Z 8 
2020-03-18T00:33:54.3791664Z + warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3792211Z +   --> $DIR/condition-in-trait-const-arg.rs:8:18
2020-03-18T00:33:54.3792395Z +    |
2020-03-18T00:33:54.3792577Z + LL | impl IsZeroTrait<{0u8 == 0u8}> for () {}
2020-03-18T00:33:54.3793264Z +    |
2020-03-18T00:33:54.3793481Z +    = note: `#[warn(unused_parens)]` on by default
2020-03-18T00:33:54.3793679Z + 
2020-03-18T00:33:54.3793930Z 9 
2020-03-18T00:33:54.3793930Z 9 
2020-03-18T00:33:54.3794027Z 
2020-03-18T00:33:54.3794131Z 
2020-03-18T00:33:54.3794321Z The actual stderr differed from the expected stderr.
2020-03-18T00:33:54.3795094Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/condition-in-trait-const-arg/condition-in-trait-const-arg.stderr
2020-03-18T00:33:54.3795819Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T00:33:54.3796335Z To only update this specific test, also pass `--test-args const-generics/condition-in-trait-const-arg.rs`
2020-03-18T00:33:54.3796726Z error: 1 errors occurred comparing output.
2020-03-18T00:33:54.3796918Z status: exit code: 0
2020-03-18T00:33:54.3796918Z status: exit code: 0
2020-03-18T00:33:54.3798501Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/condition-in-trait-const-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/condition-in-trait-const-arg/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/condition-in-trait-const-arg/auxiliary"
2020-03-18T00:33:54.3799797Z ------------------------------------------
2020-03-18T00:33:54.3799951Z 
2020-03-18T00:33:54.3800241Z ------------------------------------------
2020-03-18T00:33:54.3800402Z stderr:
---
2020-03-18T00:33:54.3802556Z 
2020-03-18T00:33:54.3802734Z warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3803214Z   --> /checkout/src/test/ui/const-generics/condition-in-trait-const-arg.rs:8:18
2020-03-18T00:33:54.3803435Z    |
2020-03-18T00:33:54.3803598Z LL | impl IsZeroTrait<{0u8 == 0u8}> for () {}
2020-03-18T00:33:54.3804074Z    |
2020-03-18T00:33:54.3804433Z    = note: `#[warn(unused_parens)]` on by default
2020-03-18T00:33:54.3804621Z 
2020-03-18T00:33:54.3804711Z 
---
2020-03-18T00:33:54.3806626Z 8 
2020-03-18T00:33:54.3806840Z + warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3807308Z +   --> $DIR/issue-62579-no-match.rs:14:11
2020-03-18T00:33:54.3807507Z +    |
2020-03-18T00:33:54.3807681Z + LL |     foo::<{NoMatch}>();
2020-03-18T00:33:54.3808184Z +    |
2020-03-18T00:33:54.3808390Z +    = note: `#[warn(unused_parens)]` on by default
2020-03-18T00:33:54.3808600Z + 
2020-03-18T00:33:54.3808712Z 9 
2020-03-18T00:33:54.3808712Z 9 
2020-03-18T00:33:54.3808804Z 
2020-03-18T00:33:54.3808893Z 
2020-03-18T00:33:54.3809095Z The actual stderr differed from the expected stderr.
2020-03-18T00:33:54.3810651Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62579-no-match/issue-62579-no-match.stderr
2020-03-18T00:33:54.3812196Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T00:33:54.3813003Z To only update this specific test, also pass `--test-args const-generics/issues/issue-62579-no-match.rs`
2020-03-18T00:33:54.3813505Z error: 1 errors occurred comparing output.
2020-03-18T00:33:54.3813767Z status: exit code: 0
2020-03-18T00:33:54.3813767Z status: exit code: 0
2020-03-18T00:33:54.3815823Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-62579-no-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62579-no-match/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62579-no-match/auxiliary"
2020-03-18T00:33:54.3817517Z ------------------------------------------
2020-03-18T00:33:54.3817903Z 
2020-03-18T00:33:54.3818307Z ------------------------------------------
2020-03-18T00:33:54.3818514Z stderr:
---
2020-03-18T00:33:54.3821271Z 
2020-03-18T00:33:54.3821503Z warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3822110Z   --> /checkout/src/test/ui/const-generics/issues/issue-62579-no-match.rs:14:11
2020-03-18T00:33:54.3822410Z    |
2020-03-18T00:33:54.3822599Z LL |     foo::<{NoMatch}>();
2020-03-18T00:33:54.3823150Z    |
2020-03-18T00:33:54.3823376Z    = note: `#[warn(unused_parens)]` on by default
2020-03-18T00:33:54.3823573Z 
2020-03-18T00:33:54.3823674Z 
---
2020-03-18T00:33:54.3825770Z 8 
2020-03-18T00:33:54.3825994Z + warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3826475Z +   --> $DIR/raw-ptr-const-param-deref.rs:18:24
2020-03-18T00:33:54.3826705Z +    |
2020-03-18T00:33:54.3826949Z + LL |     assert_eq!(Const::<{&A as *const _}>::get(), 3)
2020-03-18T00:33:54.3827616Z +    |
2020-03-18T00:33:54.3827831Z +    = note: `#[warn(unused_parens)]` on by default
2020-03-18T00:33:54.3828035Z + 
2020-03-18T00:33:54.3828163Z 9 
2020-03-18T00:33:54.3828163Z 9 
2020-03-18T00:33:54.3828260Z 
2020-03-18T00:33:54.3828353Z 
2020-03-18T00:33:54.3828551Z The actual stderr differed from the expected stderr.
2020-03-18T00:33:54.3829271Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/raw-ptr-const-param-deref/raw-ptr-const-param-deref.stderr
2020-03-18T00:33:54.3829924Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T00:33:54.3830528Z To only update this specific test, also pass `--test-args const-generics/raw-ptr-const-param-deref.rs`
2020-03-18T00:33:54.3831068Z error: 1 errors occurred comparing output.
2020-03-18T00:33:54.3831297Z status: exit code: 0
2020-03-18T00:33:54.3831297Z status: exit code: 0
2020-03-18T00:33:54.3833251Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/raw-ptr-const-param-deref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/raw-ptr-const-param-deref/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/raw-ptr-const-param-deref/auxiliary"
2020-03-18T00:33:54.3834807Z ------------------------------------------
2020-03-18T00:33:54.3834970Z 
2020-03-18T00:33:54.3835302Z ------------------------------------------
2020-03-18T00:33:54.3835511Z stderr:
---
2020-03-18T00:33:54.3838217Z 
2020-03-18T00:33:54.3838405Z warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3838884Z   --> /checkout/src/test/ui/const-generics/raw-ptr-const-param-deref.rs:18:24
2020-03-18T00:33:54.3839113Z    |
2020-03-18T00:33:54.3839332Z LL |     assert_eq!(Const::<{&A as *const _}>::get(), 3)
2020-03-18T00:33:54.3840147Z    |
2020-03-18T00:33:54.3840367Z    = note: `#[warn(unused_parens)]` on by default
2020-03-18T00:33:54.3840552Z 
2020-03-18T00:33:54.3840643Z 
---
2020-03-18T00:33:54.3842579Z 8 
2020-03-18T00:33:54.3842802Z + warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3843255Z +   --> $DIR/slice-const-param.rs:18:38
2020-03-18T00:33:54.3843465Z +    |
2020-03-18T00:33:54.3843748Z + LL |     assert_eq!(function_with_bytes::<{&[0x41, 0x41, 0x41, 0x41]}>(), b"AAAA");
2020-03-18T00:33:54.3844557Z +    |
2020-03-18T00:33:54.3844770Z +    = note: `#[warn(unused_parens)]` on by default
2020-03-18T00:33:54.3844976Z + 
2020-03-18T00:33:54.3845092Z 9 
2020-03-18T00:33:54.3845092Z 9 
2020-03-18T00:33:54.3845202Z 
2020-03-18T00:33:54.3845294Z 
2020-03-18T00:33:54.3845493Z The actual stderr differed from the expected stderr.
2020-03-18T00:33:54.3846158Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/slice-const-param/slice-const-param.stderr
2020-03-18T00:33:54.3846800Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T00:33:54.3847594Z To only update this specific test, also pass `--test-args const-generics/slice-const-param.rs`
2020-03-18T00:33:54.3848086Z error: 1 errors occurred comparing output.
2020-03-18T00:33:54.3848333Z status: exit code: 0
2020-03-18T00:33:54.3848333Z status: exit code: 0
2020-03-18T00:33:54.3850357Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/slice-const-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/slice-const-param/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/slice-const-param/auxiliary"
2020-03-18T00:33:54.3852040Z ------------------------------------------
2020-03-18T00:33:54.3852217Z 
2020-03-18T00:33:54.3852576Z ------------------------------------------
2020-03-18T00:33:54.3852781Z stderr:
---
2020-03-18T00:33:54.3855494Z 
2020-03-18T00:33:54.3855737Z warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3856307Z   --> /checkout/src/test/ui/const-generics/slice-const-param.rs:18:38
2020-03-18T00:33:54.3856572Z    |
2020-03-18T00:33:54.3856883Z LL |     assert_eq!(function_with_bytes::<{&[0x41, 0x41, 0x41, 0x41]}>(), b"AAAA");
2020-03-18T00:33:54.3861914Z    |
2020-03-18T00:33:54.3862182Z    = note: `#[warn(unused_parens)]` on by default
2020-03-18T00:33:54.3862384Z 
2020-03-18T00:33:54.3862485Z 
2020-03-18T00:33:54.3862485Z 
2020-03-18T00:33:54.3863088Z ------------------------------------------
2020-03-18T00:33:54.3863267Z 
2020-03-18T00:33:54.3863384Z 
2020-03-18T00:33:54.3863803Z ---- [ui] ui/const-generics/unused_parens.rs stdout ----
2020-03-18T00:33:54.3864017Z 
2020-03-18T00:33:54.3864422Z error: test compilation failed although it shouldn't!
2020-03-18T00:33:54.3864711Z status: exit code: 1
2020-03-18T00:33:54.3866759Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/unused_parens.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/unused_parens" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/unused_parens/auxiliary"
2020-03-18T00:33:54.3868498Z ------------------------------------------
2020-03-18T00:33:54.3868666Z 
2020-03-18T00:33:54.3869046Z ------------------------------------------
2020-03-18T00:33:54.3869251Z stderr:
2020-03-18T00:33:54.3869251Z stderr:
2020-03-18T00:33:54.3871977Z ------------------------------------------
2020-03-18T00:33:54.3872487Z error: expected one of `!`, `(`, `+`, `,`, `::`, `<`, or `>`, found `=`
2020-03-18T00:33:54.3873326Z    |
2020-03-18T00:33:54.3873326Z    |
2020-03-18T00:33:54.3873538Z LL | struct A<const N: usize = { 7 }> {
2020-03-18T00:33:54.3873834Z    |                         ^ expected one of 7 possible tokens
2020-03-18T00:33:54.3874223Z error: aborting due to previous error
2020-03-18T00:33:54.3874383Z 
2020-03-18T00:33:54.3874476Z 
2020-03-18T00:33:54.3874810Z ------------------------------------------
2020-03-18T00:33:54.3874810Z ------------------------------------------
2020-03-18T00:33:54.3874972Z 
2020-03-18T00:33:54.3875080Z 
2020-03-18T00:33:54.3875568Z ---- [ui] ui/issues/issue-23898.rs stdout ----
2020-03-18T00:33:54.3875795Z normalized stderr:
2020-03-18T00:33:54.3876062Z warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3876564Z   --> $DIR/issue-23898.rs:9:22
2020-03-18T00:33:54.3876742Z    |
2020-03-18T00:33:54.3876991Z LL |     [State::ST_NULL; (State::ST_WHITESPACE as usize)];
2020-03-18T00:33:54.3877683Z    |
2020-03-18T00:33:54.3877893Z    = note: `#[warn(unused_parens)]` on by default
2020-03-18T00:33:54.3878090Z 
2020-03-18T00:33:54.3878182Z 
2020-03-18T00:33:54.3878182Z 
2020-03-18T00:33:54.3878274Z 
2020-03-18T00:33:54.3878366Z 
2020-03-18T00:33:54.3878578Z The actual stderr differed from the expected stderr.
2020-03-18T00:33:54.3879200Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23898/issue-23898.stderr
2020-03-18T00:33:54.3879797Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T00:33:54.3880368Z To only update this specific test, also pass `--test-args issues/issue-23898.rs`
2020-03-18T00:33:54.3880791Z error: 1 errors occurred comparing output.
2020-03-18T00:33:54.3881019Z status: exit code: 0
2020-03-18T00:33:54.3881019Z status: exit code: 0
2020-03-18T00:33:54.3882754Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23898.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23898/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23898/auxiliary"
2020-03-18T00:33:54.3884198Z ------------------------------------------
2020-03-18T00:33:54.3884362Z 
2020-03-18T00:33:54.3884700Z ------------------------------------------
2020-03-18T00:33:54.3884907Z stderr:
2020-03-18T00:33:54.3884907Z stderr:
2020-03-18T00:33:54.3885406Z ------------------------------------------
2020-03-18T00:33:54.3886617Z warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3887218Z   --> /checkout/src/test/ui/issues/issue-23898.rs:9:22
2020-03-18T00:33:54.3887442Z    |
2020-03-18T00:33:54.3888332Z LL |     [State::ST_NULL; (State::ST_WHITESPACE as usize)];
2020-03-18T00:33:54.3889261Z    |
2020-03-18T00:33:54.3889472Z    = note: `#[warn(unused_parens)]` on by default
2020-03-18T00:33:54.3889675Z 
2020-03-18T00:33:54.3889768Z 
2020-03-18T00:33:54.3889768Z 
2020-03-18T00:33:54.3890164Z ------------------------------------------
2020-03-18T00:33:54.3890327Z 
2020-03-18T00:33:54.3890436Z 
2020-03-18T00:33:54.3890798Z ---- [ui] ui/lint/unused_parens_brace.rs stdout ----
2020-03-18T00:33:54.3891026Z diff of stderr:
2020-03-18T00:33:54.3896861Z 
2020-03-18T00:33:54.3897075Z 22 LL |     if let 7 = { 7 } {
2020-03-18T00:33:54.3897700Z 24 
2020-03-18T00:33:54.3897949Z + warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3904053Z +   --> $DIR/unused_parens_brace.rs:26:17
2020-03-18T00:33:54.3904283Z +    |
2020-03-18T00:33:54.3904283Z +    |
2020-03-18T00:33:54.3904494Z + LL |     let _: [u8; { 7 }];
2020-03-18T00:33:54.3905005Z + 
2020-03-18T00:33:54.3905122Z 25 
2020-03-18T00:33:54.3905239Z 
2020-03-18T00:33:54.3905334Z 
2020-03-18T00:33:54.3905334Z 
2020-03-18T00:33:54.3905536Z The actual stderr differed from the expected stderr.
2020-03-18T00:33:54.3906220Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_brace/unused_parens_brace.stderr
2020-03-18T00:33:54.3906855Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T00:33:54.3907574Z To only update this specific test, also pass `--test-args lint/unused_parens_brace.rs`
2020-03-18T00:33:54.3908100Z error: 1 errors occurred comparing output.
2020-03-18T00:33:54.3908330Z status: exit code: 0
2020-03-18T00:33:54.3908330Z status: exit code: 0
2020-03-18T00:33:54.3910528Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused_parens_brace.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_brace" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_brace/auxiliary"
2020-03-18T00:33:54.3912318Z ------------------------------------------
2020-03-18T00:33:54.3912499Z 
2020-03-18T00:33:54.3912863Z ------------------------------------------
2020-03-18T00:33:54.3913069Z stderr:
---
2020-03-18T00:33:54.3918462Z 
2020-03-18T00:33:54.3918713Z warning: unnecessary parentheses around `let` head expression
2020-03-18T00:33:54.3919266Z   --> /checkout/src/test/ui/lint/unused_parens_brace.rs:22:16
2020-03-18T00:33:54.3919517Z    |
2020-03-18T00:33:54.3919707Z LL |     if let 7 = { 7 } {
2020-03-18T00:33:54.3920207Z 
2020-03-18T00:33:54.3920441Z warning: unnecessary parentheses around constant expression
2020-03-18T00:33:54.3920999Z   --> /checkout/src/test/ui/lint/unused_parens_brace.rs:26:17
2020-03-18T00:33:54.3921249Z    |
2020-03-18T00:33:54.3921249Z    |
2020-03-18T00:33:54.3921435Z LL |     let _: [u8; { 7 }];
2020-03-18T00:33:54.3921971Z 
2020-03-18T00:33:54.3922072Z 
2020-03-18T00:33:54.3922434Z ------------------------------------------
2020-03-18T00:33:54.3922622Z 
---
2020-03-18T00:33:54.3927230Z test result: FAILED. 9735 passed; 8 failed; 57 ignored; 0 measured; 0 filtered out
2020-03-18T00:33:54.3927575Z 
2020-03-18T00:33:54.3929361Z 
2020-03-18T00:33:54.3929541Z 
2020-03-18T00:33:54.3933498Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-18T00:33:54.3936781Z 
2020-03-18T00:33:54.3936886Z 
2020-03-18T00:33:54.3937575Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-18T00:33:54.3938239Z Build completed unsuccessfully in 0:59:20
2020-03-18T00:33:54.3938239Z Build completed unsuccessfully in 0:59:20
2020-03-18T00:33:54.3938982Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-18T00:33:54.3939533Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-18T00:33:54.3939942Z == clock drift check ==
2020-03-18T00:33:54.3940649Z   local time: Wed Mar 18 00:33:54 UTC 2020
2020-03-18T00:33:54.6852378Z   network time: Wed, 18 Mar 2020 00:33:54 GMT
2020-03-18T00:33:54.6852831Z == end clock drift check ==
2020-03-18T00:33:55.2404499Z 
2020-03-18T00:33:55.2465159Z ##[error]Bash exited with code '1'.
2020-03-18T00:33:55.2478939Z ##[section]Finishing: Run build
2020-03-18T00:33:55.2539390Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-18T00:33:55.2544416Z Task         : Get sources
2020-03-18T00:33:55.2544759Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T00:33:55.2545073Z Version      : 1.0.0
2020-03-18T00:33:55.2545307Z Author       : Microsoft
2020-03-18T00:33:55.2545307Z Author       : Microsoft
2020-03-18T00:33:55.2545661Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T00:33:55.2546061Z ==============================================================================
2020-03-18T00:33:55.5311361Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T00:33:55.5318557Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-18T00:33:55.5394327Z Cleaning up task key
2020-03-18T00:33:55.5395276Z Start cleaning up orphan processes.
2020-03-18T00:33:55.5656235Z Terminate orphan process: pid (3649) (python)
2020-03-18T00:33:55.5702475Z ##[section]Finishing: Finalize Job
