plain
2020-03-14T04:08:06.6602540Z ========================== Starting Command Output ===========================
2020-03-14T04:08:06.6605001Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/119ba53c-4e49-4133-84bd-ea6bfa1457c7.sh
2020-03-14T04:08:06.6605286Z 
2020-03-14T04:08:06.6609034Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-14T04:08:06.6629584Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69995/merge to s
2020-03-14T04:08:06.6633187Z Task         : Get sources
2020-03-14T04:08:06.6633493Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T04:08:06.6633809Z Version      : 1.0.0
2020-03-14T04:08:06.6634010Z Author       : Microsoft
---
2020-03-14T04:08:07.6752270Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-14T04:08:07.6757553Z ##[command]git config gc.auto 0
2020-03-14T04:08:07.6761120Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-14T04:08:07.6764419Z ##[command]git config --get-all http.proxy
2020-03-14T04:08:07.6770341Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69995/merge:refs/remotes/pull/69995/merge
---
2020-03-14T05:01:51.2105304Z .................................................................................................... 1700/9770
2020-03-14T05:01:55.1605959Z .................................................................................................... 1800/9770
2020-03-14T05:02:05.8462589Z ...................................................................i................................ 1900/9770
2020-03-14T05:02:12.0824775Z .................................................................................................... 2000/9770
2020-03-14T05:02:25.0017341Z .........................................................iiiii...................................... 2100/9770
2020-03-14T05:02:34.6825016Z ............F....................................................................................... 2300/9770
2020-03-14T05:02:36.6069944Z .................................................................................................... 2400/9770
2020-03-14T05:02:39.3693409Z .................................................................................................... 2500/9770
2020-03-14T05:02:59.0154119Z .................................................................................................... 2600/9770
---
2020-03-14T05:05:27.4836021Z ............................i...............i....................................................... 5000/9770
2020-03-14T05:05:36.8871750Z .................................................................................................... 5100/9770
2020-03-14T05:05:42.2825038Z .......................................................................i............................ 5200/9770
2020-03-14T05:05:47.8286444Z .......F............................................................................................ 5300/9770
2020-03-14T05:05:56.8749850Z ....................................................ii.ii........i...i.............................. 5400/9770
2020-03-14T05:06:04.6076662Z ............F...............................................F...............FFFF.................... 5600/9770
2020-03-14T05:06:13.4718218Z ..............F..................................................................................... 5700/9770
2020-03-14T05:06:19.3688445Z ............................................i....................................................... 5800/9770
2020-03-14T05:06:25.3702764Z .................................................................................................... 5900/9770
2020-03-14T05:06:25.3702764Z .................................................................................................... 5900/9770
2020-03-14T05:06:34.7743411Z .................................................................................................... 6000/9770
2020-03-14T05:06:40.4925058Z ......................................ii...i..ii...........i........................................ 6100/9770
2020-03-14T05:06:59.2964344Z .................................................................................................... 6300/9770
2020-03-14T05:07:02.5683212Z .................................................................................................... 6400/9770
2020-03-14T05:07:02.5683212Z .................................................................................................... 6400/9770
2020-03-14T05:07:07.1365057Z ....................................................................i..ii........................... 6500/9770
2020-03-14T05:07:27.2843876Z .................................................................................................... 6700/9770
2020-03-14T05:07:34.9193944Z ..................................................................i................................. 6800/9770
2020-03-14T05:07:36.7002842Z .................................................................................................... 6900/9770
2020-03-14T05:07:38.6837767Z .................................................................................................... 7000/9770
---
2020-03-14T05:09:06.8199835Z .................................................................................................... 7700/9770
2020-03-14T05:09:11.6723261Z .................................................................................................... 7800/9770
2020-03-14T05:09:17.1678111Z .................................................................................................... 7900/9770
2020-03-14T05:09:22.4772268Z ..................................................i................................................. 8000/9770
2020-03-14T05:09:31.7933100Z ...................................................................................................i 8100/9770
2020-03-14T05:09:36.7941372Z iiiiiiiii.i......................................................................................... 8200/9770
2020-03-14T05:09:49.3358283Z .................................................................................................... 8400/9770
2020-03-14T05:09:59.3233508Z .................................................................................................... 8500/9770
2020-03-14T05:10:10.6626371Z .................................................................................................... 8600/9770
2020-03-14T05:10:15.8011251Z .................................................................................................... 8700/9770
---
2020-03-14T05:11:56.1201088Z 
2020-03-14T05:11:56.1201402Z 9    |
2020-03-14T05:11:56.1201830Z 10 LL | #![deny(overflowing_literals)]
2020-03-14T05:11:56.1202199Z 11    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1203127Z +    = note: the literal `223` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1204059Z 13 error: literal out of range for `i16`
2020-03-14T05:11:56.1204862Z 14   --> $DIR/enum-discrim-too-small2.rs:15:12
2020-03-14T05:11:56.1205336Z 
2020-03-14T05:11:56.1205592Z 15    |
2020-03-14T05:11:56.1205592Z 15    |
2020-03-14T05:11:56.1205900Z 16 LL |     Ci16 = 55555,
2020-03-14T05:11:56.1206376Z 17    |            ^^^^^
2020-03-14T05:11:56.1206662Z +    |
2020-03-14T05:11:56.1207448Z +    = note: the literal `55555` does not fit into an `i16` whose maximum is 32767, minimum is -32768
2020-03-14T05:11:56.1208289Z 19 error: literal out of range for `i32`
2020-03-14T05:11:56.1209076Z 20   --> $DIR/enum-discrim-too-small2.rs:22:12
2020-03-14T05:11:56.1209541Z 
2020-03-14T05:11:56.1209903Z 21    |
2020-03-14T05:11:56.1209903Z 21    |
2020-03-14T05:11:56.1210256Z 22 LL |     Ci32 = 3_000_000_000,
2020-03-14T05:11:56.1210650Z 23    |            ^^^^^^^^^^^^^
2020-03-14T05:11:56.1210925Z +    |
2020-03-14T05:11:56.1211711Z +    = note: the literal `3_000_000_000` does not fit into an `i32` whose maximum is 2147483647, minimum is -2147483648
2020-03-14T05:11:56.1212542Z 25 error: literal out of range for `i64`
2020-03-14T05:11:56.1213260Z 26   --> $DIR/enum-discrim-too-small2.rs:29:12
2020-03-14T05:11:56.1216986Z 
2020-03-14T05:11:56.1217158Z 27    |
2020-03-14T05:11:56.1217158Z 27    |
2020-03-14T05:11:56.1217347Z 28 LL |     Ci64 = 9223372036854775809,
2020-03-14T05:11:56.1217580Z 29    |            ^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1217774Z +    |
2020-03-14T05:11:56.1218480Z +    = note: the literal `9223372036854775809` does not fit into an `i64` whose maximum is 9223372036854775807, minimum is -9223372036854775808
2020-03-14T05:11:56.1219047Z 31 error: aborting due to 4 previous errors
2020-03-14T05:11:56.1219237Z 32 
2020-03-14T05:11:56.1219336Z 
2020-03-14T05:11:56.1219427Z 
2020-03-14T05:11:56.1219427Z 
2020-03-14T05:11:56.1219638Z The actual stderr differed from the expected stderr.
2020-03-14T05:11:56.1220302Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/enum-discrim-too-small2/enum-discrim-too-small2.stderr
2020-03-14T05:11:56.1220913Z To update references, rerun the tests and pass the `--bless` flag
2020-03-14T05:11:56.1221490Z To only update this specific test, also pass `--test-args enum/enum-discrim-too-small2.rs`
2020-03-14T05:11:56.1221915Z error: 1 errors occurred comparing output.
2020-03-14T05:11:56.1222137Z status: exit code: 1
2020-03-14T05:11:56.1222137Z status: exit code: 1
2020-03-14T05:11:56.1224122Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/enum/enum-discrim-too-small2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/enum-discrim-too-small2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/enum-discrim-too-small2/auxiliary"
2020-03-14T05:11:56.1225656Z ------------------------------------------
2020-03-14T05:11:56.1225819Z 
2020-03-14T05:11:56.1226170Z ------------------------------------------
2020-03-14T05:11:56.1226426Z stderr:
2020-03-14T05:11:56.1226426Z stderr:
2020-03-14T05:11:56.1226778Z ------------------------------------------
2020-03-14T05:11:56.1227189Z error: literal out of range for `i8`
2020-03-14T05:11:56.1227709Z   --> /checkout/src/test/ui/enum/enum-discrim-too-small2.rs:8:11
2020-03-14T05:11:56.1227966Z    |
2020-03-14T05:11:56.1228204Z LL |     Ci8 = 223, //~ ERROR literal out of range for `i8`
2020-03-14T05:11:56.1228635Z    |
2020-03-14T05:11:56.1228828Z note: the lint level is defined here
2020-03-14T05:11:56.1229349Z   --> /checkout/src/test/ui/enum/enum-discrim-too-small2.rs:1:9
2020-03-14T05:11:56.1229601Z    |
2020-03-14T05:11:56.1229601Z    |
2020-03-14T05:11:56.1229795Z LL | #![deny(overflowing_literals)]
2020-03-14T05:11:56.1230050Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1230615Z    = note: the literal `223` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1231234Z error: literal out of range for `i16`
2020-03-14T05:11:56.1231954Z   --> /checkout/src/test/ui/enum/enum-discrim-too-small2.rs:15:12
2020-03-14T05:11:56.1232226Z    |
2020-03-14T05:11:56.1232226Z    |
2020-03-14T05:11:56.1232484Z LL |     Ci16 = 55555, //~ ERROR literal out of range for `i16`
2020-03-14T05:11:56.1232910Z    |
2020-03-14T05:11:56.1232910Z    |
2020-03-14T05:11:56.1233477Z    = note: the literal `55555` does not fit into an `i16` whose maximum is 32767, minimum is -32768
2020-03-14T05:11:56.1233967Z error: literal out of range for `i32`
2020-03-14T05:11:56.1234478Z   --> /checkout/src/test/ui/enum/enum-discrim-too-small2.rs:22:12
2020-03-14T05:11:56.1234749Z    |
2020-03-14T05:11:56.1234749Z    |
2020-03-14T05:11:56.1235009Z LL |     Ci32 = 3_000_000_000, //~ ERROR literal out of range for `i32`
2020-03-14T05:11:56.1235495Z    |
2020-03-14T05:11:56.1235495Z    |
2020-03-14T05:11:56.1236081Z    = note: the literal `3_000_000_000` does not fit into an `i32` whose maximum is 2147483647, minimum is -2147483648
2020-03-14T05:11:56.1236608Z error: literal out of range for `i64`
2020-03-14T05:11:56.1237117Z   --> /checkout/src/test/ui/enum/enum-discrim-too-small2.rs:29:12
2020-03-14T05:11:56.1237377Z    |
2020-03-14T05:11:56.1237377Z    |
2020-03-14T05:11:56.1237656Z LL |     Ci64 = 9223372036854775809, //~ ERROR literal out of range for `i64`
2020-03-14T05:11:56.1238157Z    |
2020-03-14T05:11:56.1238157Z    |
2020-03-14T05:11:56.1238801Z    = note: the literal `9223372036854775809` does not fit into an `i64` whose maximum is 9223372036854775807, minimum is -9223372036854775808
2020-03-14T05:11:56.1239370Z error: aborting due to 4 previous errors
2020-03-14T05:11:56.1239546Z 
2020-03-14T05:11:56.1239647Z 
2020-03-14T05:11:56.1240022Z ------------------------------------------
2020-03-14T05:11:56.1240022Z ------------------------------------------
2020-03-14T05:11:56.1240194Z 
2020-03-14T05:11:56.1240292Z 
2020-03-14T05:11:56.1240665Z ---- [ui] ui/issues/issue-63364.rs stdout ----
2020-03-14T05:11:56.1240912Z diff of stderr:
2020-03-14T05:11:56.1241037Z 
2020-03-14T05:11:56.1241246Z 5    |              ^^^^^^^
2020-03-14T05:11:56.1241422Z 6    |
2020-03-14T05:11:56.1241676Z 7    = note: `#[deny(overflowing_literals)]` on by default
2020-03-14T05:11:56.1242185Z +    = note: the literal `100_000` does not fit into an `u16` whose maximum is 65535, minimum is 0
2020-03-14T05:11:56.1242660Z 9 error: aborting due to previous error
2020-03-14T05:11:56.1242840Z 10 
2020-03-14T05:11:56.1242937Z 
2020-03-14T05:11:56.1243027Z 
2020-03-14T05:11:56.1243027Z 
2020-03-14T05:11:56.1243236Z The actual stderr differed from the expected stderr.
2020-03-14T05:11:56.1243916Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-63364/issue-63364.stderr
2020-03-14T05:11:56.1244508Z To update references, rerun the tests and pass the `--bless` flag
2020-03-14T05:11:56.1247743Z To only update this specific test, also pass `--test-args issues/issue-63364.rs`
2020-03-14T05:11:56.1248166Z error: 1 errors occurred comparing output.
2020-03-14T05:11:56.1248535Z status: exit code: 1
2020-03-14T05:11:56.1248535Z status: exit code: 1
2020-03-14T05:11:56.1250329Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-63364.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-63364" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-63364/auxiliary"
2020-03-14T05:11:56.1251811Z ------------------------------------------
2020-03-14T05:11:56.1251975Z 
2020-03-14T05:11:56.1252328Z ------------------------------------------
2020-03-14T05:11:56.1252518Z stderr:
2020-03-14T05:11:56.1252518Z stderr:
2020-03-14T05:11:56.1252860Z ------------------------------------------
2020-03-14T05:11:56.1253130Z error: literal out of range for `u16`
2020-03-14T05:11:56.1253581Z   --> /checkout/src/test/ui/issues/issue-63364.rs:6:14
2020-03-14T05:11:56.1253799Z    |
2020-03-14T05:11:56.1253978Z LL |     for n in 100_000.. {
2020-03-14T05:11:56.1254330Z    |
2020-03-14T05:11:56.1254545Z    = note: `#[deny(overflowing_literals)]` on by default
2020-03-14T05:11:56.1254545Z    = note: `#[deny(overflowing_literals)]` on by default
2020-03-14T05:11:56.1254940Z    = note: the literal `100_000` does not fit into an `u16` whose maximum is 65535, minimum is 0
2020-03-14T05:11:56.1255369Z error: aborting due to previous error
2020-03-14T05:11:56.1255539Z 
2020-03-14T05:11:56.1255629Z 
2020-03-14T05:11:56.1255965Z ------------------------------------------
2020-03-14T05:11:56.1255965Z ------------------------------------------
2020-03-14T05:11:56.1256127Z 
2020-03-14T05:11:56.1256234Z 
2020-03-14T05:11:56.1256610Z ---- [ui] ui/lint/deny-overflowing-literals.rs stdout ----
2020-03-14T05:11:56.1256839Z diff of stderr:
2020-03-14T05:11:56.1256957Z 
2020-03-14T05:11:56.1257124Z 5    |                 ^^^
2020-03-14T05:11:56.1257289Z 6    |
2020-03-14T05:11:56.1257511Z 7    = note: `#[deny(overflowing_literals)]` on by default
2020-03-14T05:11:56.1258923Z +    = note: the literal `256` does not fit into an `u8` whose maximum is 255, minimum is 0
2020-03-14T05:11:56.1261386Z 9 error: range endpoint is out of range for `u8`
2020-03-14T05:11:56.1262014Z 10   --> $DIR/deny-overflowing-literals.rs:5:14
2020-03-14T05:11:56.1262200Z 
2020-03-14T05:11:56.1262292Z 
2020-03-14T05:11:56.1262292Z 
2020-03-14T05:11:56.1262483Z The actual stderr differed from the expected stderr.
2020-03-14T05:11:56.1263177Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/deny-overflowing-literals/deny-overflowing-literals.stderr
2020-03-14T05:11:56.1263805Z To update references, rerun the tests and pass the `--bless` flag
2020-03-14T05:11:56.1264378Z To only update this specific test, also pass `--test-args lint/deny-overflowing-literals.rs`
2020-03-14T05:11:56.1264824Z error: 1 errors occurred comparing output.
2020-03-14T05:11:56.1265047Z status: exit code: 1
2020-03-14T05:11:56.1265047Z status: exit code: 1
2020-03-14T05:11:56.1267020Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/deny-overflowing-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/deny-overflowing-literals" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/deny-overflowing-literals/auxiliary"
2020-03-14T05:11:56.1268573Z ------------------------------------------
2020-03-14T05:11:56.1268736Z 
2020-03-14T05:11:56.1269069Z ------------------------------------------
2020-03-14T05:11:56.1269277Z stderr:
2020-03-14T05:11:56.1269277Z stderr:
2020-03-14T05:11:56.1269687Z ------------------------------------------
2020-03-14T05:11:56.1269929Z error: literal out of range for `u8`
2020-03-14T05:11:56.1271427Z   --> /checkout/src/test/ui/lint/deny-overflowing-literals.rs:2:17
2020-03-14T05:11:56.1271680Z    |
2020-03-14T05:11:56.1271851Z LL |     let x: u8 = 256;
2020-03-14T05:11:56.1272066Z    |                 ^^^
2020-03-14T05:11:56.1272219Z    |
2020-03-14T05:11:56.1272432Z    = note: `#[deny(overflowing_literals)]` on by default
2020-03-14T05:11:56.1272793Z    = note: the literal `256` does not fit into an `u8` whose maximum is 255, minimum is 0
2020-03-14T05:11:56.1273254Z error: range endpoint is out of range for `u8`
2020-03-14T05:11:56.1273749Z   --> /checkout/src/test/ui/lint/deny-overflowing-literals.rs:5:14
2020-03-14T05:11:56.1274006Z    |
2020-03-14T05:11:56.1274169Z LL |     for _ in 0..256u8 {}
---
2020-03-14T05:11:56.1276171Z ---- [ui] ui/lint/lint-range-endpoint-overflow.rs stdout ----
2020-03-14T05:11:56.1276423Z diff of stderr:
2020-03-14T05:11:56.1276540Z 
2020-03-14T05:11:56.1276659Z 15    |
2020-03-14T05:11:56.1276834Z 16 LL |     let range_c = 0..=256;
2020-03-14T05:11:56.1277426Z +    |
2020-03-14T05:11:56.1277426Z +    |
2020-03-14T05:11:56.1277727Z +    = note: the literal `256` does not fit into an `u8` whose maximum is 255, minimum is 0
2020-03-14T05:11:56.1278230Z 19 error: literal out of range for `u8`
2020-03-14T05:11:56.1278714Z 20   --> $DIR/lint-range-endpoint-overflow.rs:7:19
2020-03-14T05:11:56.1278917Z 
2020-03-14T05:11:56.1279064Z 21    |
2020-03-14T05:11:56.1279064Z 21    |
2020-03-14T05:11:56.1279256Z 22 LL |     let range_d = 256..5;
2020-03-14T05:11:56.1279680Z +    |
2020-03-14T05:11:56.1279680Z +    |
2020-03-14T05:11:56.1279979Z +    = note: the literal `256` does not fit into an `u8` whose maximum is 255, minimum is 0
2020-03-14T05:11:56.1280478Z 25 error: literal out of range for `u8`
2020-03-14T05:11:56.1280958Z 26   --> $DIR/lint-range-endpoint-overflow.rs:8:22
2020-03-14T05:11:56.1281165Z 
2020-03-14T05:11:56.1281294Z 27    |
2020-03-14T05:11:56.1281294Z 27    |
2020-03-14T05:11:56.1281499Z 28 LL |     let range_e = 0..257;
2020-03-14T05:11:56.1281909Z +    |
2020-03-14T05:11:56.1281909Z +    |
2020-03-14T05:11:56.1282226Z +    = note: the literal `257` does not fit into an `u8` whose maximum is 255, minimum is 0
2020-03-14T05:11:56.1282728Z 31 error: range endpoint is out of range for `u8`
2020-03-14T05:11:56.1283237Z 32   --> $DIR/lint-range-endpoint-overflow.rs:9:20
2020-03-14T05:11:56.1283442Z 
2020-03-14T05:11:56.1283545Z 
2020-03-14T05:11:56.1283545Z 
2020-03-14T05:11:56.1283757Z The actual stderr differed from the expected stderr.
2020-03-14T05:11:56.1284505Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-range-endpoint-overflow/lint-range-endpoint-overflow.stderr
2020-03-14T05:11:56.1285182Z To update references, rerun the tests and pass the `--bless` flag
2020-03-14T05:11:56.1285802Z To only update this specific test, also pass `--test-args lint/lint-range-endpoint-overflow.rs`
2020-03-14T05:11:56.1286278Z error: 1 errors occurred comparing output.
2020-03-14T05:11:56.1286517Z status: exit code: 1
2020-03-14T05:11:56.1286517Z status: exit code: 1
2020-03-14T05:11:56.1288664Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-range-endpoint-overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-range-endpoint-overflow" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-range-endpoint-overflow/auxiliary"
2020-03-14T05:11:56.1290670Z ------------------------------------------
2020-03-14T05:11:56.1290833Z 
2020-03-14T05:11:56.1291167Z ------------------------------------------
2020-03-14T05:11:56.1291374Z stderr:
2020-03-14T05:11:56.1291374Z stderr:
2020-03-14T05:11:56.1291717Z ------------------------------------------
2020-03-14T05:11:56.1291982Z error: range endpoint is out of range for `u8`
2020-03-14T05:11:56.1292497Z   --> /checkout/src/test/ui/lint/lint-range-endpoint-overflow.rs:4:19
2020-03-14T05:11:56.1292745Z    |
2020-03-14T05:11:56.1292997Z LL |     let range_a = 0..256; //~ ERROR range endpoint is out of range for `u8`
2020-03-14T05:11:56.1293410Z    |                   ^^^^^^ help: use an inclusive range instead: `0..=255`
2020-03-14T05:11:56.1293858Z note: the lint level is defined here
2020-03-14T05:11:56.1294352Z   --> /checkout/src/test/ui/lint/lint-range-endpoint-overflow.rs:1:9
2020-03-14T05:11:56.1294594Z    |
2020-03-14T05:11:56.1294767Z LL | #![deny(overflowing_literals)]
2020-03-14T05:11:56.1294767Z LL | #![deny(overflowing_literals)]
2020-03-14T05:11:56.1295001Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1295147Z 
2020-03-14T05:11:56.1295316Z error: literal out of range for `u8`
2020-03-14T05:11:56.1295793Z   --> /checkout/src/test/ui/lint/lint-range-endpoint-overflow.rs:6:23
2020-03-14T05:11:56.1296054Z    |
2020-03-14T05:11:56.1296294Z LL |     let range_c = 0..=256; //~ ERROR literal out of range for `u8`
2020-03-14T05:11:56.1296743Z    |
2020-03-14T05:11:56.1296743Z    |
2020-03-14T05:11:56.1297012Z    = note: the literal `256` does not fit into an `u8` whose maximum is 255, minimum is 0
2020-03-14T05:11:56.1297441Z error: literal out of range for `u8`
2020-03-14T05:11:56.1297925Z   --> /checkout/src/test/ui/lint/lint-range-endpoint-overflow.rs:7:19
2020-03-14T05:11:56.1298174Z    |
2020-03-14T05:11:56.1298174Z    |
2020-03-14T05:11:56.1298407Z LL |     let range_d = 256..5; //~ ERROR literal out of range for `u8`
2020-03-14T05:11:56.1298846Z    |
2020-03-14T05:11:56.1298846Z    |
2020-03-14T05:11:56.1299120Z    = note: the literal `256` does not fit into an `u8` whose maximum is 255, minimum is 0
2020-03-14T05:11:56.1299551Z error: literal out of range for `u8`
2020-03-14T05:11:56.1300029Z   --> /checkout/src/test/ui/lint/lint-range-endpoint-overflow.rs:8:22
2020-03-14T05:11:56.1300294Z    |
2020-03-14T05:11:56.1300294Z    |
2020-03-14T05:11:56.1300530Z LL |     let range_e = 0..257; //~ ERROR literal out of range for `u8`
2020-03-14T05:11:56.1300974Z    |
2020-03-14T05:11:56.1300974Z    |
2020-03-14T05:11:56.1301238Z    = note: the literal `257` does not fit into an `u8` whose maximum is 255, minimum is 0
2020-03-14T05:11:56.1301672Z error: range endpoint is out of range for `u8`
2020-03-14T05:11:56.1302186Z   --> /checkout/src/test/ui/lint/lint-range-endpoint-overflow.rs:9:20
2020-03-14T05:11:56.1302428Z    |
2020-03-14T05:11:56.1302428Z    |
2020-03-14T05:11:56.1302681Z LL |     let _range_f = 0..256u8;  //~ ERROR range endpoint is out of range for `u8`
2020-03-14T05:11:56.1303108Z    |                    ^^^^^^^^ help: use an inclusive range instead: `0..=255u8`
2020-03-14T05:11:56.1303551Z error: range endpoint is out of range for `i8`
2020-03-14T05:11:56.1304256Z   --> /checkout/src/test/ui/lint/lint-range-endpoint-overflow.rs:10:20
2020-03-14T05:11:56.1304523Z    |
2020-03-14T05:11:56.1304523Z    |
2020-03-14T05:11:56.1304845Z LL |     let _range_g = 0..128i8;  //~ ERROR range endpoint is out of range for `i8`
2020-03-14T05:11:56.1305311Z    |                    ^^^^^^^^ help: use an inclusive range instead: `0..=127i8`
2020-03-14T05:11:56.1305782Z error: aborting due to 6 previous errors
2020-03-14T05:11:56.1305954Z 
2020-03-14T05:11:56.1306100Z 
2020-03-14T05:11:56.1306489Z ------------------------------------------
---
2020-03-14T05:11:56.1307519Z 
2020-03-14T05:11:56.1307648Z 17    |
2020-03-14T05:11:56.1307861Z 18 LL | #![warn(overflowing_literals)]
2020-03-14T05:11:56.1308112Z 19    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1308688Z +    = note: the literal `128` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1309214Z 21 error: aborting due to previous error
2020-03-14T05:11:56.1309411Z 22 
2020-03-14T05:11:56.1309515Z 
2020-03-14T05:11:56.1309628Z 
2020-03-14T05:11:56.1309628Z 
2020-03-14T05:11:56.1309839Z The actual stderr differed from the expected stderr.
2020-03-14T05:11:56.1310510Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-limits2/lint-type-limits2.stderr
2020-03-14T05:11:56.1311294Z To update references, rerun the tests and pass the `--bless` flag
2020-03-14T05:11:56.1311893Z To only update this specific test, also pass `--test-args lint/lint-type-limits2.rs`
2020-03-14T05:11:56.1312350Z error: 1 errors occurred comparing output.
2020-03-14T05:11:56.1312589Z status: exit code: 1
2020-03-14T05:11:56.1312589Z status: exit code: 1
2020-03-14T05:11:56.1314594Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-type-limits2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-limits2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-D" "unused-comparisons" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-limits2/auxiliary"
2020-03-14T05:11:56.1316299Z ------------------------------------------
2020-03-14T05:11:56.1316481Z 
2020-03-14T05:11:56.1316815Z ------------------------------------------
2020-03-14T05:11:56.1317004Z stderr:
2020-03-14T05:11:56.1317004Z stderr:
2020-03-14T05:11:56.1317361Z ------------------------------------------
2020-03-14T05:11:56.1317621Z error: comparison is useless due to type limits
2020-03-14T05:11:56.1318096Z   --> /checkout/src/test/ui/lint/lint-type-limits2.rs:13:5
2020-03-14T05:11:56.1318339Z    |
2020-03-14T05:11:56.1318575Z LL |     128 > bar() //~ ERROR comparison is useless due to type limits
2020-03-14T05:11:56.1318984Z    |
2020-03-14T05:11:56.1319414Z    = note: requested on the command line with `-D unused-comparisons`
2020-03-14T05:11:56.1319629Z 
2020-03-14T05:11:56.1319803Z warning: literal out of range for `i8`
2020-03-14T05:11:56.1319803Z warning: literal out of range for `i8`
2020-03-14T05:11:56.1320272Z   --> /checkout/src/test/ui/lint/lint-type-limits2.rs:13:5
2020-03-14T05:11:56.1320498Z    |
2020-03-14T05:11:56.1320730Z LL |     128 > bar() //~ ERROR comparison is useless due to type limits
2020-03-14T05:11:56.1321132Z    |
2020-03-14T05:11:56.1321306Z note: the lint level is defined here
2020-03-14T05:11:56.1321751Z   --> /checkout/src/test/ui/lint/lint-type-limits2.rs:2:9
2020-03-14T05:11:56.1321986Z    |
2020-03-14T05:11:56.1321986Z    |
2020-03-14T05:11:56.1322160Z LL | #![warn(overflowing_literals)]
2020-03-14T05:11:56.1322382Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1324329Z    = note: the literal `128` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1324883Z error: aborting due to previous error
2020-03-14T05:11:56.1325061Z 
2020-03-14T05:11:56.1325153Z 
2020-03-14T05:11:56.1325524Z ------------------------------------------
---
2020-03-14T05:11:56.1326553Z 
2020-03-14T05:11:56.1326687Z 17    |
2020-03-14T05:11:56.1326873Z 18 LL | #![warn(overflowing_literals)]
2020-03-14T05:11:56.1327280Z 19    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1327868Z +    = note: the literal `200` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1328383Z 21 error: aborting due to previous error
2020-03-14T05:11:56.1328582Z 22 
2020-03-14T05:11:56.1328704Z 
2020-03-14T05:11:56.1328803Z 
2020-03-14T05:11:56.1328803Z 
2020-03-14T05:11:56.1329012Z The actual stderr differed from the expected stderr.
2020-03-14T05:11:56.1329688Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-limits3/lint-type-limits3.stderr
2020-03-14T05:11:56.1330350Z To update references, rerun the tests and pass the `--bless` flag
2020-03-14T05:11:56.1330944Z To only update this specific test, also pass `--test-args lint/lint-type-limits3.rs`
2020-03-14T05:11:56.1331411Z error: 1 errors occurred comparing output.
2020-03-14T05:11:56.1331656Z status: exit code: 1
2020-03-14T05:11:56.1331656Z status: exit code: 1
2020-03-14T05:11:56.1333872Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-type-limits3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-limits3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-D" "unused-comparisons" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-limits3/auxiliary"
2020-03-14T05:11:56.1335539Z ------------------------------------------
2020-03-14T05:11:56.1335716Z 
2020-03-14T05:11:56.1336079Z ------------------------------------------
2020-03-14T05:11:56.1336292Z stderr:
2020-03-14T05:11:56.1336292Z stderr:
2020-03-14T05:11:56.1336681Z ------------------------------------------
2020-03-14T05:11:56.1336961Z error: comparison is useless due to type limits
2020-03-14T05:11:56.1337465Z   --> /checkout/src/test/ui/lint/lint-type-limits3.rs:9:11
2020-03-14T05:11:56.1337732Z    |
2020-03-14T05:11:56.1337993Z LL |     while 200 != i { //~ ERROR comparison is useless due to type limits
2020-03-14T05:11:56.1338465Z    |
2020-03-14T05:11:56.1338909Z    = note: requested on the command line with `-D unused-comparisons`
2020-03-14T05:11:56.1339141Z 
2020-03-14T05:11:56.1340148Z warning: literal out of range for `i8`
2020-03-14T05:11:56.1340148Z warning: literal out of range for `i8`
2020-03-14T05:11:56.1340736Z   --> /checkout/src/test/ui/lint/lint-type-limits3.rs:9:11
2020-03-14T05:11:56.1340965Z    |
2020-03-14T05:11:56.1341209Z LL |     while 200 != i { //~ ERROR comparison is useless due to type limits
2020-03-14T05:11:56.1342039Z    |
2020-03-14T05:11:56.1342233Z note: the lint level is defined here
2020-03-14T05:11:56.1342898Z   --> /checkout/src/test/ui/lint/lint-type-limits3.rs:2:9
2020-03-14T05:11:56.1343123Z    |
2020-03-14T05:11:56.1343123Z    |
2020-03-14T05:11:56.1343297Z LL | #![warn(overflowing_literals)]
2020-03-14T05:11:56.1343537Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1344058Z    = note: the literal `200` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1344504Z error: aborting due to previous error
2020-03-14T05:11:56.1344660Z 
2020-03-14T05:11:56.1344751Z 
2020-03-14T05:11:56.1350598Z ------------------------------------------
---
2020-03-14T05:11:56.1359025Z 
2020-03-14T05:11:56.1359179Z 9    |
2020-03-14T05:11:56.1359377Z 10 LL | #![deny(overflowing_literals)]
2020-03-14T05:11:56.1359625Z 11    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1360089Z +    = note: the literal `256` does not fit into an `u8` whose maximum is 255, minimum is 0
2020-03-14T05:11:56.1360581Z 13 error: literal out of range for `u8`
2020-03-14T05:11:56.1361099Z 14   --> $DIR/lint-type-overflow.rs:13:14
2020-03-14T05:11:56.1361290Z 
2020-03-14T05:11:56.1361420Z 15    |
2020-03-14T05:11:56.1361420Z 15    |
2020-03-14T05:11:56.1361600Z 16 LL |     let x1 = 256_u8;
2020-03-14T05:11:56.1361835Z 17    |              ^^^^^^
2020-03-14T05:11:56.1362011Z +    |
2020-03-14T05:11:56.1362319Z +    = note: the literal `256_u8` does not fit into an `u8` whose maximum is 255, minimum is 0
2020-03-14T05:11:56.1362838Z 19 error: literal out of range for `i8`
2020-03-14T05:11:56.1363293Z 20   --> $DIR/lint-type-overflow.rs:16:18
2020-03-14T05:11:56.1363494Z 
2020-03-14T05:11:56.1363623Z 21    |
2020-03-14T05:11:56.1363623Z 21    |
2020-03-14T05:11:56.1363815Z 22 LL |     let x1: i8 = 128;
2020-03-14T05:11:56.1364045Z 23    |                  ^^^
2020-03-14T05:11:56.1364238Z +    |
2020-03-14T05:11:56.1364774Z +    = note: the literal `128` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1365285Z 25 error: literal out of range for `i8`
2020-03-14T05:11:56.1365731Z 26   --> $DIR/lint-type-overflow.rs:18:19
2020-03-14T05:11:56.1365919Z 
2020-03-14T05:11:56.1366066Z 27    |
2020-03-14T05:11:56.1366066Z 27    |
2020-03-14T05:11:56.1366427Z 28 LL |     let x3: i8 = -129;
2020-03-14T05:11:56.1366663Z 29    |                   ^^^
2020-03-14T05:11:56.1366840Z +    |
2020-03-14T05:11:56.1367379Z +    = note: the literal `129` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1367875Z 31 error: literal out of range for `i8`
2020-03-14T05:11:56.1368339Z 32   --> $DIR/lint-type-overflow.rs:19:19
2020-03-14T05:11:56.1368530Z 
2020-03-14T05:11:56.1368661Z 33    |
2020-03-14T05:11:56.1368661Z 33    |
2020-03-14T05:11:56.1369021Z 34 LL |     let x3: i8 = -(129);
2020-03-14T05:11:56.1369470Z +    |
2020-03-14T05:11:56.1369470Z +    |
2020-03-14T05:11:56.1369995Z +    = note: the literal `129` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1370503Z 37 error: literal out of range for `i8`
2020-03-14T05:11:56.1370952Z 38   --> $DIR/lint-type-overflow.rs:20:20
2020-03-14T05:11:56.1371157Z 
2020-03-14T05:11:56.1371287Z 39    |
2020-03-14T05:11:56.1371287Z 39    |
2020-03-14T05:11:56.1371649Z 40 LL |     let x3: i8 = -{129};
2020-03-14T05:11:56.1372091Z +    |
2020-03-14T05:11:56.1372091Z +    |
2020-03-14T05:11:56.1372612Z +    = note: the literal `129` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1373124Z 43 error: literal out of range for `i8`
2020-03-14T05:11:56.1373573Z 44   --> $DIR/lint-type-overflow.rs:22:10
2020-03-14T05:11:56.1373761Z 
2020-03-14T05:11:56.1373909Z 45    |
2020-03-14T05:11:56.1373909Z 45    |
2020-03-14T05:11:56.1374078Z 46 LL |     test(1000);
2020-03-14T05:11:56.1374274Z 47    |          ^^^^
2020-03-14T05:11:56.1374439Z +    |
2020-03-14T05:11:56.1374980Z +    = note: the literal `1000` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1375480Z 49 error: literal out of range for `i8`
2020-03-14T05:11:56.1375943Z 50   --> $DIR/lint-type-overflow.rs:24:13
2020-03-14T05:11:56.1376131Z 
2020-03-14T05:11:56.1376261Z 51    |
2020-03-14T05:11:56.1376261Z 51    |
2020-03-14T05:11:56.1376453Z 52 LL |     let x = 128_i8;
2020-03-14T05:11:56.1376665Z 53    |             ^^^^^^
2020-03-14T05:11:56.1376838Z +    |
2020-03-14T05:11:56.1377368Z +    = note: the literal `128_i8` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1377938Z 55 error: literal out of range for `i8`
2020-03-14T05:11:56.1378400Z 56   --> $DIR/lint-type-overflow.rs:28:14
2020-03-14T05:11:56.1378601Z 
2020-03-14T05:11:56.1378731Z 57    |
2020-03-14T05:11:56.1378731Z 57    |
2020-03-14T05:11:56.1379069Z 58 LL |     let x = -129_i8;
2020-03-14T05:11:56.1379303Z 59    |              ^^^^^^
2020-03-14T05:11:56.1379528Z +    |
2020-03-14T05:11:56.1380067Z +    = note: the literal `129_i8` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1380589Z 61 error: literal out of range for `i32`
2020-03-14T05:11:56.1381040Z 62   --> $DIR/lint-type-overflow.rs:32:18
2020-03-14T05:11:56.1381227Z 
2020-03-14T05:11:56.1381374Z 63    |
2020-03-14T05:11:56.1381374Z 63    |
2020-03-14T05:11:56.1381579Z 64 LL |     let x: i32 = 2147483648;
2020-03-14T05:11:56.1381833Z 65    |                  ^^^^^^^^^^
2020-03-14T05:11:56.1382037Z +    |
2020-03-14T05:11:56.1382628Z +    = note: the literal `2147483648` does not fit into an `i32` whose maximum is 2147483647, minimum is -2147483648
2020-03-14T05:11:56.1383184Z 67 error: literal out of range for `i32`
2020-03-14T05:11:56.1383636Z 68   --> $DIR/lint-type-overflow.rs:33:13
2020-03-14T05:11:56.1383824Z 
2020-03-14T05:11:56.1383953Z 69    |
2020-03-14T05:11:56.1383953Z 69    |
2020-03-14T05:11:56.1384160Z 70 LL |     let x = 2147483648_i32;
2020-03-14T05:11:56.1384405Z 71    |             ^^^^^^^^^^^^^^
2020-03-14T05:11:56.1384594Z +    |
2020-03-14T05:11:56.1385204Z +    = note: the literal `2147483648_i32` does not fit into an `i32` whose maximum is 2147483647, minimum is -2147483648
2020-03-14T05:11:56.1385750Z 73 error: literal out of range for `i32`
2020-03-14T05:11:56.1386215Z 74   --> $DIR/lint-type-overflow.rs:36:19
2020-03-14T05:11:56.1386401Z 
2020-03-14T05:11:56.1386531Z 75    |
2020-03-14T05:11:56.1386531Z 75    |
2020-03-14T05:11:56.1386905Z 76 LL |     let x: i32 = -2147483649;
2020-03-14T05:11:56.1387176Z 77    |                   ^^^^^^^^^^
2020-03-14T05:11:56.1387366Z +    |
2020-03-14T05:11:56.1387948Z +    = note: the literal `2147483649` does not fit into an `i32` whose maximum is 2147483647, minimum is -2147483648
2020-03-14T05:11:56.1388498Z 79 error: literal out of range for `i32`
2020-03-14T05:11:56.1388949Z 80   --> $DIR/lint-type-overflow.rs:37:14
2020-03-14T05:11:56.1389150Z 
2020-03-14T05:11:56.1389280Z 81    |
2020-03-14T05:11:56.1389280Z 81    |
2020-03-14T05:11:56.1389648Z 82 LL |     let x = -2147483649_i32;
2020-03-14T05:11:56.1389896Z 83    |              ^^^^^^^^^^^^^^
2020-03-14T05:11:56.1390099Z +    |
2020-03-14T05:11:56.1390691Z +    = note: the literal `2147483649_i32` does not fit into an `i32` whose maximum is 2147483647, minimum is -2147483648
2020-03-14T05:11:56.1391385Z 85 error: literal out of range for `i32`
2020-03-14T05:11:56.1391843Z 86   --> $DIR/lint-type-overflow.rs:38:13
2020-03-14T05:11:56.1392030Z 
2020-03-14T05:11:56.1392178Z 87    |
2020-03-14T05:11:56.1392178Z 87    |
2020-03-14T05:11:56.1392363Z 88 LL |     let x = 2147483648;
2020-03-14T05:11:56.1392728Z 89    |             ^^^^^^^^^^
2020-03-14T05:11:56.1406499Z +    |
2020-03-14T05:11:56.1407327Z +    = note: the literal `2147483648` does not fit into an `i32` whose maximum is 2147483647, minimum is -2147483648
2020-03-14T05:11:56.1407861Z 91 error: literal out of range for `i64`
2020-03-14T05:11:56.1408328Z 92   --> $DIR/lint-type-overflow.rs:40:13
2020-03-14T05:11:56.1408511Z 
2020-03-14T05:11:56.1408633Z 93    |
2020-03-14T05:11:56.1408633Z 93    |
2020-03-14T05:11:56.1408844Z 94 LL |     let x = 9223372036854775808_i64;
2020-03-14T05:11:56.1409096Z 95    |             ^^^^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1409284Z +    |
2020-03-14T05:11:56.1409968Z +    = note: the literal `9223372036854775808_i64` does not fit into an `i64` whose maximum is 9223372036854775807, minimum is -9223372036854775808
2020-03-14T05:11:56.1410523Z 97 error: literal out of range for `i64`
2020-03-14T05:11:56.1411130Z 98   --> $DIR/lint-type-overflow.rs:42:13
2020-03-14T05:11:56.1411305Z 
2020-03-14T05:11:56.1411427Z 99    |
2020-03-14T05:11:56.1411427Z 99    |
2020-03-14T05:11:56.1411754Z 100 LL |     let x = 18446744073709551615_i64;
2020-03-14T05:11:56.1412034Z 101    |             ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1412223Z +    |
2020-03-14T05:11:56.1413639Z +    = note: the literal `18446744073709551615_i64` does not fit into an `i64` whose maximum is 9223372036854775807, minimum is -9223372036854775808
2020-03-14T05:11:56.1414312Z 103 error: literal out of range for `i64`
2020-03-14T05:11:56.1414775Z 104   --> $DIR/lint-type-overflow.rs:43:19
2020-03-14T05:11:56.1414969Z 
2020-03-14T05:11:56.1415094Z 105    |
2020-03-14T05:11:56.1415094Z 105    |
2020-03-14T05:11:56.1415471Z 106 LL |     let x: i64 = -9223372036854775809;
2020-03-14T05:11:56.1415763Z 107    |                   ^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1415959Z +    |
2020-03-14T05:11:56.1416566Z +    = note: the literal `9223372036854775809` does not fit into an `i64` whose maximum is 9223372036854775807, minimum is -9223372036854775808
2020-03-14T05:11:56.1417131Z 109 error: literal out of range for `i64`
2020-03-14T05:11:56.1417554Z 110   --> $DIR/lint-type-overflow.rs:44:14
2020-03-14T05:11:56.1417730Z 
2020-03-14T05:11:56.1417866Z 111    |
2020-03-14T05:11:56.1417866Z 111    |
2020-03-14T05:11:56.1418413Z 112 LL |     let x = -9223372036854775809_i64;
2020-03-14T05:11:56.1418695Z 113    |              ^^^^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1418922Z +    |
2020-03-14T05:11:56.1419590Z +    = note: the literal `9223372036854775809_i64` does not fit into an `i64` whose maximum is 9223372036854775807, minimum is -9223372036854775808
2020-03-14T05:11:56.1420212Z 115 error: aborting due to 18 previous errors
2020-03-14T05:11:56.1420419Z 116 
2020-03-14T05:11:56.1420524Z 
2020-03-14T05:11:56.1420623Z 
2020-03-14T05:11:56.1420623Z 
2020-03-14T05:11:56.1420848Z The actual stderr differed from the expected stderr.
2020-03-14T05:11:56.1421526Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow/lint-type-overflow.stderr
2020-03-14T05:11:56.1422178Z To update references, rerun the tests and pass the `--bless` flag
2020-03-14T05:11:56.1422783Z To only update this specific test, also pass `--test-args lint/lint-type-overflow.rs`
2020-03-14T05:11:56.1423226Z error: 1 errors occurred comparing output.
2020-03-14T05:11:56.1423466Z status: exit code: 1
2020-03-14T05:11:56.1423466Z status: exit code: 1
2020-03-14T05:11:56.1425423Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-type-overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow/auxiliary"
2020-03-14T05:11:56.1427042Z ------------------------------------------
2020-03-14T05:11:56.1427218Z 
2020-03-14T05:11:56.1427596Z ------------------------------------------
2020-03-14T05:11:56.1427802Z stderr:
---
2020-03-14T05:11:56.1430667Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:1:9
2020-03-14T05:11:56.1431012Z    |
2020-03-14T05:11:56.1431319Z LL | #![deny(overflowing_literals)]
2020-03-14T05:11:56.1431578Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1431993Z    = note: the literal `256` does not fit into an `u8` whose maximum is 255, minimum is 0
2020-03-14T05:11:56.1432468Z error: literal out of range for `u8`
2020-03-14T05:11:56.1432982Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:13:14
2020-03-14T05:11:56.1433225Z    |
2020-03-14T05:11:56.1433225Z    |
2020-03-14T05:11:56.1433507Z LL |     let x1 = 256_u8; //~ error: literal out of range for `u8`
2020-03-14T05:11:56.1434029Z    |
2020-03-14T05:11:56.1434029Z    |
2020-03-14T05:11:56.1434323Z    = note: the literal `256_u8` does not fit into an `u8` whose maximum is 255, minimum is 0
2020-03-14T05:11:56.1434791Z error: literal out of range for `i8`
2020-03-14T05:11:56.1435296Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:16:18
2020-03-14T05:11:56.1435554Z    |
2020-03-14T05:11:56.1435554Z    |
2020-03-14T05:11:56.1435833Z LL |     let x1: i8 = 128; //~ error: literal out of range for `i8`
2020-03-14T05:11:56.1436323Z    |
2020-03-14T05:11:56.1436323Z    |
2020-03-14T05:11:56.1436840Z    = note: the literal `128` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1437296Z error: literal out of range for `i8`
2020-03-14T05:11:56.1437802Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:18:19
2020-03-14T05:11:56.1438044Z    |
2020-03-14T05:11:56.1438044Z    |
2020-03-14T05:11:56.1438521Z LL |     let x3: i8 = -129; //~ error: literal out of range for `i8`
2020-03-14T05:11:56.1439023Z    |
2020-03-14T05:11:56.1439023Z    |
2020-03-14T05:11:56.1439535Z    = note: the literal `129` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1440005Z error: literal out of range for `i8`
2020-03-14T05:11:56.1440526Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:19:19
2020-03-14T05:11:56.1440775Z    |
2020-03-14T05:11:56.1440775Z    |
2020-03-14T05:11:56.1441270Z LL |     let x3: i8 = -(129); //~ error: literal out of range for `i8`
2020-03-14T05:11:56.1441765Z    |
2020-03-14T05:11:56.1441765Z    |
2020-03-14T05:11:56.1442289Z    = note: the literal `129` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1442740Z error: literal out of range for `i8`
2020-03-14T05:11:56.1443246Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:20:20
2020-03-14T05:11:56.1443496Z    |
2020-03-14T05:11:56.1443496Z    |
2020-03-14T05:11:56.1443973Z LL |     let x3: i8 = -{129}; //~ error: literal out of range for `i8`
2020-03-14T05:11:56.1444578Z    |
2020-03-14T05:11:56.1444578Z    |
2020-03-14T05:11:56.1445050Z    = note: the literal `129` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1445481Z error: literal out of range for `i8`
2020-03-14T05:11:56.1445934Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:22:10
2020-03-14T05:11:56.1446158Z    |
2020-03-14T05:11:56.1446407Z LL |     test(1000); //~ error: literal out of range for `i8`
2020-03-14T05:11:56.1446407Z LL |     test(1000); //~ error: literal out of range for `i8`
2020-03-14T05:11:56.1446664Z    |          ^^^^
2020-03-14T05:11:56.1446811Z    |
2020-03-14T05:11:56.1447300Z    = note: the literal `1000` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1447719Z error: literal out of range for `i8`
2020-03-14T05:11:56.1448169Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:24:13
2020-03-14T05:11:56.1448414Z    |
2020-03-14T05:11:56.1448414Z    |
2020-03-14T05:11:56.1448653Z LL |     let x = 128_i8; //~ error: literal out of range for `i8`
2020-03-14T05:11:56.1449091Z    |
2020-03-14T05:11:56.1449091Z    |
2020-03-14T05:11:56.1449574Z    = note: the literal `128_i8` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1450012Z error: literal out of range for `i8`
2020-03-14T05:11:56.1450472Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:28:14
2020-03-14T05:11:56.1450697Z    |
2020-03-14T05:11:56.1450697Z    |
2020-03-14T05:11:56.1451119Z LL |     let x = -129_i8; //~ error: literal out of range for `i8`
2020-03-14T05:11:56.1451622Z    |
2020-03-14T05:11:56.1451622Z    |
2020-03-14T05:11:56.1452116Z    = note: the literal `129_i8` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1452557Z error: literal out of range for `i32`
2020-03-14T05:11:56.1453011Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:32:18
2020-03-14T05:11:56.1453301Z    |
2020-03-14T05:11:56.1453301Z    |
2020-03-14T05:11:56.1453569Z LL |     let x: i32 = 2147483648; //~ error: literal out of range for `i32`
2020-03-14T05:11:56.1454065Z    |
2020-03-14T05:11:56.1454065Z    |
2020-03-14T05:11:56.1454604Z    = note: the literal `2147483648` does not fit into an `i32` whose maximum is 2147483647, minimum is -2147483648
2020-03-14T05:11:56.1455070Z error: literal out of range for `i32`
2020-03-14T05:11:56.1455546Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:33:13
2020-03-14T05:11:56.1455773Z    |
2020-03-14T05:11:56.1455773Z    |
2020-03-14T05:11:56.1456039Z LL |     let x = 2147483648_i32; //~ error: literal out of range for `i32`
2020-03-14T05:11:56.1456524Z    |
2020-03-14T05:11:56.1456524Z    |
2020-03-14T05:11:56.1457064Z    = note: the literal `2147483648_i32` does not fit into an `i32` whose maximum is 2147483647, minimum is -2147483648
2020-03-14T05:11:56.1457548Z error: literal out of range for `i32`
2020-03-14T05:11:56.1458003Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:36:19
2020-03-14T05:11:56.1458248Z    |
2020-03-14T05:11:56.1458248Z    |
2020-03-14T05:11:56.1458705Z LL |     let x: i32 = -2147483649; //~ error: literal out of range for `i32`
2020-03-14T05:11:56.1459797Z    |
2020-03-14T05:11:56.1459797Z    |
2020-03-14T05:11:56.1460514Z    = note: the literal `2147483649` does not fit into an `i32` whose maximum is 2147483647, minimum is -2147483648
2020-03-14T05:11:56.1460982Z error: literal out of range for `i32`
2020-03-14T05:11:56.1461480Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:37:14
2020-03-14T05:11:56.1461707Z    |
2020-03-14T05:11:56.1461707Z    |
2020-03-14T05:11:56.1462151Z LL |     let x = -2147483649_i32; //~ error: literal out of range for `i32`
2020-03-14T05:11:56.1462644Z    |
2020-03-14T05:11:56.1462644Z    |
2020-03-14T05:11:56.1463183Z    = note: the literal `2147483649_i32` does not fit into an `i32` whose maximum is 2147483647, minimum is -2147483648
2020-03-14T05:11:56.1463849Z error: literal out of range for `i32`
2020-03-14T05:11:56.1464340Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:38:13
2020-03-14T05:11:56.1464583Z    |
2020-03-14T05:11:56.1467692Z LL |     let x = 2147483648; //~ error: literal out of range for `i32`
2020-03-14T05:11:56.1467692Z LL |     let x = 2147483648; //~ error: literal out of range for `i32`
2020-03-14T05:11:56.1468021Z    |             ^^^^^^^^^^
2020-03-14T05:11:56.1468193Z    |
2020-03-14T05:11:56.1468935Z    = note: the literal `2147483648` does not fit into an `i32` whose maximum is 2147483647, minimum is -2147483648
2020-03-14T05:11:56.1469447Z error: literal out of range for `i64`
2020-03-14T05:11:56.1469969Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:40:13
2020-03-14T05:11:56.1470213Z    |
2020-03-14T05:11:56.1470213Z    |
2020-03-14T05:11:56.1470511Z LL |     let x = 9223372036854775808_i64; //~ error: literal out of range for `i64`
2020-03-14T05:11:56.1471305Z    |
2020-03-14T05:11:56.1471305Z    |
2020-03-14T05:11:56.1471980Z    = note: the literal `9223372036854775808_i64` does not fit into an `i64` whose maximum is 9223372036854775807, minimum is -9223372036854775808
2020-03-14T05:11:56.1472559Z error: literal out of range for `i64`
2020-03-14T05:11:56.1473060Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:42:13
2020-03-14T05:11:56.1473306Z    |
2020-03-14T05:11:56.1473306Z    |
2020-03-14T05:11:56.1473621Z LL |     let x = 18446744073709551615_i64; //~ error: literal out of range for `i64`
2020-03-14T05:11:56.1474175Z    |
2020-03-14T05:11:56.1474175Z    |
2020-03-14T05:11:56.1474986Z    = note: the literal `18446744073709551615_i64` does not fit into an `i64` whose maximum is 9223372036854775807, minimum is -9223372036854775808
2020-03-14T05:11:56.1475543Z error: literal out of range for `i64`
2020-03-14T05:11:56.1476064Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:43:19
2020-03-14T05:11:56.1476374Z    |
2020-03-14T05:11:56.1476374Z    |
2020-03-14T05:11:56.1476902Z LL |     let x: i64 = -9223372036854775809; //~ error: literal out of range for `i64`
2020-03-14T05:11:56.1477486Z    |
2020-03-14T05:11:56.1477486Z    |
2020-03-14T05:11:56.1478137Z    = note: the literal `9223372036854775809` does not fit into an `i64` whose maximum is 9223372036854775807, minimum is -9223372036854775808
2020-03-14T05:11:56.1478699Z error: literal out of range for `i64`
2020-03-14T05:11:56.1479194Z   --> /checkout/src/test/ui/lint/lint-type-overflow.rs:44:14
2020-03-14T05:11:56.1479440Z    |
2020-03-14T05:11:56.1479440Z    |
2020-03-14T05:11:56.1479974Z LL |     let x = -9223372036854775809_i64; //~ error: literal out of range for `i64`
2020-03-14T05:11:56.1480531Z    |
2020-03-14T05:11:56.1480531Z    |
2020-03-14T05:11:56.1481209Z    = note: the literal `9223372036854775809_i64` does not fit into an `i64` whose maximum is 9223372036854775807, minimum is -9223372036854775808
2020-03-14T05:11:56.1481773Z error: aborting due to 18 previous errors
2020-03-14T05:11:56.1481964Z 
2020-03-14T05:11:56.1482063Z 
2020-03-14T05:11:56.1482427Z ------------------------------------------
---
2020-03-14T05:11:56.1483475Z 
2020-03-14T05:11:56.1483601Z 9    |
2020-03-14T05:11:56.1483814Z 10 LL | #![deny(overflowing_literals)]
2020-03-14T05:11:56.1484063Z 11    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1484650Z +    = note: the literal `128` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1485167Z 13 error: literal out of range for `f32`
2020-03-14T05:11:56.1485625Z 14   --> $DIR/lint-type-overflow2.rs:9:14
2020-03-14T05:11:56.1485830Z 
2020-03-14T05:11:56.1485962Z 15    |
2020-03-14T05:11:56.1485962Z 15    |
2020-03-14T05:11:56.1486343Z 16 LL |     let x = -3.40282357e+38_f32;
2020-03-14T05:11:56.1486819Z +    |
2020-03-14T05:11:56.1486819Z +    |
2020-03-14T05:11:56.1487134Z +    = note: the literal `3.40282357e+38_f32` does not fit into an `f32` as it is represents infinity
2020-03-14T05:11:56.1487656Z 19 error: literal out of range for `f32`
2020-03-14T05:11:56.1488110Z 20   --> $DIR/lint-type-overflow2.rs:10:14
2020-03-14T05:11:56.1488297Z 
2020-03-14T05:11:56.1488443Z 21    |
2020-03-14T05:11:56.1488443Z 21    |
2020-03-14T05:11:56.1488644Z 22 LL |     let x =  3.40282357e+38_f32;
2020-03-14T05:11:56.1489097Z +    |
2020-03-14T05:11:56.1489097Z +    |
2020-03-14T05:11:56.1489425Z +    = note: the literal `3.40282357e+38_f32` does not fit into an `f32` as it is represents infinity
2020-03-14T05:11:56.1489930Z 25 error: literal out of range for `f64`
2020-03-14T05:11:56.1490397Z 26   --> $DIR/lint-type-overflow2.rs:11:14
2020-03-14T05:11:56.1490590Z 
2020-03-14T05:11:56.1490719Z 27    |
2020-03-14T05:11:56.1490719Z 27    |
2020-03-14T05:11:56.1491134Z 28 LL |     let x = -1.7976931348623159e+308_f64;
2020-03-14T05:11:56.1491636Z +    |
2020-03-14T05:11:56.1491636Z +    |
2020-03-14T05:11:56.1491980Z +    = note: the literal `1.7976931348623159e+308_f64` does not fit into an `f64` as it is represents infinity
2020-03-14T05:11:56.1492505Z 31 error: literal out of range for `f64`
2020-03-14T05:11:56.1492981Z 32   --> $DIR/lint-type-overflow2.rs:12:14
2020-03-14T05:11:56.1493169Z 
2020-03-14T05:11:56.1493299Z 33    |
2020-03-14T05:11:56.1493299Z 33    |
2020-03-14T05:11:56.1493568Z 34 LL |     let x =  1.7976931348623159e+308_f64;
2020-03-14T05:11:56.1494086Z +    |
2020-03-14T05:11:56.1494086Z +    |
2020-03-14T05:11:56.1494416Z +    = note: the literal `1.7976931348623159e+308_f64` does not fit into an `f64` as it is represents infinity
2020-03-14T05:11:56.1494950Z 37 error: aborting due to 5 previous errors
2020-03-14T05:11:56.1495303Z 38 
2020-03-14T05:11:56.1495399Z 
2020-03-14T05:11:56.1495505Z 
2020-03-14T05:11:56.1495505Z 
2020-03-14T05:11:56.1495699Z The actual stderr differed from the expected stderr.
2020-03-14T05:11:56.1496347Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/lint-type-overflow2.stderr
2020-03-14T05:11:56.1496963Z To update references, rerun the tests and pass the `--bless` flag
2020-03-14T05:11:56.1497513Z To only update this specific test, also pass `--test-args lint/lint-type-overflow2.rs`
2020-03-14T05:11:56.1497926Z error: 1 errors occurred comparing output.
2020-03-14T05:11:56.1498168Z status: exit code: 1
2020-03-14T05:11:56.1498168Z status: exit code: 1
2020-03-14T05:11:56.1499980Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-type-overflow2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/auxiliary"
2020-03-14T05:11:56.1501484Z ------------------------------------------
2020-03-14T05:11:56.1501647Z 
2020-03-14T05:11:56.1501998Z ------------------------------------------
2020-03-14T05:11:56.1502188Z stderr:
2020-03-14T05:11:56.1502188Z stderr:
2020-03-14T05:11:56.1502528Z ------------------------------------------
2020-03-14T05:11:56.1502788Z error: literal out of range for `i8`
2020-03-14T05:11:56.1503243Z   --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:7:20
2020-03-14T05:11:56.1503471Z    |
2020-03-14T05:11:56.1503907Z LL |     let x2: i8 = --128; //~ ERROR literal out of range for `i8`
2020-03-14T05:11:56.1504346Z    |
2020-03-14T05:11:56.1504538Z note: the lint level is defined here
2020-03-14T05:11:56.1504993Z   --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:3:9
2020-03-14T05:11:56.1505220Z    |
2020-03-14T05:11:56.1505220Z    |
2020-03-14T05:11:56.1505409Z LL | #![deny(overflowing_literals)]
2020-03-14T05:11:56.1505633Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1506157Z    = note: the literal `128` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1506607Z error: literal out of range for `f32`
2020-03-14T05:11:56.1507062Z   --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:9:14
2020-03-14T05:11:56.1507309Z    |
2020-03-14T05:11:56.1507309Z    |
2020-03-14T05:11:56.1507743Z LL |     let x = -3.40282357e+38_f32; //~ ERROR literal out of range for `f32`
2020-03-14T05:11:56.1509012Z    |
2020-03-14T05:11:56.1509012Z    |
2020-03-14T05:11:56.1509340Z    = note: the literal `3.40282357e+38_f32` does not fit into an `f32` as it is represents infinity
2020-03-14T05:11:56.1509812Z error: literal out of range for `f32`
2020-03-14T05:11:56.1510353Z   --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:10:14
2020-03-14T05:11:56.1510603Z    |
2020-03-14T05:11:56.1510603Z    |
2020-03-14T05:11:56.1510973Z LL |     let x =  3.40282357e+38_f32; //~ ERROR literal out of range for `f32`
2020-03-14T05:11:56.1511498Z    |
2020-03-14T05:11:56.1511498Z    |
2020-03-14T05:11:56.1511806Z    = note: the literal `3.40282357e+38_f32` does not fit into an `f32` as it is represents infinity
2020-03-14T05:11:56.1512287Z error: literal out of range for `f64`
2020-03-14T05:11:56.1512891Z   --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:11:14
2020-03-14T05:11:56.1513142Z    |
2020-03-14T05:11:56.1513142Z    |
2020-03-14T05:11:56.1513666Z LL |     let x = -1.7976931348623159e+308_f64; //~ ERROR literal out of range for `f64`
2020-03-14T05:11:56.1514227Z    |
2020-03-14T05:11:56.1514227Z    |
2020-03-14T05:11:56.1514623Z    = note: the literal `1.7976931348623159e+308_f64` does not fit into an `f64` as it is represents infinity
2020-03-14T05:11:56.1515101Z error: literal out of range for `f64`
2020-03-14T05:11:56.1515624Z   --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:12:14
2020-03-14T05:11:56.1515872Z    |
2020-03-14T05:11:56.1515872Z    |
2020-03-14T05:11:56.1516155Z LL |     let x =  1.7976931348623159e+308_f64; //~ ERROR literal out of range for `f64`
2020-03-14T05:11:56.1516720Z    |
2020-03-14T05:11:56.1516720Z    |
2020-03-14T05:11:56.1517043Z    = note: the literal `1.7976931348623159e+308_f64` does not fit into an `f64` as it is represents infinity
2020-03-14T05:11:56.1517544Z error: aborting due to 5 previous errors
2020-03-14T05:11:56.1517717Z 
2020-03-14T05:11:56.1517814Z 
2020-03-14T05:11:56.1518181Z ------------------------------------------
---
2020-03-14T05:11:56.1519219Z 
2020-03-14T05:11:56.1519346Z 9    |
2020-03-14T05:11:56.1519543Z 10 LL | #![warn(overflowing_literals)]
2020-03-14T05:11:56.1519790Z 11    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1520391Z +    = note: the literal `255i8` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1521004Z 13 warning: literal out of range for i8
2020-03-14T05:11:56.1521431Z 14   --> $DIR/type-overflow.rs:10:16
2020-03-14T05:11:56.1521595Z 
2020-03-14T05:11:56.1521685Z 
2020-03-14T05:11:56.1521685Z 
2020-03-14T05:11:56.1521883Z The actual stderr differed from the expected stderr.
2020-03-14T05:11:56.1522499Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow/type-overflow.stderr
2020-03-14T05:11:56.1523076Z To update references, rerun the tests and pass the `--bless` flag
2020-03-14T05:11:56.1523610Z To only update this specific test, also pass `--test-args lint/type-overflow.rs`
2020-03-14T05:11:56.1524035Z error: 1 errors occurred comparing output.
2020-03-14T05:11:56.1524255Z status: exit code: 0
2020-03-14T05:11:56.1524255Z status: exit code: 0
2020-03-14T05:11:56.1526038Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/type-overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow/auxiliary"
2020-03-14T05:11:56.1527705Z ------------------------------------------
2020-03-14T05:11:56.1527884Z 
2020-03-14T05:11:56.1528242Z ------------------------------------------
2020-03-14T05:11:56.1528468Z stderr:
2020-03-14T05:11:56.1528468Z stderr:
2020-03-14T05:11:56.1528837Z ------------------------------------------
2020-03-14T05:11:56.1529098Z warning: literal out of range for `i8`
2020-03-14T05:11:56.1529592Z   --> /checkout/src/test/ui/lint/type-overflow.rs:5:17
2020-03-14T05:11:56.1529825Z    |
2020-03-14T05:11:56.1530073Z LL |     let error = 255i8; //~WARNING literal out of range for `i8`
2020-03-14T05:11:56.1530543Z    |
2020-03-14T05:11:56.1530733Z note: the lint level is defined here
2020-03-14T05:11:56.1531221Z   --> /checkout/src/test/ui/lint/type-overflow.rs:2:9
2020-03-14T05:11:56.1531501Z    |
2020-03-14T05:11:56.1531501Z    |
2020-03-14T05:11:56.1531692Z LL | #![warn(overflowing_literals)]
2020-03-14T05:11:56.1531926Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-14T05:11:56.1532524Z    = note: the literal `255i8` does not fit into an `i8` whose maximum is 127, minimum is -128
2020-03-14T05:11:56.1532985Z warning: literal out of range for i8
2020-03-14T05:11:56.1533543Z   --> /checkout/src/test/ui/lint/type-overflow.rs:10:16
2020-03-14T05:11:56.1533775Z    |
2020-03-14T05:11:56.1533775Z    |
2020-03-14T05:11:56.1534031Z LL |     let fail = 0b1000_0001i8; //~WARNING literal out of range for i8
2020-03-14T05:11:56.1534477Z    |                ^^^^^^^^^^^^^ help: consider using `u8` instead: `0b1000_0001u8`
2020-03-14T05:11:56.1534780Z    |
2020-03-14T05:11:56.1535331Z    = note: the literal `0b1000_0001i8` (decimal `129`) does not fit into an `i8` and will become `-127i8`
2020-03-14T05:11:56.1535826Z warning: literal out of range for i64
2020-03-14T05:11:56.1536308Z   --> /checkout/src/test/ui/lint/type-overflow.rs:12:16
2020-03-14T05:11:56.1536555Z    |
2020-03-14T05:11:56.1536555Z    |
2020-03-14T05:11:56.1536830Z LL |     let fail = 0x8000_0000_0000_0000i64; //~WARNING literal out of range for i64
2020-03-14T05:11:56.1537325Z    |                ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `u64` instead: `0x8000_0000_0000_0000u64`
2020-03-14T05:11:56.1537689Z    |
2020-03-14T05:11:56.1538366Z    = note: the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into an `i64` and will become `-9223372036854775808i64`
2020-03-14T05:11:56.1538947Z warning: literal out of range for u32
2020-03-14T05:11:56.1539426Z   --> /checkout/src/test/ui/lint/type-overflow.rs:14:16
2020-03-14T05:11:56.1539665Z    |
2020-03-14T05:11:56.1539665Z    |
2020-03-14T05:11:56.1539927Z LL |     let fail = 0x1_FFFF_FFFFu32; //~WARNING literal out of range for u32
2020-03-14T05:11:56.1540391Z    |                ^^^^^^^^^^^^^^^^ help: consider using `u64` instead: `0x1_FFFF_FFFFu64`
2020-03-14T05:11:56.1540746Z    |
2020-03-14T05:11:56.1541101Z    = note: the literal `0x1_FFFF_FFFFu32` (decimal `8589934591`) does not fit into an `u32` and will become `4294967295u32`
2020-03-14T05:11:56.1541627Z warning: literal out of range for i128
2020-03-14T05:11:56.1542110Z   --> /checkout/src/test/ui/lint/type-overflow.rs:16:22
2020-03-14T05:11:56.1542365Z    |
2020-03-14T05:11:56.1542365Z    |
2020-03-14T05:11:56.1542619Z LL |     let fail: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000;
2020-03-14T05:11:56.1543239Z    |
2020-03-14T05:11:56.1543239Z    |
2020-03-14T05:11:56.1544075Z    = note: the literal `0x8000_0000_0000_0000_0000_0000_0000_0000` (decimal `170141183460469231731687303715884105728`) does not fit into an `i128` and will become `-170141183460469231731687303715884105728i128`
2020-03-14T05:11:56.1544920Z    = help: consider using `u128` instead
2020-03-14T05:11:56.1545298Z warning: literal out of range for i32
2020-03-14T05:11:56.1545780Z   --> /checkout/src/test/ui/lint/type-overflow.rs:19:16
2020-03-14T05:11:56.1546034Z    |
2020-03-14T05:11:56.1546034Z    |
2020-03-14T05:11:56.1546306Z LL |     let fail = 0x8FFF_FFFF_FFFF_FFFE; //~WARNING literal out of range for i32
2020-03-14T05:11:56.1546852Z    |
2020-03-14T05:11:56.1546852Z    |
2020-03-14T05:11:56.1547469Z    = note: the literal `0x8FFF_FFFF_FFFF_FFFE` (decimal `10376293541461622782`) does not fit into an `i32` and will become `-2i32`
2020-03-14T05:11:56.1547903Z    = help: consider using `i128` instead
2020-03-14T05:11:56.1548277Z warning: literal out of range for i8
2020-03-14T05:11:56.1548751Z   --> /checkout/src/test/ui/lint/type-overflow.rs:21:17
2020-03-14T05:11:56.1548998Z    |
2020-03-14T05:11:56.1548998Z    |
2020-03-14T05:11:56.1549452Z LL |     let fail = -0b1111_1111i8; //~WARNING literal out of range for i8
2020-03-14T05:11:56.1549890Z    |                 ^^^^^^^^^^^^^ help: consider using `i16` instead: `0b1111_1111i16`
2020-03-14T05:11:56.1550218Z    |
2020-03-14T05:11:56.1550811Z    = note: the literal `0b1111_1111i8` (decimal `255`) does not fit into an `i8` and will become `-1i8`
2020-03-14T05:11:56.1551308Z 
2020-03-14T05:11:56.1551703Z ------------------------------------------
2020-03-14T05:11:56.1551880Z 
2020-03-14T05:11:56.1551978Z 
---
2020-03-14T05:11:56.7195448Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-14T05:11:56.7196226Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-14T05:11:56.7196604Z 
2020-03-14T05:11:56.7196871Z 
2020-03-14T05:11:56.7200386Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-14T05:11:56.7203102Z 
2020-03-14T05:11:56.7203220Z 
2020-03-14T05:11:56.7203465Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-14T05:11:56.7203791Z Build completed unsuccessfully in 0:58:18
2020-03-14T05:11:56.7203791Z Build completed unsuccessfully in 0:58:18
2020-03-14T05:11:56.7204124Z == clock drift check ==
2020-03-14T05:11:56.7204380Z   local time: Sat Mar 14 05:11:56 UTC 2020
2020-03-14T05:11:56.7204774Z   network time: Sat, 14 Mar 2020 05:11:56 GMT
2020-03-14T05:11:56.7205108Z == end clock drift check ==
2020-03-14T05:11:56.7205371Z 
2020-03-14T05:11:56.7238382Z ##[error]Bash exited with code '1'.
2020-03-14T05:11:56.7250318Z ##[section]Finishing: Run build
2020-03-14T05:11:56.7304935Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69995/merge to s
2020-03-14T05:11:56.7309946Z Task         : Get sources
2020-03-14T05:11:56.7310266Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T05:11:56.7310563Z Version      : 1.0.0
2020-03-14T05:11:56.7310789Z Author       : Microsoft
2020-03-14T05:11:56.7310789Z Author       : Microsoft
2020-03-14T05:11:56.7311892Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-14T05:11:56.7312304Z ==============================================================================
2020-03-14T05:11:57.0532064Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-14T05:11:57.0539006Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69995/merge to s
2020-03-14T05:11:57.0645917Z Cleaning up task key
2020-03-14T05:11:57.0647169Z Start cleaning up orphan processes.
2020-03-14T05:11:57.0818619Z Terminate orphan process: pid (3708) (python)
2020-03-14T05:11:57.0986459Z ##[section]Finishing: Finalize Job
