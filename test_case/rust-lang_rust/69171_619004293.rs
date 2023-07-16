plain
2020-04-24T12:17:23.7250732Z ========================== Starting Command Output ===========================
2020-04-24T12:17:23.7252891Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2ab14bc0-7464-4f09-a487-6e4a69969585.sh
2020-04-24T12:17:23.7253108Z 
2020-04-24T12:17:23.7255897Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T12:17:23.7273736Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69171/merge to s
2020-04-24T12:17:23.7276575Z Task         : Get sources
2020-04-24T12:17:23.7276837Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T12:17:23.7277087Z Version      : 1.0.0
2020-04-24T12:17:23.7277262Z Author       : Microsoft
---
2020-04-24T12:17:24.7182200Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T12:17:24.7190746Z ##[command]git config gc.auto 0
2020-04-24T12:17:24.7197125Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T12:17:24.7203211Z ##[command]git config --get-all http.proxy
2020-04-24T12:17:24.7212791Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69171/merge:refs/remotes/pull/69171/merge
---
2020-04-24T12:19:48.8305972Z  ---> cb2676f08729
2020-04-24T12:19:48.8308399Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-24T12:19:48.8309358Z  ---> Using cache
2020-04-24T12:19:48.8309827Z  ---> df25ce111862
2020-04-24T12:19:48.8312222Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-24T12:19:48.8313734Z  ---> 599b9ac96b27
2020-04-24T12:19:48.8314022Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-24T12:19:48.8316079Z  ---> Using cache
2020-04-24T12:19:48.8316734Z  ---> 091087e35a36
---
2020-04-24T12:19:49.7546437Z Looks like docker image is the same as before, not uploading
2020-04-24T12:19:55.9290100Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-24T12:19:56.0058279Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-24T12:19:56.0091018Z == clock drift check ==
2020-04-24T12:19:56.0111564Z   local time: Fri Apr 24 12:19:56 UTC 2020
2020-04-24T12:19:56.1072084Z   network time: Fri, 24 Apr 2020 12:19:56 GMT
2020-04-24T12:19:56.1091270Z Starting sccache server...
2020-04-24T12:19:56.1895250Z configure: processing command line
2020-04-24T12:19:56.1895940Z configure: 
2020-04-24T12:19:56.1897111Z configure: rust.dist-src        := False
---
2020-04-24T12:24:33.6760861Z    Compiling rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-24T12:24:41.8781546Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-24T12:24:57.5605787Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T12:24:58.9917492Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T12:25:00.9591802Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T12:25:14.9929590Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-24T12:25:18.6085121Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T12:25:29.3720478Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T12:25:33.0410038Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-24T12:46:28.8363553Z    Compiling rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-24T12:46:37.5645140Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-24T12:46:54.3882254Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T12:46:55.8905750Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T12:46:59.0328312Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T12:47:13.4842162Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-24T12:47:17.3228398Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T12:47:29.1093711Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T12:47:31.9645410Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-24T13:09:09.3783153Z .................................................................................................... 1700/9928
2020-04-24T13:09:13.7086150Z .................................................................................................... 1800/9928
2020-04-24T13:09:21.3417534Z .................................................................................................... 1900/9928
2020-04-24T13:09:29.6455258Z .........i.......................................................................................... 2000/9928
2020-04-24T13:09:35.8797458Z ...................................................................................................i 2100/9928
2020-04-24T13:09:48.5755797Z iiii................................................................................................ 2200/9928
2020-04-24T13:09:56.8535992Z .................................................................................................... 2400/9928
2020-04-24T13:09:59.1289149Z .................................................................................................... 2500/9928
2020-04-24T13:10:04.4743802Z .................................................................................................... 2600/9928
2020-04-24T13:10:21.8400929Z .................................................................................................... 2700/9928
---
2020-04-24T13:12:55.0817984Z .................................................................................................... 5100/9928
2020-04-24T13:13:01.8066847Z .................................................................................................... 5200/9928
2020-04-24T13:13:06.2976067Z ......................i............................................................................. 5300/9928
2020-04-24T13:13:15.6678645Z ............i....................................................................................... 5400/9928
2020-04-24T13:13:21.0508136Z ............ii.ii........i...i...................................................................... 5500/9928
2020-04-24T13:13:28.5537060Z ...........................................................i........................................ 5700/9928
2020-04-24T13:13:37.0013313Z .................................................................................................ii. 5800/9928
2020-04-24T13:13:43.3097174Z ....................................i..F............................................................ 5900/9928
2020-04-24T13:13:48.7871152Z ................F................................................................................... 6000/9928
2020-04-24T13:13:48.7871152Z ................F................................................................................... 6000/9928
2020-04-24T13:13:58.5099984Z .................................................................................................... 6100/9928
2020-04-24T13:14:07.4508867Z ..............................ii...i..ii...........i................................................ 6200/9928
2020-04-24T13:14:23.7330148Z .................................................................................................... 6400/9928
2020-04-24T13:14:28.1250891Z .................................................................................................... 6500/9928
2020-04-24T13:14:28.1250891Z .................................................................................................... 6500/9928
2020-04-24T13:14:33.8219369Z ............................................................i..ii................................... 6600/9928
2020-04-24T13:14:55.8659173Z .................................................................................................... 6800/9928
2020-04-24T13:15:00.4933522Z .............................................................i...................................... 6900/9928
2020-04-24T13:15:02.4903719Z .................................................................................................... 7000/9928
2020-04-24T13:15:04.5131405Z .................................................................................................... 7100/9928
---
2020-04-24T13:16:33.6482311Z .................................................................................................... 7900/9928
2020-04-24T13:16:38.8295352Z .................................................................................................... 8000/9928
2020-04-24T13:16:44.7940760Z ......................................................................i............................. 8100/9928
2020-04-24T13:16:54.4656503Z .................................................................................................... 8200/9928
2020-04-24T13:16:59.5810880Z ....................iiiiiiiiiii.i................................................................... 8300/9928
2020-04-24T13:17:12.3842323Z .................................................................................................... 8500/9928
2020-04-24T13:17:17.7048427Z .................................................................................................... 8600/9928
2020-04-24T13:17:32.0975588Z .................................................................................................... 8700/9928
2020-04-24T13:17:38.8094912Z .................................................................................................... 8800/9928
---
2020-04-24T13:19:20.7109987Z 
2020-04-24T13:19:20.7160841Z ---- [ui] ui/asm/bad-options.rs stdout ----
2020-04-24T13:19:20.7183587Z diff of stderr:
2020-04-24T13:19:20.7184235Z 
2020-04-24T13:19:20.7184625Z 28 LL |         asm!("{}", out(reg) foo, options(noreturn));
2020-04-24T13:19:20.7185285Z 30 
2020-04-24T13:19:20.7186073Z - error: aborting due to 5 previous errors
2020-04-24T13:19:20.7186073Z - error: aborting due to 5 previous errors
2020-04-24T13:19:20.7186844Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7187844Z +    |
2020-04-24T13:19:20.7187844Z +    |
2020-04-24T13:19:20.7188149Z + LL |         asm!("", options(nomem, readonly));
2020-04-24T13:19:20.7188608Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7189546Z +    = note: `#[warn(deprecated)]` on by default
2020-04-24T13:19:20.7189862Z + 
2020-04-24T13:19:20.7189862Z + 
2020-04-24T13:19:20.7190648Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7191665Z +    |
2020-04-24T13:19:20.7191665Z +    |
2020-04-24T13:19:20.7192003Z + LL |         asm!("", options(pure, nomem, noreturn));
2020-04-24T13:19:20.7192470Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7192844Z + 
2020-04-24T13:19:20.7193576Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7194557Z +    |
2020-04-24T13:19:20.7194557Z +    |
2020-04-24T13:19:20.7194898Z + LL |         asm!("{}", in(reg) foo, options(pure, nomem));
2020-04-24T13:19:20.7195356Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7195744Z + 
2020-04-24T13:19:20.7196412Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7197389Z +    |
2020-04-24T13:19:20.7197389Z +    |
2020-04-24T13:19:20.7197724Z + LL |         asm!("{}", out(reg) foo, options(noreturn));
2020-04-24T13:19:20.7198195Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7198563Z + 
2020-04-24T13:19:20.7198885Z + error: aborting due to 5 previous errors; 4 warnings emitted
2020-04-24T13:19:20.7199446Z 33 
2020-04-24T13:19:20.7199650Z 
2020-04-24T13:19:20.7199867Z 
2020-04-24T13:19:20.7200174Z The actual stderr differed from the expected stderr.
2020-04-24T13:19:20.7200174Z The actual stderr differed from the expected stderr.
2020-04-24T13:19:20.7200905Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-options/bad-options.stderr
2020-04-24T13:19:20.7201618Z To update references, rerun the tests and pass the `--bless` flag
2020-04-24T13:19:20.7202320Z To only update this specific test, also pass `--test-args asm/bad-options.rs`
2020-04-24T13:19:20.7202965Z error: 1 errors occurred comparing output.
2020-04-24T13:19:20.7203297Z status: exit code: 1
2020-04-24T13:19:20.7203297Z status: exit code: 1
2020-04-24T13:19:20.7205169Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-options.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-options" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-options/auxiliary"
2020-04-24T13:19:20.7206814Z ------------------------------------------
2020-04-24T13:19:20.7207098Z 
2020-04-24T13:19:20.7207583Z ------------------------------------------
2020-04-24T13:19:20.7207899Z stderr:
2020-04-24T13:19:20.7207899Z stderr:
2020-04-24T13:19:20.7208378Z ------------------------------------------
2020-04-24T13:19:20.7208801Z error: the `nomem` and `readonly` options are mutually exclusive
2020-04-24T13:19:20.7209440Z   --> /checkout/src/test/ui/asm/bad-options.rs:8:18
2020-04-24T13:19:20.7209780Z    |
2020-04-24T13:19:20.7210082Z LL |         asm!("", options(nomem, readonly));
2020-04-24T13:19:20.7210731Z 
2020-04-24T13:19:20.7211054Z error: the `pure` and `noreturn` options are mutually exclusive
2020-04-24T13:19:20.7211686Z   --> /checkout/src/test/ui/asm/bad-options.rs:10:18
2020-04-24T13:19:20.7212037Z    |
2020-04-24T13:19:20.7212037Z    |
2020-04-24T13:19:20.7212359Z LL |         asm!("", options(pure, nomem, noreturn));
2020-04-24T13:19:20.7213020Z 
2020-04-24T13:19:20.7213020Z 
2020-04-24T13:19:20.7213337Z error: asm with `pure` option must have at least one output
2020-04-24T13:19:20.7214346Z   --> /checkout/src/test/ui/asm/bad-options.rs:10:18
2020-04-24T13:19:20.7214954Z    |
2020-04-24T13:19:20.7215286Z LL |         asm!("", options(pure, nomem, noreturn));
2020-04-24T13:19:20.7216211Z 
2020-04-24T13:19:20.7216211Z 
2020-04-24T13:19:20.7216529Z error: asm with `pure` option must have at least one output
2020-04-24T13:19:20.7217507Z   --> /checkout/src/test/ui/asm/bad-options.rs:13:33
2020-04-24T13:19:20.7217872Z    |
2020-04-24T13:19:20.7218187Z LL |         asm!("{}", in(reg) foo, options(pure, nomem));
2020-04-24T13:19:20.7218883Z 
2020-04-24T13:19:20.7219199Z error: asm outputs are not allowed with the `noreturn` option
2020-04-24T13:19:20.7219842Z   --> /checkout/src/test/ui/asm/bad-options.rs:15:20
2020-04-24T13:19:20.7220175Z    |
2020-04-24T13:19:20.7220175Z    |
2020-04-24T13:19:20.7220504Z LL |         asm!("{}", out(reg) foo, options(noreturn));
2020-04-24T13:19:20.7221128Z 
2020-04-24T13:19:20.7221128Z 
2020-04-24T13:19:20.7221814Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7231822Z   --> /checkout/src/test/ui/asm/bad-options.rs:8:9
2020-04-24T13:19:20.7232063Z    |
2020-04-24T13:19:20.7232257Z LL |         asm!("", options(nomem, readonly));
2020-04-24T13:19:20.7232581Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7246074Z    = note: `#[warn(deprecated)]` on by default
2020-04-24T13:19:20.7248472Z 
2020-04-24T13:19:20.7248472Z 
2020-04-24T13:19:20.7249234Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7249868Z   --> /checkout/src/test/ui/asm/bad-options.rs:10:9
2020-04-24T13:19:20.7250081Z    |
2020-04-24T13:19:20.7250282Z LL |         asm!("", options(pure, nomem, noreturn));
2020-04-24T13:19:20.7256241Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7256500Z 
2020-04-24T13:19:20.7257284Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7257899Z   --> /checkout/src/test/ui/asm/bad-options.rs:13:9
2020-04-24T13:19:20.7258109Z    |
2020-04-24T13:19:20.7258317Z LL |         asm!("{}", in(reg) foo, options(pure, nomem));
2020-04-24T13:19:20.7258658Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7259482Z 
2020-04-24T13:19:20.7260080Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7260654Z   --> /checkout/src/test/ui/asm/bad-options.rs:15:9
2020-04-24T13:19:20.7260882Z    |
2020-04-24T13:19:20.7261087Z LL |         asm!("{}", out(reg) foo, options(noreturn));
2020-04-24T13:19:20.7261423Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7261682Z 
2020-04-24T13:19:20.7261888Z error: aborting due to 5 previous errors; 4 warnings emitted
2020-04-24T13:19:20.7262335Z 
2020-04-24T13:19:20.7262738Z ------------------------------------------
2020-04-24T13:19:20.7262896Z 
2020-04-24T13:19:20.7262987Z 
2020-04-24T13:19:20.7262987Z 
2020-04-24T13:19:20.7263347Z ---- [ui] ui/asm/bad-template.rs stdout ----
2020-04-24T13:19:20.7263575Z diff of stderr:
2020-04-24T13:19:20.7263701Z 
2020-04-24T13:19:20.7263890Z 82 LL |         asm!("{:foo}", in(reg) foo);
2020-04-24T13:19:20.7264295Z 84 
2020-04-24T13:19:20.7264673Z - error: aborting due to 10 previous errors
2020-04-24T13:19:20.7264673Z - error: aborting due to 10 previous errors
2020-04-24T13:19:20.7265278Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7265817Z +   --> $DIR/bad-template.rs:8:9
2020-04-24T13:19:20.7266000Z +    |
2020-04-24T13:19:20.7266159Z + LL |         asm!("{}");
2020-04-24T13:19:20.7266641Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7267105Z +    = note: `#[warn(deprecated)]` on by default
2020-04-24T13:19:20.7267366Z + 
2020-04-24T13:19:20.7267366Z + 
2020-04-24T13:19:20.7267941Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7268469Z +   --> $DIR/bad-template.rs:10:9
2020-04-24T13:19:20.7268651Z +    |
2020-04-24T13:19:20.7268849Z + LL |         asm!("{1}", in(reg) foo);
2020-04-24T13:19:20.7269164Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7269420Z + 
2020-04-24T13:19:20.7269974Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7270497Z +   --> $DIR/bad-template.rs:13:9
2020-04-24T13:19:20.7270680Z +    |
2020-04-24T13:19:20.7270861Z + LL |         asm!("{a}");
2020-04-24T13:19:20.7271161Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7271416Z + 
2020-04-24T13:19:20.7273209Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7273745Z +   --> $DIR/bad-template.rs:15:9
2020-04-24T13:19:20.7273926Z +    |
2020-04-24T13:19:20.7274128Z + LL |         asm!("{}", a = in(reg) foo);
2020-04-24T13:19:20.7274448Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7274702Z + 
2020-04-24T13:19:20.7275253Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7275782Z +   --> $DIR/bad-template.rs:18:9
2020-04-24T13:19:20.7275965Z +    |
2020-04-24T13:19:20.7276168Z + LL |         asm!("{1}", a = in(reg) foo);
2020-04-24T13:19:20.7276494Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7276748Z + 
2020-04-24T13:19:20.7277304Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7277833Z +   --> $DIR/bad-template.rs:21:9
2020-04-24T13:19:20.7278015Z +    |
2020-04-24T13:19:20.7278217Z + LL |         asm!("{}", in("eax") foo);
2020-04-24T13:19:20.7278536Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7278790Z + 
2020-04-24T13:19:20.7279349Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7279873Z +   --> $DIR/bad-template.rs:23:9
2020-04-24T13:19:20.7280055Z +    |
2020-04-24T13:19:20.7280251Z + LL |         asm!("{:foo}", in(reg) foo);
2020-04-24T13:19:20.7280601Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7280855Z + 
2020-04-24T13:19:20.7281069Z + error: aborting due to 10 previous errors; 7 warnings emitted
2020-04-24T13:19:20.7281419Z 87 
2020-04-24T13:19:20.7281515Z 
2020-04-24T13:19:20.7281606Z 
2020-04-24T13:19:20.7281814Z The actual stderr differed from the expected stderr.
2020-04-24T13:19:20.7281814Z The actual stderr differed from the expected stderr.
2020-04-24T13:19:20.7282425Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template/bad-template.stderr
2020-04-24T13:19:20.7283004Z To update references, rerun the tests and pass the `--bless` flag
2020-04-24T13:19:20.7283566Z To only update this specific test, also pass `--test-args asm/bad-template.rs`
2020-04-24T13:19:20.7283955Z error: 1 errors occurred comparing output.
2020-04-24T13:19:20.7284188Z status: exit code: 1
2020-04-24T13:19:20.7284188Z status: exit code: 1
2020-04-24T13:19:20.7286021Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-template.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template/auxiliary"
2020-04-24T13:19:20.7287459Z ------------------------------------------
2020-04-24T13:19:20.7287609Z 
2020-04-24T13:19:20.7287980Z ------------------------------------------
2020-04-24T13:19:20.7288173Z stderr:
2020-04-24T13:19:20.7288173Z stderr:
2020-04-24T13:19:20.7288535Z ------------------------------------------
2020-04-24T13:19:20.7288804Z error: invalid reference to argument at index 0
2020-04-24T13:19:20.7289270Z   --> /checkout/src/test/ui/asm/bad-template.rs:8:15
2020-04-24T13:19:20.7289469Z    |
2020-04-24T13:19:20.7289619Z LL |         asm!("{}");
2020-04-24T13:19:20.7289835Z    |               ^^ from here
2020-04-24T13:19:20.7290172Z    = note: no arguments were given
2020-04-24T13:19:20.7290338Z 
2020-04-24T13:19:20.7290527Z error: invalid reference to argument at index 1
2020-04-24T13:19:20.7290999Z   --> /checkout/src/test/ui/asm/bad-template.rs:10:15
2020-04-24T13:19:20.7290999Z   --> /checkout/src/test/ui/asm/bad-template.rs:10:15
2020-04-24T13:19:20.7291225Z    |
2020-04-24T13:19:20.7291405Z LL |         asm!("{1}", in(reg) foo);
2020-04-24T13:19:20.7291621Z    |               ^^^ from here
2020-04-24T13:19:20.7291972Z    = note: there is 1 argument
2020-04-24T13:19:20.7292115Z 
2020-04-24T13:19:20.7292269Z error: argument never used
2020-04-24T13:19:20.7292726Z   --> /checkout/src/test/ui/asm/bad-template.rs:10:21
2020-04-24T13:19:20.7292726Z   --> /checkout/src/test/ui/asm/bad-template.rs:10:21
2020-04-24T13:19:20.7292936Z    |
2020-04-24T13:19:20.7293111Z LL |         asm!("{1}", in(reg) foo);
2020-04-24T13:19:20.7293558Z 
2020-04-24T13:19:20.7293729Z error: there is no argument named `a`
2020-04-24T13:19:20.7294181Z   --> /checkout/src/test/ui/asm/bad-template.rs:13:15
2020-04-24T13:19:20.7294407Z    |
2020-04-24T13:19:20.7294407Z    |
2020-04-24T13:19:20.7294560Z LL |         asm!("{a}");
2020-04-24T13:19:20.7294889Z 
2020-04-24T13:19:20.7295074Z error: invalid reference to argument at index 0
2020-04-24T13:19:20.7295546Z   --> /checkout/src/test/ui/asm/bad-template.rs:15:15
2020-04-24T13:19:20.7295754Z    |
2020-04-24T13:19:20.7295754Z    |
2020-04-24T13:19:20.7295950Z LL |         asm!("{}", a = in(reg) foo);
2020-04-24T13:19:20.7296397Z    |               ^^   --------------- named argument
2020-04-24T13:19:20.7296823Z    |               from here
2020-04-24T13:19:20.7296981Z    |
2020-04-24T13:19:20.7297166Z    = note: no positional arguments were given
2020-04-24T13:19:20.7297439Z note: named arguments cannot be referenced by position
2020-04-24T13:19:20.7297439Z note: named arguments cannot be referenced by position
2020-04-24T13:19:20.7297937Z   --> /checkout/src/test/ui/asm/bad-template.rs:15:20
2020-04-24T13:19:20.7298136Z    |
2020-04-24T13:19:20.7298330Z LL |         asm!("{}", a = in(reg) foo);
2020-04-24T13:19:20.7298715Z 
2020-04-24T13:19:20.7298876Z error: named argument never used
2020-04-24T13:19:20.7299336Z   --> /checkout/src/test/ui/asm/bad-template.rs:15:20
2020-04-24T13:19:20.7299543Z    |
2020-04-24T13:19:20.7299543Z    |
2020-04-24T13:19:20.7299725Z LL |         asm!("{}", a = in(reg) foo);
2020-04-24T13:19:20.7300011Z    |                    ^^^^^^^^^^^^^^^ named argument never used
2020-04-24T13:19:20.7300383Z error: invalid reference to argument at index 1
2020-04-24T13:19:20.7300863Z   --> /checkout/src/test/ui/asm/bad-template.rs:18:15
2020-04-24T13:19:20.7301071Z    |
2020-04-24T13:19:20.7301071Z    |
2020-04-24T13:19:20.7301250Z LL |         asm!("{1}", a = in(reg) foo);
2020-04-24T13:19:20.7301470Z    |               ^^^ from here
2020-04-24T13:19:20.7302627Z    = note: no positional arguments were given
2020-04-24T13:19:20.7302792Z 
2020-04-24T13:19:20.7302971Z error: named argument never used
2020-04-24T13:19:20.7303459Z   --> /checkout/src/test/ui/asm/bad-template.rs:18:21
2020-04-24T13:19:20.7303459Z   --> /checkout/src/test/ui/asm/bad-template.rs:18:21
2020-04-24T13:19:20.7303667Z    |
2020-04-24T13:19:20.7303961Z LL |         asm!("{1}", a = in(reg) foo);
2020-04-24T13:19:20.7304236Z    |                     ^^^^^^^^^^^^^^^ named argument never used
2020-04-24T13:19:20.7304665Z error: invalid reference to argument at index 0
2020-04-24T13:19:20.7305162Z   --> /checkout/src/test/ui/asm/bad-template.rs:21:15
2020-04-24T13:19:20.7305373Z    |
2020-04-24T13:19:20.7305373Z    |
2020-04-24T13:19:20.7305548Z LL |         asm!("{}", in("eax") foo);
2020-04-24T13:19:20.7306028Z    |               ^^   ------------- explicit register argument
2020-04-24T13:19:20.7306455Z    |               from here
2020-04-24T13:19:20.7306880Z    |
2020-04-24T13:19:20.7307071Z    = note: no positional arguments were given
2020-04-24T13:19:20.7307359Z note: explicit register arguments cannot be used in the asm template
2020-04-24T13:19:20.7307359Z note: explicit register arguments cannot be used in the asm template
2020-04-24T13:19:20.7307886Z   --> /checkout/src/test/ui/asm/bad-template.rs:21:20
2020-04-24T13:19:20.7308097Z    |
2020-04-24T13:19:20.7308275Z LL |         asm!("{}", in("eax") foo);
2020-04-24T13:19:20.7308667Z 
2020-04-24T13:19:20.7308864Z error: asm template modifier must be a single character
2020-04-24T13:19:20.7309353Z   --> /checkout/src/test/ui/asm/bad-template.rs:23:17
2020-04-24T13:19:20.7309578Z    |
2020-04-24T13:19:20.7309578Z    |
2020-04-24T13:19:20.7309768Z LL |         asm!("{:foo}", in(reg) foo);
2020-04-24T13:19:20.7310118Z 
2020-04-24T13:19:20.7310118Z 
2020-04-24T13:19:20.7310668Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7311240Z   --> /checkout/src/test/ui/asm/bad-template.rs:8:9
2020-04-24T13:19:20.7311470Z    |
2020-04-24T13:19:20.7311625Z LL |         asm!("{}");
2020-04-24T13:19:20.7311914Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7312379Z    = note: `#[warn(deprecated)]` on by default
2020-04-24T13:19:20.7312551Z 
2020-04-24T13:19:20.7312551Z 
2020-04-24T13:19:20.7313083Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7313673Z   --> /checkout/src/test/ui/asm/bad-template.rs:10:9
2020-04-24T13:19:20.7313886Z    |
2020-04-24T13:19:20.7314062Z LL |         asm!("{1}", in(reg) foo);
2020-04-24T13:19:20.7314386Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7314619Z 
2020-04-24T13:19:20.7315147Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7315722Z   --> /checkout/src/test/ui/asm/bad-template.rs:13:9
2020-04-24T13:19:20.7315944Z    |
2020-04-24T13:19:20.7316099Z LL |         asm!("{a}");
2020-04-24T13:19:20.7316390Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7316635Z 
2020-04-24T13:19:20.7317171Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7317789Z   --> /checkout/src/test/ui/asm/bad-template.rs:15:9
2020-04-24T13:19:20.7318015Z    |
2020-04-24T13:19:20.7318202Z LL |         asm!("{}", a = in(reg) foo);
2020-04-24T13:19:20.7318520Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7318750Z 
2020-04-24T13:19:20.7319305Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7319877Z   --> /checkout/src/test/ui/asm/bad-template.rs:18:9
2020-04-24T13:19:20.7320084Z    |
2020-04-24T13:19:20.7320282Z LL |         asm!("{1}", a = in(reg) foo);
2020-04-24T13:19:20.7320599Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7320833Z 
2020-04-24T13:19:20.7321378Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7321953Z   --> /checkout/src/test/ui/asm/bad-template.rs:21:9
2020-04-24T13:19:20.7322237Z    |
2020-04-24T13:19:20.7322430Z LL |         asm!("{}", in("eax") foo);
2020-04-24T13:19:20.7322743Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7322976Z 
2020-04-24T13:19:20.7323571Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7324183Z   --> /checkout/src/test/ui/asm/bad-template.rs:23:9
2020-04-24T13:19:20.7324392Z    |
2020-04-24T13:19:20.7324582Z LL |         asm!("{:foo}", in(reg) foo);
2020-04-24T13:19:20.7324921Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7325151Z 
2020-04-24T13:19:20.7325356Z error: aborting due to 10 previous errors; 7 warnings emitted
2020-04-24T13:19:20.7325649Z 
2020-04-24T13:19:20.7326014Z ------------------------------------------
2020-04-24T13:19:20.7326171Z 
2020-04-24T13:19:20.7326261Z 
2020-04-24T13:19:20.7326261Z 
2020-04-24T13:19:20.7326616Z ---- [ui] ui/asm/bad-reg.rs stdout ----
2020-04-24T13:19:20.7326827Z diff of stderr:
2020-04-24T13:19:20.7326951Z 
2020-04-24T13:19:20.7327149Z 144 LL |         asm!("", in("xmm0") foo, out("ymm0") bar);
2020-04-24T13:19:20.7327602Z 146 
2020-04-24T13:19:20.7327972Z - error: aborting due to 19 previous errors
2020-04-24T13:19:20.7327972Z - error: aborting due to 19 previous errors
2020-04-24T13:19:20.7328598Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7329119Z +   --> $DIR/bad-reg.rs:12:9
2020-04-24T13:19:20.7329293Z +    |
2020-04-24T13:19:20.7329486Z + LL |         asm!("{}", in(foo) foo);
2020-04-24T13:19:20.7329799Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7330278Z +    = note: `#[warn(deprecated)]` on by default
2020-04-24T13:19:20.7330468Z + 
2020-04-24T13:19:20.7330468Z + 
2020-04-24T13:19:20.7331007Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7331540Z +   --> $DIR/bad-reg.rs:14:9
2020-04-24T13:19:20.7331714Z +    |
2020-04-24T13:19:20.7331897Z + LL |         asm!("", in("foo") foo);
2020-04-24T13:19:20.7332233Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7332491Z + 
2020-04-24T13:19:20.7333032Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7333557Z +   --> $DIR/bad-reg.rs:16:9
2020-04-24T13:19:20.7333730Z +    |
2020-04-24T13:19:20.7333923Z + LL |         asm!("{:z}", in(reg) foo);
2020-04-24T13:19:20.7334253Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7334523Z + 
2020-04-24T13:19:20.7335058Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7335567Z +   --> $DIR/bad-reg.rs:18:9
2020-04-24T13:19:20.7335754Z +    |
2020-04-24T13:19:20.7335958Z + LL |         asm!("{:r}", in(xmm_reg) foo);
2020-04-24T13:19:20.7336292Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7336560Z + 
2020-04-24T13:19:20.7337110Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7337629Z +   --> $DIR/bad-reg.rs:20:9
2020-04-24T13:19:20.7337814Z +    |
2020-04-24T13:19:20.7338002Z + LL |         asm!("{:a}", const 0);
2020-04-24T13:19:20.7338323Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7338592Z + 
2020-04-24T13:19:20.7339125Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7339633Z +   --> $DIR/bad-reg.rs:22:9
2020-04-24T13:19:20.7339818Z +    |
2020-04-24T13:19:20.7340005Z + LL |         asm!("{:a}", sym main);
2020-04-24T13:19:20.7340334Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7340667Z + 
2020-04-24T13:19:20.7341212Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7341722Z +   --> $DIR/bad-reg.rs:24:9
2020-04-24T13:19:20.7341956Z +    |
2020-04-24T13:19:20.7342970Z + LL |         asm!("{}", in(zmm_reg) foo);
2020-04-24T13:19:20.7343291Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7343560Z + 
2020-04-24T13:19:20.7344130Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7344645Z +   --> $DIR/bad-reg.rs:26:9
2020-04-24T13:19:20.7344835Z +    |
2020-04-24T13:19:20.7345013Z + LL |         asm!("", in("zmm0") foo);
2020-04-24T13:19:20.7345326Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7345580Z + 
2020-04-24T13:19:20.7346139Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7346660Z +   --> $DIR/bad-reg.rs:28:9
2020-04-24T13:19:20.7346836Z +    |
2020-04-24T13:19:20.7347030Z + LL |         asm!("", in("ah") foo);
2020-04-24T13:19:20.7347347Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7347601Z + 
2020-04-24T13:19:20.7348159Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7348668Z +   --> $DIR/bad-reg.rs:30:9
2020-04-24T13:19:20.7348838Z +    |
2020-04-24T13:19:20.7349031Z + LL |         asm!("", in("ebp") foo);
2020-04-24T13:19:20.7349342Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7349598Z + 
2020-04-24T13:19:20.7350155Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7350649Z +   --> $DIR/bad-reg.rs:32:9
2020-04-24T13:19:20.7350820Z +    |
2020-04-24T13:19:20.7351021Z + LL |         asm!("", in("rsp") foo);
2020-04-24T13:19:20.7351334Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7351587Z + 
2020-04-24T13:19:20.7352144Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7352658Z +   --> $DIR/bad-reg.rs:34:9
2020-04-24T13:19:20.7352829Z +    |
2020-04-24T13:19:20.7353025Z + LL |         asm!("", in("ip") foo);
2020-04-24T13:19:20.7353337Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7353584Z + 
2020-04-24T13:19:20.7354136Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7354652Z +   --> $DIR/bad-reg.rs:36:9
2020-04-24T13:19:20.7354823Z +    |
2020-04-24T13:19:20.7355017Z + LL |         asm!("", in("st(2)") foo);
2020-04-24T13:19:20.7355333Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7355589Z + 
2020-04-24T13:19:20.7356124Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7356649Z +   --> $DIR/bad-reg.rs:38:9
2020-04-24T13:19:20.7356823Z +    |
2020-04-24T13:19:20.7357001Z + LL |         asm!("", in("mm0") foo);
2020-04-24T13:19:20.7357331Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7357586Z + 
2020-04-24T13:19:20.7358129Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7358658Z +   --> $DIR/bad-reg.rs:40:9
2020-04-24T13:19:20.7358829Z +    |
2020-04-24T13:19:20.7359004Z + LL |         asm!("", in("k0") foo);
2020-04-24T13:19:20.7359333Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7359586Z + 
2020-04-24T13:19:20.7360121Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7360749Z +   --> $DIR/bad-reg.rs:46:9
2020-04-24T13:19:20.7360921Z +    |
2020-04-24T13:19:20.7361119Z + LL |         asm!("", in("eax") foo, in("al") bar);
2020-04-24T13:19:20.7361519Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7364850Z + 
2020-04-24T13:19:20.7365594Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7366156Z +   --> $DIR/bad-reg.rs:48:9
2020-04-24T13:19:20.7366327Z +    |
2020-04-24T13:19:20.7366852Z + LL |         asm!("", in("rax") foo, out("rax") bar);
2020-04-24T13:19:20.7367210Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7367467Z + 
2020-04-24T13:19:20.7368028Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7368558Z +   --> $DIR/bad-reg.rs:50:9
2020-04-24T13:19:20.7368750Z +    |
2020-04-24T13:19:20.7368958Z + LL |         asm!("", in("al") foo, lateout("al") bar);
2020-04-24T13:19:20.7369313Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7369569Z + 
2020-04-24T13:19:20.7370120Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7370653Z +   --> $DIR/bad-reg.rs:51:9
2020-04-24T13:19:20.7370825Z +    |
2020-04-24T13:19:20.7371028Z + LL |         asm!("", in("xmm0") foo, in("ymm0") bar);
2020-04-24T13:19:20.7371367Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7371638Z + 
2020-04-24T13:19:20.7372276Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7372792Z +   --> $DIR/bad-reg.rs:53:9
2020-04-24T13:19:20.7372981Z +    |
2020-04-24T13:19:20.7373187Z + LL |         asm!("", in("xmm0") foo, out("ymm0") bar);
2020-04-24T13:19:20.7373530Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7373797Z + 
2020-04-24T13:19:20.7374337Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7374850Z +   --> $DIR/bad-reg.rs:55:9
2020-04-24T13:19:20.7375039Z +    |
2020-04-24T13:19:20.7375250Z + LL |         asm!("", in("xmm0") foo, lateout("ymm0") bar);
2020-04-24T13:19:20.7375594Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7375864Z + 
2020-04-24T13:19:20.7376078Z + error: aborting due to 19 previous errors; 21 warnings emitted
2020-04-24T13:19:20.7376433Z 149 
2020-04-24T13:19:20.7376533Z 
2020-04-24T13:19:20.7376623Z 
2020-04-24T13:19:20.7376813Z The actual stderr differed from the expected stderr.
2020-04-24T13:19:20.7376813Z The actual stderr differed from the expected stderr.
2020-04-24T13:19:20.7377414Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-reg/bad-reg.stderr
2020-04-24T13:19:20.7377989Z To update references, rerun the tests and pass the `--bless` flag
2020-04-24T13:19:20.7378518Z To only update this specific test, also pass `--test-args asm/bad-reg.rs`
2020-04-24T13:19:20.7378922Z error: 1 errors occurred comparing output.
2020-04-24T13:19:20.7379142Z status: exit code: 1
2020-04-24T13:19:20.7379142Z status: exit code: 1
2020-04-24T13:19:20.7380936Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-reg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-reg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "target-feature=+avx2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-reg/auxiliary"
2020-04-24T13:19:20.7383595Z ------------------------------------------
2020-04-24T13:19:20.7383756Z 
2020-04-24T13:19:20.7384116Z ------------------------------------------
2020-04-24T13:19:20.7384328Z stderr:
2020-04-24T13:19:20.7384328Z stderr:
2020-04-24T13:19:20.7384759Z ------------------------------------------
2020-04-24T13:19:20.7385049Z error: invalid register class `foo`: unknown register class
2020-04-24T13:19:20.7385572Z   --> /checkout/src/test/ui/asm/bad-reg.rs:12:20
2020-04-24T13:19:20.7385775Z    |
2020-04-24T13:19:20.7385949Z LL |         asm!("{}", in(foo) foo);
2020-04-24T13:19:20.7386330Z 
2020-04-24T13:19:20.7386528Z error: invalid register `foo`: unknown register
2020-04-24T13:19:20.7387000Z   --> /checkout/src/test/ui/asm/bad-reg.rs:14:18
2020-04-24T13:19:20.7387212Z    |
2020-04-24T13:19:20.7387212Z    |
2020-04-24T13:19:20.7387385Z LL |         asm!("", in("foo") foo);
2020-04-24T13:19:20.7387749Z 
2020-04-24T13:19:20.7387971Z error: invalid asm template modifier for this register class
2020-04-24T13:19:20.7388443Z   --> /checkout/src/test/ui/asm/bad-reg.rs:16:15
2020-04-24T13:19:20.7388643Z    |
2020-04-24T13:19:20.7388643Z    |
2020-04-24T13:19:20.7388848Z LL |         asm!("{:z}", in(reg) foo);
2020-04-24T13:19:20.7389283Z    |               ^^^^   ----------- argument
2020-04-24T13:19:20.7389708Z    |               template modifier
2020-04-24T13:19:20.7389878Z    |
2020-04-24T13:19:20.7389878Z    |
2020-04-24T13:19:20.7390188Z    = note: the `reg` register class supports the following template modifiers: `l`, `h`, `x`, `e`, `r`
2020-04-24T13:19:20.7390691Z error: invalid asm template modifier for this register class
2020-04-24T13:19:20.7391162Z   --> /checkout/src/test/ui/asm/bad-reg.rs:18:15
2020-04-24T13:19:20.7391376Z    |
2020-04-24T13:19:20.7391376Z    |
2020-04-24T13:19:20.7391705Z LL |         asm!("{:r}", in(xmm_reg) foo);
2020-04-24T13:19:20.7392122Z    |               ^^^^   --------------- argument
---
2020-04-24T13:19:20.7520855Z 158    |                     |
2020-04-24T13:19:20.7521094Z 159    |                     explicit register argument
2020-04-24T13:19:20.7521307Z 160 
2020-04-24T13:19:20.7521763Z - error: aborting due to 24 previous errors
2020-04-24T13:19:20.7522384Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7523130Z +    |
2020-04-24T13:19:20.7523130Z +    |
2020-04-24T13:19:20.7523284Z + LL |         asm!();
2020-04-24T13:19:20.7523592Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7524058Z +    = note: `#[warn(deprecated)]` on by default
2020-04-24T13:19:20.7524249Z + 
2020-04-24T13:19:20.7524249Z + 
2020-04-24T13:19:20.7525180Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7526129Z +    |
2020-04-24T13:19:20.7526129Z +    |
2020-04-24T13:19:20.7526309Z + LL |         asm!(foo);
2020-04-24T13:19:20.7526604Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7526859Z + 
2020-04-24T13:19:20.7527475Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7528178Z +    |
2020-04-24T13:19:20.7528178Z +    |
2020-04-24T13:19:20.7528357Z + LL |         asm!("{}" foo);
2020-04-24T13:19:20.7528658Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7528912Z + 
2020-04-24T13:19:20.7529465Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7530356Z +    |
2020-04-24T13:19:20.7530356Z +    |
2020-04-24T13:19:20.7530536Z + LL |         asm!("{}", foo);
2020-04-24T13:19:20.7530842Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7531158Z + 
2020-04-24T13:19:20.7531724Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7532422Z +    |
2020-04-24T13:19:20.7532422Z +    |
2020-04-24T13:19:20.7532608Z + LL |         asm!("{}", in foo);
2020-04-24T13:19:20.7532915Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7533169Z + 
2020-04-24T13:19:20.7533704Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7534419Z +    |
2020-04-24T13:19:20.7534419Z +    |
2020-04-24T13:19:20.7534597Z + LL |         asm!("{}", in(reg foo));
2020-04-24T13:19:20.7534934Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7535188Z + 
2020-04-24T13:19:20.7535728Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7536440Z +    |
2020-04-24T13:19:20.7536440Z +    |
2020-04-24T13:19:20.7536611Z + LL |         asm!("{}", in(reg));
2020-04-24T13:19:20.7536934Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7537188Z + 
2020-04-24T13:19:20.7537721Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7538433Z +    |
2020-04-24T13:19:20.7538433Z +    |
2020-04-24T13:19:20.7538623Z + LL |         asm!("{}", inout(=) foo => bar);
2020-04-24T13:19:20.7538963Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7539219Z + 
2020-04-24T13:19:20.7539754Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7540467Z +    |
2020-04-24T13:19:20.7540467Z +    |
2020-04-24T13:19:20.7540658Z + LL |         asm!("{}", inout(reg) foo =>);
2020-04-24T13:19:20.7540995Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7541252Z + 
2020-04-24T13:19:20.7541786Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7542711Z +    |
2020-04-24T13:19:20.7542711Z +    |
2020-04-24T13:19:20.7542901Z + LL |         asm!("{}", in(reg) foo => bar);
2020-04-24T13:19:20.7543238Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7543492Z + 
2020-04-24T13:19:20.7544028Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7544743Z +    |
2020-04-24T13:19:20.7544743Z +    |
2020-04-24T13:19:20.7544924Z + LL |         asm!("{}", sym foo + bar);
2020-04-24T13:19:20.7545246Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7545515Z + 
2020-04-24T13:19:20.7546049Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7546759Z +    |
2020-04-24T13:19:20.7546759Z +    |
2020-04-24T13:19:20.7546936Z + LL |         asm!("", options(foo));
2020-04-24T13:19:20.7547247Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7547514Z + 
2020-04-24T13:19:20.7548046Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7548754Z +    |
2020-04-24T13:19:20.7548754Z +    |
2020-04-24T13:19:20.7549009Z + LL |         asm!("", options(nomem foo));
2020-04-24T13:19:20.7549329Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7549597Z + 
2020-04-24T13:19:20.7550192Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7550916Z +    |
2020-04-24T13:19:20.7550916Z +    |
2020-04-24T13:19:20.7551103Z + LL |         asm!("", options(nomem, foo));
2020-04-24T13:19:20.7551424Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7551692Z + 
2020-04-24T13:19:20.7552793Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7553532Z +    |
2020-04-24T13:19:20.7553532Z +    |
2020-04-24T13:19:20.7553722Z + LL |         asm!("", options(), options());
2020-04-24T13:19:20.7554047Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7554325Z + 
2020-04-24T13:19:20.7554861Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7555578Z +    |
2020-04-24T13:19:20.7555578Z +    |
2020-04-24T13:19:20.7555785Z + LL |         asm!("", options(), options(), options());
2020-04-24T13:19:20.7556126Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7556391Z + 
2020-04-24T13:19:20.7556926Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7557637Z +    |
2020-04-24T13:19:20.7557637Z +    |
2020-04-24T13:19:20.7557829Z + LL |         asm!("{}", options(), const foo);
2020-04-24T13:19:20.7558153Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7558408Z + 
2020-04-24T13:19:20.7558958Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7559659Z +    |
2020-04-24T13:19:20.7559659Z +    |
2020-04-24T13:19:20.7559885Z + LL |         asm!("{a}", a = const foo, a = const bar);
2020-04-24T13:19:20.7560224Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7560477Z + 
2020-04-24T13:19:20.7561025Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7561724Z +    |
2020-04-24T13:19:20.7561724Z +    |
2020-04-24T13:19:20.7561925Z + LL |         asm!("", a = in("eax") foo);
2020-04-24T13:19:20.7562243Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7562495Z + 
2020-04-24T13:19:20.7563041Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7563741Z +    |
2020-04-24T13:19:20.7563741Z +    |
2020-04-24T13:19:20.7563959Z + LL |         asm!("{a}", in("eax") foo, a = const bar);
2020-04-24T13:19:20.7564299Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7564554Z + 
2020-04-24T13:19:20.7565105Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7565804Z +    |
2020-04-24T13:19:20.7565804Z +    |
2020-04-24T13:19:20.7566022Z + LL |         asm!("{a}", in("eax") foo, a = const bar);
2020-04-24T13:19:20.7566360Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7566613Z + 
2020-04-24T13:19:20.7567161Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7567860Z +    |
2020-04-24T13:19:20.7567860Z +    |
2020-04-24T13:19:20.7568074Z + LL |         asm!("{1}", in("eax") foo, const bar);
2020-04-24T13:19:20.7568477Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7568733Z + 
2020-04-24T13:19:20.7569006Z + error: aborting due to 24 previous errors; 22 warnings emitted
2020-04-24T13:19:20.7569348Z 163 
2020-04-24T13:19:20.7569449Z 
2020-04-24T13:19:20.7569554Z 
2020-04-24T13:19:20.7569745Z The actual stderr differed from the expected stderr.
2020-04-24T13:19:20.7569745Z The actual stderr differed from the expected stderr.
2020-04-24T13:19:20.7570354Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/parse-error/parse-error.stderr
2020-04-24T13:19:20.7570940Z To update references, rerun the tests and pass the `--bless` flag
2020-04-24T13:19:20.7571474Z To only update this specific test, also pass `--test-args asm/parse-error.rs`
2020-04-24T13:19:20.7571861Z error: 1 errors occurred comparing output.
2020-04-24T13:19:20.7572096Z status: exit code: 1
2020-04-24T13:19:20.7572096Z status: exit code: 1
2020-04-24T13:19:20.7573838Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/parse-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/parse-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/parse-error/auxiliary"
2020-04-24T13:19:20.7575246Z ------------------------------------------
2020-04-24T13:19:20.7575406Z 
2020-04-24T13:19:20.7575774Z ------------------------------------------
2020-04-24T13:19:20.7575964Z stderr:
---
2020-04-24T13:19:20.7579052Z 
2020-04-24T13:19:20.7579212Z error: expected token: `,`
2020-04-24T13:19:20.7579651Z   --> /checkout/src/test/ui/asm/parse-error.rs:13:19
2020-04-24T13:19:20.7579871Z    |
2020-04-24T13:19:20.7580029Z LL |         asm!("{}" foo);
2020-04-24T13:19:20.7580240Z    |                   ^^^ expected `,`
2020-04-24T13:19:20.7580404Z 
2020-04-24T13:19:20.7580681Z error: expected one of `const`, `in`, `inlateout`, `inout`, `lateout`, `options`, `out`, or `sym`, found `foo`
2020-04-24T13:19:20.7581463Z    |
2020-04-24T13:19:20.7581625Z LL |         asm!("{}", foo);
2020-04-24T13:19:20.7581871Z    |                    ^^^ expected one of 8 possible tokens
2020-04-24T13:19:20.7582194Z 
---
2020-04-24T13:19:20.7583590Z 
2020-04-24T13:19:20.7583752Z error: expected `)`, found `foo`
2020-04-24T13:19:20.7584206Z   --> /checkout/src/test/ui/asm/parse-error.rs:19:27
2020-04-24T13:19:20.7584411Z    |
2020-04-24T13:19:20.7584583Z LL |         asm!("{}", in(reg foo));
2020-04-24T13:19:20.7584836Z    |                           ^^^ expected `)`
2020-04-24T13:19:20.7585197Z error: expected expression, found end of macro arguments
2020-04-24T13:19:20.7585775Z   --> /checkout/src/test/ui/asm/parse-error.rs:21:27
2020-04-24T13:19:20.7585981Z    |
2020-04-24T13:19:20.7585981Z    |
2020-04-24T13:19:20.7586152Z LL |         asm!("{}", in(reg));
2020-04-24T13:19:20.7586637Z 
2020-04-24T13:19:20.7586828Z error: expected register class or explicit register
2020-04-24T13:19:20.7587301Z   --> /checkout/src/test/ui/asm/parse-error.rs:23:26
2020-04-24T13:19:20.7587522Z    |
2020-04-24T13:19:20.7587522Z    |
2020-04-24T13:19:20.7587707Z LL |         asm!("{}", inout(=) foo => bar);
2020-04-24T13:19:20.7588070Z 
2020-04-24T13:19:20.7588281Z error: expected expression, found end of macro arguments
2020-04-24T13:19:20.7588755Z   --> /checkout/src/test/ui/asm/parse-error.rs:25:37
2020-04-24T13:19:20.7588959Z    |
2020-04-24T13:19:20.7588959Z    |
2020-04-24T13:19:20.7589159Z LL |         asm!("{}", inout(reg) foo =>);
2020-04-24T13:19:20.7589616Z 
2020-04-24T13:19:20.7589616Z 
2020-04-24T13:19:20.7589884Z error: expected one of `!`, `,`, `.`, `::`, `?`, `{`, or an operator, found `=>`
2020-04-24T13:19:20.7590623Z    |
2020-04-24T13:19:20.7590623Z    |
2020-04-24T13:19:20.7590828Z LL |         asm!("{}", in(reg) foo => bar);
2020-04-24T13:19:20.7591115Z    |                                ^^ expected one of 7 possible tokens
2020-04-24T13:19:20.7591502Z error: argument to `sym` must be a path expression
2020-04-24T13:19:20.7591983Z   --> /checkout/src/test/ui/asm/parse-error.rs:29:24
2020-04-24T13:19:20.7592186Z    |
2020-04-24T13:19:20.7592186Z    |
2020-04-24T13:19:20.7592361Z LL |         asm!("{}", sym foo + bar);
2020-04-24T13:19:20.7592749Z 
2020-04-24T13:19:20.7592749Z 
2020-04-24T13:19:20.7593034Z error: expected one of `)`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, or `readonly`, found `foo`
2020-04-24T13:19:20.7593817Z    |
2020-04-24T13:19:20.7593817Z    |
2020-04-24T13:19:20.7593988Z LL |         asm!("", options(foo));
2020-04-24T13:19:20.7594255Z    |                          ^^^ expected one of 7 possible tokens
2020-04-24T13:19:20.7594468Z 
2020-04-24T13:19:20.7594652Z error: expected one of `)` or `,`, found `foo`
2020-04-24T13:19:20.7595330Z    |
2020-04-24T13:19:20.7595330Z    |
2020-04-24T13:19:20.7595514Z LL |         asm!("", options(nomem foo));
2020-04-24T13:19:20.7595788Z    |                                ^^^ expected one of `)` or `,`
2020-04-24T13:19:20.7595979Z 
2020-04-24T13:19:20.7596276Z error: expected one of `)`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, or `readonly`, found `foo`
2020-04-24T13:19:20.7597036Z    |
2020-04-24T13:19:20.7597036Z    |
2020-04-24T13:19:20.7597234Z LL |         asm!("", options(nomem, foo));
2020-04-24T13:19:20.7597525Z    |                                 ^^^ expected one of 7 possible tokens
2020-04-24T13:19:20.7597935Z error: asm options cannot be specified multiple times
2020-04-24T13:19:20.7598412Z   --> /checkout/src/test/ui/asm/parse-error.rs:37:29
2020-04-24T13:19:20.7598618Z    |
2020-04-24T13:19:20.7598618Z    |
2020-04-24T13:19:20.7598816Z LL |         asm!("", options(), options());
2020-04-24T13:19:20.7599279Z    |                  ---------  ^^^^^^^^^ duplicate options
2020-04-24T13:19:20.7599737Z    |                  previously here
2020-04-24T13:19:20.7599885Z 
2020-04-24T13:19:20.7600077Z error: asm options cannot be specified multiple times
2020-04-24T13:19:20.7600544Z   --> /checkout/src/test/ui/asm/parse-error.rs:39:29
2020-04-24T13:19:20.7600544Z   --> /checkout/src/test/ui/asm/parse-error.rs:39:29
2020-04-24T13:19:20.7600764Z    |
2020-04-24T13:19:20.7600964Z LL |         asm!("", options(), options(), options());
2020-04-24T13:19:20.7601437Z    |                  ---------  ^^^^^^^^^ duplicate options
2020-04-24T13:19:20.7601955Z    |                  previously here
2020-04-24T13:19:20.7602103Z 
2020-04-24T13:19:20.7602299Z error: asm options cannot be specified multiple times
2020-04-24T13:19:20.7602839Z   --> /checkout/src/test/ui/asm/parse-error.rs:39:40
2020-04-24T13:19:20.7602839Z   --> /checkout/src/test/ui/asm/parse-error.rs:39:40
2020-04-24T13:19:20.7603050Z    |
2020-04-24T13:19:20.7603248Z LL |         asm!("", options(), options(), options());
2020-04-24T13:19:20.7603772Z    |                  ---------             ^^^^^^^^^ duplicate options
2020-04-24T13:19:20.7604227Z    |                  previously here
2020-04-24T13:19:20.7604387Z 
2020-04-24T13:19:20.7604570Z error: arguments are not allowed after options
2020-04-24T13:19:20.7605029Z   --> /checkout/src/test/ui/asm/parse-error.rs:42:31
2020-04-24T13:19:20.7605029Z   --> /checkout/src/test/ui/asm/parse-error.rs:42:31
2020-04-24T13:19:20.7605236Z    |
2020-04-24T13:19:20.7605437Z LL |         asm!("{}", options(), const foo);
2020-04-24T13:19:20.7605879Z    |                    ---------  ^^^^^^^^^ argument
2020-04-24T13:19:20.7606339Z    |                    previous options
2020-04-24T13:19:20.7606490Z 
2020-04-24T13:19:20.7606656Z error: duplicate argument named `a`
2020-04-24T13:19:20.7607114Z   --> /checkout/src/test/ui/asm/parse-error.rs:44:36
2020-04-24T13:19:20.7607114Z   --> /checkout/src/test/ui/asm/parse-error.rs:44:36
2020-04-24T13:19:20.7607319Z    |
2020-04-24T13:19:20.7607518Z LL |         asm!("{a}", a = const foo, a = const bar);
2020-04-24T13:19:20.7608034Z    |                     -------------  ^^^^^^^^^^^^^ duplicate argument
2020-04-24T13:19:20.7608503Z    |                     previously here
2020-04-24T13:19:20.7608654Z 
2020-04-24T13:19:20.7608822Z error: argument never used
2020-04-24T13:19:20.7609250Z   --> /checkout/src/test/ui/asm/parse-error.rs:44:36
2020-04-24T13:19:20.7609250Z   --> /checkout/src/test/ui/asm/parse-error.rs:44:36
2020-04-24T13:19:20.7609455Z    |
2020-04-24T13:19:20.7609668Z LL |         asm!("{a}", a = const foo, a = const bar);
2020-04-24T13:19:20.7610176Z 
2020-04-24T13:19:20.7610368Z error: explicit register arguments cannot have names
2020-04-24T13:19:20.7610847Z   --> /checkout/src/test/ui/asm/parse-error.rs:47:18
2020-04-24T13:19:20.7611050Z    |
2020-04-24T13:19:20.7611050Z    |
2020-04-24T13:19:20.7611232Z LL |         asm!("", a = in("eax") foo);
2020-04-24T13:19:20.7611628Z 
2020-04-24T13:19:20.7611838Z error: named arguments cannot follow explicit register arguments
2020-04-24T13:19:20.7612338Z   --> /checkout/src/test/ui/asm/parse-error.rs:49:36
2020-04-24T13:19:20.7612546Z    |
2020-04-24T13:19:20.7612546Z    |
2020-04-24T13:19:20.7612746Z LL |         asm!("{a}", in("eax") foo, a = const bar);
2020-04-24T13:19:20.7613253Z    |                     -------------  ^^^^^^^^^^^^^ named argument
2020-04-24T13:19:20.7613735Z    |                     explicit register argument
2020-04-24T13:19:20.7613942Z 
2020-04-24T13:19:20.7614613Z error: named arguments cannot follow explicit register arguments
2020-04-24T13:19:20.7615130Z   --> /checkout/src/test/ui/asm/parse-error.rs:51:36
2020-04-24T13:19:20.7615130Z   --> /checkout/src/test/ui/asm/parse-error.rs:51:36
2020-04-24T13:19:20.7615337Z    |
2020-04-24T13:19:20.7615552Z LL |         asm!("{a}", in("eax") foo, a = const bar);
2020-04-24T13:19:20.7616048Z    |                     -------------  ^^^^^^^^^^^^^ named argument
2020-04-24T13:19:20.7616546Z    |                     explicit register argument
2020-04-24T13:19:20.7616713Z 
2020-04-24T13:19:20.7616957Z error: positional arguments cannot follow named arguments or explicit register arguments
2020-04-24T13:19:20.7617481Z   --> /checkout/src/test/ui/asm/parse-error.rs:53:36
2020-04-24T13:19:20.7617481Z   --> /checkout/src/test/ui/asm/parse-error.rs:53:36
2020-04-24T13:19:20.7617701Z    |
2020-04-24T13:19:20.7617894Z LL |         asm!("{1}", in("eax") foo, const bar);
2020-04-24T13:19:20.7618384Z    |                     -------------  ^^^^^^^^^ positional argument
2020-04-24T13:19:20.7618881Z    |                     explicit register argument
2020-04-24T13:19:20.7619125Z 
2020-04-24T13:19:20.7619125Z 
2020-04-24T13:19:20.7619678Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7620520Z    |
2020-04-24T13:19:20.7620682Z LL |         asm!();
2020-04-24T13:19:20.7620966Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7621220Z    |
2020-04-24T13:19:20.7621220Z    |
2020-04-24T13:19:20.7621426Z    = note: `#[warn(deprecated)]` on by default
2020-04-24T13:19:20.7621594Z 
2020-04-24T13:19:20.7623809Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7624659Z    |
2020-04-24T13:19:20.7624813Z LL |         asm!(foo);
2020-04-24T13:19:20.7625100Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7625355Z 
2020-04-24T13:19:20.7625355Z 
2020-04-24T13:19:20.7625888Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7626678Z    |
2020-04-24T13:19:20.7626678Z    |
2020-04-24T13:19:20.7626839Z LL |         asm!("{}" foo);
2020-04-24T13:19:20.7627133Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7628531Z 
2020-04-24T13:19:20.7629105Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7629879Z    |
2020-04-24T13:19:20.7630056Z LL |         asm!("{}", foo);
2020-04-24T13:19:20.7630350Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7630579Z 
2020-04-24T13:19:20.7630579Z 
2020-04-24T13:19:20.7631112Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7631904Z    |
2020-04-24T13:19:20.7632068Z LL |         asm!("{}", in foo);
2020-04-24T13:19:20.7632386Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7632619Z 
2020-04-24T13:19:20.7632619Z 
2020-04-24T13:19:20.7633149Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7633935Z    |
2020-04-24T13:19:20.7633935Z    |
2020-04-24T13:19:20.7634110Z LL |         asm!("{}", in(reg foo));
2020-04-24T13:19:20.7634431Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7634662Z 
2020-04-24T13:19:20.7635188Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7635967Z    |
2020-04-24T13:19:20.7635967Z    |
2020-04-24T13:19:20.7636137Z LL |         asm!("{}", in(reg));
2020-04-24T13:19:20.7636439Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7636684Z 
2020-04-24T13:19:20.7637213Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7637995Z    |
2020-04-24T13:19:20.7637995Z    |
2020-04-24T13:19:20.7638181Z LL |         asm!("{}", inout(=) foo => bar);
2020-04-24T13:19:20.7638496Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7638726Z 
2020-04-24T13:19:20.7639264Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7640030Z    |
2020-04-24T13:19:20.7640030Z    |
2020-04-24T13:19:20.7640227Z LL |         asm!("{}", inout(reg) foo =>);
2020-04-24T13:19:20.7640544Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7640893Z 
2020-04-24T13:19:20.7641443Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7643280Z    |
2020-04-24T13:19:20.7643280Z    |
2020-04-24T13:19:20.7643479Z LL |         asm!("{}", in(reg) foo => bar);
2020-04-24T13:19:20.7643795Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7644024Z 
2020-04-24T13:19:20.7644574Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7645358Z    |
2020-04-24T13:19:20.7645358Z    |
2020-04-24T13:19:20.7645533Z LL |         asm!("{}", sym foo + bar);
2020-04-24T13:19:20.7645857Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7646106Z 
2020-04-24T13:19:20.7677623Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7678429Z    |
2020-04-24T13:19:20.7678429Z    |
2020-04-24T13:19:20.7678610Z LL |         asm!("", options(foo));
2020-04-24T13:19:20.7678931Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7679163Z 
2020-04-24T13:19:20.7679692Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7680475Z    |
2020-04-24T13:19:20.7680475Z    |
2020-04-24T13:19:20.7680655Z LL |         asm!("", options(nomem foo));
2020-04-24T13:19:20.7680970Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7681214Z 
2020-04-24T13:19:20.7681742Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7682526Z    |
2020-04-24T13:19:20.7682526Z    |
2020-04-24T13:19:20.7682710Z LL |         asm!("", options(nomem, foo));
2020-04-24T13:19:20.7683030Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7683262Z 
2020-04-24T13:19:20.7683808Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7684579Z    |
2020-04-24T13:19:20.7684579Z    |
2020-04-24T13:19:20.7684779Z LL |         asm!("", options(), options());
2020-04-24T13:19:20.7685093Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7685324Z 
2020-04-24T13:19:20.7685868Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7686637Z    |
2020-04-24T13:19:20.7686637Z    |
2020-04-24T13:19:20.7686855Z LL |         asm!("", options(), options(), options());
2020-04-24T13:19:20.7687187Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7687417Z 
2020-04-24T13:19:20.7687947Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7688726Z    |
2020-04-24T13:19:20.7688726Z    |
2020-04-24T13:19:20.7688913Z LL |         asm!("{}", options(), const foo);
2020-04-24T13:19:20.7689247Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7689476Z 
2020-04-24T13:19:20.7690002Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7690782Z    |
2020-04-24T13:19:20.7690782Z    |
2020-04-24T13:19:20.7690981Z LL |         asm!("{a}", a = const foo, a = const bar);
2020-04-24T13:19:20.7691475Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7691705Z 
2020-04-24T13:19:20.7692296Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7693088Z    |
2020-04-24T13:19:20.7693088Z    |
2020-04-24T13:19:20.7693266Z LL |         asm!("", a = in("eax") foo);
2020-04-24T13:19:20.7693577Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7693821Z 
2020-04-24T13:19:20.7694345Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7695126Z    |
2020-04-24T13:19:20.7695126Z    |
2020-04-24T13:19:20.7695325Z LL |         asm!("{a}", in("eax") foo, a = const bar);
2020-04-24T13:19:20.7695654Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7695902Z 
2020-04-24T13:19:20.7696427Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7697194Z    |
2020-04-24T13:19:20.7697194Z    |
2020-04-24T13:19:20.7697407Z LL |         asm!("{a}", in("eax") foo, a = const bar);
2020-04-24T13:19:20.7697740Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7697969Z 
2020-04-24T13:19:20.7698509Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7699273Z    |
2020-04-24T13:19:20.7699273Z    |
2020-04-24T13:19:20.7699483Z LL |         asm!("{1}", in("eax") foo, const bar);
2020-04-24T13:19:20.7699810Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7700039Z 
2020-04-24T13:19:20.7700245Z error: aborting due to 24 previous errors; 22 warnings emitted
2020-04-24T13:19:20.7700541Z 
2020-04-24T13:19:20.7700888Z ------------------------------------------
2020-04-24T13:19:20.7701047Z 
2020-04-24T13:19:20.7701153Z 
2020-04-24T13:19:20.7701153Z 
2020-04-24T13:19:20.7701512Z ---- [ui] ui/asm/rustfix-asm.rs stdout ----
2020-04-24T13:19:20.7701721Z diff of stderr:
2020-04-24T13:19:20.7701844Z 
2020-04-24T13:19:20.7701995Z 14    |         |
2020-04-24T13:19:20.7702422Z 15    |         help: replace with: `llvm_asm!`
2020-04-24T13:19:20.7702625Z 16 
2020-04-24T13:19:20.7703013Z - error: aborting due to 2 previous errors
2020-04-24T13:19:20.7703620Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7704144Z +   --> $DIR/rustfix-asm.rs:10:9
2020-04-24T13:19:20.7704341Z +    |
2020-04-24T13:19:20.7704530Z + LL |         asm!("" :: "r" (x));
2020-04-24T13:19:20.7704853Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7705339Z +    = note: `#[warn(deprecated)]` on by default
2020-04-24T13:19:20.7705528Z + 
2020-04-24T13:19:20.7705528Z + 
2020-04-24T13:19:20.7706068Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7706605Z +   --> $DIR/rustfix-asm.rs:12:9
2020-04-24T13:19:20.7706786Z +    |
2020-04-24T13:19:20.7706969Z + LL |         asm!("" : "=r" (y));
2020-04-24T13:19:20.7707305Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7707560Z + 
2020-04-24T13:19:20.7707770Z + error: aborting due to 2 previous errors; 2 warnings emitted
2020-04-24T13:19:20.7708110Z 19 
2020-04-24T13:19:20.7708207Z 
2020-04-24T13:19:20.7708297Z 
2020-04-24T13:19:20.7708499Z The actual stderr differed from the expected stderr.
2020-04-24T13:19:20.7708499Z The actual stderr differed from the expected stderr.
2020-04-24T13:19:20.7709096Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/rustfix-asm/rustfix-asm.stderr
2020-04-24T13:19:20.7710331Z thread '[ui] ui/asm/rustfix-asm.rs' panicked at 'failed to apply suggestions for "/checkout/src/test/ui/asm/rustfix-asm.rs" with rustfix: Could not replace range 221...224 in file -- maybe parts of it were already replaced?', src/tools/compiletest/src/runtest.rs:2965:30
2020-04-24T13:19:20.7711229Z 
2020-04-24T13:19:20.7711605Z ---- [ui] ui/asm/type-check-1.rs stdout ----
2020-04-24T13:19:20.7711816Z diff of stderr:
2020-04-24T13:19:20.7711938Z 
2020-04-24T13:19:20.7711938Z 
2020-04-24T13:19:20.7712471Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7713188Z +    |
2020-04-24T13:19:20.7713188Z +    |
2020-04-24T13:19:20.7713372Z + LL |         asm!("{}", in(reg) 1 + 2);
2020-04-24T13:19:20.7713704Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7714168Z +    = note: `#[warn(deprecated)]` on by default
2020-04-24T13:19:20.7714373Z + 
2020-04-24T13:19:20.7714373Z + 
2020-04-24T13:19:20.7714908Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7715630Z +    |
2020-04-24T13:19:20.7715630Z +    |
2020-04-24T13:19:20.7715813Z + LL |         asm!("{}", out(reg) 1 + 2);
2020-04-24T13:19:20.7716129Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7716397Z + 
2020-04-24T13:19:20.7716929Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7717636Z +    |
2020-04-24T13:19:20.7717636Z +    |
2020-04-24T13:19:20.7717836Z + LL |         asm!("{}", inout(reg) 1 + 2);
2020-04-24T13:19:20.7718156Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7718411Z + 
2020-04-24T13:19:20.7718966Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7719666Z +    |
2020-04-24T13:19:20.7719666Z +    |
2020-04-24T13:19:20.7719867Z + LL |         asm!("{}", in(reg) v[..]);
2020-04-24T13:19:20.7720182Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7720435Z + 
2020-04-24T13:19:20.7720984Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7721689Z +    |
2020-04-24T13:19:20.7721689Z +    |
2020-04-24T13:19:20.7721885Z + LL |         asm!("{}", out(reg) v[..]);
2020-04-24T13:19:20.7722202Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7722455Z + 
2020-04-24T13:19:20.7723002Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7723710Z +    |
2020-04-24T13:19:20.7723710Z +    |
2020-04-24T13:19:20.7723911Z + LL |         asm!("{}", inout(reg) v[..]);
2020-04-24T13:19:20.7724235Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7724664Z 1 error: invalid asm output
2020-04-24T13:19:20.7725055Z 2   --> $DIR/type-check-1.rs:10:29
2020-04-24T13:19:20.7725240Z 3    |
2020-04-24T13:19:20.7725339Z 
2020-04-24T13:19:20.7725339Z 
2020-04-24T13:19:20.7725968Z 40    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2020-04-24T13:19:20.7726426Z 41    = note: all inline asm arguments must have a statically known size
2020-04-24T13:19:20.7726654Z 42 
2020-04-24T13:19:20.7727025Z - error: aborting due to 5 previous errors
2020-04-24T13:19:20.7727305Z + error: aborting due to 5 previous errors; 6 warnings emitted
2020-04-24T13:19:20.7727956Z 45 For more information about this error, try `rustc --explain E0277`.
2020-04-24T13:19:20.7728239Z 46 
2020-04-24T13:19:20.7728336Z 
2020-04-24T13:19:20.7728427Z 
2020-04-24T13:19:20.7728427Z 
2020-04-24T13:19:20.7728632Z The actual stderr differed from the expected stderr.
2020-04-24T13:19:20.7729285Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1/type-check-1.stderr
2020-04-24T13:19:20.7729869Z To update references, rerun the tests and pass the `--bless` flag
2020-04-24T13:19:20.7730426Z To only update this specific test, also pass `--test-args asm/type-check-1.rs`
2020-04-24T13:19:20.7730814Z error: 1 errors occurred comparing output.
2020-04-24T13:19:20.7731047Z status: exit code: 1
2020-04-24T13:19:20.7731047Z status: exit code: 1
2020-04-24T13:19:20.7732806Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/type-check-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1/auxiliary"
2020-04-24T13:19:20.7734207Z ------------------------------------------
2020-04-24T13:19:20.7734365Z 
2020-04-24T13:19:20.7734728Z ------------------------------------------
2020-04-24T13:19:20.7734918Z stderr:
2020-04-24T13:19:20.7734918Z stderr:
2020-04-24T13:19:20.7735276Z ------------------------------------------
2020-04-24T13:19:20.7735863Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7736444Z   --> /checkout/src/test/ui/asm/type-check-1.rs:9:9
2020-04-24T13:19:20.7736651Z    |
2020-04-24T13:19:20.7736843Z LL |         asm!("{}", in(reg) 1 + 2);
2020-04-24T13:19:20.7737153Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7737603Z    = note: `#[warn(deprecated)]` on by default
2020-04-24T13:19:20.7737784Z 
2020-04-24T13:19:20.7737784Z 
2020-04-24T13:19:20.7738316Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7738883Z   --> /checkout/src/test/ui/asm/type-check-1.rs:10:9
2020-04-24T13:19:20.7739106Z    |
2020-04-24T13:19:20.7739283Z LL |         asm!("{}", out(reg) 1 + 2);
2020-04-24T13:19:20.7739594Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7739837Z 
2020-04-24T13:19:20.7740364Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7740932Z   --> /checkout/src/test/ui/asm/type-check-1.rs:12:9
2020-04-24T13:19:20.7741152Z    |
2020-04-24T13:19:20.7741331Z LL |         asm!("{}", inout(reg) 1 + 2);
2020-04-24T13:19:20.7741645Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7741878Z 
2020-04-24T13:19:20.7742538Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7743111Z   --> /checkout/src/test/ui/asm/type-check-1.rs:18:9
2020-04-24T13:19:20.7743316Z    |
2020-04-24T13:19:20.7743507Z LL |         asm!("{}", in(reg) v[..]);
2020-04-24T13:19:20.7743816Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7744047Z 
2020-04-24T13:19:20.7744574Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7745141Z   --> /checkout/src/test/ui/asm/type-check-1.rs:20:9
2020-04-24T13:19:20.7745346Z    |
2020-04-24T13:19:20.7745522Z LL |         asm!("{}", out(reg) v[..]);
2020-04-24T13:19:20.7745848Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7746076Z 
2020-04-24T13:19:20.7746672Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7748450Z   --> /checkout/src/test/ui/asm/type-check-1.rs:22:9
2020-04-24T13:19:20.7748761Z    |
2020-04-24T13:19:20.7749259Z LL |         asm!("{}", inout(reg) v[..]);
2020-04-24T13:19:20.7749700Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7750163Z error: invalid asm output
2020-04-24T13:19:20.7750780Z   --> /checkout/src/test/ui/asm/type-check-1.rs:10:29
2020-04-24T13:19:20.7751048Z    |
2020-04-24T13:19:20.7751048Z    |
2020-04-24T13:19:20.7751261Z LL |         asm!("{}", out(reg) 1 + 2);
2020-04-24T13:19:20.7751842Z 
2020-04-24T13:19:20.7752024Z error: invalid asm output
2020-04-24T13:19:20.7752554Z   --> /checkout/src/test/ui/asm/type-check-1.rs:12:31
2020-04-24T13:19:20.7752818Z    |
2020-04-24T13:19:20.7752818Z    |
2020-04-24T13:19:20.7753033Z LL |         asm!("{}", inout(reg) 1 + 2);
2020-04-24T13:19:20.7753611Z 
2020-04-24T13:19:20.7753925Z error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
2020-04-24T13:19:20.7754557Z   --> /checkout/src/test/ui/asm/type-check-1.rs:18:28
2020-04-24T13:19:20.7754803Z    |
2020-04-24T13:19:20.7754803Z    |
2020-04-24T13:19:20.7755026Z LL |         asm!("{}", in(reg) v[..]);
2020-04-24T13:19:20.7755908Z    |
2020-04-24T13:19:20.7756214Z    = help: the trait `std::marker::Sized` is not implemented for `[u64]`
2020-04-24T13:19:20.7757091Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2020-04-24T13:19:20.7757631Z    = note: all inline asm arguments must have a statically known size
2020-04-24T13:19:20.7757631Z    = note: all inline asm arguments must have a statically known size
2020-04-24T13:19:20.7757890Z 
2020-04-24T13:19:20.7758194Z error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
2020-04-24T13:19:20.7758828Z   --> /checkout/src/test/ui/asm/type-check-1.rs:20:29
2020-04-24T13:19:20.7759091Z    |
2020-04-24T13:19:20.7759305Z LL |         asm!("{}", out(reg) v[..]);
2020-04-24T13:19:20.7760204Z    |
2020-04-24T13:19:20.7760495Z    = help: the trait `std::marker::Sized` is not implemented for `[u64]`
2020-04-24T13:19:20.7761368Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2020-04-24T13:19:20.7761892Z    = note: all inline asm arguments must have a statically known size
2020-04-24T13:19:20.7761892Z    = note: all inline asm arguments must have a statically known size
2020-04-24T13:19:20.7762123Z 
2020-04-24T13:19:20.7762413Z error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
2020-04-24T13:19:20.7763120Z   --> /checkout/src/test/ui/asm/type-check-1.rs:22:31
2020-04-24T13:19:20.7763333Z    |
2020-04-24T13:19:20.7763511Z LL |         asm!("{}", inout(reg) v[..]);
2020-04-24T13:19:20.7764276Z    |
2020-04-24T13:19:20.7764526Z    = help: the trait `std::marker::Sized` is not implemented for `[u64]`
2020-04-24T13:19:20.7765254Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2020-04-24T13:19:20.7765714Z    = note: all inline asm arguments must have a statically known size
2020-04-24T13:19:20.7765714Z    = note: all inline asm arguments must have a statically known size
2020-04-24T13:19:20.7765915Z 
2020-04-24T13:19:20.7766119Z error: aborting due to 5 previous errors; 6 warnings emitted
2020-04-24T13:19:20.7766734Z For more information about this error, try `rustc --explain E0277`.
2020-04-24T13:19:20.7766928Z 
2020-04-24T13:19:20.7767277Z ------------------------------------------
2020-04-24T13:19:20.7767448Z 
2020-04-24T13:19:20.7767448Z 
2020-04-24T13:19:20.7767540Z 
2020-04-24T13:19:20.7768045Z ---- [ui] ui/asm/type-check-2.rs stdout ----
2020-04-24T13:19:20.7768257Z diff of stderr:
2020-04-24T13:19:20.7768397Z 
2020-04-24T13:19:20.7768995Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7769735Z +    |
2020-04-24T13:19:20.7769735Z +    |
2020-04-24T13:19:20.7769912Z + LL |         asm!("{}", in(reg) x);
2020-04-24T13:19:20.7770224Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7771818Z +    = note: `#[warn(deprecated)]` on by default
2020-04-24T13:19:20.7772010Z + 
2020-04-24T13:19:20.7772010Z + 
2020-04-24T13:19:20.7772606Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7773339Z +    |
2020-04-24T13:19:20.7773339Z +    |
2020-04-24T13:19:20.7773520Z + LL |         asm!("{}", inout(reg) y);
2020-04-24T13:19:20.7773861Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7774116Z + 
2020-04-24T13:19:20.7774662Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7775390Z +    |
2020-04-24T13:19:20.7775390Z +    |
2020-04-24T13:19:20.7775570Z + LL |         asm!("{}", in(reg) v[0]);
2020-04-24T13:19:20.7775898Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7776154Z + 
2020-04-24T13:19:20.7776695Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7777426Z +    |
2020-04-24T13:19:20.7777426Z +    |
2020-04-24T13:19:20.7777605Z + LL |         asm!("{}", out(reg) v[0]);
2020-04-24T13:19:20.7777934Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7778189Z + 
2020-04-24T13:19:20.7778735Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7779459Z +    |
2020-04-24T13:19:20.7779459Z +    |
2020-04-24T13:19:20.7779647Z + LL |         asm!("{}", inout(reg) v[0]);
2020-04-24T13:19:20.7779979Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7780234Z + 
2020-04-24T13:19:20.7780771Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.7781496Z +    |
2020-04-24T13:19:20.7781496Z +    |
2020-04-24T13:19:20.7781671Z + LL |         asm!("{}", const 0i32);
2020-04-24T13:19:20.7781995Z +    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.7782256Z + 
2020-04-24T13:19:20.7782794Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
---
2020-04-24T13:19:20.8159344Z 99 LL |     trace_macros!(invalid);
2020-04-24T13:19:20.8159576Z 100    |     ^^^^^^^^^^^^^^^^^^^^^^^
2020-04-24T13:19:20.8159744Z 101 
2020-04-24T13:19:20.8160107Z - error: aborting due to 15 previous errors
2020-04-24T13:19:20.8160729Z + warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.8161482Z +    |
2020-04-24T13:19:20.8161482Z +    |
2020-04-24T13:19:20.8161655Z + LL |     asm!(invalid);
2020-04-24T13:19:20.8161946Z +    |     ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.8162413Z +    = note: `#[warn(deprecated)]` on by default
2020-04-24T13:19:20.8162604Z + 
2020-04-24T13:19:20.8162604Z + 
2020-04-24T13:19:20.8162815Z + error: aborting due to 15 previous errors; 1 warning emitted
2020-04-24T13:19:20.8163472Z 104 For more information about this error, try `rustc --explain E0665`.
2020-04-24T13:19:20.8163695Z 105 
2020-04-24T13:19:20.8163792Z 
2020-04-24T13:19:20.8163898Z 
2020-04-24T13:19:20.8163898Z 
2020-04-24T13:19:20.8164087Z The actual stderr differed from the expected stderr.
2020-04-24T13:19:20.8164747Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors/macros-nonfatal-errors.stderr
2020-04-24T13:19:20.8165374Z To update references, rerun the tests and pass the `--bless` flag
2020-04-24T13:19:20.8165949Z To only update this specific test, also pass `--test-args macros/macros-nonfatal-errors.rs`
2020-04-24T13:19:20.8166373Z error: 1 errors occurred comparing output.
2020-04-24T13:19:20.8166591Z status: exit code: 1
2020-04-24T13:19:20.8166591Z status: exit code: 1
2020-04-24T13:19:20.8168426Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macros-nonfatal-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors/auxiliary"
2020-04-24T13:19:20.8169882Z ------------------------------------------
2020-04-24T13:19:20.8170054Z 
2020-04-24T13:19:20.8170406Z ------------------------------------------
2020-04-24T13:19:20.8170596Z stderr:
---
2020-04-24T13:19:20.8173357Z 
2020-04-24T13:19:20.8173538Z error: asm template must be a string literal
2020-04-24T13:19:20.8174051Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:13:10
2020-04-24T13:19:20.8174346Z    |
2020-04-24T13:19:20.8174511Z LL |     asm!(invalid); //~ ERROR
2020-04-24T13:19:20.8174841Z 
2020-04-24T13:19:20.8175026Z error: inline assembly must be a string literal
2020-04-24T13:19:20.8175609Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:14:15
2020-04-24T13:19:20.8175844Z    |
2020-04-24T13:19:20.8175844Z    |
2020-04-24T13:19:20.8176017Z LL |     llvm_asm!(invalid); //~ ERROR
2020-04-24T13:19:20.8176371Z 
2020-04-24T13:19:20.8176371Z 
2020-04-24T13:19:20.8176546Z error: concat_idents! requires ident args.
2020-04-24T13:19:20.8177281Z    |
2020-04-24T13:19:20.8177281Z    |
2020-04-24T13:19:20.8177476Z LL |     concat_idents!("not", "idents"); //~ ERROR
2020-04-24T13:19:20.8177877Z 
2020-04-24T13:19:20.8178065Z error: argument must be a string literal
2020-04-24T13:19:20.8178557Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:18:17
2020-04-24T13:19:20.8178792Z    |
2020-04-24T13:19:20.8178792Z    |
2020-04-24T13:19:20.8178983Z LL |     option_env!(invalid); //~ ERROR
2020-04-24T13:19:20.8179330Z 
2020-04-24T13:19:20.8179507Z error: expected string literal
2020-04-24T13:19:20.8179980Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:19:10
2020-04-24T13:19:20.8180209Z    |
2020-04-24T13:19:20.8180209Z    |
2020-04-24T13:19:20.8180374Z LL |     env!(invalid); //~ ERROR
2020-04-24T13:19:20.8180704Z 
2020-04-24T13:19:20.8180862Z error: expected string literal
2020-04-24T13:19:20.8181349Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:20:10
2020-04-24T13:19:20.8181582Z    |
2020-04-24T13:19:20.8181582Z    |
2020-04-24T13:19:20.8181756Z LL |     env!(foo, abr, baz); //~ ERROR
2020-04-24T13:19:20.8182183Z 
2020-04-24T13:19:20.8182183Z 
2020-04-24T13:19:20.8182411Z error: environment variable `RUST_HOPEFULLY_THIS_DOESNT_EXIST` not defined
2020-04-24T13:19:20.8183214Z    |
2020-04-24T13:19:20.8183214Z    |
2020-04-24T13:19:20.8183424Z LL |     env!("RUST_HOPEFULLY_THIS_DOESNT_EXIST"); //~ ERROR
2020-04-24T13:19:20.8183890Z 
2020-04-24T13:19:20.8184073Z error: format argument must be a string literal
2020-04-24T13:19:20.8184573Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:23:13
2020-04-24T13:19:20.8184803Z    |
2020-04-24T13:19:20.8184803Z    |
2020-04-24T13:19:20.8184987Z LL |     format!(invalid); //~ ERROR
2020-04-24T13:19:20.8185339Z    |
2020-04-24T13:19:20.8185563Z help: you might be missing a string literal to format with
2020-04-24T13:19:20.8185769Z    |
2020-04-24T13:19:20.8185947Z LL |     format!("{}", invalid); //~ ERROR
2020-04-24T13:19:20.8185947Z LL |     format!("{}", invalid); //~ ERROR
2020-04-24T13:19:20.8186167Z    |             ^^^^^
2020-04-24T13:19:20.8186290Z 
2020-04-24T13:19:20.8186462Z error: argument must be a string literal
2020-04-24T13:19:20.8186955Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:25:14
2020-04-24T13:19:20.8187200Z    |
2020-04-24T13:19:20.8187371Z LL |     include!(invalid); //~ ERROR
2020-04-24T13:19:20.8187724Z 
2020-04-24T13:19:20.8187897Z error: argument must be a string literal
2020-04-24T13:19:20.8188386Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:27:18
2020-04-24T13:19:20.8188630Z    |
2020-04-24T13:19:20.8188630Z    |
2020-04-24T13:19:20.8188808Z LL |     include_str!(invalid); //~ ERROR
2020-04-24T13:19:20.8189023Z    |                  ^^^^^^^
2020-04-24T13:19:20.8189159Z 
2020-04-24T13:19:20.8189808Z error: couldn't read /checkout/src/test/ui/macros/i'd be quite surprised if a file with this name existed: No such file or directory (os error 2)
2020-04-24T13:19:20.8190712Z    |
2020-04-24T13:19:20.8190712Z    |
2020-04-24T13:19:20.8191205Z LL |     include_str!("i'd be quite surprised if a file with this name existed"); //~ ERROR
2020-04-24T13:19:20.8191883Z    |
2020-04-24T13:19:20.8192475Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-04-24T13:19:20.8192739Z 
2020-04-24T13:19:20.8192913Z error: argument must be a string literal
2020-04-24T13:19:20.8192913Z error: argument must be a string literal
2020-04-24T13:19:20.8193426Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:29:20
2020-04-24T13:19:20.8193655Z    |
2020-04-24T13:19:20.8193836Z LL |     include_bytes!(invalid); //~ ERROR
2020-04-24T13:19:20.8194072Z    |                    ^^^^^^^
2020-04-24T13:19:20.8194214Z 
2020-04-24T13:19:20.8194848Z error: couldn't read /checkout/src/test/ui/macros/i'd be quite surprised if a file with this name existed: No such file or directory (os error 2)
2020-04-24T13:19:20.8195763Z    |
2020-04-24T13:19:20.8195763Z    |
2020-04-24T13:19:20.8196246Z LL |     include_bytes!("i'd be quite surprised if a file with this name existed"); //~ ERROR
2020-04-24T13:19:20.8196879Z    |
2020-04-24T13:19:20.8197396Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-04-24T13:19:20.8197656Z 
2020-04-24T13:19:20.8197862Z error: trace_macros! accepts only `true` or `false`
2020-04-24T13:19:20.8197862Z error: trace_macros! accepts only `true` or `false`
2020-04-24T13:19:20.8198364Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:32:5
2020-04-24T13:19:20.8198592Z    |
2020-04-24T13:19:20.8198786Z LL |     trace_macros!(invalid); //~ ERROR
2020-04-24T13:19:20.8199147Z 
2020-04-24T13:19:20.8199147Z 
2020-04-24T13:19:20.8199674Z warning: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-24T13:19:20.8200526Z    |
2020-04-24T13:19:20.8200526Z    |
2020-04-24T13:19:20.8200690Z LL |     asm!(invalid); //~ ERROR
2020-04-24T13:19:20.8201000Z    |     ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-24T13:19:20.8201439Z    = note: `#[warn(deprecated)]` on by default
2020-04-24T13:19:20.8201620Z 
2020-04-24T13:19:20.8201620Z 
2020-04-24T13:19:20.8201825Z error: aborting due to 15 previous errors; 1 warning emitted
2020-04-24T13:19:20.8202424Z For more information about this error, try `rustc --explain E0665`.
2020-04-24T13:19:20.8202632Z 
2020-04-24T13:19:20.8202980Z ------------------------------------------
2020-04-24T13:19:20.8203135Z 
2020-04-24T13:19:20.8203135Z 
2020-04-24T13:19:20.8203225Z 
2020-04-24T13:19:20.8203597Z ---- [ui] ui/target-feature/gate.rs stdout ----
2020-04-24T13:19:20.8203814Z diff of stderr:
2020-04-24T13:19:20.8203936Z 
2020-04-24T13:19:20.8204155Z 1 error[E0658]: the target feature `avx512bw` is currently unstable
2020-04-24T13:19:20.8204601Z -   --> $DIR/gate.rs:29:18
2020-04-24T13:19:20.8204968Z +   --> $DIR/gate.rs:30:18
2020-04-24T13:19:20.8205136Z 3    |
2020-04-24T13:19:20.8205340Z 4 LL | #[target_feature(enable = "avx512bw")]
2020-04-24T13:19:20.8205743Z 
2020-04-24T13:19:20.8205833Z 
2020-04-24T13:19:20.8206037Z The actual stderr differed from the expected stderr.
2020-04-24T13:19:20.8206629Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/gate/gate.stderr
2020-04-24T13:19:20.8206629Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/gate/gate.stderr
2020-04-24T13:19:20.8207194Z To update references, rerun the tests and pass the `--bless` flag
2020-04-24T13:19:20.8207754Z To only update this specific test, also pass `--test-args target-feature/gate.rs`
2020-04-24T13:19:20.8208145Z error: 1 errors occurred comparing output.
2020-04-24T13:19:20.8208374Z status: exit code: 1
2020-04-24T13:19:20.8208374Z status: exit code: 1
2020-04-24T13:19:20.8210202Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature/gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/gate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/gate/auxiliary"
2020-04-24T13:19:20.8211673Z ------------------------------------------
2020-04-24T13:19:20.8211832Z 
2020-04-24T13:19:20.8212200Z ------------------------------------------
2020-04-24T13:19:20.8212389Z stderr:
2020-04-24T13:19:20.8212389Z stderr:
2020-04-24T13:19:20.8212749Z ------------------------------------------
2020-04-24T13:19:20.8213043Z error[E0658]: the target feature `avx512bw` is currently unstable
2020-04-24T13:19:20.8213762Z    |
2020-04-24T13:19:20.8213762Z    |
2020-04-24T13:19:20.8213959Z LL | #[target_feature(enable = "avx512bw")]
2020-04-24T13:19:20.8214369Z    |
2020-04-24T13:19:20.8214369Z    |
2020-04-24T13:19:20.8215028Z    = note: see issue #44839 <***/issues/44839> for more information
2020-04-24T13:19:20.8215385Z    = help: add `#![feature(avx512_target_feature)]` to the crate attributes to enable
2020-04-24T13:19:20.8215796Z error: aborting due to previous error
2020-04-24T13:19:20.8215947Z 
2020-04-24T13:19:20.8216369Z For more information about this error, try `rustc --explain E0658`.
2020-04-24T13:19:20.8216561Z 
---
2020-04-24T13:19:20.8222763Z 
2020-04-24T13:19:20.8223234Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-24T13:19:20.8223462Z 
2020-04-24T13:19:20.8223567Z 
2020-04-24T13:19:20.8227045Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-24T13:19:20.8229524Z 
2020-04-24T13:19:20.8229617Z 
2020-04-24T13:19:20.8230108Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-24T13:19:20.8230461Z Build completed unsuccessfully in 0:57:52
2020-04-24T13:19:20.8230461Z Build completed unsuccessfully in 0:57:52
2020-04-24T13:19:20.8230692Z == clock drift check ==
2020-04-24T13:19:20.8230916Z   local time: Fri Apr 24 13:19:20 UTC 2020
2020-04-24T13:19:21.0192328Z   network time: Fri, 24 Apr 2020 13:19:21 GMT
2020-04-24T13:19:21.3733625Z 
2020-04-24T13:19:21.3733625Z 
2020-04-24T13:19:21.3768836Z ##[error]Bash exited with code '1'.
2020-04-24T13:19:21.3802792Z ##[section]Finishing: Run build
2020-04-24T13:19:21.3847956Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69171/merge to s
2020-04-24T13:19:21.3852908Z Task         : Get sources
2020-04-24T13:19:21.3853244Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T13:19:21.3853530Z Version      : 1.0.0
2020-04-24T13:19:21.3853735Z Author       : Microsoft
2020-04-24T13:19:21.3853735Z Author       : Microsoft
2020-04-24T13:19:21.3854079Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T13:19:21.3854442Z ==============================================================================
2020-04-24T13:19:21.7230879Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T13:19:21.7292445Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69171/merge to s
2020-04-24T13:19:21.7378290Z Cleaning up task key
2020-04-24T13:19:21.7379554Z Start cleaning up orphan processes.
2020-04-24T13:19:21.7556645Z Terminate orphan process: pid (3912) (python)
2020-04-24T13:19:21.7785053Z ##[section]Finishing: Finalize Job
