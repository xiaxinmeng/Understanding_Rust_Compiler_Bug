plain
2020-04-15T00:52:16.0854236Z ========================== Starting Command Output ===========================
2020-04-15T00:52:16.0856408Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8b041405-3ad2-4fb8-8070-50b7d26647b8.sh
2020-04-15T00:52:16.0856625Z 
2020-04-15T00:52:16.0860374Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-15T00:52:16.0877559Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-15T00:52:16.0881932Z Task         : Get sources
2020-04-15T00:52:16.0882446Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T00:52:16.0882719Z Version      : 1.0.0
2020-04-15T00:52:16.0882924Z Author       : Microsoft
---
2020-04-15T00:52:17.4281034Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-15T00:52:17.4290506Z ##[command]git config gc.auto 0
2020-04-15T00:52:17.4295772Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-15T00:52:17.4298032Z ##[command]git config --get-all http.proxy
2020-04-15T00:52:17.4303044Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71158/merge:refs/remotes/pull/71158/merge
---
2020-04-15T00:54:24.1672671Z  ---> f58a2bb1e753
2020-04-15T00:54:24.1674547Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-15T00:54:24.1681605Z  ---> Using cache
2020-04-15T00:54:24.1682243Z  ---> d079cc6b6db8
2020-04-15T00:54:24.1684017Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-15T00:54:24.1691233Z  ---> 4183ca46ee56
2020-04-15T00:54:24.1691627Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-15T00:54:24.1696400Z  ---> Using cache
2020-04-15T00:54:24.1696640Z  ---> 69e7f8a2a2fb
---
2020-04-15T00:54:24.2058580Z Looks like docker image is the same as before, not uploading
2020-04-15T00:54:30.4487490Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-15T00:54:30.9647933Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-15T00:54:30.9649578Z == clock drift check ==
2020-04-15T00:54:30.9649942Z   local time: Wed Apr 15 00:54:30 UTC 2020
2020-04-15T00:54:30.9650374Z   network time: Wed, 15 Apr 2020 00:54:30 GMT
2020-04-15T00:54:30.9651027Z Starting sccache server...
2020-04-15T00:54:30.9651397Z configure: processing command line
2020-04-15T00:54:30.9651693Z configure: 
2020-04-15T00:54:30.9652299Z configure: rust.dist-src        := False
---
2020-04-15T00:58:48.6868675Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T00:58:49.8257141Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T00:58:51.0210404Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-15T00:58:52.0989914Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T00:58:59.1041076Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T00:59:01.0541957Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-15T00:59:04.5737457Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-15T00:59:07.8874298Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-15T00:59:15.5575175Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-15T01:17:22.5704523Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T01:17:23.9993961Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T01:17:25.5804082Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-15T01:17:26.5367237Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T01:17:35.8870887Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T01:17:37.7460196Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-15T01:17:41.9963212Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-15T01:17:46.4974652Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-15T01:17:56.3090588Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-15T01:38:49.5174905Z .................................................................................................... 1700/9895
2020-04-15T01:38:53.1475595Z .................................................................................................... 1800/9895
2020-04-15T01:39:00.1493369Z .................................................................................................... 1900/9895
2020-04-15T01:39:07.3400444Z ......i............................................................................................. 2000/9895
2020-04-15T01:39:12.5945861Z ................................................................................................iiii 2100/9895
2020-04-15T01:39:24.6522125Z i................................................................................................... 2200/9895
2020-04-15T01:39:31.4292345Z .................................................................................................... 2400/9895
2020-04-15T01:39:33.2830688Z .................................................................................................... 2500/9895
2020-04-15T01:39:38.1000570Z .................................................................................................... 2600/9895
2020-04-15T01:39:53.9092612Z .................................................................................................... 2700/9895
---
2020-04-15T01:42:08.3806044Z .................................................................F.................................. 5100/9895
2020-04-15T01:42:14.6835908Z .................................................................................................... 5200/9895
2020-04-15T01:42:18.5707100Z .................i.................................................................................. 5300/9895
2020-04-15T01:42:27.0888530Z .......i............................................................................................ 5400/9895
2020-04-15T01:42:31.6556051Z .......ii.ii........i...i........................................................................... 5500/9895
2020-04-15T01:42:38.2830662Z .....................................................i.............................................. 5700/9895
2020-04-15T01:42:46.7444151Z .........................................................................ii......................... 5800/9895
2020-04-15T01:42:52.2926646Z ............i....................................................................................... 5900/9895
2020-04-15T01:42:56.8264096Z .................................................................................................... 6000/9895
2020-04-15T01:42:56.8264096Z .................................................................................................... 6000/9895
2020-04-15T01:43:05.6790007Z .................................................................................................... 6100/9895
2020-04-15T01:43:14.8903328Z ......ii...i..ii...........i........................................................................ 6200/9895
2020-04-15T01:43:26.0606794Z .................................................................................................... 6400/9895
2020-04-15T01:43:28.7305194Z .................................................................................................... 6500/9895
2020-04-15T01:43:28.7305194Z .................................................................................................... 6500/9895
2020-04-15T01:43:38.5516094Z ....................................i..ii........................................................... 6600/9895
2020-04-15T01:43:56.2468111Z .................................................................................................... 6800/9895
2020-04-15T01:43:57.9033056Z ....................................i............................................................... 6900/9895
2020-04-15T01:43:59.6372057Z .................................................................................................... 7000/9895
2020-04-15T01:44:01.3523224Z ...........................................................................i........................ 7100/9895
---
2020-04-15T01:45:21.3794038Z .................................................................................................... 7800/9895
2020-04-15T01:45:24.8237383Z .................................................................................................... 7900/9895
2020-04-15T01:45:30.2247639Z .................................................................................................... 8000/9895
2020-04-15T01:45:35.5169028Z .........................................i.......................................................... 8100/9895
2020-04-15T01:45:43.3603198Z .........................................................................................iiiiii.iiii 8200/9895
2020-04-15T01:45:48.5778412Z i.i................................................................................................. 8300/9895
2020-04-15T01:45:59.7581531Z .................................................................................................... 8500/9895
2020-04-15T01:46:08.1155303Z .................................................................................................... 8600/9895
2020-04-15T01:46:19.8118590Z .................................................................................................... 8700/9895
2020-04-15T01:46:25.1865812Z .................................................................................................... 8800/9895
---
2020-04-15T01:47:58.6067695Z ---- [ui] ui/fmt/format-string-error-2.rs stdout ----
2020-04-15T01:47:58.6069291Z diff of stderr:
2020-04-15T01:47:58.6069655Z 
2020-04-15T01:47:58.6069835Z 131    |
2020-04-15T01:47:58.6070087Z 132    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6070457Z 133 
2020-04-15T01:47:58.6070944Z + warning: format string warning: expected `'}'`, found `'\n'`
2020-04-15T01:47:58.6071784Z +    |
2020-04-15T01:47:58.6072061Z + LL |     {asdf
2020-04-15T01:47:58.6072308Z +    |          ^ expected `}` in format string
2020-04-15T01:47:58.6072514Z +    |
2020-04-15T01:47:58.6072514Z +    |
2020-04-15T01:47:58.6072814Z +    = note: Placing whitespace before the `}` in a format specifier is now deprecated.
2020-04-15T01:47:58.6073142Z + 
2020-04-15T01:47:58.6073547Z 134 error: invalid format string: expected `'}'`, found `'a'`
2020-04-15T01:47:58.6082935Z 136    |
2020-04-15T01:47:58.6083127Z 
2020-04-15T01:47:58.6083127Z 
2020-04-15T01:47:58.6083348Z 147 LL |     println!("\t{}");
2020-04-15T01:47:58.6083746Z 149 
2020-04-15T01:47:58.6083746Z 149 
2020-04-15T01:47:58.6084236Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-15T01:47:58.6084577Z + error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6085419Z 152    |
2020-04-15T01:47:58.6085419Z 152    |
2020-04-15T01:47:58.6085625Z 153 LL |     println!("\x7B}\u{8} {", 1);
2020-04-15T01:47:58.6085819Z 
2020-04-15T01:47:58.6086197Z -    |                          -^ expected `'}'` in format string
2020-04-15T01:47:58.6086636Z +    |                          -^ expected `}` in format string
2020-04-15T01:47:58.6087264Z 156    |                          because of this opening brace
2020-04-15T01:47:58.6087489Z 157    |
2020-04-15T01:47:58.6087649Z 
2020-04-15T01:47:58.6087807Z 173    |
2020-04-15T01:47:58.6087807Z 173    |
2020-04-15T01:47:58.6088054Z 174    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-15T01:47:58.6088644Z - error: aborting due to 18 previous errors
2020-04-15T01:47:58.6088644Z - error: aborting due to 18 previous errors
2020-04-15T01:47:58.6088922Z + error: aborting due to 18 previous errors; 1 warning emitted
2020-04-15T01:47:58.6089316Z 178 
2020-04-15T01:47:58.6089456Z 
2020-04-15T01:47:58.6089612Z 
2020-04-15T01:47:58.6089822Z The actual stderr differed from the expected stderr.
2020-04-15T01:47:58.6089822Z The actual stderr differed from the expected stderr.
2020-04-15T01:47:58.6090357Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2/format-string-error-2.stderr
2020-04-15T01:47:58.6090886Z To update references, rerun the tests and pass the `--bless` flag
2020-04-15T01:47:58.6091371Z To only update this specific test, also pass `--test-args fmt/format-string-error-2.rs`
2020-04-15T01:47:58.6091822Z error: 1 errors occurred comparing output.
2020-04-15T01:47:58.6092048Z status: exit code: 1
2020-04-15T01:47:58.6092048Z status: exit code: 1
2020-04-15T01:47:58.6093418Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/format-string-error-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2/auxiliary"
2020-04-15T01:47:58.6094600Z ------------------------------------------
2020-04-15T01:47:58.6094792Z 
2020-04-15T01:47:58.6095127Z ------------------------------------------
2020-04-15T01:47:58.6095341Z stderr:
2020-04-15T01:47:58.6095341Z stderr:
2020-04-15T01:47:58.6095864Z ------------------------------------------
2020-04-15T01:47:58.6096167Z error: incorrect unicode escape sequence
2020-04-15T01:47:58.6096833Z   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:77:20
2020-04-15T01:47:58.6097160Z    |
2020-04-15T01:47:58.6097594Z LL |     println!("\x7B}\u8 {", 1);
2020-04-15T01:47:58.6098423Z    |                      |
2020-04-15T01:47:58.6098423Z    |                      |
2020-04-15T01:47:58.6098855Z    |                      help: format of unicode escape sequences uses braces: `\u{8}`
2020-04-15T01:47:58.6099227Z 
2020-04-15T01:47:58.6099765Z error: invalid format string: expected `'}'`, found `'a'`
2020-04-15T01:47:58.6100769Z    |
2020-04-15T01:47:58.6101024Z LL |     format!("{
2020-04-15T01:47:58.6101556Z    |              - because of this opening brace
2020-04-15T01:47:58.6101888Z LL |     a");
2020-04-15T01:47:58.6101888Z LL |     a");
2020-04-15T01:47:58.6102205Z    |     ^ expected `}` in format string
2020-04-15T01:47:58.6102486Z    |
2020-04-15T01:47:58.6102823Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6103130Z 
2020-04-15T01:47:58.6103662Z error: invalid format string: expected `'}'`, found `'b'`
2020-04-15T01:47:58.6104667Z    |
2020-04-15T01:47:58.6104922Z LL |     format!("{ \
2020-04-15T01:47:58.6105435Z    |              - because of this opening brace
2020-04-15T01:47:58.6105848Z LL | 
2020-04-15T01:47:58.6105848Z LL | 
2020-04-15T01:47:58.6106106Z LL |     b");
2020-04-15T01:47:58.6106421Z    |     ^ expected `}` in format string
2020-04-15T01:47:58.6106704Z    |
2020-04-15T01:47:58.6107036Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6107358Z 
2020-04-15T01:47:58.6107947Z error: invalid format string: expected `'}'`, found `'\\'`
2020-04-15T01:47:58.6109089Z    |
2020-04-15T01:47:58.6109089Z    |
2020-04-15T01:47:58.6109331Z LL |     format!(r#"{ \
2020-04-15T01:47:58.6109991Z    |                - ^ expected `}` in format string
2020-04-15T01:47:58.6110470Z    |                because of this opening brace
2020-04-15T01:47:58.6110676Z    |
2020-04-15T01:47:58.6110676Z    |
2020-04-15T01:47:58.6110938Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6111151Z 
2020-04-15T01:47:58.6112154Z error: invalid format string: expected `'}'`, found `'\\'`
2020-04-15T01:47:58.6113401Z    |
2020-04-15T01:47:58.6113401Z    |
2020-04-15T01:47:58.6113665Z LL |     format!(r#"{ \n
2020-04-15T01:47:58.6114198Z    |                - ^ expected `}` in format string
2020-04-15T01:47:58.6114890Z    |                because of this opening brace
2020-04-15T01:47:58.6115206Z    |
2020-04-15T01:47:58.6115206Z    |
2020-04-15T01:47:58.6115619Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6115942Z 
2020-04-15T01:47:58.6116466Z error: invalid format string: expected `'}'`, found `'e'`
2020-04-15T01:47:58.6117472Z    |
2020-04-15T01:47:58.6117731Z LL |     format!("{ \n
2020-04-15T01:47:58.6118245Z    |              - because of this opening brace
2020-04-15T01:47:58.6118577Z LL | \n
2020-04-15T01:47:58.6118577Z LL | \n
2020-04-15T01:47:58.6118827Z LL |     e");
2020-04-15T01:47:58.6119134Z    |     ^ expected `}` in format string
2020-04-15T01:47:58.6119433Z    |
2020-04-15T01:47:58.6119768Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6120074Z 
2020-04-15T01:47:58.6120608Z error: invalid format string: expected `'}'`, found `'a'`
2020-04-15T01:47:58.6126421Z    |
2020-04-15T01:47:58.6126727Z LL |     {
2020-04-15T01:47:58.6127367Z    |     - because of this opening brace
2020-04-15T01:47:58.6127858Z LL |     a");
2020-04-15T01:47:58.6127858Z LL |     a");
2020-04-15T01:47:58.6128120Z    |     ^ expected `}` in format string
2020-04-15T01:47:58.6128264Z    |
2020-04-15T01:47:58.6133901Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6134089Z 
2020-04-15T01:47:58.6134601Z error: invalid format string: expected `'}'`, found `'a'`
2020-04-15T01:47:58.6135205Z    |
2020-04-15T01:47:58.6135339Z LL |     {
2020-04-15T01:47:58.6135636Z    |     - because of this opening brace
2020-04-15T01:47:58.6135930Z LL |     a
2020-04-15T01:47:58.6135930Z LL |     a
2020-04-15T01:47:58.6136096Z    |     ^ expected `}` in format string
2020-04-15T01:47:58.6136238Z    |
2020-04-15T01:47:58.6136419Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6136586Z 
2020-04-15T01:47:58.6136939Z error: invalid format string: expected `'}'`, found `'b'`
2020-04-15T01:47:58.6137529Z    |
2020-04-15T01:47:58.6137651Z LL |     { \
2020-04-15T01:47:58.6137938Z    |     - because of this opening brace
2020-04-15T01:47:58.6138094Z LL |         \
2020-04-15T01:47:58.6138094Z LL |         \
2020-04-15T01:47:58.6138229Z LL |     b");
2020-04-15T01:47:58.6138381Z    |     ^ expected `}` in format string
2020-04-15T01:47:58.6138522Z    |
2020-04-15T01:47:58.6138716Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6138880Z 
2020-04-15T01:47:58.6139324Z error: invalid format string: expected `'}'`, found `'b'`
2020-04-15T01:47:58.6139924Z    |
2020-04-15T01:47:58.6140033Z LL |     { \
2020-04-15T01:47:58.6140317Z    |     - because of this opening brace
2020-04-15T01:47:58.6140549Z LL |         \
2020-04-15T01:47:58.6140549Z LL |         \
2020-04-15T01:47:58.6140672Z LL |     b \
2020-04-15T01:47:58.6140822Z    |     ^ expected `}` in format string
2020-04-15T01:47:58.6140976Z    |
2020-04-15T01:47:58.6141157Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6141319Z 
2020-04-15T01:47:58.6141645Z error: invalid format string: expected `'}'`, found `'\\'`
2020-04-15T01:47:58.6142244Z    |
2020-04-15T01:47:58.6142244Z    |
2020-04-15T01:47:58.6142352Z LL | raw  { \
2020-04-15T01:47:58.6142658Z    |      - ^ expected `}` in format string
2020-04-15T01:47:58.6142967Z    |      because of this opening brace
2020-04-15T01:47:58.6143120Z    |
2020-04-15T01:47:58.6143120Z    |
2020-04-15T01:47:58.6143301Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6143461Z 
2020-04-15T01:47:58.6143776Z error: invalid format string: expected `'}'`, found `'\\'`
2020-04-15T01:47:58.6146021Z    |
2020-04-15T01:47:58.6146021Z    |
2020-04-15T01:47:58.6146131Z LL | raw  { \n
2020-04-15T01:47:58.6146479Z    |      - ^ expected `}` in format string
2020-04-15T01:47:58.6146785Z    |      because of this opening brace
2020-04-15T01:47:58.6146938Z    |
2020-04-15T01:47:58.6146938Z    |
2020-04-15T01:47:58.6147118Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6147278Z 
2020-04-15T01:47:58.6147599Z error: invalid format string: expected `'}'`, found `'e'`
2020-04-15T01:47:58.6148209Z    |
2020-04-15T01:47:58.6148315Z LL |   { \n
2020-04-15T01:47:58.6148608Z    |   - because of this opening brace
2020-04-15T01:47:58.6148753Z LL | \n
2020-04-15T01:47:58.6148753Z LL | \n
2020-04-15T01:47:58.6148865Z LL |     e");
2020-04-15T01:47:58.6149030Z    |     ^ expected `}` in format string
2020-04-15T01:47:58.6149172Z    |
2020-04-15T01:47:58.6149355Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6149515Z 
2020-04-15T01:47:58.6151630Z warning: format string warning: expected `'}'`, found `'\n'`
2020-04-15T01:47:58.6159656Z    |
2020-04-15T01:47:58.6159794Z LL |     {asdf
2020-04-15T01:47:58.6159955Z    |          ^ expected `}` in format string
2020-04-15T01:47:58.6160102Z    |
2020-04-15T01:47:58.6160102Z    |
2020-04-15T01:47:58.6160319Z    = note: Placing whitespace before the `}` in a format specifier is now deprecated.
2020-04-15T01:47:58.6160507Z 
2020-04-15T01:47:58.6161187Z error: invalid format string: expected `'}'`, found `'a'`
2020-04-15T01:47:58.6162499Z    |
2020-04-15T01:47:58.6162632Z LL |     {
2020-04-15T01:47:58.6163006Z    |     - because of this opening brace
2020-04-15T01:47:58.6163222Z LL |     asdf}
2020-04-15T01:47:58.6163222Z LL |     asdf}
2020-04-15T01:47:58.6163418Z    |     ^ expected `}` in format string
2020-04-15T01:47:58.6163595Z    |
2020-04-15T01:47:58.6163834Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6164260Z error: 1 positional argument in format string, but no arguments were given
2020-04-15T01:47:58.6164802Z   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:70:17
2020-04-15T01:47:58.6165027Z    |
2020-04-15T01:47:58.6165027Z    |
2020-04-15T01:47:58.6165183Z LL |     println!("\t{}");
2020-04-15T01:47:58.6165507Z 
2020-04-15T01:47:58.6165733Z error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6166254Z   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:74:27
2020-04-15T01:47:58.6166642Z    |
2020-04-15T01:47:58.6166642Z    |
2020-04-15T01:47:58.6166813Z LL |     println!("\x7B}\u{8} {", 1);
2020-04-15T01:47:58.6167265Z    |                          -^ expected `}` in format string
2020-04-15T01:47:58.6167839Z    |                          because of this opening brace
2020-04-15T01:47:58.6168047Z    |
2020-04-15T01:47:58.6168047Z    |
2020-04-15T01:47:58.6168285Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6168684Z error: invalid format string: unmatched `}` found
2020-04-15T01:47:58.6171043Z   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:81:21
2020-04-15T01:47:58.6171307Z    |
2020-04-15T01:47:58.6171307Z    |
2020-04-15T01:47:58.6171487Z LL |     println!(r#"\x7B}\u{8} {"#, 1);
2020-04-15T01:47:58.6171743Z    |                     ^ unmatched `}` in format string
2020-04-15T01:47:58.6171957Z    |
2020-04-15T01:47:58.6172182Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-15T01:47:58.6172593Z error: invalid format string: unmatched `}` found
2020-04-15T01:47:58.6173108Z   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:84:21
2020-04-15T01:47:58.6173335Z    |
2020-04-15T01:47:58.6173335Z    |
2020-04-15T01:47:58.6173616Z LL |     println!(r#"\x7B}\u8 {"#, 1);
2020-04-15T01:47:58.6173870Z    |                     ^ unmatched `}` in format string
2020-04-15T01:47:58.6174054Z    |
2020-04-15T01:47:58.6174261Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-15T01:47:58.6174457Z 
2020-04-15T01:47:58.6174645Z error: aborting due to 18 previous errors; 1 warning emitted
2020-04-15T01:47:58.6174903Z 
2020-04-15T01:47:58.6175240Z ------------------------------------------
2020-04-15T01:47:58.6175386Z 
2020-04-15T01:47:58.6175468Z 
2020-04-15T01:47:58.6175468Z 
2020-04-15T01:47:58.6175849Z ---- [ui] ui/fmt/format-string-error.rs stdout ----
2020-04-15T01:47:58.6176068Z diff of stderr:
2020-04-15T01:47:58.6176182Z 
2020-04-15T01:47:58.6176584Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-15T01:47:58.6176914Z + error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6177538Z 3    |
2020-04-15T01:47:58.6177687Z 4 LL |     println!("{");
2020-04-15T01:47:58.6177818Z 
2020-04-15T01:47:58.6177818Z 
2020-04-15T01:47:58.6178415Z -    |               -^ expected `'}'` in format string
2020-04-15T01:47:58.6178832Z +    |               -^ expected `}` in format string
2020-04-15T01:47:58.6179257Z 7    |               because of this opening brace
2020-04-15T01:47:58.6179434Z 8    |
2020-04-15T01:47:58.6179527Z 
2020-04-15T01:47:58.6179651Z 40    |
2020-04-15T01:47:58.6179651Z 40    |
2020-04-15T01:47:58.6179851Z 41    = note: argument name cannot be a single underscore
2020-04-15T01:47:58.6180041Z 42 
2020-04-15T01:47:58.6180455Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-15T01:47:58.6180788Z + error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6181425Z 45    |
2020-04-15T01:47:58.6181581Z 46 LL |     let _ = format!("{");
2020-04-15T01:47:58.6181713Z 
2020-04-15T01:47:58.6181713Z 
2020-04-15T01:47:58.6182075Z -    |                      -^ expected `'}'` in format string
2020-04-15T01:47:58.6182531Z +    |                      -^ expected `}` in format string
2020-04-15T01:47:58.6182984Z 49    |                      because of this opening brace
2020-04-15T01:47:58.6183187Z 50    |
2020-04-15T01:47:58.6183281Z 
2020-04-15T01:47:58.6183391Z 68    |
2020-04-15T01:47:58.6183391Z 68    |
2020-04-15T01:47:58.6183609Z 69    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6183834Z 70 
2020-04-15T01:47:58.6184232Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-15T01:47:58.6184560Z + error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6185340Z 73    |
2020-04-15T01:47:58.6185340Z 73    |
2020-04-15T01:47:58.6185514Z 74 LL |     let _ = format!("\n\n\n{\n\n\n");
2020-04-15T01:47:58.6185675Z 
2020-04-15T01:47:58.6186119Z -    |                            -      ^ expected `'}'` in format string
2020-04-15T01:47:58.6186611Z +    |                            -      ^ expected `}` in format string
2020-04-15T01:47:58.6187124Z 77    |                            because of this opening brace
2020-04-15T01:47:58.6187435Z 78    |
2020-04-15T01:47:58.6187518Z 
2020-04-15T01:47:58.6187518Z 
2020-04-15T01:47:58.6187896Z 79    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6188108Z 80 
2020-04-15T01:47:58.6188504Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-15T01:47:58.6188847Z + error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6189470Z 83    |
2020-04-15T01:47:58.6189470Z 83    |
2020-04-15T01:47:58.6189623Z 84 LL |     {"###);
2020-04-15T01:47:58.6189733Z 
2020-04-15T01:47:58.6190053Z -    |     -^ expected `'}'` in format string
2020-04-15T01:47:58.6190432Z +    |     -^ expected `}` in format string
2020-04-15T01:47:58.6190808Z 87    |     because of this opening brace
2020-04-15T01:47:58.6190975Z 88    |
2020-04-15T01:47:58.6191081Z 
2020-04-15T01:47:58.6191081Z 
2020-04-15T01:47:58.6191290Z 89    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6191502Z 90 
2020-04-15T01:47:58.6191915Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-15T01:47:58.6192243Z + error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6192874Z 93    |
2020-04-15T01:47:58.6193003Z 94 LL |     {
2020-04-15T01:47:58.6193109Z 
2020-04-15T01:47:58.6193425Z 95    |     - because of this opening brace
2020-04-15T01:47:58.6193425Z 95    |     - because of this opening brace
2020-04-15T01:47:58.6193614Z 96 LL | 
2020-04-15T01:47:58.6193746Z 97 LL | "###);
2020-04-15T01:47:58.6194073Z -    | ^ expected `'}'` in format string
2020-04-15T01:47:58.6194301Z +    | ^ expected `}` in format string
2020-04-15T01:47:58.6194462Z 99    |
2020-04-15T01:47:58.6194684Z 100    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6195000Z 
2020-04-15T01:47:58.6195084Z 
2020-04-15T01:47:58.6195259Z The actual stderr differed from the expected stderr.
2020-04-15T01:47:58.6195843Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error/format-string-error.stderr
2020-04-15T01:47:58.6195843Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error/format-string-error.stderr
2020-04-15T01:47:58.6196404Z To update references, rerun the tests and pass the `--bless` flag
2020-04-15T01:47:58.6196909Z To only update this specific test, also pass `--test-args fmt/format-string-error.rs`
2020-04-15T01:47:58.6197294Z error: 1 errors occurred comparing output.
2020-04-15T01:47:58.6197496Z status: exit code: 1
2020-04-15T01:47:58.6197496Z status: exit code: 1
2020-04-15T01:47:58.6199156Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/format-string-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error/auxiliary"
2020-04-15T01:47:58.6200472Z ------------------------------------------
2020-04-15T01:47:58.6200619Z 
2020-04-15T01:47:58.6201019Z ------------------------------------------
2020-04-15T01:47:58.6201171Z stderr:
2020-04-15T01:47:58.6201171Z stderr:
2020-04-15T01:47:58.6203674Z ------------------------------------------
2020-04-15T01:47:58.6203949Z error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6204433Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:4:16
2020-04-15T01:47:58.6204661Z    |
2020-04-15T01:47:58.6204802Z LL |     println!("{");
2020-04-15T01:47:58.6205262Z    |               -^ expected `}` in format string
2020-04-15T01:47:58.6205681Z    |               because of this opening brace
2020-04-15T01:47:58.6205854Z    |
2020-04-15T01:47:58.6205854Z    |
2020-04-15T01:47:58.6206078Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6206447Z error: invalid format string: unmatched `}` found
2020-04-15T01:47:58.6206925Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:7:15
2020-04-15T01:47:58.6207127Z    |
2020-04-15T01:47:58.6207267Z LL |     println!("}");
2020-04-15T01:47:58.6207267Z LL |     println!("}");
2020-04-15T01:47:58.6207470Z    |               ^ unmatched `}` in format string
2020-04-15T01:47:58.6207666Z    |
2020-04-15T01:47:58.6207875Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-15T01:47:58.6208262Z error: invalid format string: invalid argument name `_`
2020-04-15T01:47:58.6208832Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:9:23
2020-04-15T01:47:58.6209010Z    |
2020-04-15T01:47:58.6209010Z    |
2020-04-15T01:47:58.6209170Z LL |     let _ = format!("{_}", _ = 6usize);
2020-04-15T01:47:58.6209392Z    |                       ^ invalid argument name in format string
2020-04-15T01:47:58.6209741Z    = note: argument name cannot be a single underscore
2020-04-15T01:47:58.6209884Z 
2020-04-15T01:47:58.6210050Z error: invalid format string: invalid argument name `_`
2020-04-15T01:47:58.6210449Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:11:25
2020-04-15T01:47:58.6210449Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:11:25
2020-04-15T01:47:58.6210641Z    |
2020-04-15T01:47:58.6210807Z LL |     let _ = format!("{a:_}", a = "", _ = 0);
2020-04-15T01:47:58.6211049Z    |                         ^ invalid argument name in format string
2020-04-15T01:47:58.6211406Z    = note: argument name cannot be a single underscore
2020-04-15T01:47:58.6211548Z 
2020-04-15T01:47:58.6211714Z error: invalid format string: invalid argument name `_`
2020-04-15T01:47:58.6212130Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:13:26
2020-04-15T01:47:58.6212130Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:13:26
2020-04-15T01:47:58.6212306Z    |
2020-04-15T01:47:58.6212474Z LL |     let _ = format!("{a:._$}", a = "", _ = 0);
2020-04-15T01:47:58.6212734Z    |                          ^ invalid argument name in format string
2020-04-15T01:47:58.6213072Z    = note: argument name cannot be a single underscore
2020-04-15T01:47:58.6213228Z 
2020-04-15T01:47:58.6213411Z error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6213918Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:15:23
2020-04-15T01:47:58.6213918Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:15:23
2020-04-15T01:47:58.6214085Z    |
2020-04-15T01:47:58.6214197Z LL |     let _ = format!("{");
2020-04-15T01:47:58.6214490Z    |                      -^ expected `}` in format string
2020-04-15T01:47:58.6214833Z    |                      because of this opening brace
2020-04-15T01:47:58.6214968Z    |
2020-04-15T01:47:58.6214968Z    |
2020-04-15T01:47:58.6215126Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6215414Z error: invalid format string: unmatched `}` found
2020-04-15T01:47:58.6215751Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:17:22
2020-04-15T01:47:58.6215917Z    |
2020-04-15T01:47:58.6216029Z LL |     let _ = format!("}");
2020-04-15T01:47:58.6216029Z LL |     let _ = format!("}");
2020-04-15T01:47:58.6216196Z    |                      ^ unmatched `}` in format string
2020-04-15T01:47:58.6216348Z    |
2020-04-15T01:47:58.6216503Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-15T01:47:58.6216640Z 
2020-04-15T01:47:58.6216910Z error: invalid format string: expected `'}'`, found `'\\'`
2020-04-15T01:47:58.6217478Z    |
2020-04-15T01:47:58.6217478Z    |
2020-04-15T01:47:58.6217594Z LL |     let _ = format!("{\\}");
2020-04-15T01:47:58.6217901Z    |                      -^ expected `}` in format string
2020-04-15T01:47:58.6218264Z    |                      because of this opening brace
2020-04-15T01:47:58.6218418Z    |
2020-04-15T01:47:58.6218418Z    |
2020-04-15T01:47:58.6218572Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6218866Z error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6219243Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:21:35
2020-04-15T01:47:58.6219395Z    |
2020-04-15T01:47:58.6219395Z    |
2020-04-15T01:47:58.6219519Z LL |     let _ = format!("\n\n\n{\n\n\n");
2020-04-15T01:47:58.6219858Z    |                            -      ^ expected `}` in format string
2020-04-15T01:47:58.6220222Z    |                            because of this opening brace
2020-04-15T01:47:58.6220377Z    |
2020-04-15T01:47:58.6220377Z    |
2020-04-15T01:47:58.6220533Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6220840Z error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6221197Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:27:3
2020-04-15T01:47:58.6221349Z    |
2020-04-15T01:47:58.6221349Z    |
2020-04-15T01:47:58.6221447Z LL |     {"###);
2020-04-15T01:47:58.6221712Z    |     -^ expected `}` in format string
2020-04-15T01:47:58.6221970Z    |     because of this opening brace
2020-04-15T01:47:58.6222102Z    |
2020-04-15T01:47:58.6222102Z    |
2020-04-15T01:47:58.6222258Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6222565Z error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6222920Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:35:1
2020-04-15T01:47:58.6223074Z    |
2020-04-15T01:47:58.6223178Z LL |     {
2020-04-15T01:47:58.6223178Z LL |     {
2020-04-15T01:47:58.6223420Z    |     - because of this opening brace
2020-04-15T01:47:58.6223544Z LL | 
2020-04-15T01:47:58.6223637Z LL | "###);
2020-04-15T01:47:58.6223776Z    | ^ expected `}` in format string
2020-04-15T01:47:58.6223892Z    |
2020-04-15T01:47:58.6224049Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6224337Z error: invalid format string: unmatched `}` found
2020-04-15T01:47:58.6224673Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:41:2
2020-04-15T01:47:58.6224838Z    |
2020-04-15T01:47:58.6224928Z LL |     }
2020-04-15T01:47:58.6224928Z LL |     }
2020-04-15T01:47:58.6225056Z    |     ^ unmatched `}` in format string
2020-04-15T01:47:58.6225178Z    |
2020-04-15T01:47:58.6225346Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-15T01:47:58.6225619Z error: invalid format string: unmatched `}` found
2020-04-15T01:47:58.6225970Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:49:9
2020-04-15T01:47:58.6226122Z    |
2020-04-15T01:47:58.6226217Z LL |         }
2020-04-15T01:47:58.6226217Z LL |         }
2020-04-15T01:47:58.6226355Z    |         ^ unmatched `}` in format string
2020-04-15T01:47:58.6226494Z    |
2020-04-15T01:47:58.6226837Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-15T01:47:58.6227189Z error: 3 positional arguments in format string, but there are 2 arguments
2020-04-15T01:47:58.6227601Z   --> /checkout/src/test/ui/fmt/format-string-error.rs:53:15
2020-04-15T01:47:58.6227778Z    |
2020-04-15T01:47:58.6227928Z LL |     println!("{} {} {}", 1, 2);
2020-04-15T01:47:58.6227928Z LL |     println!("{} {} {}", 1, 2);
2020-04-15T01:47:58.6228230Z    |               ^^ ^^ ^^   -  -
2020-04-15T01:47:58.6228488Z error: aborting due to 14 previous errors
2020-04-15T01:47:58.6228627Z 
2020-04-15T01:47:58.6228700Z 
2020-04-15T01:47:58.6228972Z ------------------------------------------
---
2020-04-15T01:47:58.6229779Z 
2020-04-15T01:47:58.6229908Z 164    |             |
2020-04-15T01:47:58.6230083Z 165    |             formatting specifier missing
2020-04-15T01:47:58.6230234Z 166 
2020-04-15T01:47:58.6230619Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-15T01:47:58.6230921Z + error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6231297Z 168   --> $DIR/ifmt-bad-arg.rs:51:15
2020-04-15T01:47:58.6231594Z 170 LL |     format!("{");
2020-04-15T01:47:58.6231698Z 
2020-04-15T01:47:58.6231698Z 
2020-04-15T01:47:58.6231992Z -    |              -^ expected `'}'` in format string
2020-04-15T01:47:58.6232364Z +    |              -^ expected `}` in format string
2020-04-15T01:47:58.6232722Z 173    |              because of this opening brace
2020-04-15T01:47:58.6232899Z 174    |
2020-04-15T01:47:58.6232982Z 
2020-04-15T01:47:58.6233054Z 
2020-04-15T01:47:58.6233054Z 
2020-04-15T01:47:58.6233205Z The actual stderr differed from the expected stderr.
2020-04-15T01:47:58.6233692Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-bad-arg/ifmt-bad-arg.stderr
2020-04-15T01:47:58.6234151Z To update references, rerun the tests and pass the `--bless` flag
2020-04-15T01:47:58.6234571Z To only update this specific test, also pass `--test-args if/ifmt-bad-arg.rs`
2020-04-15T01:47:58.6234893Z error: 1 errors occurred comparing output.
2020-04-15T01:47:58.6235067Z status: exit code: 1
2020-04-15T01:47:58.6235067Z status: exit code: 1
2020-04-15T01:47:58.6236453Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/if/ifmt-bad-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-bad-arg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-bad-arg/auxiliary"
2020-04-15T01:47:58.6237579Z ------------------------------------------
2020-04-15T01:47:58.6237707Z 
2020-04-15T01:47:58.6237978Z ------------------------------------------
2020-04-15T01:47:58.6238144Z stderr:
---
2020-04-15T01:47:58.6246466Z 
2020-04-15T01:47:58.6246620Z error: invalid reference to positional argument 2 (there are 2 arguments)
2020-04-15T01:47:58.6246970Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:22:28
2020-04-15T01:47:58.6247113Z    |
2020-04-15T01:47:58.6247247Z LL |     format!("{} {value} {} {}", 1, value=2);
2020-04-15T01:47:58.6247538Z    |
2020-04-15T01:47:58.6247787Z    = note: positional arguments are zero-based
2020-04-15T01:47:58.6247902Z 
2020-04-15T01:47:58.6247902Z 
2020-04-15T01:47:58.6248080Z error: invalid reference to positional arguments 3, 4 and 5 (there are 3 arguments)
2020-04-15T01:47:58.6248567Z    |
2020-04-15T01:47:58.6248567Z    |
2020-04-15T01:47:58.6248738Z LL |     format!("{name} {value} {} {} {} {} {} {}", 0, name=1, value=2);
2020-04-15T01:47:58.6248941Z    |                                      ^^ ^^ ^^
2020-04-15T01:47:58.6249336Z    = note: positional arguments are zero-based
2020-04-15T01:47:58.6249451Z 
2020-04-15T01:47:58.6249569Z error: there is no argument named `foo`
2020-04-15T01:47:58.6249871Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:27:17
2020-04-15T01:47:58.6249871Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:27:17
2020-04-15T01:47:58.6250025Z    |
2020-04-15T01:47:58.6250159Z LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);
2020-04-15T01:47:58.6250417Z 
2020-04-15T01:47:58.6250536Z error: there is no argument named `bar`
2020-04-15T01:47:58.6250838Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:27:26
2020-04-15T01:47:58.6250979Z    |
2020-04-15T01:47:58.6250979Z    |
2020-04-15T01:47:58.6251124Z LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);
2020-04-15T01:47:58.6251389Z 
2020-04-15T01:47:58.6251521Z error: there is no argument named `foo`
2020-04-15T01:47:58.6252030Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:31:14
2020-04-15T01:47:58.6252198Z    |
2020-04-15T01:47:58.6252198Z    |
2020-04-15T01:47:58.6252600Z LL |     format!("{foo}");                //~ ERROR: no argument named `foo`
2020-04-15T01:47:58.6252972Z 
2020-04-15T01:47:58.6253137Z error: multiple unused formatting arguments
2020-04-15T01:47:58.6253743Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:32:17
2020-04-15T01:47:58.6253945Z    |
2020-04-15T01:47:58.6253945Z    |
2020-04-15T01:47:58.6254228Z LL |     format!("", 1, 2);               //~ ERROR: multiple unused formatting arguments
2020-04-15T01:47:58.6254742Z    |             --  ^  ^ argument never used
2020-04-15T01:47:58.6255146Z    |             |   argument never used
2020-04-15T01:47:58.6255412Z    |             multiple missing formatting specifiers
2020-04-15T01:47:58.6255585Z 
2020-04-15T01:47:58.6255736Z error: argument never used
---
2020-04-15T01:47:58.6260106Z 
2020-04-15T01:47:58.6260266Z error: named argument never used
2020-04-15T01:47:58.6260691Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:35:26
2020-04-15T01:47:58.6260910Z    |
2020-04-15T01:47:58.6261172Z LL |     format!("{}", 1, foo=2);         //~ ERROR: named argument never used
2020-04-15T01:47:58.6261684Z    |             ----         ^ named argument never used
2020-04-15T01:47:58.6262129Z    |             formatting specifier missing
2020-04-15T01:47:58.6262287Z 
2020-04-15T01:47:58.6262442Z error: argument never used
2020-04-15T01:47:58.6262873Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:36:22
2020-04-15T01:47:58.6262873Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:36:22
2020-04-15T01:47:58.6263079Z    |
2020-04-15T01:47:58.6263322Z LL |     format!("{foo}", 1, foo=2);      //~ ERROR: argument never used
2020-04-15T01:47:58.6263797Z    |             -------  ^ argument never used
2020-04-15T01:47:58.6264204Z    |             formatting specifier missing
2020-04-15T01:47:58.6264362Z 
2020-04-15T01:47:58.6264537Z error: named argument never used
2020-04-15T01:47:58.6265050Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:37:21
2020-04-15T01:47:58.6265050Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:37:21
2020-04-15T01:47:58.6265241Z    |
2020-04-15T01:47:58.6265497Z LL |     format!("", foo=2);              //~ ERROR: named argument never used
2020-04-15T01:47:58.6266011Z    |             --      ^ named argument never used
2020-04-15T01:47:58.6266359Z    |             formatting specifier missing
2020-04-15T01:47:58.6266484Z 
2020-04-15T01:47:58.6266629Z error: multiple unused formatting arguments
2020-04-15T01:47:58.6266985Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:38:32
2020-04-15T01:47:58.6266985Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:38:32
2020-04-15T01:47:58.6267161Z    |
2020-04-15T01:47:58.6267395Z LL |     format!("{} {}", 1, 2, foo=1, bar=2);  //~ ERROR: multiple unused formatting arguments
2020-04-15T01:47:58.6268268Z    |             -------            ^      ^ named argument never used
2020-04-15T01:47:58.6269136Z    |             |                  named argument never used
2020-04-15T01:47:58.6269420Z    |             multiple missing formatting specifiers
2020-04-15T01:47:58.6269607Z 
2020-04-15T01:47:58.6269779Z error: duplicate argument named `foo`
2020-04-15T01:47:58.6269779Z error: duplicate argument named `foo`
2020-04-15T01:47:58.6270236Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:40:33
2020-04-15T01:47:58.6270456Z    |
2020-04-15T01:47:58.6270697Z LL |     format!("{foo}", foo=1, foo=2);  //~ ERROR: duplicate argument
2020-04-15T01:47:58.6271188Z    |                          -      ^ duplicate argument
2020-04-15T01:47:58.6271673Z    |                          previously here
2020-04-15T01:47:58.6271831Z 
2020-04-15T01:47:58.6272029Z error: positional arguments cannot follow named arguments
2020-04-15T01:47:58.6272508Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:41:35
2020-04-15T01:47:58.6272508Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:41:35
2020-04-15T01:47:58.6272712Z    |
2020-04-15T01:47:58.6272995Z LL |     format!("{foo} {} {}", foo=1, 2);   //~ ERROR: positional arguments cannot follow
2020-04-15T01:47:58.6273617Z    |                                -  ^ positional arguments must be before named arguments
2020-04-15T01:47:58.6274237Z    |                                named argument
2020-04-15T01:47:58.6274418Z 
2020-04-15T01:47:58.6274594Z error: there is no argument named `valueb`
2020-04-15T01:47:58.6275086Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:45:23
2020-04-15T01:47:58.6275086Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:45:23
2020-04-15T01:47:58.6275304Z    |
2020-04-15T01:47:58.6275559Z LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);
2020-04-15T01:47:58.6275957Z 
2020-04-15T01:47:58.6276131Z error: named argument never used
2020-04-15T01:47:58.6276567Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:45:51
2020-04-15T01:47:58.6276771Z    |
2020-04-15T01:47:58.6276771Z    |
2020-04-15T01:47:58.6276987Z LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);
2020-04-15T01:47:58.6277508Z    |             -------------------                   ^ named argument never used
2020-04-15T01:47:58.6277988Z    |             formatting specifier missing
2020-04-15T01:47:58.6278146Z 
2020-04-15T01:47:58.6278378Z error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6278868Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:51:15
2020-04-15T01:47:58.6278868Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:51:15
2020-04-15T01:47:58.6279086Z    |
2020-04-15T01:47:58.6279542Z LL |     format!("{"); //~ ERROR: expected `'}'` but string was terminated
2020-04-15T01:47:58.6280048Z    |              -^ expected `}` in format string
2020-04-15T01:47:58.6280489Z    |              because of this opening brace
2020-04-15T01:47:58.6280674Z    |
2020-04-15T01:47:58.6280674Z    |
2020-04-15T01:47:58.6281016Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6281701Z error: invalid format string: unmatched `}` found
2020-04-15T01:47:58.6282142Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:53:18
2020-04-15T01:47:58.6282349Z    |
2020-04-15T01:47:58.6282349Z    |
2020-04-15T01:47:58.6282559Z LL |     format!("foo } bar"); //~ ERROR: unmatched `}` found
2020-04-15T01:47:58.6282825Z    |                  ^ unmatched `}` in format string
2020-04-15T01:47:58.6283020Z    |
2020-04-15T01:47:58.6283229Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-15T01:47:58.6283612Z error: invalid format string: unmatched `}` found
2020-04-15T01:47:58.6284048Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:54:18
2020-04-15T01:47:58.6284239Z    |
2020-04-15T01:47:58.6284239Z    |
2020-04-15T01:47:58.6284459Z LL |     format!("foo }"); //~ ERROR: unmatched `}` found
2020-04-15T01:47:58.6284723Z    |                  ^ unmatched `}` in format string
2020-04-15T01:47:58.6284898Z    |
2020-04-15T01:47:58.6285121Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-15T01:47:58.6285950Z error: argument never used
2020-04-15T01:47:58.6286316Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:56:27
2020-04-15T01:47:58.6286494Z    |
2020-04-15T01:47:58.6286494Z    |
2020-04-15T01:47:58.6286688Z LL |     format!("foo %s baz", "bar"); //~ ERROR: argument never used
2020-04-15T01:47:58.6287280Z    |                  |
2020-04-15T01:47:58.6287280Z    |                  |
2020-04-15T01:47:58.6287496Z    |                  help: format specifiers use curly braces: `{}`
2020-04-15T01:47:58.6287923Z    = note: printf formatting not supported; see the documentation for `std::fmt`
2020-04-15T01:47:58.6288125Z 
2020-04-15T01:47:58.6288264Z error: there is no argument named `foo`
2020-04-15T01:47:58.6288618Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:60:9
2020-04-15T01:47:58.6288618Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:60:9
2020-04-15T01:47:58.6288797Z    |
2020-04-15T01:47:58.6288915Z LL |         {foo}
2020-04-15T01:47:58.6289047Z    |         ^^^^^
2020-04-15T01:47:58.6289155Z 
2020-04-15T01:47:58.6289471Z error: invalid format string: expected `'}'`, found `'t'`
2020-04-15T01:47:58.6290017Z    |
2020-04-15T01:47:58.6290017Z    |
2020-04-15T01:47:58.6290157Z LL | ninth number: {
2020-04-15T01:47:58.6290476Z    |               - because of this opening brace
2020-04-15T01:47:58.6290738Z LL | tenth number: {}",
2020-04-15T01:47:58.6290921Z    | ^ expected `}` in format string
2020-04-15T01:47:58.6291057Z    |
2020-04-15T01:47:58.6291237Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6291648Z error: 4 positional arguments in format string, but there are 3 arguments
2020-04-15T01:47:58.6292056Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:78:15
2020-04-15T01:47:58.6292221Z    |
2020-04-15T01:47:58.6292221Z    |
2020-04-15T01:47:58.6292393Z LL |     println!("{} {:.*} {}", 1, 3.2, 4);
2020-04-15T01:47:58.6292730Z    |               ^^ ^^--^ ^^   -  ---  -
2020-04-15T01:47:58.6293167Z    |                    |           this parameter corresponds to the precision flag
2020-04-15T01:47:58.6293167Z    |                    |           this parameter corresponds to the precision flag
2020-04-15T01:47:58.6293522Z    |                    this precision flag adds an extra required argument at position 1, which is why there are 4 arguments expected
2020-04-15T01:47:58.6294092Z    = note: positional arguments are zero-based
2020-04-15T01:47:58.6294092Z    = note: positional arguments are zero-based
2020-04-15T01:47:58.6294572Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-15T01:47:58.6294987Z error: 4 positional arguments in format string, but there are 3 arguments
2020-04-15T01:47:58.6295384Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:81:15
2020-04-15T01:47:58.6295550Z    |
2020-04-15T01:47:58.6295550Z    |
2020-04-15T01:47:58.6295723Z LL |     println!("{} {:07$.*} {}", 1, 3.2, 4);
2020-04-15T01:47:58.6296068Z    |               ^^ ^^^----^ ^^   -  ---  -
2020-04-15T01:47:58.6296522Z    |                     | |           this parameter corresponds to the precision flag
2020-04-15T01:47:58.6296522Z    |                     | |           this parameter corresponds to the precision flag
2020-04-15T01:47:58.6296881Z    |                     | this precision flag adds an extra required argument at position 1, which is why there are 4 arguments expected
2020-04-15T01:47:58.6297344Z    |                     this width flag expects an `usize` argument at position 7, but there are 3 arguments
2020-04-15T01:47:58.6297807Z    = note: positional arguments are zero-based
2020-04-15T01:47:58.6297807Z    = note: positional arguments are zero-based
2020-04-15T01:47:58.6298214Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-15T01:47:58.6298574Z error: invalid reference to positional argument 7 (there are 3 arguments)
2020-04-15T01:47:58.6298916Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:84:18
2020-04-15T01:47:58.6299071Z    |
2020-04-15T01:47:58.6299071Z    |
2020-04-15T01:47:58.6299207Z LL |     println!("{} {:07$} {}", 1, 3.2, 4);
2020-04-15T01:47:58.6299475Z    |                  ^^^--^
2020-04-15T01:47:58.6299624Z    |                     |
2020-04-15T01:47:58.6299842Z    |                     this width flag expects an `usize` argument at position 7, but there are 3 arguments
2020-04-15T01:47:58.6300298Z    = note: positional arguments are zero-based
2020-04-15T01:47:58.6300298Z    = note: positional arguments are zero-based
2020-04-15T01:47:58.6300708Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-15T01:47:58.6301011Z error: unknown format trait `foo`
2020-04-15T01:47:58.6301321Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:86:17
2020-04-15T01:47:58.6301464Z    |
2020-04-15T01:47:58.6301464Z    |
2020-04-15T01:47:58.6301626Z LL |     println!("{:foo}", 1); //~ ERROR unknown format trait `foo`
2020-04-15T01:47:58.6301926Z    |
2020-04-15T01:47:58.6302080Z    = note: the only appropriate formatting traits are:
2020-04-15T01:47:58.6302412Z            - ``, which uses the `Display` trait
2020-04-15T01:47:58.6302704Z            - `?`, which uses the `Debug` trait
2020-04-15T01:47:58.6302704Z            - `?`, which uses the `Debug` trait
2020-04-15T01:47:58.6302997Z            - `e`, which uses the `LowerExp` trait
2020-04-15T01:47:58.6303309Z            - `E`, which uses the `UpperExp` trait
2020-04-15T01:47:58.6303603Z            - `o`, which uses the `Octal` trait
2020-04-15T01:47:58.6303898Z            - `p`, which uses the `Pointer` trait
2020-04-15T01:47:58.6304250Z            - `b`, which uses the `Binary` trait
2020-04-15T01:47:58.6304643Z            - `x`, which uses the `LowerHex` trait
2020-04-15T01:47:58.6304940Z            - `X`, which uses the `UpperHex` trait
2020-04-15T01:47:58.6305055Z 
2020-04-15T01:47:58.6305271Z error: invalid reference to positional arguments 4, 5, 6 and 7 (there is 1 argument)
2020-04-15T01:47:58.6305777Z    |
2020-04-15T01:47:58.6305777Z    |
2020-04-15T01:47:58.6305927Z LL |     println!("{5} {:4$} {6:7$}", 1);
2020-04-15T01:47:58.6306206Z    |               ^^^ ^^--^ ^^^--^
2020-04-15T01:47:58.6306355Z    |                     |      |
2020-04-15T01:47:58.6306601Z    |                     |      this width flag expects an `usize` argument at position 7, but there is 1 argument
2020-04-15T01:47:58.6306910Z    |                     this width flag expects an `usize` argument at position 4, but there is 1 argument
2020-04-15T01:47:58.6307372Z    = note: positional arguments are zero-based
2020-04-15T01:47:58.6307372Z    = note: positional arguments are zero-based
2020-04-15T01:47:58.6307778Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-15T01:47:58.6308142Z error: 2 positional arguments in format string, but no arguments were given
2020-04-15T01:47:58.6308484Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:92:15
2020-04-15T01:47:58.6308627Z    |
2020-04-15T01:47:58.6308627Z    |
2020-04-15T01:47:58.6308757Z LL |     println!("{:.*}");
2020-04-15T01:47:58.6308998Z    |               ^^--^
2020-04-15T01:47:58.6309125Z    |                 |
2020-04-15T01:47:58.6309363Z    |                 this precision flag adds an extra required argument at position 0, which is why there are 2 arguments expected
2020-04-15T01:47:58.6309844Z    = note: positional arguments are zero-based
2020-04-15T01:47:58.6309844Z    = note: positional arguments are zero-based
2020-04-15T01:47:58.6310249Z    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
2020-04-15T01:47:58.6310571Z error[E0308]: mismatched types
2020-04-15T01:47:58.6310869Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:78:32
2020-04-15T01:47:58.6311025Z    |
2020-04-15T01:47:58.6311025Z    |
2020-04-15T01:47:58.6311160Z LL |     println!("{} {:.*} {}", 1, 3.2, 4);
2020-04-15T01:47:58.6311525Z    |                                ^^^ expected `usize`, found floating-point number
2020-04-15T01:47:58.6311832Z    = note: expected reference `&usize`
2020-04-15T01:47:58.6311832Z    = note: expected reference `&usize`
2020-04-15T01:47:58.6311992Z               found reference `&{float}`
2020-04-15T01:47:58.6312576Z 
2020-04-15T01:47:58.6312689Z error[E0308]: mismatched types
2020-04-15T01:47:58.6312987Z   --> /checkout/src/test/ui/if/ifmt-bad-arg.rs:81:35
2020-04-15T01:47:58.6313142Z    |
2020-04-15T01:47:58.6313142Z    |
2020-04-15T01:47:58.6313280Z LL |     println!("{} {:07$.*} {}", 1, 3.2, 4);
2020-04-15T01:47:58.6313651Z    |                                   ^^^ expected `usize`, found floating-point number
2020-04-15T01:47:58.6313960Z    = note: expected reference `&usize`
2020-04-15T01:47:58.6313960Z    = note: expected reference `&usize`
2020-04-15T01:47:58.6314122Z               found reference `&{float}`
2020-04-15T01:47:58.6314706Z 
2020-04-15T01:47:58.6314828Z error: aborting due to 36 previous errors
2020-04-15T01:47:58.6314935Z 
2020-04-15T01:47:58.6315227Z For more information about this error, try `rustc --explain E0308`.
2020-04-15T01:47:58.6315227Z For more information about this error, try `rustc --explain E0308`.
2020-04-15T01:47:58.6315363Z 
2020-04-15T01:47:58.6315598Z ------------------------------------------
2020-04-15T01:47:58.6315706Z 
2020-04-15T01:47:58.6315782Z 
2020-04-15T01:47:58.6316025Z ---- [ui] ui/issues/issue-51848.rs stdout ----
2020-04-15T01:47:58.6316172Z diff of stderr:
2020-04-15T01:47:58.6316256Z 
2020-04-15T01:47:58.6316564Z - error: invalid format string: expected `'}'` but string was terminated
2020-04-15T01:47:58.6316847Z + error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6317302Z 3    |
2020-04-15T01:47:58.6317417Z 4 LL |         println!("{");
2020-04-15T01:47:58.6317510Z 
2020-04-15T01:47:58.6317510Z 
2020-04-15T01:47:58.6317997Z -    |                   -^ expected `'}'` in format string
2020-04-15T01:47:58.6318565Z +    |                   -^ expected `}` in format string
2020-04-15T01:47:58.6318997Z 7    |                   because of this opening brace
2020-04-15T01:47:58.6319197Z 8 ...
2020-04-15T01:47:58.6319288Z 
2020-04-15T01:47:58.6319371Z 
2020-04-15T01:47:58.6319371Z 
2020-04-15T01:47:58.6319544Z The actual stderr differed from the expected stderr.
2020-04-15T01:47:58.6320107Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51848/issue-51848.stderr
2020-04-15T01:47:58.6320632Z To update references, rerun the tests and pass the `--bless` flag
2020-04-15T01:47:58.6321127Z To only update this specific test, also pass `--test-args issues/issue-51848.rs`
2020-04-15T01:47:58.6321605Z error: 1 errors occurred comparing output.
2020-04-15T01:47:58.6321888Z status: exit code: 1
2020-04-15T01:47:58.6321888Z status: exit code: 1
2020-04-15T01:47:58.6323515Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-51848.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51848" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51848/auxiliary"
2020-04-15T01:47:58.6324814Z ------------------------------------------
2020-04-15T01:47:58.6324964Z 
2020-04-15T01:47:58.6325278Z ------------------------------------------
2020-04-15T01:47:58.6325467Z stderr:
2020-04-15T01:47:58.6325467Z stderr:
2020-04-15T01:47:58.6325791Z ------------------------------------------
2020-04-15T01:47:58.6326061Z error: invalid format string: expected `}` but string was terminated
2020-04-15T01:47:58.6326542Z   --> /checkout/src/test/ui/issues/issue-51848.rs:6:20
2020-04-15T01:47:58.6326737Z    |
2020-04-15T01:47:58.6326907Z LL |         println!("{"); //~ ERROR invalid
2020-04-15T01:47:58.6327328Z    |                   -^ expected `}` in format string
2020-04-15T01:47:58.6327749Z    |                   because of this opening brace
2020-04-15T01:47:58.6327939Z ...
2020-04-15T01:47:58.6328086Z LL |     macro_with_error!();
2020-04-15T01:47:58.6328462Z    |     -------------------- in this macro invocation
2020-04-15T01:47:58.6328462Z    |     -------------------- in this macro invocation
2020-04-15T01:47:58.6328658Z    |
2020-04-15T01:47:58.6328868Z    = note: if you intended to print `{`, you can escape it using `{{`
2020-04-15T01:47:58.6329670Z 
2020-04-15T01:47:58.6329869Z error: invalid format string: unmatched `}` found
2020-04-15T01:47:58.6330385Z   --> /checkout/src/test/ui/issues/issue-51848.rs:18:15
2020-04-15T01:47:58.6330556Z    |
2020-04-15T01:47:58.6330556Z    |
2020-04-15T01:47:58.6330715Z LL |     println!("}"); //~ ERROR invalid
2020-04-15T01:47:58.6331040Z    |               ^ unmatched `}` in format string
2020-04-15T01:47:58.6331173Z    |
2020-04-15T01:47:58.6331341Z    = note: if you intended to print `}`, you can escape it using `}}`
2020-04-15T01:47:58.6331599Z error: aborting due to 2 previous errors
2020-04-15T01:47:58.6331707Z 
2020-04-15T01:47:58.6331783Z 
2020-04-15T01:47:58.6332019Z ------------------------------------------
---
2020-04-15T01:47:58.6334418Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-15T01:47:58.6334675Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-15T01:47:58.6334834Z 
2020-04-15T01:47:58.6334896Z 
2020-04-15T01:47:58.6337261Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-15T01:47:58.6338907Z 
2020-04-15T01:47:58.6338971Z 
2020-04-15T01:47:58.6339299Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-15T01:47:58.6339548Z Build completed unsuccessfully in 0:52:05
2020-04-15T01:47:58.6339548Z Build completed unsuccessfully in 0:52:05
2020-04-15T01:47:58.6339739Z == clock drift check ==
2020-04-15T01:47:58.6339896Z   local time: Wed Apr 15 01:47:58 UTC 2020
2020-04-15T01:47:58.9093153Z   network time: Wed, 15 Apr 2020 01:47:58 GMT
2020-04-15T01:47:59.3160205Z 
2020-04-15T01:47:59.3160205Z 
2020-04-15T01:47:59.3222836Z ##[error]Bash exited with code '1'.
2020-04-15T01:47:59.3246476Z ##[section]Finishing: Run build
2020-04-15T01:47:59.3295457Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-15T01:47:59.3300851Z Task         : Get sources
2020-04-15T01:47:59.3301184Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T01:47:59.3301488Z Version      : 1.0.0
2020-04-15T01:47:59.3301691Z Author       : Microsoft
2020-04-15T01:47:59.3301691Z Author       : Microsoft
2020-04-15T01:47:59.3302015Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-15T01:47:59.3302400Z ==============================================================================
2020-04-15T01:47:59.5994497Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-15T01:47:59.6038327Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-15T01:47:59.6107431Z Cleaning up task key
2020-04-15T01:47:59.6108632Z Start cleaning up orphan processes.
2020-04-15T01:47:59.6256367Z Terminate orphan process: pid (3501) (python)
2020-04-15T01:47:59.6387997Z ##[section]Finishing: Finalize Job
