plain
2019-08-11T21:31:46.5608090Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-11T21:31:46.5801586Z ##[command]git config gc.auto 0
2019-08-11T21:31:46.5882428Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-11T21:31:46.5933568Z ##[command]git config --get-all http.proxy
2019-08-11T21:31:46.6073537Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63462/merge:refs/remotes/pull/63462/merge
---
2019-08-11T21:32:20.2743652Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-11T21:32:20.2743726Z 
2019-08-11T21:32:20.2743962Z   git checkout -b <new-branch-name>
2019-08-11T21:32:20.2743996Z 
2019-08-11T21:32:20.2744050Z HEAD is now at a29afd60f Merge 5c61363886cc23827f8c4548c9c10ea6d45a2688 into 8a068699a24de306334a1f66b9a83552766d853c
2019-08-11T21:32:20.2900897Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-11T21:32:20.2903971Z ==============================================================================
2019-08-11T21:32:20.2904031Z Task         : Bash
2019-08-11T21:32:20.2904076Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-11T22:32:33.0650268Z .................................................................................................... 1300/8873
2019-08-11T22:32:39.6902034Z .................................................................................................... 1400/8873
2019-08-11T22:32:45.9411433Z .................................................................................................... 1500/8873
2019-08-11T22:32:56.4910719Z ....................................................................................i............... 1600/8873
2019-08-11T22:33:04.3279663Z i................................................................................................... 1700/8873
2019-08-11T22:33:11.1963119Z ............................................................................iiiii................... 1800/8873
2019-08-11T22:33:33.3183677Z .................................................................................................... 2000/8873
2019-08-11T22:33:35.8598376Z .................................................................................................... 2100/8873
2019-08-11T22:33:38.6571775Z .................................................................................................... 2200/8873
2019-08-11T22:33:46.5037900Z .................................................................................................... 2300/8873
---
2019-08-11T22:37:44.5613542Z .................................................................................................... 5300/8873
2019-08-11T22:37:51.7757040Z ........i........................................................................................... 5400/8873
2019-08-11T22:37:57.2820617Z .................................................................................................... 5500/8873
2019-08-11T22:38:09.6977010Z .................................................................................................... 5600/8873
2019-08-11T22:38:23.9559620Z ...ii...i..ii...........i........................................................................... 5700/8873
2019-08-11T22:38:39.9510901Z .................................................................................................... 5900/8873
2019-08-11T22:38:44.7496499Z .................................................................................................... 6000/8873
2019-08-11T22:38:44.7496499Z .................................................................................................... 6000/8873
2019-08-11T22:38:59.1949036Z ....i..ii........................................................................................... 6100/8873
2019-08-11T22:39:17.9582644Z ...............................................i.................................................... 6300/8873
2019-08-11T22:39:20.1526812Z .................................................................................................... 6400/8873
2019-08-11T22:39:22.6549246Z ...................i................................................................................ 6500/8873
2019-08-11T22:39:27.3596656Z .................................................................................................... 6600/8873
---
2019-08-11T22:44:10.4212310Z  finished in 22.451
2019-08-11T22:44:10.4405454Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-11T22:44:10.6162375Z 
2019-08-11T22:44:10.6163174Z running 146 tests
2019-08-11T22:44:14.0097778Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-08-11T22:44:15.8393371Z iii..............i.........iii.i......ii......
2019-08-11T22:44:15.8396367Z 
2019-08-11T22:44:15.8396740Z  finished in 5.399
2019-08-11T22:44:15.8572832Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-11T22:44:16.0185042Z 
---
2019-08-11T22:44:18.0347463Z  finished in 2.177
2019-08-11T22:44:18.0531781Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-11T22:44:18.2094263Z 
2019-08-11T22:44:18.2094748Z running 9 tests
2019-08-11T22:44:18.2095747Z iiiiiiiii
2019-08-11T22:44:18.2096396Z 
2019-08-11T22:44:18.2096634Z  finished in 0.156
2019-08-11T22:44:18.2270811Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-11T22:44:18.3844989Z 
---
2019-08-11T22:44:36.7158801Z  finished in 18.488
2019-08-11T22:44:36.7369787Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-11T22:44:36.8965997Z 
2019-08-11T22:44:36.8967199Z running 122 tests
2019-08-11T22:45:01.3389708Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-11T22:45:06.0841154Z .i.i......iii.i.....ii
2019-08-11T22:45:06.0843134Z 
2019-08-11T22:45:06.0846032Z  finished in 29.346
2019-08-11T22:45:06.0850291Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-11T22:45:06.0854077Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-08-11T22:45:06.0854077Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-08-11T22:45:06.1083580Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-11T22:45:06.2891863Z 
2019-08-11T22:45:06.2892921Z running 69 tests
2019-08-11T22:46:02.6989169Z ..FFFFF...F.........FF..............................F...........F....
2019-08-11T22:46:02.7010868Z 
2019-08-11T22:46:02.7013573Z ---- [ui] ui-fulldeps/derive-no-std-not-supported.rs stdout ----
2019-08-11T22:46:02.7014708Z 
2019-08-11T22:46:02.7014708Z 
2019-08-11T22:46:02.7017370Z error: test compilation failed although it shouldn't!
2019-08-11T22:46:02.7017737Z status: exit code: 1
2019-08-11T22:46:02.7020552Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/derive-no-std-not-supported.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/derive-no-std-not-supported/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/derive-no-std-not-supported/auxiliary" "-A" "unused"
2019-08-11T22:46:02.7021847Z ------------------------------------------
2019-08-11T22:46:02.7022044Z 
2019-08-11T22:46:02.7022398Z ------------------------------------------
2019-08-11T22:46:02.7022821Z stderr:
2019-08-11T22:46:02.7022821Z stderr:
2019-08-11T22:46:02.7023218Z ------------------------------------------
2019-08-11T22:46:02.7023392Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7023956Z    |
2019-08-11T22:46:02.7024082Z LL | #[derive(RustcEncodable)]
2019-08-11T22:46:02.7024082Z LL | #[derive(RustcEncodable)]
2019-08-11T22:46:02.7024241Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7024370Z 
2019-08-11T22:46:02.7024789Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7029935Z    |
2019-08-11T22:46:02.7029998Z LL | #[derive(RustcEncodable)]
2019-08-11T22:46:02.7029998Z LL | #[derive(RustcEncodable)]
2019-08-11T22:46:02.7030064Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7030097Z 
2019-08-11T22:46:02.7030144Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7030864Z    |
2019-08-11T22:46:02.7030917Z LL | #[derive(RustcDecodable)]
2019-08-11T22:46:02.7030917Z LL | #[derive(RustcDecodable)]
2019-08-11T22:46:02.7030984Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7031016Z 
2019-08-11T22:46:02.7031062Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7031458Z    |
2019-08-11T22:46:02.7031498Z LL | #[derive(RustcDecodable)]
2019-08-11T22:46:02.7031498Z LL | #[derive(RustcDecodable)]
2019-08-11T22:46:02.7031547Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7031637Z error: aborting due to 4 previous errors
2019-08-11T22:46:02.7031665Z 
2019-08-11T22:46:02.7031910Z For more information about this error, try `rustc --explain E0433`.
2019-08-11T22:46:02.7031953Z 
2019-08-11T22:46:02.7031953Z 
2019-08-11T22:46:02.7032179Z ------------------------------------------
2019-08-11T22:46:02.7032210Z 
2019-08-11T22:46:02.7032235Z 
2019-08-11T22:46:02.7032465Z ---- [ui] ui-fulldeps/deriving-encodable-decodable-box.rs stdout ----
2019-08-11T22:46:02.7032515Z 
2019-08-11T22:46:02.7032732Z error: test compilation failed although it shouldn't!
2019-08-11T22:46:02.7032780Z status: exit code: 1
2019-08-11T22:46:02.7033691Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/deriving-encodable-decodable-box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deriving-encodable-decodable-box/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deriving-encodable-decodable-box/auxiliary" "-A" "unused"
2019-08-11T22:46:02.7034072Z ------------------------------------------
2019-08-11T22:46:02.7034105Z 
2019-08-11T22:46:02.7034316Z ------------------------------------------
2019-08-11T22:46:02.7034379Z stderr:
2019-08-11T22:46:02.7034379Z stderr:
2019-08-11T22:46:02.7034980Z ------------------------------------------
2019-08-11T22:46:02.7035037Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7035379Z    |
2019-08-11T22:46:02.7035423Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7035423Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7035472Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7035519Z 
2019-08-11T22:46:02.7035564Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7036009Z    |
2019-08-11T22:46:02.7036052Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7036052Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7036102Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7036134Z 
2019-08-11T22:46:02.7036196Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7036520Z    |
2019-08-11T22:46:02.7036588Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7036588Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7036641Z    |                          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7036675Z 
2019-08-11T22:46:02.7036738Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7037039Z    |
2019-08-11T22:46:02.7037106Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7037106Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7037158Z    |                          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7037232Z error: aborting due to 4 previous errors
2019-08-11T22:46:02.7037277Z 
2019-08-11T22:46:02.7037514Z For more information about this error, try `rustc --explain E0433`.
2019-08-11T22:46:02.7037548Z 
2019-08-11T22:46:02.7037548Z 
2019-08-11T22:46:02.7037752Z ------------------------------------------
2019-08-11T22:46:02.7037806Z 
2019-08-11T22:46:02.7037832Z 
2019-08-11T22:46:02.7038072Z ---- [ui] ui-fulldeps/deriving-encodable-decodable-cell-refcell.rs stdout ----
2019-08-11T22:46:02.7038105Z 
2019-08-11T22:46:02.7038320Z error: test compilation failed although it shouldn't!
2019-08-11T22:46:02.7038383Z status: exit code: 1
2019-08-11T22:46:02.7039156Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/deriving-encodable-decodable-cell-refcell.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deriving-encodable-decodable-cell-refcell/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deriving-encodable-decodable-cell-refcell/auxiliary" "-A" "unused"
2019-08-11T22:46:02.7039572Z ------------------------------------------
2019-08-11T22:46:02.7039629Z 
2019-08-11T22:46:02.7039868Z ------------------------------------------
2019-08-11T22:46:02.7039912Z stderr:
2019-08-11T22:46:02.7039912Z stderr:
2019-08-11T22:46:02.7040116Z ------------------------------------------
2019-08-11T22:46:02.7040184Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7040499Z    |
2019-08-11T22:46:02.7040562Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7040562Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7040835Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7040886Z 
2019-08-11T22:46:02.7040952Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7041329Z    |
2019-08-11T22:46:02.7041399Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7041399Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7041449Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7041480Z 
2019-08-11T22:46:02.7041525Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7041849Z    |
2019-08-11T22:46:02.7041890Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7041890Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7042078Z    |                          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7042112Z 
2019-08-11T22:46:02.7042157Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7042511Z    |
2019-08-11T22:46:02.7042554Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7042554Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7042614Z    |                          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7042667Z 
2019-08-11T22:46:02.7042711Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7043035Z    |
2019-08-11T22:46:02.7043077Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7043077Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7043134Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7043164Z 
2019-08-11T22:46:02.7043227Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7043533Z    |
2019-08-11T22:46:02.7043591Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7043591Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7043648Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7043679Z 
2019-08-11T22:46:02.7043739Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7044045Z    |
2019-08-11T22:46:02.7044102Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7044102Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7044153Z    |                          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7044194Z 
2019-08-11T22:46:02.7044240Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7044562Z    |
2019-08-11T22:46:02.7044603Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7044603Z LL | #[derive(RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7044659Z    |                          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7044836Z error: aborting due to 8 previous errors
2019-08-11T22:46:02.7044865Z 
2019-08-11T22:46:02.7045128Z For more information about this error, try `rustc --explain E0433`.
2019-08-11T22:46:02.7045179Z 
2019-08-11T22:46:02.7045179Z 
2019-08-11T22:46:02.7045388Z ------------------------------------------
2019-08-11T22:46:02.7045419Z 
2019-08-11T22:46:02.7045444Z 
2019-08-11T22:46:02.7045675Z ---- [ui] ui-fulldeps/deriving-global.rs stdout ----
2019-08-11T22:46:02.7045707Z 
2019-08-11T22:46:02.7045935Z error: test compilation failed although it shouldn't!
2019-08-11T22:46:02.7045982Z status: exit code: 1
2019-08-11T22:46:02.7046702Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/deriving-global.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deriving-global/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deriving-global/auxiliary" "-A" "unused"
2019-08-11T22:46:02.7047019Z ------------------------------------------
2019-08-11T22:46:02.7047052Z 
2019-08-11T22:46:02.7047261Z ------------------------------------------
2019-08-11T22:46:02.7047321Z stderr:
2019-08-11T22:46:02.7047321Z stderr:
2019-08-11T22:46:02.7047529Z ------------------------------------------
2019-08-11T22:46:02.7047659Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7048000Z    |
2019-08-11T22:46:02.7048043Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7048043Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7048111Z    |                ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7048144Z 
2019-08-11T22:46:02.7048198Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7048500Z    |
2019-08-11T22:46:02.7048542Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7048542Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7048591Z    |                ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7048640Z 
2019-08-11T22:46:02.7048685Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7048976Z    |
2019-08-11T22:46:02.7049034Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7049034Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7049087Z    |                                ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7049120Z 
2019-08-11T22:46:02.7049183Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7049473Z    |
2019-08-11T22:46:02.7049531Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7049531Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7049583Z    |                                ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7049617Z 
2019-08-11T22:46:02.7049663Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7049971Z    |
2019-08-11T22:46:02.7050013Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7050013Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7050080Z    |                ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7050111Z 
2019-08-11T22:46:02.7050156Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7050454Z    |
2019-08-11T22:46:02.7050572Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7050572Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7050827Z    |                ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7050892Z 
2019-08-11T22:46:02.7050939Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7051315Z    |
2019-08-11T22:46:02.7051357Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7051357Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7051420Z    |                                ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7051451Z 
2019-08-11T22:46:02.7051515Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7051800Z    |
2019-08-11T22:46:02.7051859Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7051859Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7051919Z    |                                ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7051952Z 
2019-08-11T22:46:02.7052014Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7052294Z    |
2019-08-11T22:46:02.7052338Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7052338Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7052404Z    |                ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7052546Z 
2019-08-11T22:46:02.7052592Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7052916Z    |
2019-08-11T22:46:02.7052958Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7052958Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7053025Z    |                ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7053065Z 
2019-08-11T22:46:02.7053110Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7053408Z    |
2019-08-11T22:46:02.7053450Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7053450Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7053501Z    |                                ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7053550Z 
2019-08-11T22:46:02.7053604Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7053901Z    |
2019-08-11T22:46:02.7053943Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7053943Z LL |                RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7053995Z    |                                ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7054094Z error: aborting due to 12 previous errors
2019-08-11T22:46:02.7054123Z 
2019-08-11T22:46:02.7054359Z For more information about this error, try `rustc --explain E0433`.
2019-08-11T22:46:02.7054393Z 
2019-08-11T22:46:02.7054393Z 
2019-08-11T22:46:02.7054615Z ------------------------------------------
2019-08-11T22:46:02.7054645Z 
2019-08-11T22:46:02.7054669Z 
2019-08-11T22:46:02.7054884Z ---- [ui] ui-fulldeps/deriving-hygiene.rs stdout ----
2019-08-11T22:46:02.7054931Z 
2019-08-11T22:46:02.7055148Z error: test compilation failed although it shouldn't!
2019-08-11T22:46:02.7055205Z status: exit code: 1
2019-08-11T22:46:02.7055995Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/deriving-hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deriving-hygiene/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deriving-hygiene/auxiliary" "-A" "unused"
2019-08-11T22:46:02.7056348Z ------------------------------------------
2019-08-11T22:46:02.7056380Z 
2019-08-11T22:46:02.7056592Z ------------------------------------------
2019-08-11T22:46:02.7056635Z stderr:
2019-08-11T22:46:02.7056635Z stderr:
2019-08-11T22:46:02.7056858Z ------------------------------------------
2019-08-11T22:46:02.7056909Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7057217Z    |
2019-08-11T22:46:02.7057217Z    |
2019-08-11T22:46:02.7057314Z LL | #[derive(Ord,Eq,PartialOrd,PartialEq,Debug,RustcDecodable,RustcEncodable,Hash)]
2019-08-11T22:46:02.7057372Z    |                                            ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7057425Z 
2019-08-11T22:46:02.7057481Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7057790Z    |
2019-08-11T22:46:02.7057790Z    |
2019-08-11T22:46:02.7057837Z LL | #[derive(Ord,Eq,PartialOrd,PartialEq,Debug,RustcDecodable,RustcEncodable,Hash)]
2019-08-11T22:46:02.7057894Z    |                                                           ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7057986Z error: aborting due to 2 previous errors
2019-08-11T22:46:02.7058094Z 
2019-08-11T22:46:02.7058352Z For more information about this error, try `rustc --explain E0433`.
2019-08-11T22:46:02.7058401Z 
2019-08-11T22:46:02.7058401Z 
2019-08-11T22:46:02.7058608Z ------------------------------------------
2019-08-11T22:46:02.7058639Z 
2019-08-11T22:46:02.7058664Z 
2019-08-11T22:46:02.7058889Z ---- [ui] ui-fulldeps/empty-struct-braces-derive.rs stdout ----
2019-08-11T22:46:02.7058937Z 
2019-08-11T22:46:02.7059162Z error: test compilation failed although it shouldn't!
2019-08-11T22:46:02.7059209Z status: exit code: 1
2019-08-11T22:46:02.7059948Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/empty-struct-braces-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/empty-struct-braces-derive/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/empty-struct-braces-derive/auxiliary" "-A" "unused"
2019-08-11T22:46:02.7060272Z ------------------------------------------
2019-08-11T22:46:02.7060304Z 
2019-08-11T22:46:02.7060512Z ------------------------------------------
2019-08-11T22:46:02.7060571Z stderr:
2019-08-11T22:46:02.7060571Z stderr:
2019-08-11T22:46:02.7061164Z ------------------------------------------
2019-08-11T22:46:02.7061238Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7061563Z    |
2019-08-11T22:46:02.7061563Z    |
2019-08-11T22:46:02.7061607Z LL |          Default, Debug, RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7061674Z    |                          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7061708Z 
2019-08-11T22:46:02.7061762Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7062072Z    |
2019-08-11T22:46:02.7062072Z    |
2019-08-11T22:46:02.7062116Z LL |          Default, Debug, RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7062170Z    |                                          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7062222Z 
2019-08-11T22:46:02.7062391Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7062746Z    |
2019-08-11T22:46:02.7062746Z    |
2019-08-11T22:46:02.7062790Z LL |          Default, Debug, RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7062841Z    |                          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7062873Z 
2019-08-11T22:46:02.7062934Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7063235Z    |
2019-08-11T22:46:02.7063235Z    |
2019-08-11T22:46:02.7063295Z LL |          Default, Debug, RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7063348Z    |                                          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7063381Z 
2019-08-11T22:46:02.7063450Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7063745Z    |
2019-08-11T22:46:02.7063787Z LL |          Debug, RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7063787Z LL |          Debug, RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7063854Z    |                 ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7063886Z 
2019-08-11T22:46:02.7063930Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7064343Z    |
2019-08-11T22:46:02.7064385Z LL |          Debug, RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7064385Z LL |          Debug, RustcEncodable, RustcDecodable)]
2019-08-11T22:46:02.7064623Z    |                                 ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7064701Z error: aborting due to 6 previous errors
2019-08-11T22:46:02.7064729Z 
2019-08-11T22:46:02.7067307Z For more information about this error, try `rustc --explain E0433`.
2019-08-11T22:46:02.7067361Z 
2019-08-11T22:46:02.7067361Z 
2019-08-11T22:46:02.7067659Z ------------------------------------------
2019-08-11T22:46:02.7067690Z 
2019-08-11T22:46:02.7067715Z 
2019-08-11T22:46:02.7067950Z ---- [ui] ui-fulldeps/issue-11881.rs stdout ----
2019-08-11T22:46:02.7067982Z 
2019-08-11T22:46:02.7068197Z error: test compilation failed although it shouldn't!
2019-08-11T22:46:02.7068263Z status: exit code: 1
2019-08-11T22:46:02.7068969Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-11881.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-11881/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-11881/auxiliary" "-A" "unused"
2019-08-11T22:46:02.7069298Z ------------------------------------------
2019-08-11T22:46:02.7069330Z 
2019-08-11T22:46:02.7069542Z ------------------------------------------
2019-08-11T22:46:02.7069602Z stderr:
2019-08-11T22:46:02.7069602Z stderr:
2019-08-11T22:46:02.7069808Z ------------------------------------------
2019-08-11T22:46:02.7069860Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7070110Z   --> /checkout/src/test/ui-fulldeps/issue-11881.rs:21:10
2019-08-11T22:46:02.7070209Z LL | #[derive(RustcEncodable)]
2019-08-11T22:46:02.7070209Z LL | #[derive(RustcEncodable)]
2019-08-11T22:46:02.7070274Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7070305Z 
2019-08-11T22:46:02.7070351Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7070606Z   --> /checkout/src/test/ui-fulldeps/issue-11881.rs:21:10
2019-08-11T22:46:02.7071103Z LL | #[derive(RustcEncodable)]
2019-08-11T22:46:02.7071103Z LL | #[derive(RustcEncodable)]
2019-08-11T22:46:02.7071162Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7071216Z 
2019-08-11T22:46:02.7071261Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7071580Z   --> /checkout/src/test/ui-fulldeps/issue-11881.rs:26:10
2019-08-11T22:46:02.7071683Z LL | #[derive(RustcEncodable)]
2019-08-11T22:46:02.7071683Z LL | #[derive(RustcEncodable)]
2019-08-11T22:46:02.7071730Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7071772Z 
2019-08-11T22:46:02.7071834Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7072069Z   --> /checkout/src/test/ui-fulldeps/issue-11881.rs:26:10
2019-08-11T22:46:02.7072170Z LL | #[derive(RustcEncodable)]
2019-08-11T22:46:02.7072170Z LL | #[derive(RustcEncodable)]
2019-08-11T22:46:02.7072218Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7072298Z error: aborting due to 4 previous errors
2019-08-11T22:46:02.7072346Z 
2019-08-11T22:46:02.7072585Z For more information about this error, try `rustc --explain E0433`.
2019-08-11T22:46:02.7072618Z 
2019-08-11T22:46:02.7072618Z 
2019-08-11T22:46:02.7072823Z ------------------------------------------
2019-08-11T22:46:02.7072869Z 
2019-08-11T22:46:02.7072894Z 
2019-08-11T22:46:02.7073110Z ---- [ui] ui-fulldeps/issue-14021.rs stdout ----
2019-08-11T22:46:02.7073142Z 
2019-08-11T22:46:02.7073374Z error: test compilation failed although it shouldn't!
2019-08-11T22:46:02.7073519Z status: exit code: 1
2019-08-11T22:46:02.7074250Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-14021.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-14021/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-14021/auxiliary" "-A" "unused"
2019-08-11T22:46:02.7074565Z ------------------------------------------
2019-08-11T22:46:02.7074612Z 
2019-08-11T22:46:02.7074824Z ------------------------------------------
2019-08-11T22:46:02.7074867Z stderr:
2019-08-11T22:46:02.7074867Z stderr:
2019-08-11T22:46:02.7075072Z ------------------------------------------
2019-08-11T22:46:02.7075140Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7075384Z   --> /checkout/src/test/ui-fulldeps/issue-14021.rs:13:10
2019-08-11T22:46:02.7075430Z    |
2019-08-11T22:46:02.7075491Z LL | #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
2019-08-11T22:46:02.7075541Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7075572Z 
2019-08-11T22:46:02.7075633Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7075875Z   --> /checkout/src/test/ui-fulldeps/issue-14021.rs:13:26
2019-08-11T22:46:02.7075920Z    |
2019-08-11T22:46:02.7075981Z LL | #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
2019-08-11T22:46:02.7076033Z    |                          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7076106Z error: aborting due to 2 previous errors
2019-08-11T22:46:02.7076150Z 
2019-08-11T22:46:02.7076386Z For more information about this error, try `rustc --explain E0433`.
2019-08-11T22:46:02.7076428Z 
2019-08-11T22:46:02.7076428Z 
2019-08-11T22:46:02.7076634Z ------------------------------------------
2019-08-11T22:46:02.7076680Z 
2019-08-11T22:46:02.7076705Z 
2019-08-11T22:46:02.7076919Z ---- [ui] ui-fulldeps/newtype_index.rs stdout ----
2019-08-11T22:46:02.7076951Z 
2019-08-11T22:46:02.7077183Z error: test compilation failed although it shouldn't!
2019-08-11T22:46:02.7077229Z status: exit code: 1
2019-08-11T22:46:02.7078000Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/newtype_index.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/newtype_index/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/newtype_index/auxiliary" "-A" "unused"
2019-08-11T22:46:02.7078362Z ------------------------------------------
2019-08-11T22:46:02.7078394Z 
2019-08-11T22:46:02.7078622Z ------------------------------------------
2019-08-11T22:46:02.7078665Z stderr:
2019-08-11T22:46:02.7078665Z stderr:
2019-08-11T22:46:02.7078869Z ------------------------------------------
2019-08-11T22:46:02.7078938Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7079227Z    |
2019-08-11T22:46:02.7079227Z    |
2019-08-11T22:46:02.7079289Z LL | newtype_index!(struct MyIdx { MAX = 0xFFFF_FFFA });
2019-08-11T22:46:02.7079341Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7079714Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-08-11T22:46:02.7079836Z 
2019-08-11T22:46:02.7079836Z 
2019-08-11T22:46:02.7079886Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7080172Z   --> <::rustc_data_structures::indexed_vec::newtype_index macros>:15:57
2019-08-11T22:46:02.7080217Z    |
2019-08-11T22:46:02.7080264Z LL |   (Copy , PartialEq , Eq , Hash , PartialOrd , Ord , $ ($ derives) , *)] #
2019-08-11T22:46:02.7080338Z    |                                                         ^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7080424Z error: aborting due to 2 previous errors
2019-08-11T22:46:02.7080453Z 
2019-08-11T22:46:02.7080948Z For more information about this error, try `rustc --explain E0433`.
2019-08-11T22:46:02.7080990Z 
2019-08-11T22:46:02.7080990Z 
2019-08-11T22:46:02.7081224Z ------------------------------------------
2019-08-11T22:46:02.7081255Z 
2019-08-11T22:46:02.7081297Z 
2019-08-11T22:46:02.7081522Z ---- [ui] ui-fulldeps/rustc_encodable_hygiene.rs stdout ----
2019-08-11T22:46:02.7081567Z 
2019-08-11T22:46:02.7081785Z error: test compilation failed although it shouldn't!
2019-08-11T22:46:02.7081849Z status: exit code: 1
2019-08-11T22:46:02.7082575Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/rustc_encodable_hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/rustc_encodable_hygiene/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/rustc_encodable_hygiene/auxiliary" "-A" "unused"
2019-08-11T22:46:02.7082898Z ------------------------------------------
2019-08-11T22:46:02.7082931Z 
2019-08-11T22:46:02.7083156Z ------------------------------------------
2019-08-11T22:46:02.7083200Z stderr:
2019-08-11T22:46:02.7083200Z stderr:
2019-08-11T22:46:02.7083417Z ------------------------------------------
2019-08-11T22:46:02.7083485Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7083773Z    |
2019-08-11T22:46:02.7083773Z    |
2019-08-11T22:46:02.7083832Z LL | #[derive(RustcDecodable, RustcEncodable,Debug)]
2019-08-11T22:46:02.7083883Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7083914Z 
2019-08-11T22:46:02.7084063Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7084411Z    |
2019-08-11T22:46:02.7084411Z    |
2019-08-11T22:46:02.7084454Z LL | #[derive(RustcDecodable, RustcEncodable,Debug)]
2019-08-11T22:46:02.7084519Z    |          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7084550Z 
2019-08-11T22:46:02.7084594Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7084909Z    |
2019-08-11T22:46:02.7084909Z    |
2019-08-11T22:46:02.7084952Z LL | #[derive(RustcDecodable, RustcEncodable,Debug)]
2019-08-11T22:46:02.7085003Z    |                          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7085052Z 
2019-08-11T22:46:02.7085096Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7087870Z    |
2019-08-11T22:46:02.7087870Z    |
2019-08-11T22:46:02.7087915Z LL | #[derive(RustcDecodable, RustcEncodable,Debug)]
2019-08-11T22:46:02.7087966Z    |                          ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-08-11T22:46:02.7088057Z error: aborting due to 4 previous errors
2019-08-11T22:46:02.7088086Z 
2019-08-11T22:46:02.7088414Z For more information about this error, try `rustc --explain E0433`.
2019-08-11T22:46:02.7088579Z 
---
2019-08-11T22:46:02.7092527Z test result: FAILED. 59 passed; 10 failed; 0 ignored; 0 measured; 0 filtered out
2019-08-11T22:46:02.7092562Z 
2019-08-11T22:46:02.7092586Z 
2019-08-11T22:46:02.7092610Z 
2019-08-11T22:46:02.7094642Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-11T22:46:02.7097214Z 
2019-08-11T22:46:02.7097269Z 
2019-08-11T22:46:02.7097353Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-11T22:46:02.7097417Z Build completed unsuccessfully in 1:07:28
2019-08-11T22:46:02.7097417Z Build completed unsuccessfully in 1:07:28
2019-08-11T22:46:03.3174780Z ##[error]Bash exited with code '1'.
2019-08-11T22:46:03.3229219Z ##[section]Starting: Checkout
2019-08-11T22:46:03.3231350Z ==============================================================================
2019-08-11T22:46:03.3231408Z Task         : Get sources
2019-08-11T22:46:03.3231456Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
