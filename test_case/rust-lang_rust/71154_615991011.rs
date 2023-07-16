plain
2020-04-18T23:26:44.5735618Z ========================== Starting Command Output ===========================
2020-04-18T23:26:44.5738579Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/43e4471b-e6db-4e3d-8548-428934487af6.sh
2020-04-18T23:26:44.5738946Z 
2020-04-18T23:26:44.5744092Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T23:26:44.5760524Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-18T23:26:44.5763688Z Task         : Get sources
2020-04-18T23:26:44.5763961Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T23:26:44.5764231Z Version      : 1.0.0
2020-04-18T23:26:44.5764413Z Author       : Microsoft
---
2020-04-18T23:26:45.5643992Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T23:26:45.5649312Z ##[command]git config gc.auto 0
2020-04-18T23:26:45.5652789Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T23:26:45.5656029Z ##[command]git config --get-all http.proxy
2020-04-18T23:26:45.5661394Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71154/merge:refs/remotes/pull/71154/merge
---
2020-04-18T23:28:51.4329131Z  ---> 318032b5f0e2
2020-04-18T23:28:51.4330145Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-18T23:28:51.4331768Z  ---> Using cache
2020-04-18T23:28:51.4332406Z  ---> d44a858fd1ce
2020-04-18T23:28:51.4333585Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-18T23:28:51.4335810Z  ---> 58b910f50f5a
2020-04-18T23:28:51.4336136Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-18T23:28:51.4337621Z  ---> Using cache
2020-04-18T23:28:51.4338257Z  ---> ee7702aadba1
---
2020-04-18T23:28:51.4712325Z Looks like docker image is the same as before, not uploading
2020-04-18T23:28:58.3892702Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T23:28:58.4127402Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T23:28:58.4157485Z == clock drift check ==
2020-04-18T23:28:58.4163445Z   local time: Sat Apr 18 23:28:58 UTC 2020
2020-04-18T23:28:58.7333235Z   network time: Sat, 18 Apr 2020 23:28:58 GMT
2020-04-18T23:28:58.7360996Z Starting sccache server...
2020-04-18T23:28:58.8150466Z configure: processing command line
2020-04-18T23:28:58.8151000Z configure: 
2020-04-18T23:28:58.8152286Z configure: rust.dist-src        := False
---
2020-04-18T23:33:46.5924277Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T23:33:47.9823555Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T23:33:49.5227286Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T23:33:50.4525934Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T23:33:59.0906850Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T23:34:01.0893401Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T23:34:05.2702214Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T23:34:09.0994117Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T23:34:18.5654290Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T23:54:49.6521873Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T23:54:51.1451781Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T23:54:52.5777267Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T23:54:52.8448775Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T23:55:02.1191700Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T23:55:04.1791647Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T23:55:08.3837163Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T23:55:12.2997410Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T23:55:21.7505308Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T00:16:22.6280791Z ......................................................................F............................. 1700/9913
2020-04-19T00:16:26.6707866Z .................................................................................................... 1800/9913
2020-04-19T00:16:34.3139792Z .................................................................................................... 1900/9913
2020-04-19T00:16:41.6197289Z ....i............................................................................................... 2000/9913
2020-04-19T00:16:47.3730819Z ..............................................................................................iiiii. 2100/9913
2020-04-19T00:17:05.2976876Z .................................................................................................... 2300/9913
2020-04-19T00:17:07.3630258Z .................................................................................................... 2400/9913
2020-04-19T00:17:09.4621491Z .................................................................................................... 2500/9913
2020-04-19T00:17:14.7848757Z .................................................................................................... 2600/9913
---
2020-04-19T00:19:53.3070186Z .................................................................................................... 5100/9913
2020-04-19T00:19:59.8166222Z .................................................................................................... 5200/9913
2020-04-19T00:20:04.3254492Z ................i................................................................................... 5300/9913
2020-04-19T00:20:13.5546735Z ......i............................................................................................. 5400/9913
2020-04-19T00:20:18.7350084Z ......ii.ii........i...i............................................................................ 5500/9913
2020-04-19T00:20:26.3283254Z .....................................................i.............................................. 5700/9913
2020-04-19T00:20:34.8685217Z .....................................................................................ii............. 5800/9913
2020-04-19T00:20:41.8316824Z ........................i........................................................................... 5900/9913
2020-04-19T00:20:47.3237366Z .................................................................................................... 6000/9913
2020-04-19T00:20:47.3237366Z .................................................................................................... 6000/9913
2020-04-19T00:20:57.9804658Z .................................................................................................... 6100/9913
2020-04-19T00:21:07.1930570Z ..................ii...i..ii...........i............................................................ 6200/9913
2020-04-19T00:21:22.1259352Z .................................................................................................... 6400/9913
2020-04-19T00:21:25.3316068Z .................................................................................................... 6500/9913
2020-04-19T00:21:25.3316068Z .................................................................................................... 6500/9913
2020-04-19T00:21:33.8476899Z ................................................i..ii............................................... 6600/9913
2020-04-19T00:21:54.9741660Z .................................................................................................... 6800/9913
2020-04-19T00:21:57.1119618Z .................................................i.................................................. 6900/9913
2020-04-19T00:21:59.1764725Z .................................................................................................... 7000/9913
2020-04-19T00:22:01.2710887Z .........................................................................................i.......... 7100/9913
---
2020-04-19T00:23:30.2427346Z .................................................................................................... 7900/9913
2020-04-19T00:23:36.3309354Z .................................................................................................... 8000/9913
2020-04-19T00:23:41.5879863Z .......................................................i............................................ 8100/9913
2020-04-19T00:23:51.4494505Z .................................................................................................... 8200/9913
2020-04-19T00:23:56.2719205Z ....i.iiiiiiiiii.i.................................................................................. 8300/9913
2020-04-19T00:24:08.7402247Z .................................................................................................... 8500/9913
2020-04-19T00:24:16.1969521Z .................................................................................................... 8600/9913
2020-04-19T00:24:28.7919787Z .................................................................................................... 8700/9913
2020-04-19T00:24:34.7004035Z .................................................................................................... 8800/9913
---
2020-04-19T00:26:16.3912765Z 
2020-04-19T00:26:16.3914336Z ---- [ui] ui/associated-const/defaults-cyclic-fail.rs stdout ----
2020-04-19T00:26:16.3914843Z diff of stderr:
2020-04-19T00:26:16.3915183Z 
2020-04-19T00:26:16.3915605Z 1 error[E0391]: cycle detected when normalizing `<() as Tr>::A`
2020-04-19T00:26:16.3916053Z 2    |
2020-04-19T00:26:16.3916755Z - note: ...which requires const-evaluating + checking `Tr::A`...
2020-04-19T00:26:16.3917551Z + note: ...which requires const-evaluating + checking `Tr::A`[None]...
2020-04-19T00:26:16.3918686Z 5    |
2020-04-19T00:26:16.3919062Z 6 LL |     const A: u8 = Self::B;
2020-04-19T00:26:16.3919387Z 
2020-04-19T00:26:16.3919745Z 7    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3919745Z 7    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3920675Z - note: ...which requires const-evaluating + checking `Tr::A`...
2020-04-19T00:26:16.3921462Z + note: ...which requires const-evaluating + checking `Tr::A`[None]...
2020-04-19T00:26:16.3922547Z 10    |
2020-04-19T00:26:16.3922919Z 11 LL |     const A: u8 = Self::B;
2020-04-19T00:26:16.3923241Z 
2020-04-19T00:26:16.3923675Z 12    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3923675Z 12    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3924980Z - note: ...which requires const-evaluating `Tr::A`...
2020-04-19T00:26:16.3925747Z + note: ...which requires const-evaluating `Tr::A`[None]...
2020-04-19T00:26:16.3926706Z 15    |
2020-04-19T00:26:16.3927023Z 16 LL |     const A: u8 = Self::B;
2020-04-19T00:26:16.3927288Z 
2020-04-19T00:26:16.3927564Z 17    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3927564Z 17    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3927984Z 18    = note: ...which requires normalizing `<() as Tr>::B`...
2020-04-19T00:26:16.3928660Z - note: ...which requires const-evaluating + checking `Tr::B`...
2020-04-19T00:26:16.3929386Z + note: ...which requires const-evaluating + checking `Tr::B`[None]...
2020-04-19T00:26:16.3930332Z 21    |
2020-04-19T00:26:16.3930647Z 22 LL |     const B: u8 = Self::A;
2020-04-19T00:26:16.3930910Z 
2020-04-19T00:26:16.3931188Z 23    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3931188Z 23    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3931799Z - note: ...which requires const-evaluating + checking `Tr::B`...
2020-04-19T00:26:16.3932495Z + note: ...which requires const-evaluating + checking `Tr::B`[None]...
2020-04-19T00:26:16.3933448Z 26    |
2020-04-19T00:26:16.3933743Z 27 LL |     const B: u8 = Self::A;
2020-04-19T00:26:16.3934010Z 
2020-04-19T00:26:16.3934295Z 28    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3934295Z 28    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3934890Z - note: ...which requires const-evaluating `Tr::B`...
2020-04-19T00:26:16.3935538Z + note: ...which requires const-evaluating `Tr::B`[None]...
2020-04-19T00:26:16.3936502Z 31    |
2020-04-19T00:26:16.3936816Z 32 LL |     const B: u8 = Self::A;
2020-04-19T00:26:16.3937080Z 
2020-04-19T00:26:16.3937349Z 33    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3937349Z 33    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3937835Z 34    = note: ...which again requires normalizing `<() as Tr>::A`, completing the cycle
2020-04-19T00:26:16.3938520Z - note: cycle used when const-evaluating `main`
2020-04-19T00:26:16.3939134Z + note: cycle used when const-evaluating `main`[None]
2020-04-19T00:26:16.3940061Z 37    |
2020-04-19T00:26:16.3940317Z 38 LL | fn main() {
2020-04-19T00:26:16.3940562Z 
2020-04-19T00:26:16.3940758Z 
2020-04-19T00:26:16.3940758Z 
2020-04-19T00:26:16.3941060Z The actual stderr differed from the expected stderr.
2020-04-19T00:26:16.3941874Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/defaults-cyclic-fail/defaults-cyclic-fail.stderr
2020-04-19T00:26:16.3942643Z To update references, rerun the tests and pass the `--bless` flag
2020-04-19T00:26:16.3943374Z To only update this specific test, also pass `--test-args associated-const/defaults-cyclic-fail.rs`
2020-04-19T00:26:16.3944059Z error: 1 errors occurred comparing output.
2020-04-19T00:26:16.3944413Z status: exit code: 1
2020-04-19T00:26:16.3944413Z status: exit code: 1
2020-04-19T00:26:16.3946350Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/defaults-cyclic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/defaults-cyclic-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/defaults-cyclic-fail/auxiliary"
2020-04-19T00:26:16.3952569Z ------------------------------------------
2020-04-19T00:26:16.3955361Z 
2020-04-19T00:26:16.3956217Z ------------------------------------------
2020-04-19T00:26:16.3958716Z stderr:
2020-04-19T00:26:16.3958716Z stderr:
2020-04-19T00:26:16.3959835Z ------------------------------------------
2020-04-19T00:26:16.3960556Z error[E0391]: cycle detected when normalizing `<() as Tr>::A`
2020-04-19T00:26:16.3960975Z    |
2020-04-19T00:26:16.3961655Z note: ...which requires const-evaluating + checking `Tr::A`[None]...
2020-04-19T00:26:16.3963136Z    |
2020-04-19T00:26:16.3963459Z LL |     const A: u8 = Self::B;
2020-04-19T00:26:16.3963845Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3963845Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3964558Z note: ...which requires const-evaluating + checking `Tr::A`[None]...
2020-04-19T00:26:16.3965757Z    |
2020-04-19T00:26:16.3966052Z LL |     const A: u8 = Self::B;
2020-04-19T00:26:16.3966414Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3966414Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3977201Z note: ...which requires const-evaluating `Tr::A`[None]...
2020-04-19T00:26:16.3978292Z    |
2020-04-19T00:26:16.3978718Z LL |     const A: u8 = Self::B;
2020-04-19T00:26:16.3978962Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3978962Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3979385Z    = note: ...which requires normalizing `<() as Tr>::B`...
2020-04-19T00:26:16.3979971Z note: ...which requires const-evaluating + checking `Tr::B`[None]...
2020-04-19T00:26:16.3980777Z    |
2020-04-19T00:26:16.3980953Z LL |     const B: u8 = Self::A;
2020-04-19T00:26:16.3981182Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3981182Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3981654Z note: ...which requires const-evaluating + checking `Tr::B`[None]...
2020-04-19T00:26:16.3982470Z    |
2020-04-19T00:26:16.3982645Z LL |     const B: u8 = Self::A;
2020-04-19T00:26:16.3982856Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3982856Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3983312Z note: ...which requires const-evaluating `Tr::B`[None]...
2020-04-19T00:26:16.3984085Z    |
2020-04-19T00:26:16.3984275Z LL |     const B: u8 = Self::A;
2020-04-19T00:26:16.3984491Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3984491Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3984799Z    = note: ...which again requires normalizing `<() as Tr>::A`, completing the cycle
2020-04-19T00:26:16.3985341Z note: cycle used when const-evaluating `main`[None]
2020-04-19T00:26:16.3986092Z    |
2020-04-19T00:26:16.3986248Z LL | fn main() {
2020-04-19T00:26:16.3986413Z    | ^^^^^^^^^
2020-04-19T00:26:16.3986529Z 
---
2020-04-19T00:26:16.3988067Z 
2020-04-19T00:26:16.3988435Z ---- [ui] ui/consts/const-size_of-cycle.rs stdout ----
2020-04-19T00:26:16.3988658Z diff of stderr:
2020-04-19T00:26:16.3988792Z 
2020-04-19T00:26:16.3989302Z - error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}#0`
2020-04-19T00:26:16.3989987Z + error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}#0`[None]
2020-04-19T00:26:16.3990740Z 3    |
2020-04-19T00:26:16.3990740Z 3    |
2020-04-19T00:26:16.3990962Z 4 LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2020-04-19T00:26:16.3991570Z 5    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.3991758Z 6    |
2020-04-19T00:26:16.3991758Z 6    |
2020-04-19T00:26:16.3992263Z - note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}#0`...
2020-04-19T00:26:16.3998367Z + note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}#0`[None]...
2020-04-19T00:26:16.3999561Z 9    |
2020-04-19T00:26:16.3999561Z 9    |
2020-04-19T00:26:16.3999805Z 10 LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2020-04-19T00:26:16.4000185Z 11    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4000185Z 11    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4000735Z - note: ...which requires const-evaluating `Foo::bytes::{{constant}}#0`...
2020-04-19T00:26:16.4003881Z + note: ...which requires const-evaluating `Foo::bytes::{{constant}}#0`[None]...
2020-04-19T00:26:16.4004611Z 14    |
2020-04-19T00:26:16.4004611Z 14    |
2020-04-19T00:26:16.4004846Z 15 LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2020-04-19T00:26:16.4005224Z 16    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4005735Z - note: ...which requires const-evaluating `std::mem::size_of`...
2020-04-19T00:26:16.4005735Z - note: ...which requires const-evaluating `std::mem::size_of`...
2020-04-19T00:26:16.4006292Z + note: ...which requires const-evaluating `std::mem::size_of`[None]...
2020-04-19T00:26:16.4006775Z 18   --> $SRC_DIR/libcore/mem/mod.rs:LL:COL
2020-04-19T00:26:16.4007181Z 20 LL |     intrinsics::size_of::<T>()
2020-04-19T00:26:16.4007343Z 
2020-04-19T00:26:16.4007508Z 21    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4007508Z 21    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4008053Z - note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
2020-04-19T00:26:16.4008692Z + note: ...which requires const-evaluating + checking `std::intrinsics::size_of`[None]...
2020-04-19T00:26:16.4009228Z 23   --> $SRC_DIR/libcore/intrinsics.rs:LL:COL
2020-04-19T00:26:16.4009773Z 25 LL |     pub fn size_of<T>() -> usize;
2020-04-19T00:26:16.4009930Z 
2020-04-19T00:26:16.4010116Z 26    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4010380Z 27    = note: ...which requires computing layout of `Foo`...
2020-04-19T00:26:16.4010380Z 27    = note: ...which requires computing layout of `Foo`...
2020-04-19T00:26:16.4010681Z 28    = note: ...which requires normalizing `[u8; _]`...
2020-04-19T00:26:16.4011338Z -    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}#0`, completing the cycle
2020-04-19T00:26:16.4012104Z +    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}#0`[None], completing the cycle
2020-04-19T00:26:16.4012503Z 30 note: cycle used when processing `Foo`
2020-04-19T00:26:16.4013134Z 32    |
2020-04-19T00:26:16.4013235Z 
2020-04-19T00:26:16.4013324Z 
2020-04-19T00:26:16.4013526Z The actual stderr differed from the expected stderr.
2020-04-19T00:26:16.4013526Z The actual stderr differed from the expected stderr.
2020-04-19T00:26:16.4014147Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
2020-04-19T00:26:16.4014731Z To update references, rerun the tests and pass the `--bless` flag
2020-04-19T00:26:16.4015286Z To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`
2020-04-19T00:26:16.4015680Z error: 1 errors occurred comparing output.
2020-04-19T00:26:16.4015914Z status: exit code: 1
2020-04-19T00:26:16.4015914Z status: exit code: 1
2020-04-19T00:26:16.4017659Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary"
2020-04-19T00:26:16.4019261Z ------------------------------------------
2020-04-19T00:26:16.4019416Z 
2020-04-19T00:26:16.4019773Z ------------------------------------------
2020-04-19T00:26:16.4019957Z stderr:
2020-04-19T00:26:16.4019957Z stderr:
2020-04-19T00:26:16.4020308Z ------------------------------------------
2020-04-19T00:26:16.4020961Z error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}#0`[None]
2020-04-19T00:26:16.4021783Z    |
2020-04-19T00:26:16.4021783Z    |
2020-04-19T00:26:16.4022008Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2020-04-19T00:26:16.4022459Z    |
2020-04-19T00:26:16.4022459Z    |
2020-04-19T00:26:16.4022963Z note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}#0`[None]...
2020-04-19T00:26:16.4023752Z    |
2020-04-19T00:26:16.4023752Z    |
2020-04-19T00:26:16.4023975Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2020-04-19T00:26:16.4024241Z    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4024760Z note: ...which requires const-evaluating `Foo::bytes::{{constant}}#0`[None]...
2020-04-19T00:26:16.4025538Z    |
2020-04-19T00:26:16.4025538Z    |
2020-04-19T00:26:16.4025746Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2020-04-19T00:26:16.4026027Z    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4026531Z note: ...which requires const-evaluating `std::mem::size_of`[None]...
2020-04-19T00:26:16.4027213Z    |
2020-04-19T00:26:16.4027396Z LL |     intrinsics::size_of::<T>()
2020-04-19T00:26:16.4027623Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4027623Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4028169Z note: ...which requires const-evaluating + checking `std::intrinsics::size_of`[None]...
2020-04-19T00:26:16.4028898Z    |
2020-04-19T00:26:16.4029231Z LL |     pub fn size_of<T>() -> usize;
2020-04-19T00:26:16.4029472Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4029726Z    = note: ...which requires computing layout of `Foo`...
2020-04-19T00:26:16.4029726Z    = note: ...which requires computing layout of `Foo`...
2020-04-19T00:26:16.4030012Z    = note: ...which requires normalizing `[u8; _]`...
2020-04-19T00:26:16.4030671Z    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}#0`[None], completing the cycle
2020-04-19T00:26:16.4031055Z note: cycle used when processing `Foo`
2020-04-19T00:26:16.4031749Z    |
2020-04-19T00:26:16.4031889Z LL | struct Foo {
2020-04-19T00:26:16.4032047Z    | ^^^^^^^^^^
2020-04-19T00:26:16.4032173Z 
---
2020-04-19T00:26:16.4039053Z 
2020-04-19T00:26:16.4039457Z ---- [ui] ui/infinite/infinite-recursion-const-fn.rs stdout ----
2020-04-19T00:26:16.4039692Z diff of stderr:
2020-04-19T00:26:16.4039815Z 
2020-04-19T00:26:16.4040206Z - error[E0391]: cycle detected when const-evaluating `a`
2020-04-19T00:26:16.4040711Z + error[E0391]: cycle detected when const-evaluating `a`[None]
2020-04-19T00:26:16.4041389Z 3    |
2020-04-19T00:26:16.4041389Z 3    |
2020-04-19T00:26:16.4041741Z 4 LL | const fn a() -> usize { b() }
2020-04-19T00:26:16.4042046Z 5    |                         ^^^
2020-04-19T00:26:16.4042211Z 6    |
2020-04-19T00:26:16.4042593Z - note: ...which requires const-evaluating `b`...
2020-04-19T00:26:16.4042593Z - note: ...which requires const-evaluating `b`...
2020-04-19T00:26:16.4043045Z + note: ...which requires const-evaluating `b`[None]...
2020-04-19T00:26:16.4043932Z 9    |
2020-04-19T00:26:16.4043932Z 9    |
2020-04-19T00:26:16.4044271Z 10 LL | const fn b() -> usize { a() }
2020-04-19T00:26:16.4044603Z 11    |                         ^^^
2020-04-19T00:26:16.4045158Z -    = note: ...which again requires const-evaluating `a`, completing the cycle
2020-04-19T00:26:16.4045158Z -    = note: ...which again requires const-evaluating `a`, completing the cycle
2020-04-19T00:26:16.4045703Z - note: cycle used when const-evaluating `ARR::{{constant}}#0`
2020-04-19T00:26:16.4046276Z +    = note: ...which again requires const-evaluating `a`[None], completing the cycle
2020-04-19T00:26:16.4046831Z + note: cycle used when const-evaluating `ARR::{{constant}}#0`[None]
2020-04-19T00:26:16.4047555Z 15    |
2020-04-19T00:26:16.4047555Z 15    |
2020-04-19T00:26:16.4047745Z 16 LL | const ARR: [i32; a()] = [5; 6];
2020-04-19T00:26:16.4047995Z 
2020-04-19T00:26:16.4048197Z The actual stderr differed from the expected stderr.
2020-04-19T00:26:16.4048866Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn/infinite-recursion-const-fn.stderr
2020-04-19T00:26:16.4048866Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn/infinite-recursion-const-fn.stderr
2020-04-19T00:26:16.4049810Z To update references, rerun the tests and pass the `--bless` flag
2020-04-19T00:26:16.4052455Z To only update this specific test, also pass `--test-args infinite/infinite-recursion-const-fn.rs`
2020-04-19T00:26:16.4052885Z error: 1 errors occurred comparing output.
2020-04-19T00:26:16.4053406Z status: exit code: 1
2020-04-19T00:26:16.4053406Z status: exit code: 1
2020-04-19T00:26:16.4055238Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn/auxiliary"
2020-04-19T00:26:16.4057438Z ------------------------------------------
2020-04-19T00:26:16.4057595Z 
2020-04-19T00:26:16.4057967Z ------------------------------------------
2020-04-19T00:26:16.4058153Z stderr:
2020-04-19T00:26:16.4058153Z stderr:
2020-04-19T00:26:16.4058511Z ------------------------------------------
2020-04-19T00:26:16.4058982Z error[E0391]: cycle detected when const-evaluating `a`[None]
2020-04-19T00:26:16.4059757Z    |
2020-04-19T00:26:16.4059757Z    |
2020-04-19T00:26:16.4060238Z LL | const fn a() -> usize { b() } //~ ERROR cycle detected when const-evaluating `a` [E0391]
2020-04-19T00:26:16.4060695Z    |
2020-04-19T00:26:16.4060695Z    |
2020-04-19T00:26:16.4061086Z note: ...which requires const-evaluating `b`[None]...
2020-04-19T00:26:16.4061841Z    |
2020-04-19T00:26:16.4061841Z    |
2020-04-19T00:26:16.4062182Z LL | const fn b() -> usize { a() }
2020-04-19T00:26:16.4062391Z    |                         ^^^
2020-04-19T00:26:16.4062887Z    = note: ...which again requires const-evaluating `a`[None], completing the cycle
2020-04-19T00:26:16.4063461Z note: cycle used when const-evaluating `ARR::{{constant}}#0`[None]
2020-04-19T00:26:16.4064247Z    |
2020-04-19T00:26:16.4064247Z    |
2020-04-19T00:26:16.4064441Z LL | const ARR: [i32; a()] = [5; 6];
2020-04-19T00:26:16.4064781Z 
2020-04-19T00:26:16.4064948Z error: aborting due to previous error
2020-04-19T00:26:16.4065116Z 
2020-04-19T00:26:16.4065555Z For more information about this error, try `rustc --explain E0391`.
2020-04-19T00:26:16.4065555Z For more information about this error, try `rustc --explain E0391`.
2020-04-19T00:26:16.4065898Z 
2020-04-19T00:26:16.4066250Z ------------------------------------------
2020-04-19T00:26:16.4066424Z 
2020-04-19T00:26:16.4066513Z 
2020-04-19T00:26:16.4066906Z ---- [ui] ui/recursion/recursive-static-definition.rs stdout ----
2020-04-19T00:26:16.4067145Z diff of stderr:
2020-04-19T00:26:16.4067284Z 
2020-04-19T00:26:16.4067675Z - error[E0391]: cycle detected when const-evaluating `FOO`
2020-04-19T00:26:16.4068246Z + error[E0391]: cycle detected when const-evaluating `FOO`[None]
2020-04-19T00:26:16.4068966Z 3    |
2020-04-19T00:26:16.4069140Z 4 LL | pub static FOO: u32 = FOO;
2020-04-19T00:26:16.4069286Z 
2020-04-19T00:26:16.4069458Z 5    |                       ^^^
2020-04-19T00:26:16.4069458Z 5    |                       ^^^
2020-04-19T00:26:16.4069829Z 6    |
2020-04-19T00:26:16.4070247Z - note: ...which requires const-evaluating `FOO`...
2020-04-19T00:26:16.4072624Z + note: ...which requires const-evaluating `FOO`[None]...
2020-04-19T00:26:16.4073666Z 9    |
2020-04-19T00:26:16.4073865Z 10 LL | pub static FOO: u32 = FOO;
2020-04-19T00:26:16.4074021Z 
2020-04-19T00:26:16.4074339Z 11    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4074339Z 11    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4075737Z -    = note: ...which again requires const-evaluating `FOO`, completing the cycle
2020-04-19T00:26:16.4076291Z - note: cycle used when const-evaluating + checking `FOO`
2020-04-19T00:26:16.4076839Z +    = note: ...which again requires const-evaluating `FOO`[None], completing the cycle
2020-04-19T00:26:16.4077371Z + note: cycle used when const-evaluating + checking `FOO`[None]
2020-04-19T00:26:16.4078071Z 15    |
2020-04-19T00:26:16.4078255Z 16 LL | pub static FOO: u32 = FOO;
2020-04-19T00:26:16.4078426Z 
2020-04-19T00:26:16.4078519Z 
2020-04-19T00:26:16.4078519Z 
2020-04-19T00:26:16.4086543Z The actual stderr differed from the expected stderr.
2020-04-19T00:26:16.4087457Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-static-definition/recursive-static-definition.stderr
2020-04-19T00:26:16.4088128Z To update references, rerun the tests and pass the `--bless` flag
2020-04-19T00:26:16.4088702Z To only update this specific test, also pass `--test-args recursion/recursive-static-definition.rs`
2020-04-19T00:26:16.4089138Z error: 1 errors occurred comparing output.
2020-04-19T00:26:16.4089354Z status: exit code: 1
2020-04-19T00:26:16.4089354Z status: exit code: 1
2020-04-19T00:26:16.4091208Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/recursive-static-definition.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-static-definition" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-static-definition/auxiliary"
2020-04-19T00:26:16.4092664Z ------------------------------------------
2020-04-19T00:26:16.4092821Z 
2020-04-19T00:26:16.4093162Z ------------------------------------------
2020-04-19T00:26:16.4093347Z stderr:
2020-04-19T00:26:16.4093347Z stderr:
2020-04-19T00:26:16.4093713Z ------------------------------------------
2020-04-19T00:26:16.4094172Z error[E0391]: cycle detected when const-evaluating `FOO`[None]
2020-04-19T00:26:16.4094964Z    |
2020-04-19T00:26:16.4095138Z LL | pub static FOO: u32 = FOO;
2020-04-19T00:26:16.4095349Z    |                       ^^^
2020-04-19T00:26:16.4095523Z    |
2020-04-19T00:26:16.4095523Z    |
2020-04-19T00:26:16.4095907Z note: ...which requires const-evaluating `FOO`[None]...
2020-04-19T00:26:16.4096875Z    |
2020-04-19T00:26:16.4097048Z LL | pub static FOO: u32 = FOO;
2020-04-19T00:26:16.4097260Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4097260Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4097786Z    = note: ...which again requires const-evaluating `FOO`[None], completing the cycle
2020-04-19T00:26:16.4098310Z note: cycle used when const-evaluating + checking `FOO`[None]
2020-04-19T00:26:16.4099160Z    |
2020-04-19T00:26:16.4099332Z LL | pub static FOO: u32 = FOO;
2020-04-19T00:26:16.4099542Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4099693Z 
---
2020-04-19T00:26:16.4101206Z 
2020-04-19T00:26:16.4101641Z ---- [ui] ui/type-alias-enum-variants/self-in-enum-definition.rs stdout ----
2020-04-19T00:26:16.4101893Z diff of stderr:
2020-04-19T00:26:16.4102014Z 
2020-04-19T00:26:16.4102508Z - error[E0391]: cycle detected when const-evaluating + checking `Alpha::V3::{{constant}}#0`
2020-04-19T00:26:16.4103205Z + error[E0391]: cycle detected when const-evaluating + checking `Alpha::V3::{{constant}}#0`[None]
2020-04-19T00:26:16.4103945Z 3    |
2020-04-19T00:26:16.4103945Z 3    |
2020-04-19T00:26:16.4104148Z 4 LL |     V3 = Self::V1 {} as u8 + 2,
2020-04-19T00:26:16.4104448Z 5    |          ^^^^^^^^
2020-04-19T00:26:16.4104602Z 6    |
2020-04-19T00:26:16.4104602Z 6    |
2020-04-19T00:26:16.4105100Z - note: ...which requires const-evaluating + checking `Alpha::V3::{{constant}}#0`...
2020-04-19T00:26:16.4105739Z + note: ...which requires const-evaluating + checking `Alpha::V3::{{constant}}#0`[None]...
2020-04-19T00:26:16.4106487Z 9    |
2020-04-19T00:26:16.4106487Z 9    |
2020-04-19T00:26:16.4106675Z 10 LL |     V3 = Self::V1 {} as u8 + 2,
2020-04-19T00:26:16.4107001Z 11    |          ^^^^^^^^
2020-04-19T00:26:16.4107001Z 11    |          ^^^^^^^^
2020-04-19T00:26:16.4107481Z - note: ...which requires const-evaluating `Alpha::V3::{{constant}}#0`...
2020-04-19T00:26:16.4108068Z + note: ...which requires const-evaluating `Alpha::V3::{{constant}}#0`[None]...
2020-04-19T00:26:16.4108803Z 14    |
2020-04-19T00:26:16.4108803Z 14    |
2020-04-19T00:26:16.4108993Z 15 LL |     V3 = Self::V1 {} as u8 + 2,
2020-04-19T00:26:16.4109314Z 16    |          ^^^^^^^^
2020-04-19T00:26:16.4109314Z 16    |          ^^^^^^^^
2020-04-19T00:26:16.4109558Z 17    = note: ...which requires computing layout of `Alpha`...
2020-04-19T00:26:16.4110208Z -    = note: ...which again requires const-evaluating + checking `Alpha::V3::{{constant}}#0`, completing the cycle
2020-04-19T00:26:16.4110994Z +    = note: ...which again requires const-evaluating + checking `Alpha::V3::{{constant}}#0`[None], completing the cycle
2020-04-19T00:26:16.4111615Z 19 note: cycle used when collecting item types in top-level module
2020-04-19T00:26:16.4112295Z 21    |
2020-04-19T00:26:16.4112395Z 
2020-04-19T00:26:16.4112485Z 
2020-04-19T00:26:16.4112686Z The actual stderr differed from the expected stderr.
2020-04-19T00:26:16.4112686Z The actual stderr differed from the expected stderr.
2020-04-19T00:26:16.4115304Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/self-in-enum-definition/self-in-enum-definition.stderr
2020-04-19T00:26:16.4115951Z To update references, rerun the tests and pass the `--bless` flag
2020-04-19T00:26:16.4116576Z To only update this specific test, also pass `--test-args type-alias-enum-variants/self-in-enum-definition.rs`
2020-04-19T00:26:16.4117003Z error: 1 errors occurred comparing output.
2020-04-19T00:26:16.4117240Z status: exit code: 1
2020-04-19T00:26:16.4117240Z status: exit code: 1
2020-04-19T00:26:16.4119278Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-enum-variants/self-in-enum-definition.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/self-in-enum-definition" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/self-in-enum-definition/auxiliary"
2020-04-19T00:26:16.4120845Z ------------------------------------------
2020-04-19T00:26:16.4121001Z 
2020-04-19T00:26:16.4121360Z ------------------------------------------
2020-04-19T00:26:16.4121543Z stderr:
2020-04-19T00:26:16.4121543Z stderr:
2020-04-19T00:26:16.4121892Z ------------------------------------------
2020-04-19T00:26:16.4122475Z error[E0391]: cycle detected when const-evaluating + checking `Alpha::V3::{{constant}}#0`[None]
2020-04-19T00:26:16.4123371Z    |
2020-04-19T00:26:16.4123371Z    |
2020-04-19T00:26:16.4123844Z LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when const-evaluating
2020-04-19T00:26:16.4124271Z    |
2020-04-19T00:26:16.4124271Z    |
2020-04-19T00:26:16.4124775Z note: ...which requires const-evaluating + checking `Alpha::V3::{{constant}}#0`[None]...
2020-04-19T00:26:16.4125640Z    |
2020-04-19T00:26:16.4125640Z    |
2020-04-19T00:26:16.4126110Z LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when const-evaluating
2020-04-19T00:26:16.4126385Z    |          ^^^^^^^^
2020-04-19T00:26:16.4126866Z note: ...which requires const-evaluating `Alpha::V3::{{constant}}#0`[None]...
2020-04-19T00:26:16.4127729Z    |
2020-04-19T00:26:16.4127729Z    |
2020-04-19T00:26:16.4128185Z LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when const-evaluating
2020-04-19T00:26:16.4128475Z    |          ^^^^^^^^
2020-04-19T00:26:16.4128709Z    = note: ...which requires computing layout of `Alpha`...
2020-04-19T00:26:16.4129361Z    = note: ...which again requires const-evaluating + checking `Alpha::V3::{{constant}}#0`[None], completing the cycle
2020-04-19T00:26:16.4129977Z note: cycle used when collecting item types in top-level module
2020-04-19T00:26:16.4130779Z    |
2020-04-19T00:26:16.4130937Z LL | / #[repr(u8)]
2020-04-19T00:26:16.4131099Z LL | | enum Alpha {
2020-04-19T00:26:16.4131265Z LL | |     V1 = 41,
2020-04-19T00:26:16.4131265Z LL | |     V1 = 41,
2020-04-19T00:26:16.4131512Z LL | |     V2 = Self::V1 as u8 + 1, // OK; See #50072.
2020-04-19T00:26:16.4131847Z LL | |
2020-04-19T00:26:16.4131997Z LL | | fn main() {}
2020-04-19T00:26:16.4132174Z    | |____________^
2020-04-19T00:26:16.4132291Z 
---
2020-04-19T00:26:16.4133812Z 
2020-04-19T00:26:16.4134183Z ---- [ui] ui/write-to-static-mut-in-static.rs stdout ----
2020-04-19T00:26:16.4134407Z diff of stderr:
2020-04-19T00:26:16.4134527Z 
2020-04-19T00:26:16.4134735Z 4 LL | pub static mut B: () = unsafe { A = 1; };
2020-04-19T00:26:16.4135322Z 5    |                                 ^^^^^ modifying a static's initial value from another static's initializer
2020-04-19T00:26:16.4136000Z - error[E0391]: cycle detected when const-evaluating `C`
2020-04-19T00:26:16.4136000Z - error[E0391]: cycle detected when const-evaluating `C`
2020-04-19T00:26:16.4136485Z + error[E0391]: cycle detected when const-evaluating `C`[None]
2020-04-19T00:26:16.4137276Z 9    |
2020-04-19T00:26:16.4137276Z 9    |
2020-04-19T00:26:16.4137483Z 10 LL | pub static mut C: u32 = unsafe { C = 1; 0 };
2020-04-19T00:26:16.4137852Z 11    |                                  ^^^^^
2020-04-19T00:26:16.4138039Z 12    |
2020-04-19T00:26:16.4138039Z 12    |
2020-04-19T00:26:16.4138465Z - note: ...which requires const-evaluating `C`...
2020-04-19T00:26:16.4138925Z + note: ...which requires const-evaluating `C`[None]...
2020-04-19T00:26:16.4139614Z 15    |
2020-04-19T00:26:16.4139614Z 15    |
2020-04-19T00:26:16.4139820Z 16 LL | pub static mut C: u32 = unsafe { C = 1; 0 };
2020-04-19T00:26:16.4140178Z 17    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4140664Z -    = note: ...which again requires const-evaluating `C`, completing the cycle
2020-04-19T00:26:16.4140664Z -    = note: ...which again requires const-evaluating `C`, completing the cycle
2020-04-19T00:26:16.4141174Z - note: cycle used when const-evaluating + checking `C`
2020-04-19T00:26:16.4141712Z +    = note: ...which again requires const-evaluating `C`[None], completing the cycle
2020-04-19T00:26:16.4142228Z + note: cycle used when const-evaluating + checking `C`[None]
2020-04-19T00:26:16.4142927Z 21    |
2020-04-19T00:26:16.4142927Z 21    |
2020-04-19T00:26:16.4143140Z 22 LL | pub static mut C: u32 = unsafe { C = 1; 0 };
2020-04-19T00:26:16.4143421Z 
2020-04-19T00:26:16.4143608Z The actual stderr differed from the expected stderr.
2020-04-19T00:26:16.4144259Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/write-to-static-mut-in-static/write-to-static-mut-in-static.stderr
2020-04-19T00:26:16.4144259Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/write-to-static-mut-in-static/write-to-static-mut-in-static.stderr
2020-04-19T00:26:16.4144876Z To update references, rerun the tests and pass the `--bless` flag
2020-04-19T00:26:16.4145430Z To only update this specific test, also pass `--test-args write-to-static-mut-in-static.rs`
2020-04-19T00:26:16.4145842Z error: 1 errors occurred comparing output.
2020-04-19T00:26:16.4146060Z status: exit code: 1
2020-04-19T00:26:16.4146060Z status: exit code: 1
2020-04-19T00:26:16.4147827Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/write-to-static-mut-in-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/write-to-static-mut-in-static" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/write-to-static-mut-in-static/auxiliary"
2020-04-19T00:26:16.4149241Z ------------------------------------------
2020-04-19T00:26:16.4149416Z 
2020-04-19T00:26:16.4149754Z ------------------------------------------
2020-04-19T00:26:16.4150414Z stderr:
2020-04-19T00:26:16.4150414Z stderr:
2020-04-19T00:26:16.4150818Z ------------------------------------------
2020-04-19T00:26:16.4151073Z error[E0080]: could not evaluate static initializer
2020-04-19T00:26:16.4151570Z   --> /checkout/src/test/ui/write-to-static-mut-in-static.rs:2:33
2020-04-19T00:26:16.4151817Z    |
2020-04-19T00:26:16.4152015Z LL | pub static mut B: () = unsafe { A = 1; };
2020-04-19T00:26:16.4152596Z    |                                 ^^^^^ modifying a static's initial value from another static's initializer
2020-04-19T00:26:16.4152865Z 
2020-04-19T00:26:16.4153475Z error[E0391]: cycle detected when const-evaluating `C`[None]
2020-04-19T00:26:16.4154227Z    |
2020-04-19T00:26:16.4154227Z    |
2020-04-19T00:26:16.4154424Z LL | pub static mut C: u32 = unsafe { C = 1; 0 };
2020-04-19T00:26:16.4154873Z    |
2020-04-19T00:26:16.4154873Z    |
2020-04-19T00:26:16.4155248Z note: ...which requires const-evaluating `C`[None]...
2020-04-19T00:26:16.4156082Z    |
2020-04-19T00:26:16.4156082Z    |
2020-04-19T00:26:16.4156297Z LL | pub static mut C: u32 = unsafe { C = 1; 0 };
2020-04-19T00:26:16.4156545Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T00:26:16.4157044Z    = note: ...which again requires const-evaluating `C`[None], completing the cycle
2020-04-19T00:26:16.4157645Z note: cycle used when const-evaluating + checking `C`[None]
2020-04-19T00:26:16.4158381Z    |
2020-04-19T00:26:16.4158381Z    |
2020-04-19T00:26:16.4158595Z LL | pub static mut C: u32 = unsafe { C = 1; 0 };
2020-04-19T00:26:16.4158980Z 
2020-04-19T00:26:16.4159167Z error: aborting due to 2 previous errors
2020-04-19T00:26:16.4159320Z 
2020-04-19T00:26:16.4159528Z Some errors have detailed explanations: E0080, E0391.
---
2020-04-19T00:26:16.4164973Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-19T00:26:16.4165338Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-19T00:26:16.4165540Z 
2020-04-19T00:26:16.4165629Z 
2020-04-19T00:26:16.4168974Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-19T00:26:16.4171244Z 
2020-04-19T00:26:16.4171334Z 
2020-04-19T00:26:16.4171916Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-19T00:26:16.4172248Z Build completed unsuccessfully in 0:55:47
2020-04-19T00:26:16.4172248Z Build completed unsuccessfully in 0:55:47
2020-04-19T00:26:16.4172488Z == clock drift check ==
2020-04-19T00:26:16.4172711Z   local time: Sun Apr 19 00:26:16 UTC 2020
2020-04-19T00:26:16.7703030Z   network time: Sun, 19 Apr 2020 00:26:16 GMT
2020-04-19T00:26:17.5419915Z 
2020-04-19T00:26:17.5419915Z 
2020-04-19T00:26:17.5495074Z ##[error]Bash exited with code '1'.
2020-04-19T00:26:17.5509184Z ##[section]Finishing: Run build
2020-04-19T00:26:17.5554695Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-19T00:26:17.5562510Z Task         : Get sources
2020-04-19T00:26:17.5562820Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T00:26:17.5563117Z Version      : 1.0.0
2020-04-19T00:26:17.5563326Z Author       : Microsoft
2020-04-19T00:26:17.5563326Z Author       : Microsoft
2020-04-19T00:26:17.5563653Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T00:26:17.5564029Z ==============================================================================
2020-04-19T00:26:17.8847040Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T00:26:17.8902892Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-19T00:26:17.8997817Z Cleaning up task key
2020-04-19T00:26:17.8999173Z Start cleaning up orphan processes.
2020-04-19T00:26:17.9181054Z Terminate orphan process: pid (4216) (python)
2020-04-19T00:26:17.9352020Z ##[section]Finishing: Finalize Job
