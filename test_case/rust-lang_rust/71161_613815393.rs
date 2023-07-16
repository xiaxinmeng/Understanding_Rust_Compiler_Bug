plain
2020-04-15T03:42:54.7455528Z ========================== Starting Command Output ===========================
2020-04-15T03:42:54.7476727Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8ff71d41-ca31-40fd-9cf6-a550ccd4d521.sh
2020-04-15T03:42:54.7748434Z 
2020-04-15T03:42:54.7839947Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-15T03:42:54.7870896Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71161/merge to s
2020-04-15T03:42:54.7887472Z Task         : Get sources
2020-04-15T03:42:54.7887991Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T03:42:54.7888508Z Version      : 1.0.0
2020-04-15T03:42:54.7888888Z Author       : Microsoft
---
2020-04-15T03:42:56.3914240Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-15T03:42:56.3928394Z ##[command]git config gc.auto 0
2020-04-15T03:42:56.3972906Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-15T03:42:56.3979494Z ##[command]git config --get-all http.proxy
2020-04-15T03:42:56.3991536Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71161/merge:refs/remotes/pull/71161/merge
---
2020-04-15T03:45:03.7776863Z  ---> f58a2bb1e753
2020-04-15T03:45:03.7777558Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-15T03:45:03.7778122Z  ---> Using cache
2020-04-15T03:45:03.7778426Z  ---> d079cc6b6db8
2020-04-15T03:45:03.7779267Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-15T03:45:03.7780270Z  ---> 4183ca46ee56
2020-04-15T03:45:03.7780478Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-15T03:45:03.7780791Z  ---> Using cache
2020-04-15T03:45:03.7781079Z  ---> 69e7f8a2a2fb
---
2020-04-15T03:45:03.8160262Z Looks like docker image is the same as before, not uploading
2020-04-15T03:45:11.4385050Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-15T03:45:11.4712338Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-15T03:45:11.4740587Z == clock drift check ==
2020-04-15T03:45:11.4750925Z   local time: Wed Apr 15 03:45:11 UTC 2020
2020-04-15T03:45:11.8059667Z   network time: Wed, 15 Apr 2020 03:45:11 GMT
2020-04-15T03:45:11.8069794Z Starting sccache server...
2020-04-15T03:45:11.9067346Z configure: processing command line
2020-04-15T03:45:11.9068636Z configure: 
2020-04-15T03:45:11.9069782Z configure: rust.dist-src        := False
---
2020-04-15T03:50:44.3975647Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T03:50:46.0435351Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T03:50:47.7735911Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-15T03:50:49.2444370Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T03:50:58.8927867Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T03:51:01.5612020Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-15T03:51:06.2384311Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-15T03:51:10.6738871Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-15T03:51:20.8414508Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-15T04:15:02.6752364Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T04:15:04.5890433Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T04:15:06.7371554Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-15T04:15:09.3442887Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T04:15:20.4038149Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T04:15:25.3546440Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-15T04:15:31.6336710Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-15T04:15:37.8738842Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-15T04:15:49.0139772Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-15T04:42:16.0694867Z .................................................................................................... 1700/9897
2020-04-15T04:42:20.3845923Z .................................................................................................... 1800/9897
2020-04-15T04:42:28.6427285Z .................................................................................................... 1900/9897
2020-04-15T04:42:36.9329579Z ......i............................................................................................. 2000/9897
2020-04-15T04:42:43.3011349Z ................................................................................................iiii 2100/9897
2020-04-15T04:42:58.0922796Z i................................................................................................... 2200/9897
2020-04-15T04:43:06.1622396Z .................................................................................................... 2400/9897
2020-04-15T04:43:08.4447014Z .................................................................................................... 2500/9897
2020-04-15T04:43:14.3144380Z .................................................................................................... 2600/9897
2020-04-15T04:43:33.7569453Z .................................................................................................... 2700/9897
---
2020-04-15T04:46:17.1128179Z .................................................................................................... 5100/9897
2020-04-15T04:46:24.8528563Z .................................................................................................... 5200/9897
2020-04-15T04:46:29.5783603Z .................i.................................................................................. 5300/9897
2020-04-15T04:46:39.7823567Z .......i............................................................................................ 5400/9897
2020-04-15T04:46:45.3134983Z .......ii.ii........i...i........................................................................... 5500/9897
2020-04-15T04:46:53.2296939Z .....................................................i.............................................. 5700/9897
2020-04-15T04:47:03.3186797Z .........................................................................ii......................... 5800/9897
2020-04-15T04:47:10.0128385Z ............i....................................................................................... 5900/9897
2020-04-15T04:47:15.3938430Z .................................................................................................... 6000/9897
2020-04-15T04:47:15.3938430Z .................................................................................................... 6000/9897
2020-04-15T04:47:25.8046862Z .................................................................................................... 6100/9897
2020-04-15T04:47:36.5513265Z ......ii...i..ii...........i........................................................................ 6200/9897
2020-04-15T04:47:51.7164563Z .................................................................................................... 6400/9897
2020-04-15T04:47:57.8025645Z .................................................................................................... 6500/9897
2020-04-15T04:47:57.8025645Z .................................................................................................... 6500/9897
2020-04-15T04:48:12.7183885Z ....................................i..ii........................................................... 6600/9897
2020-04-15T04:48:33.9656946Z .................................................................................................... 6800/9897
2020-04-15T04:48:35.9125130Z ....................................i............................................................... 6900/9897
2020-04-15T04:48:38.1346069Z ...............................................................................F.................... 7000/9897
2020-04-15T04:48:40.1947866Z ............................................................................i....................... 7100/9897
---
2020-04-15T04:50:16.5842174Z .................................................................................................... 7800/9897
2020-04-15T04:50:20.7233557Z .................................................................................................... 7900/9897
2020-04-15T04:50:27.2497776Z .................................................................................................... 8000/9897
2020-04-15T04:50:33.4469364Z ..........................................i......................................................... 8100/9897
2020-04-15T04:50:42.6900905Z ..........................................................................................iiiiii.iii 8200/9897
2020-04-15T04:50:48.7258505Z ii.i................................................................................................ 8300/9897
2020-04-15T04:51:01.3687433Z .................................................................................................... 8500/9897
2020-04-15T04:51:10.7350883Z .................................................................................................... 8600/9897
2020-04-15T04:51:23.7941518Z .................................................................................................... 8700/9897
2020-04-15T04:51:30.2053017Z .................................................................................................... 8800/9897
---
2020-04-15T04:53:17.7833683Z 108 
2020-04-15T04:53:17.7834228Z - error[E0493]: destructors cannot be evaluated at compile-time
2020-04-15T04:53:17.7834779Z -   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:13
2020-04-15T04:53:17.7835395Z -    |
2020-04-15T04:53:17.7835939Z - LL |         V = [PhantomData; { [ () ].len() ].len() as isize,
2020-04-15T04:53:17.7837080Z - 
2020-04-15T04:53:17.7838055Z 115 error[E0282]: type annotations needed
2020-04-15T04:53:17.7838621Z 116   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:16:29
2020-04-15T04:53:17.7838870Z 117    |
2020-04-15T04:53:17.7838870Z 117    |
2020-04-15T04:53:17.7838986Z 
2020-04-15T04:53:17.7839195Z 124 LL |         V = [Vec::new; { [0].len() ].len() as isize,
2020-04-15T04:53:17.7839719Z 126 
2020-04-15T04:53:17.7840086Z - error: aborting due to 15 previous errors
2020-04-15T04:53:17.7840330Z + error: aborting due to 14 previous errors
2020-04-15T04:53:17.7840506Z 128 
2020-04-15T04:53:17.7840506Z 128 
2020-04-15T04:53:17.7841016Z - Some errors have detailed explanations: E0282, E0493.
2020-04-15T04:53:17.7841488Z - For more information about an error, try `rustc --explain E0282`.
2020-04-15T04:53:17.7841961Z + For more information about this error, try `rustc --explain E0282`.
2020-04-15T04:53:17.7842285Z 131 
2020-04-15T04:53:17.7842377Z 
2020-04-15T04:53:17.7842576Z 
2020-04-15T04:53:17.7842748Z The actual stderr differed from the expected stderr.
2020-04-15T04:53:17.7843596Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant/issue-67377-invalid-syntax-in-enum-discriminant.stderr
2020-04-15T04:53:17.7844359Z To update references, rerun the tests and pass the `--bless` flag
2020-04-15T04:53:17.7845093Z To only update this specific test, also pass `--test-args parser/issue-67377-invalid-syntax-in-enum-discriminant.rs`
2020-04-15T04:53:17.7845538Z error: 1 errors occurred comparing output.
2020-04-15T04:53:17.7845754Z status: exit code: 1
2020-04-15T04:53:17.7845754Z status: exit code: 1
2020-04-15T04:53:17.7847782Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant/auxiliary"
2020-04-15T04:53:17.7851679Z ------------------------------------------
2020-04-15T04:53:17.7852019Z 
2020-04-15T04:53:17.7853212Z ------------------------------------------
2020-04-15T04:53:17.7853416Z stderr:
2020-04-15T04:53:17.7853416Z stderr:
2020-04-15T04:53:17.7854906Z ------------------------------------------
2020-04-15T04:53:17.7855163Z error: mismatched closing delimiter: `]`
2020-04-15T04:53:17.7855758Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:42
2020-04-15T04:53:17.7860874Z    |
2020-04-15T04:53:17.7861459Z LL |         V = [PhantomData; { [ () ].len() ].len() as isize,
2020-04-15T04:53:17.7862419Z    |             |             |
2020-04-15T04:53:17.7862668Z    |             |             unclosed delimiter
2020-04-15T04:53:17.7862941Z    |             closing delimiter possibly meant for this
2020-04-15T04:53:17.7863122Z 
2020-04-15T04:53:17.7863122Z 
2020-04-15T04:53:17.7863308Z error: mismatched closing delimiter: `]`
2020-04-15T04:53:17.7863905Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:16:36
2020-04-15T04:53:17.7864186Z    |
2020-04-15T04:53:17.7864402Z LL |         V = [Vec::new; { [].len()  ].len() as isize,
2020-04-15T04:53:17.7865184Z    |             |          |
2020-04-15T04:53:17.7865405Z    |             |          unclosed delimiter
2020-04-15T04:53:17.7865693Z    |             closing delimiter possibly meant for this
2020-04-15T04:53:17.7866025Z 
2020-04-15T04:53:17.7866025Z 
2020-04-15T04:53:17.7866215Z error: mismatched closing delimiter: `]`
2020-04-15T04:53:17.7867107Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:27:36
2020-04-15T04:53:17.7867422Z    |
2020-04-15T04:53:17.7867636Z LL |         V = [Vec::new; { [0].len() ].len() as isize,
2020-04-15T04:53:17.7872208Z    |             |          |
2020-04-15T04:53:17.7872432Z    |             |          unclosed delimiter
2020-04-15T04:53:17.7873973Z    |             closing delimiter possibly meant for this
2020-04-15T04:53:17.7874204Z 
2020-04-15T04:53:17.7874204Z 
2020-04-15T04:53:17.7875097Z error: mismatched closing delimiter: `]`
2020-04-15T04:53:17.7929489Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:42
2020-04-15T04:53:17.7930259Z    |
2020-04-15T04:53:17.7930622Z LL |         V = [PhantomData; { [ () ].len() ].len() as isize,
2020-04-15T04:53:17.7931589Z    |             |             |
2020-04-15T04:53:17.7931822Z    |             |             unclosed delimiter
2020-04-15T04:53:17.7932095Z    |             closing delimiter possibly meant for this
2020-04-15T04:53:17.7932468Z 
2020-04-15T04:53:17.7932468Z 
2020-04-15T04:53:17.7932692Z error: mismatched closing delimiter: `]`
2020-04-15T04:53:17.7933768Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:16:36
2020-04-15T04:53:17.7934312Z    |
2020-04-15T04:53:17.7934555Z LL |         V = [Vec::new; { [].len()  ].len() as isize,
2020-04-15T04:53:17.7935626Z    |             |          |
2020-04-15T04:53:17.7935987Z    |             |          unclosed delimiter
2020-04-15T04:53:17.7936256Z    |             closing delimiter possibly meant for this
2020-04-15T04:53:17.7936448Z 
2020-04-15T04:53:17.7936448Z 
2020-04-15T04:53:17.7936771Z error: mismatched closing delimiter: `]`
2020-04-15T04:53:17.7937531Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:27:36
2020-04-15T04:53:17.7937967Z    |
2020-04-15T04:53:17.7938206Z LL |         V = [Vec::new; { [0].len() ].len() as isize,
2020-04-15T04:53:17.7939180Z    |             |          |
2020-04-15T04:53:17.7939660Z    |             |          unclosed delimiter
2020-04-15T04:53:17.7942652Z    |             closing delimiter possibly meant for this
2020-04-15T04:53:17.7942838Z 
2020-04-15T04:53:17.7942838Z 
2020-04-15T04:53:17.7943138Z error: mismatched closing delimiter: `]`
2020-04-15T04:53:17.7943973Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:42
2020-04-15T04:53:17.7944244Z    |
2020-04-15T04:53:17.7944447Z LL |         V = [PhantomData; { [ () ].len() ].len() as isize,
2020-04-15T04:53:17.7945254Z    |             |             |
2020-04-15T04:53:17.7945481Z    |             |             unclosed delimiter
2020-04-15T04:53:17.7945759Z    |             closing delimiter possibly meant for this
2020-04-15T04:53:17.7945931Z 
2020-04-15T04:53:17.7945931Z 
2020-04-15T04:53:17.7946112Z error: mismatched closing delimiter: `]`
2020-04-15T04:53:17.7946809Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:16:36
2020-04-15T04:53:17.7947090Z    |
2020-04-15T04:53:17.7947306Z LL |         V = [Vec::new; { [].len()  ].len() as isize,
2020-04-15T04:53:17.7948082Z    |             |          |
2020-04-15T04:53:17.7948306Z    |             |          unclosed delimiter
2020-04-15T04:53:17.7948570Z    |             closing delimiter possibly meant for this
2020-04-15T04:53:17.7948763Z 
2020-04-15T04:53:17.7948763Z 
2020-04-15T04:53:17.7949107Z error: mismatched closing delimiter: `]`
2020-04-15T04:53:17.7949718Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:27:36
2020-04-15T04:53:17.7950015Z    |
2020-04-15T04:53:17.7950228Z LL |         V = [Vec::new; { [0].len() ].len() as isize,
2020-04-15T04:53:17.7951005Z    |             |          |
2020-04-15T04:53:17.7951228Z    |             |          unclosed delimiter
2020-04-15T04:53:17.7951490Z    |             closing delimiter possibly meant for this
2020-04-15T04:53:17.7951668Z 
2020-04-15T04:53:17.7951668Z 
2020-04-15T04:53:17.7951871Z error: mismatched closing delimiter: `]`
2020-04-15T04:53:17.7952436Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:42
2020-04-15T04:53:17.7952715Z    |
2020-04-15T04:53:17.7952942Z LL |         V = [PhantomData; { [ () ].len() ].len() as isize,
2020-04-15T04:53:17.7953826Z    |             |             |
2020-04-15T04:53:17.7954074Z    |             |             unclosed delimiter
2020-04-15T04:53:17.7954345Z    |             closing delimiter possibly meant for this
2020-04-15T04:53:17.7954524Z 
2020-04-15T04:53:17.7954524Z 
2020-04-15T04:53:17.7954724Z error: mismatched closing delimiter: `]`
2020-04-15T04:53:17.7955319Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:16:36
2020-04-15T04:53:17.7955818Z    |
2020-04-15T04:53:17.7956057Z LL |         V = [Vec::new; { [].len()  ].len() as isize,
2020-04-15T04:53:17.7957095Z    |             |          |
2020-04-15T04:53:17.7957562Z    |             |          unclosed delimiter
2020-04-15T04:53:17.7957831Z    |             closing delimiter possibly meant for this
2020-04-15T04:53:17.7958129Z 
2020-04-15T04:53:17.7958129Z 
2020-04-15T04:53:17.7958331Z error: mismatched closing delimiter: `]`
2020-04-15T04:53:17.7959161Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:27:36
2020-04-15T04:53:17.7959450Z    |
2020-04-15T04:53:17.7959784Z LL |         V = [Vec::new; { [0].len() ].len() as isize,
2020-04-15T04:53:17.7960623Z    |             |          |
2020-04-15T04:53:17.7960844Z    |             |          unclosed delimiter
2020-04-15T04:53:17.7961126Z    |             closing delimiter possibly meant for this
2020-04-15T04:53:17.7961463Z 
2020-04-15T04:53:17.7961463Z 
2020-04-15T04:53:17.7961643Z error[E0282]: type annotations needed
2020-04-15T04:53:17.7962539Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:16:29
2020-04-15T04:53:17.7962969Z    |
2020-04-15T04:53:17.7963179Z LL |         V = [Vec::new; { [].len()  ].len() as isize,
2020-04-15T04:53:17.7963723Z 
2020-04-15T04:53:17.7963888Z error[E0282]: type annotations needed
2020-04-15T04:53:17.7980461Z   --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:27:14
2020-04-15T04:53:17.7980849Z    |
2020-04-15T04:53:17.7980849Z    |
2020-04-15T04:53:17.7981344Z LL |         V = [Vec::new; { [0].len() ].len() as isize,
2020-04-15T04:53:17.7981916Z 
2020-04-15T04:53:17.7982135Z error: aborting due to 14 previous errors
2020-04-15T04:53:17.7982312Z 
2020-04-15T04:53:17.7983112Z For more information about this error, try `rustc --explain E0282`.
---
2020-04-15T04:53:17.7986699Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-15T04:53:17.7987136Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-15T04:53:17.7987388Z 
2020-04-15T04:53:17.7988341Z 
2020-04-15T04:53:17.7993283Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-15T04:53:17.7996472Z 
2020-04-15T04:53:17.7996579Z 
2020-04-15T04:53:17.7997829Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-15T04:53:17.7998262Z Build completed unsuccessfully in 1:06:26
2020-04-15T04:53:17.7998262Z Build completed unsuccessfully in 1:06:26
2020-04-15T04:53:17.7998520Z == clock drift check ==
2020-04-15T04:53:17.7999013Z   local time: Wed Apr 15 04:53:17 UTC 2020
2020-04-15T04:53:17.9791397Z   network time: Wed, 15 Apr 2020 04:53:17 GMT
2020-04-15T04:53:18.3774972Z 
2020-04-15T04:53:18.3774972Z 
2020-04-15T04:53:18.3874032Z ##[error]Bash exited with code '1'.
2020-04-15T04:53:18.3890072Z ##[section]Finishing: Run build
2020-04-15T04:53:18.3952043Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71161/merge to s
2020-04-15T04:53:18.3958131Z Task         : Get sources
2020-04-15T04:53:18.3958483Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T04:53:18.3958822Z Version      : 1.0.0
2020-04-15T04:53:18.3959053Z Author       : Microsoft
2020-04-15T04:53:18.3959053Z Author       : Microsoft
2020-04-15T04:53:18.3959418Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-15T04:53:18.3959863Z ==============================================================================
2020-04-15T04:53:18.7443966Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-15T04:53:18.7486016Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71161/merge to s
2020-04-15T04:53:18.7583998Z Cleaning up task key
2020-04-15T04:53:18.7585358Z Start cleaning up orphan processes.
2020-04-15T04:53:18.7793391Z Terminate orphan process: pid (3401) (python)
2020-04-15T04:53:18.8057895Z ##[section]Finishing: Finalize Job
