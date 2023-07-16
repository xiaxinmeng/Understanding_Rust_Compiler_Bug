plain
2020-03-03T01:13:04.1549724Z ========================== Starting Command Output ===========================
2020-03-03T01:13:04.1556448Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/65181078-3447-4d63-9c87-e5a9cc228675.sh
2020-03-03T01:13:04.1557039Z 
2020-03-03T01:13:04.1561766Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-03T01:13:04.1583501Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69653/merge to s
2020-03-03T01:13:04.1587879Z Task         : Get sources
2020-03-03T01:13:04.1588209Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T01:13:04.1588528Z Version      : 1.0.0
2020-03-03T01:13:04.1588745Z Author       : Microsoft
---
2020-03-03T01:13:05.1527664Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-03T01:13:05.1533991Z ##[command]git config gc.auto 0
2020-03-03T01:13:05.1542657Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-03T01:13:05.1600896Z ##[command]git config --get-all http.proxy
2020-03-03T01:13:05.1716163Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69653/merge:refs/remotes/pull/69653/merge
---
2020-03-03T02:15:47.9741185Z .................................................................................................... 1700/9741
2020-03-03T02:15:52.6395722Z .................................................................................................... 1800/9741
2020-03-03T02:16:04.3457276Z ......................................................................i............................. 1900/9741
2020-03-03T02:16:10.8874580Z .................................................................................................... 2000/9741
2020-03-03T02:16:25.9317285Z .................................................F..........iiiii................................... 2100/9741
2020-03-03T02:16:36.3842829Z .................................................................................................... 2300/9741
2020-03-03T02:16:38.5485639Z .................................................................................................... 2400/9741
2020-03-03T02:16:41.4657318Z .................................................................................................... 2500/9741
2020-03-03T02:17:01.3646962Z .................................................................................................... 2600/9741
---
2020-03-03T02:19:39.4375605Z .....................i...............i.............................................................. 5000/9741
2020-03-03T02:19:48.6370335Z .................................................................................................... 5100/9741
2020-03-03T02:19:54.0911586Z ................................................................i................................... 5200/9741
2020-03-03T02:20:00.2934585Z .................................................................................................... 5300/9741
2020-03-03T02:20:08.8238756Z ...........................................ii.ii........i...i....................................... 5400/9741
2020-03-03T02:20:16.6578156Z ........F........................................................................FF................. 5600/9741
2020-03-03T02:20:26.1270098Z .F.................................................................................................. 5700/9741
2020-03-03T02:20:32.8394092Z ..................................i................................................................. 5800/9741
2020-03-03T02:20:38.4680232Z .................................................................................................... 5900/9741
2020-03-03T02:20:38.4680232Z .................................................................................................... 5900/9741
2020-03-03T02:20:48.8377234Z .............................................................................................F...... 6000/9741
2020-03-03T02:20:58.8859554Z ..........................ii...i..ii...........i.................................................... 6100/9741
2020-03-03T02:21:14.9973074Z ..........................................................................................F......... 6300/9741
2020-03-03T02:21:21.9874801Z ............................................................................F....................... 6400/9741
2020-03-03T02:21:21.9874801Z ............................................................................F....................... 6400/9741
2020-03-03T02:21:30.8764594Z .........................................................i..ii...................................... 6500/9741
2020-03-03T02:21:57.3543513Z .................................................................................................... 6700/9741
2020-03-03T02:21:59.6250131Z .................................................i.................................................. 6800/9741
2020-03-03T02:22:01.5857771Z .................................................................................................... 6900/9741
2020-03-03T02:22:03.6246158Z ...............................................................................i.................... 7000/9741
---
2020-03-03T02:23:40.7597559Z .................................................................................................... 7700/9741
2020-03-03T02:23:45.1871361Z .................................................................................................... 7800/9741
2020-03-03T02:23:50.2410136Z .................................................................................................... 7900/9741
2020-03-03T02:23:57.9584521Z .........................i.......................................................................... 8000/9741
2020-03-03T02:24:06.0897433Z ..........................................................................iiiiiii.i................. 8100/9741
2020-03-03T02:24:21.8044884Z ...............i......i.........FF..F...F........................................................... 8300/9741
2020-03-03T02:24:27.0898603Z .................................................................................................... 8400/9741
2020-03-03T02:24:39.8413839Z .................................................................................................... 8500/9741
2020-03-03T02:24:49.5547518Z .................................................................................................... 8600/9741
---
2020-03-03T02:26:44.1091205Z 
2020-03-03T02:26:44.1091590Z 17 error: unexpected `println` after identifier
2020-03-03T02:26:44.1095338Z 18   --> $DIR/issue-46836-identifier-not-instead-of-negation.rs:20:9
2020-03-03T02:26:44.1095604Z 19    |
2020-03-03T02:26:44.1096130Z - LL |     if not  // lack of braces is [sic]
2020-03-03T02:26:44.1096651Z -    |        ----- help: use `!` to perform logical negation
2020-03-03T02:26:44.1097096Z - LL |         println!("Then when?");
2020-03-03T02:26:44.1097577Z -    |         ^^^^^^^
2020-03-03T02:26:44.1097795Z + LL |       if not  // lack of braces is [sic]
2020-03-03T02:26:44.1098149Z +    |  ________-
2020-03-03T02:26:44.1098345Z + LL | |         println!("Then when?");
2020-03-03T02:26:44.1098883Z +    | |_______________|
2020-03-03T02:26:44.1099137Z +    |                 help: use `!` to perform logical negation
2020-03-03T02:26:44.1099353Z 24 
2020-03-03T02:26:44.1099353Z 24 
2020-03-03T02:26:44.1099533Z 25 error: expected `{`, found `;`
2020-03-03T02:26:44.1100227Z 
2020-03-03T02:26:44.1100313Z 
2020-03-03T02:26:44.1100510Z The actual stderr differed from the expected stderr.
2020-03-03T02:26:44.1101265Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation/issue-46836-identifier-not-instead-of-negation.stderr
2020-03-03T02:26:44.1101265Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation/issue-46836-identifier-not-instead-of-negation.stderr
2020-03-03T02:26:44.1101943Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:44.1102577Z To only update this specific test, also pass `--test-args did_you_mean/issue-46836-identifier-not-instead-of-negation.rs`
2020-03-03T02:26:44.1103021Z error: 1 errors occurred comparing output.
2020-03-03T02:26:44.1103246Z status: exit code: 1
2020-03-03T02:26:44.1103246Z status: exit code: 1
2020-03-03T02:26:44.1105531Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation/auxiliary"
2020-03-03T02:26:44.1107750Z ------------------------------------------
2020-03-03T02:26:44.1107931Z 
2020-03-03T02:26:44.1108328Z ------------------------------------------
2020-03-03T02:26:44.1108642Z stderr:
2020-03-03T02:26:44.1108642Z stderr:
2020-03-03T02:26:44.1109128Z ------------------------------------------
2020-03-03T02:26:44.1109500Z error: unexpected `for_you` after identifier
2020-03-03T02:26:44.1110329Z    |
2020-03-03T02:26:44.1110329Z    |
2020-03-03T02:26:44.1110494Z LL |     if not for_you {
2020-03-03T02:26:44.1110983Z    |        |
2020-03-03T02:26:44.1111212Z    |        help: use `!` to perform logical negation
2020-03-03T02:26:44.1111389Z 
2020-03-03T02:26:44.1111389Z 
2020-03-03T02:26:44.1111564Z error: unexpected `the_worst` after identifier
2020-03-03T02:26:44.1112419Z    |
2020-03-03T02:26:44.1112572Z LL |     while not the_worst {
2020-03-03T02:26:44.1112911Z    |           ----^^^^^^^^^
2020-03-03T02:26:44.1113100Z    |           |
2020-03-03T02:26:44.1113100Z    |           |
2020-03-03T02:26:44.1113322Z    |           help: use `!` to perform logical negation
2020-03-03T02:26:44.1113502Z 
2020-03-03T02:26:44.1113690Z error: unexpected `println` after identifier
2020-03-03T02:26:44.1114232Z   --> /checkout/src/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs:20:9
2020-03-03T02:26:44.1114510Z    |
2020-03-03T02:26:44.1114702Z LL |       if not  // lack of braces is [sic]
2020-03-03T02:26:44.1115028Z    |  ________-
2020-03-03T02:26:44.1115313Z LL | |         println!("Then when?");
2020-03-03T02:26:44.1115873Z    | |_______________|
2020-03-03T02:26:44.1116117Z    |                 help: use `!` to perform logical negation
2020-03-03T02:26:44.1116385Z 
2020-03-03T02:26:44.1116385Z 
2020-03-03T02:26:44.1116550Z error: expected `{`, found `;`
2020-03-03T02:26:44.1117379Z    |
2020-03-03T02:26:44.1117379Z    |
2020-03-03T02:26:44.1117569Z LL |     if not  // lack of braces is [sic]
2020-03-03T02:26:44.1118011Z    |     -- this `if` expression has a condition, but no block
2020-03-03T02:26:44.1118268Z LL |         println!("Then when?");
2020-03-03T02:26:44.1118706Z    |                               |
2020-03-03T02:26:44.1118933Z    |                               expected `{`
2020-03-03T02:26:44.1118933Z    |                               expected `{`
2020-03-03T02:26:44.1119293Z    |                               help: try placing this code inside a block: `{ ; }`
2020-03-03T02:26:44.1119708Z error: unexpected `2` after identifier
2020-03-03T02:26:44.1120249Z   --> /checkout/src/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs:26:24
2020-03-03T02:26:44.1120550Z    |
2020-03-03T02:26:44.1120703Z LL |     let resource = not 2;
2020-03-03T02:26:44.1120703Z LL |     let resource = not 2;
2020-03-03T02:26:44.1121046Z    |                    ----^
2020-03-03T02:26:44.1121250Z    |                    |
2020-03-03T02:26:44.1121504Z    |                    help: use `!` to perform logical negation
2020-03-03T02:26:44.1121702Z 
2020-03-03T02:26:44.1121898Z error: unexpected `be_smothered_out_before` after identifier
2020-03-03T02:26:44.1122764Z    |
2020-03-03T02:26:44.1122764Z    |
2020-03-03T02:26:44.1122957Z LL |     let young_souls = not be_smothered_out_before;
2020-03-03T02:26:44.1123641Z    |                       |
2020-03-03T02:26:44.1123903Z    |                       help: use `!` to perform logical negation
2020-03-03T02:26:44.1124123Z 
2020-03-03T02:26:44.1124287Z error: aborting due to 6 previous errors
---
2020-03-03T02:26:44.1131711Z 
2020-03-03T02:26:44.1131796Z 
2020-03-03T02:26:44.1131992Z The actual stderr differed from the expected stderr.
2020-03-03T02:26:44.1132785Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern/issue-47390-unused-variable-in-struct-pattern.stderr
2020-03-03T02:26:44.1133679Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:44.1134463Z To only update this specific test, also pass `--test-args lint/issue-47390-unused-variable-in-struct-pattern.rs`
2020-03-03T02:26:44.1134898Z error: 1 errors occurred comparing output.
2020-03-03T02:26:44.1135123Z status: exit code: 0
2020-03-03T02:26:44.1135123Z status: exit code: 0
2020-03-03T02:26:44.1137032Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern/auxiliary"
2020-03-03T02:26:44.1138582Z ------------------------------------------
2020-03-03T02:26:44.1138738Z 
2020-03-03T02:26:44.1139083Z ------------------------------------------
2020-03-03T02:26:44.1139263Z stderr:
2020-03-03T02:26:44.1139263Z stderr:
2020-03-03T02:26:44.1139590Z ------------------------------------------
2020-03-03T02:26:44.1139857Z warning: unused variable: `i_think_continually`
2020-03-03T02:26:44.1140656Z    |
2020-03-03T02:26:44.1140656Z    |
2020-03-03T02:26:44.1140940Z LL |     let i_think_continually = 2; //~ WARNING unused variable: `i_think_continually`
2020-03-03T02:26:44.1141381Z    |         ^^^^^^^^^^^^^^^^^^^ help: consider prefixing with an underscore: `_i_think_continually`
2020-03-03T02:26:44.1141859Z note: the lint level is defined here
2020-03-03T02:26:44.1142371Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:5:9
2020-03-03T02:26:44.1142639Z    |
2020-03-03T02:26:44.1142639Z    |
2020-03-03T02:26:44.1143031Z LL | #![warn(unused)] // UI tests pass `-A unused` (#43896)
2020-03-03T02:26:44.1143501Z    = note: `#[warn(unused_variables)]` implied by `#[warn(unused)]`
2020-03-03T02:26:44.1143709Z 
2020-03-03T02:26:44.1143884Z warning: unused variable: `mut_unused_var`
2020-03-03T02:26:44.1144413Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:33:13
---
2020-03-03T02:26:44.1181611Z    |
2020-03-03T02:26:44.1181788Z LL |     let (mut var, unused_var) = (1, 2);
2020-03-03T02:26:44.1182146Z    |                   ^^^^^^^^^^ help: consider prefixing with an underscore: `_unused_var`
2020-03-03T02:26:44.1182406Z 
2020-03-03T02:26:44.1182588Z warning: unused variable: `corridors_of_light`
2020-03-03T02:26:44.1183419Z    |
2020-03-03T02:26:44.1183419Z    |
2020-03-03T02:26:44.1183826Z LL |     if let SoulHistory { corridors_of_light, //~ WARNING unused variable: `corridors_of_light`
2020-03-03T02:26:44.1184328Z    |                          ^^^^^^^^^^^^^^^^^^ help: try ignoring the field: `corridors_of_light: _`
2020-03-03T02:26:44.1184671Z 
2020-03-03T02:26:44.1184877Z warning: variable `hours_are_suns` is assigned to, but never used
2020-03-03T02:26:44.1185762Z    |
2020-03-03T02:26:44.1185762Z    |
2020-03-03T02:26:44.1186021Z LL |                          mut hours_are_suns, //~ WARNING `hours_are_suns` is assigned to, but
2020-03-03T02:26:44.1187307Z    |
2020-03-03T02:26:44.1187307Z    |
2020-03-03T02:26:44.1187498Z    = note: consider using `_hours_are_suns` instead
2020-03-03T02:26:44.1187669Z 
2020-03-03T02:26:44.1187876Z warning: value assigned to `hours_are_suns` is never read
2020-03-03T02:26:44.1188752Z    |
2020-03-03T02:26:44.1188752Z    |
2020-03-03T02:26:44.1188979Z LL |         hours_are_suns = false; //~ WARNING unused_assignments
2020-03-03T02:26:44.1189376Z    |
2020-03-03T02:26:44.1189556Z note: the lint level is defined here
2020-03-03T02:26:44.1190074Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:5:9
2020-03-03T02:26:44.1190336Z    |
2020-03-03T02:26:44.1190336Z    |
2020-03-03T02:26:44.1190726Z LL | #![warn(unused)] // UI tests pass `-A unused` (#43896)
2020-03-03T02:26:44.1191198Z    = note: `#[warn(unused_assignments)]` implied by `#[warn(unused)]`
2020-03-03T02:26:44.1191512Z    = help: maybe it is overwritten before being read?
2020-03-03T02:26:44.1191685Z 
2020-03-03T02:26:44.1191846Z warning: unused variable: `fire`
2020-03-03T02:26:44.1191846Z warning: unused variable: `fire`
2020-03-03T02:26:44.1192367Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:52:32
2020-03-03T02:26:44.1192648Z    |
2020-03-03T02:26:44.1192911Z LL |     let LovelyAmbition { lips, fire } = the_spirit; //~ WARNING unused variable: `fire`
2020-03-03T02:26:44.1193318Z    |                                ^^^^ help: try ignoring the field: `fire: _`
2020-03-03T02:26:44.1193734Z warning: unused variable: `case`
2020-03-03T02:26:44.1194251Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:61:23
2020-03-03T02:26:44.1194530Z    |
2020-03-03T02:26:44.1194530Z    |
2020-03-03T02:26:44.1194773Z LL |         Large::Suit { case } => {} //~ WARNING unused variable: `case`
2020-03-03T02:26:44.1195141Z    |                       ^^^^ help: try ignoring the field: `case: _`
2020-03-03T02:26:44.1195540Z warning: unused variable: `case`
2020-03-03T02:26:44.1196055Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:66:24
2020-03-03T02:26:44.1196321Z    |
2020-03-03T02:26:44.1196321Z    |
2020-03-03T02:26:44.1196585Z LL |         &Large::Suit { case } => {} //~ WARNING unused variable: `case`
2020-03-03T02:26:44.1196960Z    |                        ^^^^ help: try ignoring the field: `case: _`
2020-03-03T02:26:44.1197365Z warning: unused variable: `case`
2020-03-03T02:26:44.1197885Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:71:27
2020-03-03T02:26:44.1198150Z    |
2020-03-03T02:26:44.1198150Z    |
2020-03-03T02:26:44.1198415Z LL |         box Large::Suit { case } => {} //~ WARNING unused variable: `case`
2020-03-03T02:26:44.1198800Z    |                           ^^^^ help: try ignoring the field: `case: _`
2020-03-03T02:26:44.1199191Z warning: unused variable: `case`
2020-03-03T02:26:44.1199722Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:76:24
2020-03-03T02:26:44.1199987Z    |
2020-03-03T02:26:44.1199987Z    |
2020-03-03T02:26:44.1200235Z LL |         (Large::Suit { case },) => {} //~ WARNING unused variable: `case`
2020-03-03T02:26:44.1200724Z    |                        ^^^^ help: try ignoring the field: `case: _`
2020-03-03T02:26:44.1201120Z warning: unused variable: `case`
2020-03-03T02:26:44.1201682Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:81:24
2020-03-03T02:26:44.1202022Z    |
2020-03-03T02:26:44.1202022Z    |
2020-03-03T02:26:44.1202268Z LL |         [Large::Suit { case }] => {} //~ WARNING unused variable: `case`
2020-03-03T02:26:44.1202652Z    |                        ^^^^ help: try ignoring the field: `case: _`
2020-03-03T02:26:44.1203037Z warning: unused variable: `case`
2020-03-03T02:26:44.1203575Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:86:29
2020-03-03T02:26:44.1203857Z    |
2020-03-03T02:26:44.1203857Z    |
2020-03-03T02:26:44.1204120Z LL |         Tuple(Large::Suit { case }, ()) => {} //~ WARNING unused variable: `case`
2020-03-03T02:26:44.1204515Z    |                             ^^^^ help: try ignoring the field: `case: _`
2020-03-03T02:26:44.1204947Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1205476Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:33:9
2020-03-03T02:26:44.1205754Z    |
2020-03-03T02:26:44.1205923Z LL |     let mut mut_unused_var = 1;
2020-03-03T02:26:44.1205923Z LL |     let mut mut_unused_var = 1;
2020-03-03T02:26:44.1206178Z    |         ^^^^^^^^^^^^^^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1206400Z    |
2020-03-03T02:26:44.1206564Z note: the lint level is defined here
2020-03-03T02:26:44.1207072Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:5:9
2020-03-03T02:26:44.1207334Z    |
2020-03-03T02:26:44.1207721Z LL | #![warn(unused)] // UI tests pass `-A unused` (#43896)
2020-03-03T02:26:44.1208178Z    = note: `#[warn(unused_mut)]` implied by `#[warn(unused)]`
2020-03-03T02:26:44.1208377Z 
2020-03-03T02:26:44.1208552Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1209081Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:37:10
---
2020-03-03T02:26:44.1214266Z 10   --> $DIR/lint-unused-mut-self.rs:4:9
2020-03-03T02:26:44.1214429Z 
2020-03-03T02:26:44.1214781Z 16   --> $DIR/lint-unused-mut-self.rs:9:12
2020-03-03T02:26:44.1214978Z 17    |
2020-03-03T02:26:44.1215164Z 18 LL |     fn bar(mut self: Box<Foo>) {}
2020-03-03T02:26:44.1215847Z -    |            |
2020-03-03T02:26:44.1216207Z -    |            help: remove this `mut`
2020-03-03T02:26:44.1216490Z +    |            ^^^^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1216687Z 22 
2020-03-03T02:26:44.1216687Z 22 
2020-03-03T02:26:44.1216863Z 23 error: aborting due to 2 previous errors
2020-03-03T02:26:44.1217056Z 24 
2020-03-03T02:26:44.1217147Z 
2020-03-03T02:26:44.1217234Z 
2020-03-03T02:26:44.1217416Z The actual stderr differed from the expected stderr.
2020-03-03T02:26:44.1218044Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-mut-self/lint-unused-mut-self.stderr
2020-03-03T02:26:44.1218835Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:44.1219447Z To only update this specific test, also pass `--test-args lint/lint-unused-mut-self.rs`
2020-03-03T02:26:44.1219974Z error: 1 errors occurred comparing output.
2020-03-03T02:26:44.1220196Z status: exit code: 1
2020-03-03T02:26:44.1220196Z status: exit code: 1
2020-03-03T02:26:44.1222186Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-unused-mut-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-mut-self" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-mut-self/auxiliary"
2020-03-03T02:26:44.1223769Z ------------------------------------------
2020-03-03T02:26:44.1223940Z 
2020-03-03T02:26:44.1224298Z ------------------------------------------
2020-03-03T02:26:44.1224519Z stderr:
2020-03-03T02:26:44.1224519Z stderr:
2020-03-03T02:26:44.1224887Z ------------------------------------------
2020-03-03T02:26:44.1225149Z error: variable does not need to be mutable
2020-03-03T02:26:44.1225831Z   --> /checkout/src/test/ui/lint/lint-unused-mut-self.rs:8:12
2020-03-03T02:26:44.1226064Z    |
2020-03-03T02:26:44.1226913Z LL |     fn foo(mut self) {} //~ ERROR: variable does not need to be mutable
2020-03-03T02:26:44.1227280Z    |            ^^^^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1227717Z note: the lint level is defined here
2020-03-03T02:26:44.1228249Z   --> /checkout/src/test/ui/lint/lint-unused-mut-self.rs:4:9
2020-03-03T02:26:44.1228482Z    |
2020-03-03T02:26:44.1228628Z LL | #![deny(unused_mut)]
2020-03-03T02:26:44.1228628Z LL | #![deny(unused_mut)]
2020-03-03T02:26:44.1228805Z    |         ^^^^^^^^^^
2020-03-03T02:26:44.1228953Z 
2020-03-03T02:26:44.1229123Z error: variable does not need to be mutable
2020-03-03T02:26:44.1229580Z   --> /checkout/src/test/ui/lint/lint-unused-mut-self.rs:9:12
2020-03-03T02:26:44.1229816Z    |
2020-03-03T02:26:44.1230095Z LL |     fn bar(mut self: Box<Foo>) {} //~ ERROR: variable does not need to be mutable
2020-03-03T02:26:44.1230453Z    |            ^^^^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1230810Z error: aborting due to 2 previous errors
2020-03-03T02:26:44.1230960Z 
2020-03-03T02:26:44.1231044Z 
2020-03-03T02:26:44.1231372Z ------------------------------------------
---
2020-03-03T02:26:44.1253814Z 71 warning: variable does not need to be mutable
2020-03-03T02:26:44.1254268Z 72   --> $DIR/lint-unused-mut-variables.rs:104:14
2020-03-03T02:26:44.1254452Z 
2020-03-03T02:26:44.1254565Z 73    |
2020-03-03T02:26:44.1254892Z 74 LL |     let x = |mut y: isize| 10;
2020-03-03T02:26:44.1255598Z -    |              |
2020-03-03T02:26:44.1256058Z -    |              help: remove this `mut`
2020-03-03T02:26:44.1256351Z +    |              ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1256558Z 78 
---
2020-03-03T02:26:44.1292664Z 143 warning: variable does not need to be mutable
2020-03-03T02:26:44.1293198Z 144   --> $DIR/lint-unused-mut-variables.rs:96:8
2020-03-03T02:26:44.1293389Z 
2020-03-03T02:26:44.1293510Z 145    |
2020-03-03T02:26:44.1293771Z 146 LL |       (mut x, 1) |
2020-03-03T02:26:44.1294457Z -    |        |
2020-03-03T02:26:44.1294825Z -    |        help: remove this `mut`
2020-03-03T02:26:44.1295110Z +    |        ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1295306Z 150 
---
2020-03-03T02:26:44.1298203Z 159 warning: variable does not need to be mutable
2020-03-03T02:26:44.1298660Z 160   --> $DIR/lint-unused-mut-variables.rs:114:9
2020-03-03T02:26:44.1298842Z 
2020-03-03T02:26:44.1298955Z 161    |
2020-03-03T02:26:44.1299124Z 162 LL |     let mut b = (&mut a,);
2020-03-03T02:26:44.1299758Z -    |         |
2020-03-03T02:26:44.1300103Z -    |         help: remove this `mut`
2020-03-03T02:26:44.1300372Z +    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1300557Z 166 
---
2020-03-03T02:26:44.1303455Z 175 warning: variable does not need to be mutable
2020-03-03T02:26:44.1303944Z 176   --> $DIR/lint-unused-mut-variables.rs:129:9
2020-03-03T02:26:44.1304135Z 
2020-03-03T02:26:44.1304255Z 177    |
2020-03-03T02:26:44.1304473Z 178 LL |     let mut v : &mut Vec<()> = &mut vec![];
2020-03-03T02:26:44.1305189Z -    |         |
2020-03-03T02:26:44.1305558Z -    |         help: remove this `mut`
2020-03-03T02:26:44.1306051Z +    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1306421Z 182 
---
2020-03-03T02:26:44.1309591Z 191 warning: variable does not need to be mutable
2020-03-03T02:26:44.1310080Z 192   --> $DIR/lint-unused-mut-variables.rs:106:13
2020-03-03T02:26:44.1310273Z 
2020-03-03T02:26:44.1310501Z 193    |
2020-03-03T02:26:44.1310803Z 194 LL |     fn what(mut foo: isize) {}
2020-03-03T02:26:44.1311534Z -    |             |
2020-03-03T02:26:44.1311920Z -    |             help: remove this `mut`
2020-03-03T02:26:44.1312226Z +    |             ^^^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1312434Z 198 
2020-03-03T02:26:44.1312434Z 198 
2020-03-03T02:26:44.1312635Z 199 warning: variable does not need to be mutable
2020-03-03T02:26:44.1313216Z 200   --> $DIR/lint-unused-mut-variables.rs:124:20
2020-03-03T02:26:44.1313420Z 
2020-03-03T02:26:44.1313544Z 201    |
2020-03-03T02:26:44.1314005Z 202 LL |     fn mut_ref_arg(mut arg : &mut [u8]) -> &mut [u8] {
2020-03-03T02:26:44.1314931Z -    |                    |
2020-03-03T02:26:44.1315352Z -    |                    help: remove this `mut`
2020-03-03T02:26:44.1315688Z +    |                    ^^^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1315912Z 206 
---
2020-03-03T02:26:44.1319484Z 216   --> $DIR/lint-unused-mut-variables.rs:198:8
2020-03-03T02:26:44.1319679Z 
2020-03-03T02:26:44.1319769Z 
2020-03-03T02:26:44.1319965Z The actual stderr differed from the expected stderr.
2020-03-03T02:26:44.1320658Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-mut-variables/lint-unused-mut-variables.stderr
2020-03-03T02:26:44.1321296Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:44.1321974Z To only update this specific test, also pass `--test-args lint/lint-unused-mut-variables.rs`
2020-03-03T02:26:44.1322390Z error: 1 errors occurred comparing output.
2020-03-03T02:26:44.1322597Z status: exit code: 1
2020-03-03T02:26:44.1322597Z status: exit code: 1
2020-03-03T02:26:44.1325579Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-unused-mut-variables.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-mut-variables" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-mut-variables/auxiliary"
2020-03-03T02:26:44.1327378Z ------------------------------------------
2020-03-03T02:26:44.1327559Z 
2020-03-03T02:26:44.1331416Z ------------------------------------------
2020-03-03T02:26:44.1331647Z stderr:
---
2020-03-03T02:26:44.1350687Z 
2020-03-03T02:26:44.1350864Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1351622Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:104:14
2020-03-03T02:26:44.1351882Z    |
2020-03-03T02:26:44.1352185Z LL |     let x = |mut y: isize| 10; //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1352587Z    |              ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1352966Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1353485Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:69:9
2020-03-03T02:26:44.1353750Z    |
2020-03-03T02:26:44.1353750Z    |
2020-03-03T02:26:44.1354015Z LL |     let mut a = 3; //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1354354Z    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1354740Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1355305Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:71:9
2020-03-03T02:26:44.1355691Z    |
2020-03-03T02:26:44.1355691Z    |
2020-03-03T02:26:44.1356060Z LL |     let mut a = 2; //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1356368Z    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1356722Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1357203Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:73:9
2020-03-03T02:26:44.1357428Z    |
2020-03-03T02:26:44.1357428Z    |
2020-03-03T02:26:44.1357682Z LL |     let mut b = 3; //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1357987Z    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1358320Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1358807Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:75:9
2020-03-03T02:26:44.1359031Z    |
2020-03-03T02:26:44.1359031Z    |
2020-03-03T02:26:44.1359279Z LL |     let mut a = vec![3]; //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1359610Z    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1360032Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1360559Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:77:10
2020-03-03T02:26:44.1360859Z    |
2020-03-03T02:26:44.1360859Z    |
2020-03-03T02:26:44.1361117Z LL |     let (mut a, b) = (1, 2); //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1361456Z    |          ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1361792Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1362284Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:79:9
2020-03-03T02:26:44.1362525Z    |
2020-03-03T02:26:44.1362525Z    |
2020-03-03T02:26:44.1362758Z LL |     let mut a; //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1363058Z    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1363406Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1363874Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:83:9
2020-03-03T02:26:44.1364114Z    |
2020-03-03T02:26:44.1364114Z    |
2020-03-03T02:26:44.1364351Z LL |     let mut b; //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1364645Z    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1364998Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1365469Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:92:9
2020-03-03T02:26:44.1365693Z    |
2020-03-03T02:26:44.1365693Z    |
2020-03-03T02:26:44.1365950Z LL |         mut x => {} //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1366258Z    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1366715Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1367232Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:96:8
2020-03-03T02:26:44.1367472Z    |
2020-03-03T02:26:44.1367472Z    |
2020-03-03T02:26:44.1367726Z LL |       (mut x, 1) | //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1368067Z    |        ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1368426Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1368948Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:109:9
2020-03-03T02:26:44.1369196Z    |
2020-03-03T02:26:44.1369196Z    |
2020-03-03T02:26:44.1369461Z LL |     let mut a = &mut 5; //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1369813Z    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1370167Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1370671Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:114:9
2020-03-03T02:26:44.1370930Z    |
2020-03-03T02:26:44.1370930Z    |
2020-03-03T02:26:44.1371303Z LL |     let mut b = (&mut a,); //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1371622Z    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1371970Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1372441Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:117:9
2020-03-03T02:26:44.1372683Z    |
2020-03-03T02:26:44.1372683Z    |
2020-03-03T02:26:44.1372934Z LL |     let mut x = &mut 1; //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1373247Z    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1373600Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1374073Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:129:9
2020-03-03T02:26:44.1374300Z    |
2020-03-03T02:26:44.1374300Z    |
2020-03-03T02:26:44.1374604Z LL |     let mut v : &mut Vec<()> = &mut vec![]; //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1374962Z    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1375318Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1375788Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:184:9
2020-03-03T02:26:44.1376014Z    |
2020-03-03T02:26:44.1376014Z    |
2020-03-03T02:26:44.1376298Z LL |     let mut raw_address_of_const = 1; //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1376734Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1377110Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1377622Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:106:13
2020-03-03T02:26:44.1377910Z    |
2020-03-03T02:26:44.1377910Z    |
2020-03-03T02:26:44.1378181Z LL |     fn what(mut foo: isize) {} //~ WARN: variable does not need to be mutable
2020-03-03T02:26:44.1378545Z    |             ^^^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1378889Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1379401Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:124:20
2020-03-03T02:26:44.1379630Z    |
2020-03-03T02:26:44.1379630Z    |
2020-03-03T02:26:44.1380019Z LL |     fn mut_ref_arg(mut arg : &mut [u8]) -> &mut [u8] {
2020-03-03T02:26:44.1380347Z    |                    ^^^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1380703Z error: variable does not need to be mutable
2020-03-03T02:26:44.1381177Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:202:9
2020-03-03T02:26:44.1381420Z    |
2020-03-03T02:26:44.1381420Z    |
2020-03-03T02:26:44.1381672Z LL |     let mut b = vec![2]; //~ ERROR: variable does not need to be mutable
2020-03-03T02:26:44.1381989Z    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1382353Z note: the lint level is defined here
2020-03-03T02:26:44.1382811Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:198:8
2020-03-03T02:26:44.1383054Z    |
2020-03-03T02:26:44.1383289Z LL | #[deny(unused_mut)]
---
2020-03-03T02:26:44.1385100Z diff of stderr:
2020-03-03T02:26:44.1385230Z 
2020-03-03T02:26:44.1385550Z 30   --> $DIR/suggestions.rs:49:13
2020-03-03T02:26:44.1385726Z 31    |
2020-03-03T02:26:44.1386112Z 32 LL |         let mut registry_no = (format!("NX-{}", 74205));
2020-03-03T02:26:44.1387131Z -    |             |
2020-03-03T02:26:44.1387501Z -    |             help: remove this `mut`
2020-03-03T02:26:44.1387806Z +    |             ^^^^^^^^^^^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1388023Z 36    |
---
2020-03-03T02:26:44.1389505Z 45    |
2020-03-03T02:26:44.1389796Z - LL |            let mut
2020-03-03T02:26:44.1390112Z -    |   _____________^
2020-03-03T02:26:44.1390440Z -    |  |_____________|
2020-03-03T02:26:44.1390729Z -    | ||
2020-03-03T02:26:44.1391038Z - LL | ||             b = 1;
2020-03-03T02:26:44.1391376Z -    | ||____________-^
2020-03-03T02:26:44.1392063Z -    |               help: remove this `mut`
2020-03-03T02:26:44.1392299Z + LL |           let mut
2020-03-03T02:26:44.1392472Z +    |  _____________^
2020-03-03T02:26:44.1392647Z + LL | |             b = 1;
---
2020-03-03T02:26:44.1393830Z 
2020-03-03T02:26:44.1393915Z 
2020-03-03T02:26:44.1394097Z The actual stderr differed from the expected stderr.
2020-03-03T02:26:44.1394662Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions/suggestions.stderr
2020-03-03T02:26:44.1395335Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:44.1395885Z To only update this specific test, also pass `--test-args lint/suggestions.rs`
2020-03-03T02:26:44.1396356Z error: 1 errors occurred comparing output.
2020-03-03T02:26:44.1396565Z status: exit code: 1
2020-03-03T02:26:44.1396565Z status: exit code: 1
2020-03-03T02:26:44.1398248Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions/auxiliary"
2020-03-03T02:26:44.1399641Z ------------------------------------------
2020-03-03T02:26:44.1399812Z 
2020-03-03T02:26:44.1400140Z ------------------------------------------
2020-03-03T02:26:44.1400320Z stderr:
2020-03-03T02:26:44.1400320Z stderr:
2020-03-03T02:26:44.1400666Z ------------------------------------------
2020-03-03T02:26:44.1400922Z warning: denote infinite loops with `loop { ... }`
2020-03-03T02:26:44.1401359Z   --> /checkout/src/test/ui/lint/suggestions.rs:46:5
2020-03-03T02:26:44.1401694Z    |
2020-03-03T02:26:44.1401844Z LL |     while true {
2020-03-03T02:26:44.1402058Z    |     ^^^^^^^^^^ help: use `loop`
2020-03-03T02:26:44.1402257Z    |
2020-03-03T02:26:44.1402453Z    = note: `#[warn(while_true)]` on by default
2020-03-03T02:26:44.1402825Z warning: unnecessary parentheses around assigned value
2020-03-03T02:26:44.1403322Z   --> /checkout/src/test/ui/lint/suggestions.rs:49:31
2020-03-03T02:26:44.1403535Z    |
2020-03-03T02:26:44.1403535Z    |
2020-03-03T02:26:44.1403937Z LL |         let mut registry_no = (format!("NX-{}", 74205));
2020-03-03T02:26:44.1404743Z    |
2020-03-03T02:26:44.1404908Z note: the lint level is defined here
2020-03-03T02:26:44.1405350Z   --> /checkout/src/test/ui/lint/suggestions.rs:3:21
2020-03-03T02:26:44.1405548Z    |
2020-03-03T02:26:44.1405548Z    |
2020-03-03T02:26:44.1405986Z LL | #![warn(unused_mut, unused_parens)] // UI tests pass `-A unused`—see Issue #43896
2020-03-03T02:26:44.1406442Z 
2020-03-03T02:26:44.1406442Z 
2020-03-03T02:26:44.1407300Z warning: use of deprecated attribute `no_debug`: the `#[no_debug]` attribute was an experimental feature that has been deprecated due to lack of demand. See ***/issues/29721
2020-03-03T02:26:44.1408178Z    |
2020-03-03T02:26:44.1408403Z LL | #[no_debug] // should suggest removal of deprecated attribute
2020-03-03T02:26:44.1408683Z    | ^^^^^^^^^^^ help: remove this attribute
2020-03-03T02:26:44.1408866Z    |
2020-03-03T02:26:44.1408866Z    |
2020-03-03T02:26:44.1411796Z    = note: `#[warn(deprecated)]` on by default
2020-03-03T02:26:44.1411975Z 
2020-03-03T02:26:44.1412151Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1412740Z   --> /checkout/src/test/ui/lint/suggestions.rs:49:13
2020-03-03T02:26:44.1412960Z    |
2020-03-03T02:26:44.1413342Z LL |         let mut registry_no = (format!("NX-{}", 74205));
2020-03-03T02:26:44.1413645Z    |             ^^^^^^^^^^^^^^^ help: remove this `mut`
2020-03-03T02:26:44.1414034Z note: the lint level is defined here
2020-03-03T02:26:44.1414450Z   --> /checkout/src/test/ui/lint/suggestions.rs:3:9
2020-03-03T02:26:44.1414663Z    |
2020-03-03T02:26:44.1414663Z    |
2020-03-03T02:26:44.1415103Z LL | #![warn(unused_mut, unused_parens)] // UI tests pass `-A unused`—see Issue #43896
2020-03-03T02:26:44.1415498Z 
2020-03-03T02:26:44.1415688Z warning: variable does not need to be mutable
2020-03-03T02:26:44.1416261Z   --> /checkout/src/test/ui/lint/suggestions.rs:55:13
2020-03-03T02:26:44.1416477Z    |
---
2020-03-03T02:26:44.1424525Z    |              -----^^^^^^^^^^^^^^^^^^^^^^
2020-03-03T02:26:44.1424863Z    |              |
2020-03-03T02:26:44.1425143Z    |              help: try a static value: `pub static`
2020-03-03T02:26:44.1425448Z    |
2020-03-03T02:26:44.1425672Z    = note: `#[deny(no_mangle_const_items)]` on by default
2020-03-03T02:26:44.1426362Z warning: functions generic over types or consts must be mangled
2020-03-03T02:26:44.1427116Z   --> /checkout/src/test/ui/lint/suggestions.rs:12:1
2020-03-03T02:26:44.1427469Z    |
2020-03-03T02:26:44.1427626Z LL | #[no_mangle]
2020-03-03T02:26:44.1427626Z LL | #[no_mangle]
2020-03-03T02:26:44.1428077Z    | ------------ help: remove this attribute
2020-03-03T02:26:44.1428328Z LL | //~^ HELP remove this attribute
2020-03-03T02:26:44.1428679Z LL | pub fn defiant<T>(_t: T) {}
2020-03-03T02:26:44.1429266Z    |
2020-03-03T02:26:44.1429493Z    = note: `#[warn(no_mangle_generic_items)]` on by default
2020-03-03T02:26:44.1429758Z 
2020-03-03T02:26:44.1429758Z 
2020-03-03T02:26:44.1430007Z warning: the `warp_factor:` in this pattern is redundant
2020-03-03T02:26:44.1430955Z    |
2020-03-03T02:26:44.1430955Z    |
2020-03-03T02:26:44.1431199Z LL |             Equinox { warp_factor: warp_factor } => {}
2020-03-03T02:26:44.1431691Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^ help: use shorthand field pattern: `warp_factor`
2020-03-03T02:26:44.1432337Z    = note: `#[warn(non_shorthand_field_patterns)]` on by default
2020-03-03T02:26:44.1432611Z 
2020-03-03T02:26:44.1432834Z error: const items should never be `#[no_mangle]`
2020-03-03T02:26:44.1433416Z   --> /checkout/src/test/ui/lint/suggestions.rs:22:18
2020-03-03T02:26:44.1433416Z   --> /checkout/src/test/ui/lint/suggestions.rs:22:18
2020-03-03T02:26:44.1433740Z    |
2020-03-03T02:26:44.1433987Z LL |     #[no_mangle] pub const DAUNTLESS: bool = true;
2020-03-03T02:26:44.1434932Z    |                  |
2020-03-03T02:26:44.1435207Z    |                  help: try a static value: `pub static`
2020-03-03T02:26:44.1435406Z 
2020-03-03T02:26:44.1435722Z warning: functions generic over types or consts must be mangled
2020-03-03T02:26:44.1435722Z warning: functions generic over types or consts must be mangled
2020-03-03T02:26:44.1436321Z   --> /checkout/src/test/ui/lint/suggestions.rs:25:18
2020-03-03T02:26:44.1436643Z    |
2020-03-03T02:26:44.1436868Z LL |     #[no_mangle] pub fn val_jean<T>() {}
2020-03-03T02:26:44.1437435Z    |     ------------ ^^^^^^^^^^^^^^^^^^^^^^^
2020-03-03T02:26:44.1437813Z    |     help: remove this attribute
2020-03-03T02:26:44.1437960Z 
2020-03-03T02:26:44.1438144Z error: const items should never be `#[no_mangle]`
2020-03-03T02:26:44.1438589Z   --> /checkout/src/test/ui/lint/suggestions.rs:30:18
2020-03-03T02:26:44.1438589Z   --> /checkout/src/test/ui/lint/suggestions.rs:30:18
2020-03-03T02:26:44.1438942Z    |
2020-03-03T02:26:44.1439183Z LL |     #[no_mangle] pub(crate) const VETAR: bool = true;
2020-03-03T02:26:44.1440150Z    |                  |
2020-03-03T02:26:44.1440425Z    |                  help: try a static value: `pub static`
2020-03-03T02:26:44.1440641Z 
2020-03-03T02:26:44.1440939Z warning: functions generic over types or consts must be mangled
2020-03-03T02:26:44.1440939Z warning: functions generic over types or consts must be mangled
2020-03-03T02:26:44.1441538Z   --> /checkout/src/test/ui/lint/suggestions.rs:33:18
2020-03-03T02:26:44.1441887Z    |
2020-03-03T02:26:44.1442106Z LL |     #[no_mangle] pub(crate) fn crossfield<T>() {}
2020-03-03T02:26:44.1442799Z    |     ------------ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-03T02:26:44.1443386Z    |     help: remove this attribute
2020-03-03T02:26:44.1443620Z 
2020-03-03T02:26:44.1443920Z error: aborting due to 3 previous errors
2020-03-03T02:26:44.1444162Z 
---
2020-03-03T02:26:44.1445998Z 
2020-03-03T02:26:44.1446122Z 74    |
2020-03-03T02:26:44.1446396Z 75 help: consider changing the closure to take and ignore the expected argument
2020-03-03T02:26:44.1446634Z 76    |
2020-03-03T02:26:44.1447002Z - LL |     f(  move    |_| panic!());
2020-03-03T02:26:44.1447363Z -    |                 ^^^
2020-03-03T02:26:44.1447684Z + LL |     f(  |_| panic!());
2020-03-03T02:26:44.1448038Z 79 
2020-03-03T02:26:44.1448570Z 80 error[E0593]: closure is expected to take a single 2-tuple as argument, but it takes 2 distinct arguments
2020-03-03T02:26:44.1449255Z 81   --> $DIR/closure-arg-count.rs:18:53
2020-03-03T02:26:44.1449556Z 
2020-03-03T02:26:44.1449556Z 
2020-03-03T02:26:44.1449726Z 
2020-03-03T02:26:44.1449934Z The actual stderr differed from the expected stderr.
2020-03-03T02:26:44.1450741Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-count/closure-arg-count.stderr
2020-03-03T02:26:44.1451607Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:44.1452359Z To only update this specific test, also pass `--test-args mismatched_types/closure-arg-count.rs`
2020-03-03T02:26:44.1452961Z error: 1 errors occurred comparing output.
2020-03-03T02:26:44.1453281Z status: exit code: 1
2020-03-03T02:26:44.1453281Z status: exit code: 1
2020-03-03T02:26:44.1455341Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/closure-arg-count.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-count" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-count/auxiliary"
2020-03-03T02:26:44.1457019Z ------------------------------------------
2020-03-03T02:26:44.1457178Z 
2020-03-03T02:26:44.1457690Z ------------------------------------------
2020-03-03T02:26:44.1457997Z stderr:
2020-03-03T02:26:44.1457997Z stderr:
2020-03-03T02:26:44.1458393Z ------------------------------------------
2020-03-03T02:26:44.1458827Z error[E0593]: closure is expected to take 2 arguments, but it takes 0 arguments
2020-03-03T02:26:44.1459515Z   --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:5:15
2020-03-03T02:26:44.1459905Z    |
2020-03-03T02:26:44.1460100Z LL |     [1, 2, 3].sort_by(|| panic!());
2020-03-03T02:26:44.1460638Z    |               ^^^^^^^ -- takes 0 arguments
2020-03-03T02:26:44.1461239Z    |               expected closure that takes 2 arguments
2020-03-03T02:26:44.1461435Z    |
2020-03-03T02:26:44.1461779Z help: consider changing the closure to take and ignore the expected arguments
2020-03-03T02:26:44.1462102Z    |
2020-03-03T02:26:44.1462102Z    |
2020-03-03T02:26:44.1462325Z LL |     [1, 2, 3].sort_by(|_, _| panic!());
2020-03-03T02:26:44.1462793Z 
2020-03-03T02:26:44.1463053Z error[E0593]: closure is expected to take 2 arguments, but it takes 1 argument
2020-03-03T02:26:44.1463715Z   --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:7:15
2020-03-03T02:26:44.1464074Z    |
2020-03-03T02:26:44.1464074Z    |
2020-03-03T02:26:44.1464395Z LL |     [1, 2, 3].sort_by(|tuple| panic!());
2020-03-03T02:26:44.1465344Z    |               |
2020-03-03T02:26:44.1465801Z    |               expected closure that takes 2 arguments
2020-03-03T02:26:44.1466066Z 
2020-03-03T02:26:44.1466066Z 
2020-03-03T02:26:44.1466843Z error[E0593]: closure is expected to take 2 distinct arguments, but it takes a single 2-tuple as argument
2020-03-03T02:26:44.1467980Z    |
2020-03-03T02:26:44.1467980Z    |
2020-03-03T02:26:44.1468217Z LL |     [1, 2, 3].sort_by(|(tuple, tuple2)| panic!());
2020-03-03T02:26:44.1468849Z    |               ^^^^^^^ ----------------- takes a single 2-tuple as argument
2020-03-03T02:26:44.1469521Z    |               expected closure that takes 2 distinct arguments
2020-03-03T02:26:44.1469730Z    |
2020-03-03T02:26:44.1469969Z help: change the closure to take multiple arguments instead of a single tuple
2020-03-03T02:26:44.1470318Z    |
2020-03-03T02:26:44.1470318Z    |
2020-03-03T02:26:44.1470532Z LL |     [1, 2, 3].sort_by(|tuple, tuple2| panic!());
2020-03-03T02:26:44.1471043Z 
2020-03-03T02:26:44.1471043Z 
2020-03-03T02:26:44.1471585Z error[E0593]: closure is expected to take 2 distinct arguments, but it takes a single 2-tuple as argument
2020-03-03T02:26:44.1472677Z    |
2020-03-03T02:26:44.1472677Z    |
2020-03-03T02:26:44.1472925Z LL |     [1, 2, 3].sort_by(|(tuple, tuple2): (usize, _)| panic!());
2020-03-03T02:26:44.1473626Z    |               ^^^^^^^ ----------------------------- takes a single 2-tuple as argument
2020-03-03T02:26:44.1474294Z    |               expected closure that takes 2 distinct arguments
2020-03-03T02:26:44.1474520Z    |
2020-03-03T02:26:44.1474847Z help: change the closure to take multiple arguments instead of a single tuple
2020-03-03T02:26:44.1475189Z    |
2020-03-03T02:26:44.1475189Z    |
2020-03-03T02:26:44.1475407Z LL |     [1, 2, 3].sort_by(|tuple, tuple2| panic!());
2020-03-03T02:26:44.1475916Z 
2020-03-03T02:26:44.1476174Z error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
2020-03-03T02:26:44.1476840Z   --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:13:5
2020-03-03T02:26:44.1477209Z    |
2020-03-03T02:26:44.1477209Z    |
2020-03-03T02:26:44.1477406Z LL | fn f<F: Fn<usize>>(_: F) {}
2020-03-03T02:26:44.1477948Z    |    -    --------- required by this bound in `f`
2020-03-03T02:26:44.1478454Z LL |     f(|| panic!());
2020-03-03T02:26:44.1478924Z    |     ^ -- takes 0 arguments
2020-03-03T02:26:44.1479235Z    |     |
2020-03-03T02:26:44.1479442Z    |     expected closure that takes 1 argument
---
2020-03-03T02:26:44.1482021Z 
2020-03-03T02:26:44.1482309Z error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
2020-03-03T02:26:44.1483024Z   --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:15:5
2020-03-03T02:26:44.1483277Z    |
2020-03-03T02:26:44.1483467Z LL | fn f<F: Fn<usize>>(_: F) {}
2020-03-03T02:26:44.1483959Z    |    -    --------- required by this bound in `f`
2020-03-03T02:26:44.1484284Z ...
2020-03-03T02:26:44.1484735Z LL |     f(  move    || panic!());
2020-03-03T02:26:44.1485716Z    |     |
2020-03-03T02:26:44.1485950Z    |     expected closure that takes 1 argument
2020-03-03T02:26:44.1486130Z    |
2020-03-03T02:26:44.1486452Z help: consider changing the closure to take and ignore the expected argument
2020-03-03T02:26:44.1486452Z help: consider changing the closure to take and ignore the expected argument
2020-03-03T02:26:44.1486789Z    |
2020-03-03T02:26:44.1487080Z LL |     f(  |_| panic!());
2020-03-03T02:26:44.1487531Z 
2020-03-03T02:26:44.1488091Z error[E0593]: closure is expected to take a single 2-tuple as argument, but it takes 2 distinct arguments
2020-03-03T02:26:44.1488963Z   --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:18:53
2020-03-03T02:26:44.1489337Z    |
2020-03-03T02:26:44.1489337Z    |
2020-03-03T02:26:44.1489584Z LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(|i, x| i);
2020-03-03T02:26:44.1490283Z    |                                                     ^^^ ------ takes 2 distinct arguments
2020-03-03T02:26:44.1491449Z    |                                                     expected closure that takes a single 2-tuple as argument
2020-03-03T02:26:44.1491863Z    |
2020-03-03T02:26:44.1492105Z help: change the closure to accept a tuple instead of individual arguments
2020-03-03T02:26:44.1492328Z    |
2020-03-03T02:26:44.1492328Z    |
2020-03-03T02:26:44.1492673Z LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(|(i, x)| i);
2020-03-03T02:26:44.1493375Z 
2020-03-03T02:26:44.1493939Z error[E0593]: closure is expected to take a single 2-tuple as argument, but it takes 2 distinct arguments
2020-03-03T02:26:44.1494659Z   --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:20:53
2020-03-03T02:26:44.1495025Z    |
2020-03-03T02:26:44.1495025Z    |
2020-03-03T02:26:44.1495310Z LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(|i: usize, x| i);
2020-03-03T02:26:44.1496046Z    |                                                     ^^^ ------------- takes 2 distinct arguments
2020-03-03T02:26:44.1497229Z    |                                                     expected closure that takes a single 2-tuple as argument
2020-03-03T02:26:44.1497643Z    |
2020-03-03T02:26:44.1497892Z help: change the closure to accept a tuple instead of individual arguments
2020-03-03T02:26:44.1498201Z    |
2020-03-03T02:26:44.1498201Z    |
2020-03-03T02:26:44.1498463Z LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(|(i, x)| i);
2020-03-03T02:26:44.1499156Z 
2020-03-03T02:26:44.1499718Z error[E0593]: closure is expected to take a single 2-tuple as argument, but it takes 3 distinct arguments
2020-03-03T02:26:44.1500444Z   --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:22:53
2020-03-03T02:26:44.1500825Z    |
2020-03-03T02:26:44.1500825Z    |
2020-03-03T02:26:44.1501076Z LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(|i, x, y| i);
2020-03-03T02:26:44.1501900Z    |                                                     ^^^ --------- takes 3 distinct arguments
2020-03-03T02:26:44.1503166Z    |                                                     expected closure that takes a single 2-tuple as argument
2020-03-03T02:26:44.1503561Z 
2020-03-03T02:26:44.1504136Z error[E0593]: function is expected to take a single 2-tuple as argument, but it takes 0 arguments
2020-03-03T02:26:44.1504984Z   --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:24:57
2020-03-03T02:26:44.1504984Z   --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:24:57
2020-03-03T02:26:44.1505354Z    |
2020-03-03T02:26:44.1505588Z LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(foo);
2020-03-03T02:26:44.1506454Z    |                                                         ^^^ expected function that takes a single 2-tuple as argument
2020-03-03T02:26:45.0443816Z LL | fn foo() {}
2020-03-03T02:26:45.0448366Z    | -------- takes 0 arguments
2020-03-03T02:26:45.0448568Z 
2020-03-03T02:26:45.0449878Z error[E0593]: closure is expected to take a single 2-tuple as argument, but it takes 3 distinct arguments
2020-03-03T02:26:45.0449878Z error[E0593]: closure is expected to take a single 2-tuple as argument, but it takes 3 distinct arguments
2020-03-03T02:26:45.0450924Z   --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:27:57
2020-03-03T02:26:45.0451641Z    |
2020-03-03T02:26:45.0708532Z LL |     let bar = |i, x, y| i;
2020-03-03T02:26:45.0711743Z    |               --------- takes 3 distinct arguments
2020-03-03T02:26:45.0712168Z LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(bar);
2020-03-03T02:26:45.0713286Z    |                                                         ^^^ expected closure that takes a single 2-tuple as argument
2020-03-03T02:26:45.0714564Z error[E0593]: function is expected to take a single 2-tuple as argument, but it takes 2 distinct arguments
2020-03-03T02:26:45.0715477Z   --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:29:57
2020-03-03T02:26:45.0715807Z    |
2020-03-03T02:26:45.0715807Z    |
2020-03-03T02:26:45.0716082Z LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(qux);
2020-03-03T02:26:45.0716880Z    |                                                         ^^^ expected function that takes a single 2-tuple as argument
2020-03-03T02:26:45.0717279Z ...
2020-03-03T02:26:45.0717512Z LL | fn qux(x: usize, y: usize) {}
2020-03-03T02:26:45.0718242Z    | -------------------------- takes 2 distinct arguments
2020-03-03T02:26:45.0718804Z error[E0593]: function is expected to take 1 argument, but it takes 2 arguments
2020-03-03T02:26:45.0721470Z   --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:32:45
2020-03-03T02:26:45.0721801Z    |
2020-03-03T02:26:45.0721801Z    |
2020-03-03T02:26:45.0722103Z LL |     let _it = vec![1, 2, 3].into_iter().map(usize::checked_add);
2020-03-03T02:26:45.0722905Z 
2020-03-03T02:26:45.0723222Z error[E0593]: function is expected to take 0 arguments, but it takes 1 argument
2020-03-03T02:26:45.0723999Z   --> /checkout/src/test/ui/mismatched_types/closure-arg-count.rs:35:10
2020-03-03T02:26:45.0724298Z    |
2020-03-03T02:26:45.0724298Z    |
2020-03-03T02:26:45.0724494Z LL |     call(Foo);
2020-03-03T02:26:45.0724782Z    |          ^^^ expected function that takes 0 arguments
2020-03-03T02:26:45.0725042Z ...
2020-03-03T02:26:45.0725552Z LL | fn call<F, R>(_: F) where F: FnOnce() -> R {}
2020-03-03T02:26:45.0726867Z LL | struct Foo(u8);
2020-03-03T02:26:45.0727589Z    | --------------- takes 1 argument
2020-03-03T02:26:45.0727847Z 
2020-03-03T02:26:45.0728060Z error: aborting due to 14 previous errors
---
2020-03-03T02:26:45.0735678Z 10   --> $DIR/capture-mut-ref.rs:4:9
2020-03-03T02:26:45.0735902Z 
2020-03-03T02:26:45.0736009Z 
2020-03-03T02:26:45.0736240Z The actual stderr differed from the expected stderr.
2020-03-03T02:26:45.0737641Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/capture-mut-ref/capture-mut-ref.stderr
2020-03-03T02:26:45.0738654Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:45.0739569Z To only update this specific test, also pass `--test-args nll/capture-mut-ref.rs`
2020-03-03T02:26:45.0740301Z error: 1 errors occurred comparing output.
2020-03-03T02:26:45.0746686Z status: exit code: 1
2020-03-03T02:26:45.0746686Z status: exit code: 1
2020-03-03T02:26:45.0752746Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/capture-mut-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/capture-mut-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/capture-mut-ref/auxiliary"
2020-03-03T02:26:45.0758644Z ------------------------------------------
2020-03-03T02:26:45.0759446Z 
2020-03-03T02:26:45.0760055Z ------------------------------------------
2020-03-03T02:26:45.0760440Z stderr:
---
2020-03-03T02:26:45.0781078Z 
2020-03-03T02:26:45.0781188Z 
2020-03-03T02:26:45.0781424Z The actual stderr differed from the expected stderr.
2020-03-03T02:26:45.0782165Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-61424/issue-61424.stderr
2020-03-03T02:26:45.0782901Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:45.0783570Z To only update this specific test, also pass `--test-args nll/issue-61424.rs`
2020-03-03T02:26:45.0784080Z error: 1 errors occurred comparing output.
2020-03-03T02:26:45.0784542Z status: exit code: 1
2020-03-03T02:26:45.0784542Z status: exit code: 1
2020-03-03T02:26:45.0787075Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-61424.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-61424" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-61424/auxiliary"
2020-03-03T02:26:45.0789189Z ------------------------------------------
2020-03-03T02:26:45.0789391Z 
2020-03-03T02:26:45.0790000Z ------------------------------------------
2020-03-03T02:26:45.0790254Z stderr:
2020-03-03T02:26:45.0790254Z stderr:
2020-03-03T02:26:45.0790761Z ------------------------------------------
2020-03-03T02:26:45.0791071Z error: variable does not need to be mutable
2020-03-03T02:26:45.0791771Z   --> /checkout/src/test/ui/nll/issue-61424.rs:4:9
2020-03-03T02:26:45.0792047Z    |
2020-03-03T02:26:45.0792359Z LL |     let mut x; //~ ERROR: variable does not need to be mutable
2020-03-03T02:26:45.0792754Z    |         ^^^^^ help: remove this `mut`
2020-03-03T02:26:45.0793227Z note: the lint level is defined here
2020-03-03T02:26:45.0793766Z   --> /checkout/src/test/ui/nll/issue-61424.rs:1:9
2020-03-03T02:26:45.0794037Z    |
2020-03-03T02:26:45.0794227Z LL | #![deny(unused_mut)]
---
2020-03-03T02:26:45.0796633Z diff of stderr:
2020-03-03T02:26:45.0796800Z 
2020-03-03T02:26:45.0797249Z 2   --> $DIR/unused-mut-issue-50343.rs:4:33
2020-03-03T02:26:45.0797498Z 3    |
2020-03-03T02:26:45.0797789Z 4 LL |     vec![(42, 22)].iter().map(|(mut x, _y)| ()).count();
2020-03-03T02:26:45.0799115Z -    |                                 |
2020-03-03T02:26:45.0799702Z -    |                                 help: remove this `mut`
2020-03-03T02:26:45.0800346Z +    |                                 ^^^^^ help: remove this `mut`
2020-03-03T02:26:45.0800655Z 8    |
2020-03-03T02:26:45.0800655Z 8    |
2020-03-03T02:26:45.0800877Z 9 note: the lint level is defined here
2020-03-03T02:26:45.0801472Z 10   --> $DIR/unused-mut-issue-50343.rs:1:9
2020-03-03T02:26:45.0801834Z 
2020-03-03T02:26:45.0801948Z 
2020-03-03T02:26:45.0802195Z The actual stderr differed from the expected stderr.
2020-03-03T02:26:45.0803202Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/unused-mut-issue-50343/unused-mut-issue-50343.stderr
2020-03-03T02:26:45.0803976Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:45.0804698Z To only update this specific test, also pass `--test-args nll/unused-mut-issue-50343.rs`
2020-03-03T02:26:45.0805269Z error: 1 errors occurred comparing output.
2020-03-03T02:26:45.0805538Z status: exit code: 1
2020-03-03T02:26:45.0805538Z status: exit code: 1
2020-03-03T02:26:45.0807991Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/unused-mut-issue-50343.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/unused-mut-issue-50343" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/unused-mut-issue-50343/auxiliary"
2020-03-03T02:26:45.0810054Z ------------------------------------------
2020-03-03T02:26:45.0810400Z 
2020-03-03T02:26:45.0810960Z ------------------------------------------
2020-03-03T02:26:45.0811214Z stderr:
2020-03-03T02:26:45.0811214Z stderr:
2020-03-03T02:26:45.0811669Z ------------------------------------------
2020-03-03T02:26:45.0811976Z error: variable does not need to be mutable
2020-03-03T02:26:45.0812590Z   --> /checkout/src/test/ui/nll/unused-mut-issue-50343.rs:4:33
2020-03-03T02:26:45.0812876Z    |
2020-03-03T02:26:45.0813142Z LL |     vec![(42, 22)].iter().map(|(mut x, _y)| ()).count();
2020-03-03T02:26:45.0813570Z    |                                 ^^^^^ help: remove this `mut`
2020-03-03T02:26:45.0814076Z note: the lint level is defined here
2020-03-03T02:26:45.0814988Z   --> /checkout/src/test/ui/nll/unused-mut-issue-50343.rs:1:9
2020-03-03T02:26:45.0815279Z    |
2020-03-03T02:26:45.0815654Z LL | #![deny(unused_mut)]
---
2020-03-03T02:26:45.0828301Z 
2020-03-03T02:26:45.0828550Z 8 // Test that we DO warn when lifetime name is used only
2020-03-03T02:26:45.0828935Z 9 // once in a fn argument, even with in band lifetimes.
2020-03-03T02:26:45.0829183Z 10 
2020-03-03T02:26:45.0829586Z - fn a(x: &u32, y: &u32) {
2020-03-03T02:26:45.0829838Z + fn a(x: &, y: &) {
2020-03-03T02:26:45.0830314Z 12     //~^ ERROR `'a` only used once
2020-03-03T02:26:45.0830796Z 13     //~| ERROR `'b` only used once
2020-03-03T02:26:45.0831301Z 14     //~| HELP elide the single-use lifetime
2020-03-03T02:26:45.0831638Z 
2020-03-03T02:26:45.0831870Z The actual fixed differed from the expected fixed.
2020-03-03T02:26:45.0831870Z The actual fixed differed from the expected fixed.
2020-03-03T02:26:45.0832747Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band/one-use-in-fn-argument-in-band.fixed
2020-03-03T02:26:45.0833584Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:45.0834342Z To only update this specific test, also pass `--test-args single-use-lifetime/one-use-in-fn-argument-in-band.rs`
2020-03-03T02:26:45.0834925Z error: 2 errors occurred comparing output.
2020-03-03T02:26:45.0835194Z status: exit code: 1
2020-03-03T02:26:45.0835194Z status: exit code: 1
2020-03-03T02:26:45.0837807Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band/auxiliary"
2020-03-03T02:26:45.0839864Z ------------------------------------------
2020-03-03T02:26:45.0840169Z 
2020-03-03T02:26:45.0840624Z ------------------------------------------
2020-03-03T02:26:45.0840858Z stderr:
---
2020-03-03T02:26:45.0853038Z 
2020-03-03T02:26:45.0853258Z 13    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T02:26:45.0853744Z 14 help: elide the single-use lifetime
2020-03-03T02:26:45.0853977Z 15    |
2020-03-03T02:26:45.0854370Z - LL | fn a(x: &u32) {
2020-03-03T02:26:45.0854606Z + LL | fn a(x: &) {
2020-03-03T02:26:45.0855188Z 18 
2020-03-03T02:26:45.0855637Z 19 error: lifetime parameter `'m` only used once
2020-03-03T02:26:45.0856633Z 
2020-03-03T02:26:45.0856749Z 
2020-03-03T02:26:45.0856749Z 
2020-03-03T02:26:45.0857012Z The actual stderr differed from the expected stderr.
2020-03-03T02:26:45.0857954Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument/one-use-in-fn-argument.stderr
2020-03-03T02:26:45.0858748Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:45.0859511Z To only update this specific test, also pass `--test-args single-use-lifetime/one-use-in-fn-argument.rs`
2020-03-03T02:26:45.0860057Z error: 1 errors occurred comparing output.
2020-03-03T02:26:45.0860342Z status: exit code: 1
2020-03-03T02:26:45.0860342Z status: exit code: 1
2020-03-03T02:26:45.0862849Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/single-use-lifetime/one-use-in-fn-argument.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument/auxiliary"
2020-03-03T02:26:45.0864852Z ------------------------------------------
2020-03-03T02:26:45.0865150Z 
2020-03-03T02:26:45.0865627Z ------------------------------------------
2020-03-03T02:26:45.0865862Z stderr:
2020-03-03T02:26:45.0865862Z stderr:
2020-03-03T02:26:45.0895898Z ------------------------------------------
2020-03-03T02:26:45.0896600Z error: lifetime parameter `'a` only used once
2020-03-03T02:26:45.0897260Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-fn-argument.rs:8:6
2020-03-03T02:26:45.0897576Z    |
2020-03-03T02:26:45.0898093Z LL | fn a<'a>(x: &'a u32) { //~ ERROR `'a` only used once
2020-03-03T02:26:45.0898641Z    |      ^^      -- ...is used only here
2020-03-03T02:26:45.0899106Z    |      this lifetime...
2020-03-03T02:26:45.0899296Z    |
2020-03-03T02:26:45.0899509Z note: the lint level is defined here
2020-03-03T02:26:45.0900140Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-fn-argument.rs:1:9
2020-03-03T02:26:45.0900140Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-fn-argument.rs:1:9
2020-03-03T02:26:45.0900476Z    |
2020-03-03T02:26:45.0900684Z LL | #![deny(single_use_lifetimes)]
2020-03-03T02:26:45.0900955Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T02:26:45.0901449Z help: elide the single-use lifetime
2020-03-03T02:26:45.0901663Z    |
2020-03-03T02:26:45.0902111Z LL | fn a(x: &) { //~ ERROR `'a` only used once
2020-03-03T02:26:45.0902720Z 
2020-03-03T02:26:45.0903150Z error: lifetime parameter `'m` only used once
2020-03-03T02:26:45.0903791Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-fn-argument.rs:15:11
2020-03-03T02:26:45.0904127Z    |
2020-03-03T02:26:45.0904127Z    |
2020-03-03T02:26:45.0904814Z LL | fn center<'m>(_: Single<'m>) {} //~ ERROR `'m` only used once
2020-03-03T02:26:45.0905434Z    |           ^^            -- ...is used only here
2020-03-03T02:26:45.0905953Z    |           this lifetime...
2020-03-03T02:26:45.0906160Z    |
2020-03-03T02:26:45.0906838Z help: elide the single-use lifetime
2020-03-03T02:26:45.0907058Z    |
2020-03-03T02:26:45.0907058Z    |
2020-03-03T02:26:45.0907556Z LL | fn center(_: Single<'_>) {} //~ ERROR `'m` only used once
2020-03-03T02:26:45.0908266Z 
2020-03-03T02:26:45.0908700Z error: lifetime parameter `'y` only used once
2020-03-03T02:26:45.0909343Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-fn-argument.rs:17:13
2020-03-03T02:26:45.0909681Z    |
2020-03-03T02:26:45.0909681Z    |
2020-03-03T02:26:45.0910278Z LL | fn left<'x, 'y>(foo: Double<'x, 'y>) -> &'x u32 { foo.f } //~ ERROR `'y` only used once
2020-03-03T02:26:45.0910954Z    |             ^^ this lifetime... -- ...is used only here
2020-03-03T02:26:45.0911638Z help: elide the single-use lifetime
2020-03-03T02:26:45.0911854Z    |
2020-03-03T02:26:45.0911854Z    |
2020-03-03T02:26:45.0912449Z LL | fn left<'x>(foo: Double<'x, '_>) -> &'x u32 { foo.f } //~ ERROR `'y` only used once
2020-03-03T02:26:45.0913225Z 
2020-03-03T02:26:45.0913225Z 
2020-03-03T02:26:45.0913677Z error: lifetime parameter `'x` only used once
2020-03-03T02:26:45.0914642Z    |
2020-03-03T02:26:45.0914642Z    |
2020-03-03T02:26:45.0915259Z LL | fn right<'x, 'y>(foo: Double<'x, 'y>) -> &'y u32 { foo.f } //~ ERROR `'x` only used once
2020-03-03T02:26:45.0915927Z    |          ^^ this lifetime... -- ...is used only here
2020-03-03T02:26:45.0916587Z help: elide the single-use lifetime
2020-03-03T02:26:45.0916822Z    |
2020-03-03T02:26:45.0916822Z    |
2020-03-03T02:26:45.0917405Z LL | fn right<'y>(foo: Double<'_, 'y>) -> &'y u32 { foo.f } //~ ERROR `'x` only used once
2020-03-03T02:26:45.0918478Z 
2020-03-03T02:26:45.0918694Z error: aborting due to 4 previous errors
2020-03-03T02:26:45.0918887Z 
2020-03-03T02:26:45.0918996Z 
---
2020-03-03T02:26:45.0921139Z 
2020-03-03T02:26:45.0921342Z 13    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T02:26:45.0921867Z 14 help: elide the single-use lifetime
2020-03-03T02:26:45.0922120Z 15    |
2020-03-03T02:26:45.0922589Z - LL |     fn inherent_a(&self, data: &u32) {
2020-03-03T02:26:45.0922941Z + LL |     fn inherent_a(&self, data: &) {
2020-03-03T02:26:45.0923698Z 18 
2020-03-03T02:26:45.0924145Z 19 error: lifetime parameter `'f` only used once
2020-03-03T02:26:45.0924379Z 
2020-03-03T02:26:45.0924489Z 
2020-03-03T02:26:45.0924489Z 
2020-03-03T02:26:45.0924725Z The actual stderr differed from the expected stderr.
2020-03-03T02:26:45.0925923Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-inherent-method-argument/one-use-in-inherent-method-argument.stderr
2020-03-03T02:26:45.0926801Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:45.0927585Z To only update this specific test, also pass `--test-args single-use-lifetime/one-use-in-inherent-method-argument.rs`
2020-03-03T02:26:45.0928170Z error: 1 errors occurred comparing output.
2020-03-03T02:26:45.0928439Z status: exit code: 1
2020-03-03T02:26:45.0928439Z status: exit code: 1
2020-03-03T02:26:45.0931185Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/single-use-lifetime/one-use-in-inherent-method-argument.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-inherent-method-argument" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-inherent-method-argument/auxiliary"
2020-03-03T02:26:45.0934961Z ------------------------------------------
2020-03-03T02:26:45.0935170Z 
2020-03-03T02:26:45.0935870Z ------------------------------------------
2020-03-03T02:26:45.0936288Z stderr:
2020-03-03T02:26:45.0936288Z stderr:
2020-03-03T02:26:45.0936805Z ------------------------------------------
2020-03-03T02:26:45.0937568Z error: lifetime parameter `'a` only used once
2020-03-03T02:26:45.0938712Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-inherent-method-argument.rs:12:19
2020-03-03T02:26:45.0939112Z    |
2020-03-03T02:26:45.0956111Z LL |     fn inherent_a<'a>(&self, data: &'a u32) { //~ ERROR `'a` only used once
2020-03-03T02:26:45.0956802Z    |                   ^^                -- ...is used only here
2020-03-03T02:26:45.0957421Z    |                   this lifetime...
2020-03-03T02:26:45.0957638Z    |
2020-03-03T02:26:45.0957877Z note: the lint level is defined here
2020-03-03T02:26:45.0958555Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-inherent-method-argument.rs:1:9
2020-03-03T02:26:45.0958555Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-inherent-method-argument.rs:1:9
2020-03-03T02:26:45.0958907Z    |
2020-03-03T02:26:45.0959138Z LL | #![deny(single_use_lifetimes)]
2020-03-03T02:26:45.0959405Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T02:26:45.0959876Z help: elide the single-use lifetime
2020-03-03T02:26:45.0960112Z    |
2020-03-03T02:26:45.0960650Z LL |     fn inherent_a(&self, data: &) { //~ ERROR `'a` only used once
2020-03-03T02:26:45.0961417Z 
2020-03-03T02:26:45.0961872Z error: lifetime parameter `'f` only used once
2020-03-03T02:26:45.0962897Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-inherent-method-argument.rs:11:6
2020-03-03T02:26:45.0963244Z    |
2020-03-03T02:26:45.0963244Z    |
2020-03-03T02:26:45.0963916Z LL | impl<'f> Foo<'f> { //~ ERROR `'f` only used once
2020-03-03T02:26:45.0964511Z    |      ^^      -- ...is used only here
2020-03-03T02:26:45.0964981Z    |      this lifetime...
2020-03-03T02:26:45.0965248Z 
2020-03-03T02:26:45.0965463Z error: aborting due to 2 previous errors
2020-03-03T02:26:45.0965658Z 
---
2020-03-03T02:26:45.0967776Z 
2020-03-03T02:26:45.0967989Z 13    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T02:26:45.0968565Z 14 help: elide the single-use lifetime
2020-03-03T02:26:45.0968798Z 15    |
2020-03-03T02:26:45.0969295Z - LL |     fn next(&mut self) -> Option<Self::Item> {
2020-03-03T02:26:45.0969904Z + LL |     fn next(&) -> Option<Self::Item> {
2020-03-03T02:26:45.0970579Z 18 
2020-03-03T02:26:45.0970822Z 19 error: aborting due to previous error
2020-03-03T02:26:45.0971020Z 
2020-03-03T02:26:45.0971131Z 
2020-03-03T02:26:45.0971131Z 
2020-03-03T02:26:45.0971373Z The actual stderr differed from the expected stderr.
2020-03-03T02:26:45.0972310Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-trait-method-argument/one-use-in-trait-method-argument.stderr
2020-03-03T02:26:45.0973381Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:45.0974150Z To only update this specific test, also pass `--test-args single-use-lifetime/one-use-in-trait-method-argument.rs`
2020-03-03T02:26:45.0974734Z error: 1 errors occurred comparing output.
2020-03-03T02:26:45.0975004Z status: exit code: 1
2020-03-03T02:26:45.0975004Z status: exit code: 1
2020-03-03T02:26:45.0977506Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/single-use-lifetime/one-use-in-trait-method-argument.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-trait-method-argument" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-trait-method-argument/auxiliary"
2020-03-03T02:26:45.0979766Z ------------------------------------------
2020-03-03T02:26:45.0979962Z 
2020-03-03T02:26:45.0980389Z ------------------------------------------
2020-03-03T02:26:45.0980641Z stderr:
2020-03-03T02:26:45.0980641Z stderr:
2020-03-03T02:26:45.0981069Z ------------------------------------------
2020-03-03T02:26:45.0981584Z error: lifetime parameter `'g` only used once
2020-03-03T02:26:45.0982296Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-trait-method-argument.rs:15:13
2020-03-03T02:26:45.0982642Z    |
2020-03-03T02:26:45.0983246Z LL |     fn next<'g>(&'g mut self) -> Option<Self::Item> { //~ ERROR `'g` only used once
2020-03-03T02:26:45.0984160Z    |             ^^   -- ...is used only here
2020-03-03T02:26:45.0984656Z    |             this lifetime...
2020-03-03T02:26:45.0984879Z    |
2020-03-03T02:26:45.0985091Z note: the lint level is defined here
2020-03-03T02:26:45.0985749Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-trait-method-argument.rs:4:9
2020-03-03T02:26:45.0985749Z   --> /checkout/src/test/ui/single-use-lifetime/one-use-in-trait-method-argument.rs:4:9
2020-03-03T02:26:45.0986104Z    |
2020-03-03T02:26:45.0987246Z LL | #![deny(single_use_lifetimes)]
2020-03-03T02:26:45.0987520Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T02:26:45.0988107Z help: elide the single-use lifetime
2020-03-03T02:26:45.0988491Z    |
2020-03-03T02:26:45.0989081Z LL |     fn next(&) -> Option<Self::Item> { //~ ERROR `'g` only used once
2020-03-03T02:26:45.0989932Z 
2020-03-03T02:26:45.0990153Z error: aborting due to previous error
2020-03-03T02:26:45.0990341Z 
2020-03-03T02:26:45.0990451Z 
---
2020-03-03T02:26:45.0995923Z 10   --> $DIR/unused-mut-warning-captured-var.rs:1:11
2020-03-03T02:26:45.0996165Z 
2020-03-03T02:26:45.0996275Z 
2020-03-03T02:26:45.0996530Z The actual stderr differed from the expected stderr.
2020-03-03T02:26:45.0997400Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused/unused-mut-warning-captured-var/unused-mut-warning-captured-var.stderr
2020-03-03T02:26:45.0998468Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T02:26:45.0999234Z To only update this specific test, also pass `--test-args unused/unused-mut-warning-captured-var.rs`
2020-03-03T02:26:45.0999767Z error: 1 errors occurred comparing output.
2020-03-03T02:26:45.1000056Z status: exit code: 1
2020-03-03T02:26:45.1000056Z status: exit code: 1
2020-03-03T02:26:45.1002402Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused/unused-mut-warning-captured-var.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused/unused-mut-warning-captured-var" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused/unused-mut-warning-captured-var/auxiliary"
2020-03-03T02:26:45.1004343Z ------------------------------------------
2020-03-03T02:26:45.1004543Z 
2020-03-03T02:26:45.1004986Z ------------------------------------------
2020-03-03T02:26:45.1005217Z stderr:
---
2020-03-03T02:26:45.1007935Z    |
2020-03-03T02:26:45.1008168Z note: the lint level is defined here
2020-03-03T02:26:45.1008787Z   --> /checkout/src/test/ui/unused/unused-mut-warning-captured-var.rs:1:11
2020-03-03T02:26:45.1009103Z    |
2020-03-03T02:26:45.1009316Z LL | #![forbid(unused_mut)]
2020-03-03T02:26:45.1009715Z 
2020-03-03T02:26:45.1009922Z error: aborting due to previous error
2020-03-03T02:26:45.1010130Z 
2020-03-03T02:26:45.1010239Z 
---
2020-03-03T02:26:45.1116489Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-03T02:26:45.1116970Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-03T02:26:45.1117243Z 
2020-03-03T02:26:45.1117355Z 
2020-03-03T02:26:45.1121592Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-03T02:26:45.1124636Z 
2020-03-03T02:26:45.1124774Z 
2020-03-03T02:26:45.1125042Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-03T02:26:45.1125415Z Build completed unsuccessfully in 1:07:15
2020-03-03T02:26:45.1125415Z Build completed unsuccessfully in 1:07:15
2020-03-03T02:26:45.1125721Z == clock drift check ==
2020-03-03T02:26:45.1126003Z   local time: Tue Mar  3 02:26:44 UTC 2020
2020-03-03T02:26:45.1126353Z   network time: Tue, 03 Mar 2020 02:26:44 GMT
2020-03-03T02:26:45.1126670Z == end clock drift check ==
2020-03-03T02:26:45.1126895Z 
2020-03-03T02:26:45.1165589Z ##[error]Bash exited with code '1'.
2020-03-03T02:26:45.1197969Z ##[section]Finishing: Run build
2020-03-03T02:26:45.1259269Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69653/merge to s
2020-03-03T02:26:45.1265223Z Task         : Get sources
2020-03-03T02:26:45.1265641Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T02:26:45.1266002Z Version      : 1.0.0
2020-03-03T02:26:45.1266924Z Author       : Microsoft
2020-03-03T02:26:45.1266924Z Author       : Microsoft
2020-03-03T02:26:45.1267367Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-03T02:26:45.1267830Z ==============================================================================
2020-03-03T02:26:45.4809495Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-03T02:26:45.4855023Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69653/merge to s
2020-03-03T02:26:45.4937032Z Cleaning up task key
2020-03-03T02:26:45.4938207Z Start cleaning up orphan processes.
2020-03-03T02:26:45.5107389Z Terminate orphan process: pid (4076) (python)
2020-03-03T02:26:45.5266578Z ##[section]Finishing: Finalize Job
