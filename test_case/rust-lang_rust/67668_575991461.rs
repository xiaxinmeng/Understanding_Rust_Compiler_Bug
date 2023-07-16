plain
2020-01-19T09:36:47.8384865Z ========================== Starting Command Output ===========================
2020-01-19T09:36:47.8386806Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1d08886e-e857-4c70-babb-15631eded50c.sh
2020-01-19T09:36:47.8386865Z 
2020-01-19T09:36:47.8390681Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-19T09:36:47.8397544Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67668/merge to s
2020-01-19T09:36:47.8399939Z Task         : Get sources
2020-01-19T09:36:47.8399976Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-19T09:36:47.8400013Z Version      : 1.0.0
2020-01-19T09:36:47.8400048Z Author       : Microsoft
---
2020-01-19T09:36:48.8886564Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-19T09:36:48.8897051Z ##[command]git config gc.auto 0
2020-01-19T09:36:48.8899686Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-19T09:36:48.8901560Z ##[command]git config --get-all http.proxy
2020-01-19T09:36:48.8908329Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67668/merge:refs/remotes/pull/67668/merge
---
2020-01-19T10:32:31.1263292Z .................................................................................................... 1700/9550
2020-01-19T10:32:37.3294077Z .................................................................................................... 1800/9550
2020-01-19T10:32:49.2273643Z ....................i............................................................................... 1900/9550
2020-01-19T10:32:56.3648765Z .................................................................................................... 2000/9550
2020-01-19T10:33:12.0768579Z ..........iiiii..................................................................................... 2100/9550
2020-01-19T10:33:21.7616174Z .................................................................................................... 2300/9550
2020-01-19T10:33:24.2573447Z .................................................................................................... 2400/9550
2020-01-19T10:33:29.7806843Z .................................................................................................... 2500/9550
2020-01-19T10:33:50.4112333Z .................................................................................................... 2600/9550
---
2020-01-19T10:36:33.7587310Z ......................................................i...............i............................. 4900/9550
2020-01-19T10:36:42.1841521Z .................................................................................................... 5000/9550
2020-01-19T10:36:50.3028985Z .................................................................................................i.. 5100/9550
2020-01-19T10:36:55.5844980Z .................................................................................................... 5200/9550
2020-01-19T10:37:06.4337374Z .....................................................................ii.ii...........i.............. 5300/9550
2020-01-19T10:37:15.5862761Z ......i............................................................................................. 5500/9550
2020-01-19T10:37:25.6062102Z .................................................................................................... 5600/9550
2020-01-19T10:37:32.2339183Z .......................................................i............................................ 5700/9550
2020-01-19T10:37:39.2589379Z .................................................................................................... 5800/9550
2020-01-19T10:37:39.2589379Z .................................................................................................... 5800/9550
2020-01-19T10:37:49.3678525Z .................................................................................................... 5900/9550
2020-01-19T10:37:56.4505298Z .............................................ii...i..ii...........i................................. 6000/9550
2020-01-19T10:38:19.3861157Z .................................................................................................... 6200/9550
2020-01-19T10:38:27.9697500Z .................................................................................................... 6300/9550
2020-01-19T10:38:27.9697500Z .................................................................................................... 6300/9550
2020-01-19T10:38:37.9508996Z .........................................................................i..ii...................... 6400/9550
2020-01-19T10:39:02.1252318Z ............F..F.................................................................................... 6600/9550
2020-01-19T10:39:11.7901969Z ............................................................i....................................... 6700/9550
2020-01-19T10:39:13.9686267Z .................................................................................................... 6800/9550
2020-01-19T10:39:16.8154656Z ...........................................................i........................................ 6900/9550
---
2020-01-19T10:40:59.5182842Z .................................................................................................... 7600/9550
2020-01-19T10:41:05.0188682Z .................................................................................................... 7700/9550
2020-01-19T10:41:11.8679035Z .................................................................................................... 7800/9550
2020-01-19T10:41:22.8677732Z .................................................................................................... 7900/9550
2020-01-19T10:41:29.3537350Z ..........iiiiiii................................................................................... 8000/9550
2020-01-19T10:41:44.0668363Z .................................................................................................... 8200/9550
2020-01-19T10:41:55.6456980Z .................................................................................................... 8300/9550
2020-01-19T10:42:09.1017310Z .................................................................................................... 8400/9550
2020-01-19T10:42:15.7281841Z .................................................................................................... 8500/9550
---
2020-01-19T10:44:14.0737139Z 
2020-01-19T10:44:14.0737895Z ---- [ui] ui/or-patterns/exhaustiveness-non-exhaustive.rs stdout ----
2020-01-19T10:44:14.0738256Z diff of stderr:
2020-01-19T10:44:14.0738480Z 
2020-01-19T10:44:14.0739101Z 1 error[E0004]: non-exhaustive patterns: `(2u8..=std::u8::MAX, _)` not covered
2020-01-19T10:44:14.0739639Z -   --> $DIR/exhaustiveness-non-exhaustive.rs:7:11
2020-01-19T10:44:14.0740154Z +   --> $DIR/exhaustiveness-non-exhaustive.rs:6:11
2020-01-19T10:44:14.0740677Z 4 LL |     match (0u8, 0u8) {
2020-01-19T10:44:14.0740677Z 4 LL |     match (0u8, 0u8) {
2020-01-19T10:44:14.0740894Z 5    |           ^^^^^^^^^^ pattern `(2u8..=std::u8::MAX, _)` not covered
2020-01-19T10:44:14.0741412Z 7    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2020-01-19T10:44:14.0741612Z 8 
2020-01-19T10:44:14.0741612Z 8 
2020-01-19T10:44:14.0742089Z 9 error[E0004]: non-exhaustive patterns: `((4u8..=std::u8::MAX))` not covered
2020-01-19T10:44:14.0742584Z -   --> $DIR/exhaustiveness-non-exhaustive.rs:11:11
2020-01-19T10:44:14.0743038Z +   --> $DIR/exhaustiveness-non-exhaustive.rs:10:11
2020-01-19T10:44:14.0743304Z 11    |
2020-01-19T10:44:14.0743500Z 12 LL |     match ((0u8,),) {
2020-01-19T10:44:14.0743696Z 13    |           ^^^^^^^^^ pattern `((4u8..=std::u8::MAX))` not covered
2020-01-19T10:44:14.0744113Z 15    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2020-01-19T10:44:14.0744324Z 16 
2020-01-19T10:44:14.0744324Z 16 
2020-01-19T10:44:14.0744788Z 17 error[E0004]: non-exhaustive patterns: `(Some(2u8..=std::u8::MAX))` not covered
2020-01-19T10:44:14.0745645Z -   --> $DIR/exhaustiveness-non-exhaustive.rs:15:11
2020-01-19T10:44:14.0746280Z +   --> $DIR/exhaustiveness-non-exhaustive.rs:14:11
2020-01-19T10:44:14.0748194Z 19    |
2020-01-19T10:44:14.0748404Z 20 LL |     match (Some(0u8),) {
2020-01-19T10:44:14.0748689Z 21    |           ^^^^^^^^^^^^ pattern `(Some(2u8..=std::u8::MAX))` not covered
2020-01-19T10:44:14.0748936Z 
2020-01-19T10:44:14.0749059Z The actual stderr differed from the expected stderr.
2020-01-19T10:44:14.0749588Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-non-exhaustive/exhaustiveness-non-exhaustive.stderr
2020-01-19T10:44:14.0749588Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-non-exhaustive/exhaustiveness-non-exhaustive.stderr
2020-01-19T10:44:14.0750161Z To update references, rerun the tests and pass the `--bless` flag
2020-01-19T10:44:14.0750664Z To only update this specific test, also pass `--test-args or-patterns/exhaustiveness-non-exhaustive.rs`
2020-01-19T10:44:14.0751104Z error: 1 errors occurred comparing output.
2020-01-19T10:44:14.0751246Z status: exit code: 1
2020-01-19T10:44:14.0751246Z status: exit code: 1
2020-01-19T10:44:14.0752147Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/exhaustiveness-non-exhaustive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-non-exhaustive" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-non-exhaustive/auxiliary" "-A" "unused"
2020-01-19T10:44:14.0752701Z ------------------------------------------
2020-01-19T10:44:14.0752856Z 
2020-01-19T10:44:14.0753199Z ------------------------------------------
2020-01-19T10:44:14.0753351Z stderr:
2020-01-19T10:44:14.0753351Z stderr:
2020-01-19T10:44:14.0753654Z ------------------------------------------
2020-01-19T10:44:14.0754050Z error[E0004]: non-exhaustive patterns: `(2u8..=std::u8::MAX, _)` not covered
2020-01-19T10:44:14.0754602Z    |
2020-01-19T10:44:14.0754726Z LL |     match (0u8, 0u8) {
2020-01-19T10:44:14.0754726Z LL |     match (0u8, 0u8) {
2020-01-19T10:44:14.0754850Z    |           ^^^^^^^^^^ pattern `(2u8..=std::u8::MAX, _)` not covered
2020-01-19T10:44:14.0755139Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2020-01-19T10:44:14.0755660Z 
2020-01-19T10:44:14.0755660Z 
2020-01-19T10:44:14.0756177Z error[E0004]: non-exhaustive patterns: `((4u8..=std::u8::MAX))` not covered
2020-01-19T10:44:14.0756831Z    |
2020-01-19T10:44:14.0756831Z    |
2020-01-19T10:44:14.0756971Z LL |     match ((0u8,),) {
2020-01-19T10:44:14.0757114Z    |           ^^^^^^^^^ pattern `((4u8..=std::u8::MAX))` not covered
2020-01-19T10:44:14.0757417Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2020-01-19T10:44:14.0757544Z 
2020-01-19T10:44:14.0757544Z 
2020-01-19T10:44:14.0757959Z error[E0004]: non-exhaustive patterns: `(Some(2u8..=std::u8::MAX))` not covered
2020-01-19T10:44:14.0758684Z    |
2020-01-19T10:44:14.0758684Z    |
2020-01-19T10:44:14.0758831Z LL |     match (Some(0u8),) {
2020-01-19T10:44:14.0758956Z    |           ^^^^^^^^^^^^ pattern `(Some(2u8..=std::u8::MAX))` not covered
2020-01-19T10:44:14.0759229Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2020-01-19T10:44:14.0759351Z 
2020-01-19T10:44:14.0759473Z error: aborting due to 3 previous errors
2020-01-19T10:44:14.0759596Z 
---
2020-01-19T10:44:14.0767279Z 19 error: unreachable pattern
2020-01-19T10:44:14.0767692Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:20:9
2020-01-19T10:44:14.0769795Z +   --> $DIR/exhaustiveness-unreachable-pattern.rs:19:9
2020-01-19T10:44:14.0769862Z 21    |
2020-01-19T10:44:14.0769915Z 22 LL |         (1 | 2,) => {}
2020-01-19T10:44:14.0769979Z 
2020-01-19T10:44:14.0770014Z 24 
2020-01-19T10:44:14.0770068Z 25 error: unreachable pattern
2020-01-19T10:44:14.0770284Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:25:9
---
2020-01-19T10:44:14.0772067Z 43 error: unreachable pattern
2020-01-19T10:44:14.0772275Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:28:9
2020-01-19T10:44:14.0772477Z +   --> $DIR/exhaustiveness-unreachable-pattern.rs:27:9
2020-01-19T10:44:14.0772516Z 45    |
2020-01-19T10:44:14.0772569Z 46 LL |         (2 | 1, 4) => {}
2020-01-19T10:44:14.0772640Z 
2020-01-19T10:44:14.0772690Z 48 
2020-01-19T10:44:14.0772733Z 49 error: unreachable pattern
2020-01-19T10:44:14.0772941Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:30:9
---
2020-01-19T10:44:14.0773382Z 55 error: unreachable pattern
2020-01-19T10:44:14.0773586Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:35:9
2020-01-19T10:44:14.0773808Z +   --> $DIR/exhaustiveness-unreachable-pattern.rs:34:9
2020-01-19T10:44:14.0773847Z 57    |
2020-01-19T10:44:14.0773883Z 58 LL |         (Some(1),) => {}
2020-01-19T10:44:14.0773961Z 
2020-01-19T10:44:14.0773994Z 60 
2020-01-19T10:44:14.0774140Z 61 error: unreachable pattern
2020-01-19T10:44:14.0774405Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:36:9
2020-01-19T10:44:14.0774405Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:36:9
2020-01-19T10:44:14.0774708Z +   --> $DIR/exhaustiveness-unreachable-pattern.rs:35:9
2020-01-19T10:44:14.0774748Z 63    |
2020-01-19T10:44:14.0774801Z 64 LL |         (None,) => {}
2020-01-19T10:44:14.0774863Z 
2020-01-19T10:44:14.0774897Z 66 
2020-01-19T10:44:14.0774950Z 67 error: unreachable pattern
2020-01-19T10:44:14.0775564Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:41:9
2020-01-19T10:44:14.0775564Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:41:9
2020-01-19T10:44:14.0775839Z +   --> $DIR/exhaustiveness-unreachable-pattern.rs:40:9
2020-01-19T10:44:14.0775905Z 69    |
2020-01-19T10:44:14.0775948Z 70 LL |         ((1..=4,),) => {}
2020-01-19T10:44:14.0776021Z 
2020-01-19T10:44:14.0776076Z 72 
2020-01-19T10:44:14.0776118Z 73 error: unreachable pattern
2020-01-19T10:44:14.0776353Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:47:12
2020-01-19T10:44:14.0776353Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:47:12
2020-01-19T10:44:14.0776613Z +   --> $DIR/exhaustiveness-unreachable-pattern.rs:45:14
2020-01-19T10:44:14.0776659Z 75    |
2020-01-19T10:44:14.0776868Z - LL |          | 1,) => {}
2020-01-19T10:44:14.0777057Z -    |            ^
2020-01-19T10:44:14.0777125Z + LL |         (1 | 1,) => {}
2020-01-19T10:44:14.0777207Z 78 
2020-01-19T10:44:14.0777272Z 79 error: unreachable pattern
2020-01-19T10:44:14.0777503Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:54:15
2020-01-19T10:44:14.0777776Z +   --> $DIR/exhaustiveness-unreachable-pattern.rs:52:15
2020-01-19T10:44:14.0777776Z +   --> $DIR/exhaustiveness-unreachable-pattern.rs:52:15
2020-01-19T10:44:14.0777840Z 81    |
2020-01-19T10:44:14.0777882Z 82 LL |             | 0] => {}
2020-01-19T10:44:14.0777952Z 
2020-01-19T10:44:14.0778009Z 84 
2020-01-19T10:44:14.0778050Z 85 error: unreachable pattern
2020-01-19T10:44:14.0778289Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:52:15
---
2020-01-19T10:44:14.0780081Z 101    |              ^
2020-01-19T10:44:14.0780112Z 
2020-01-19T10:44:14.0780134Z 
2020-01-19T10:44:14.0780190Z The actual stderr differed from the expected stderr.
2020-01-19T10:44:14.0780504Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-unreachable-pattern/exhaustiveness-unreachable-pattern.stderr
2020-01-19T10:44:14.0780725Z To update references, rerun the tests and pass the `--bless` flag
2020-01-19T10:44:14.0781003Z To only update this specific test, also pass `--test-args or-patterns/exhaustiveness-unreachable-pattern.rs`
2020-01-19T10:44:14.0781072Z error: 1 errors occurred comparing output.
2020-01-19T10:44:14.0781126Z status: exit code: 1
2020-01-19T10:44:14.0781126Z status: exit code: 1
2020-01-19T10:44:14.0781979Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-unreachable-pattern" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-unreachable-pattern/auxiliary" "-A" "unused"
2020-01-19T10:44:14.0782368Z ------------------------------------------
2020-01-19T10:44:14.0782397Z 
2020-01-19T10:44:14.0782603Z ------------------------------------------
2020-01-19T10:44:14.0782642Z stderr:
2020-01-19T10:44:14.0782642Z stderr:
2020-01-19T10:44:14.0782825Z ------------------------------------------
2020-01-19T10:44:14.0782881Z error: unreachable pattern
2020-01-19T10:44:14.0783102Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:8:9
2020-01-19T10:44:14.0783146Z    |
2020-01-19T10:44:14.0783204Z LL |         (1,) => {} //~ ERROR unreachable pattern
2020-01-19T10:44:14.0783285Z    |
2020-01-19T10:44:14.0783322Z note: lint level defined here
2020-01-19T10:44:14.0783570Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:2:9
2020-01-19T10:44:14.0783612Z    |
2020-01-19T10:44:14.0783612Z    |
2020-01-19T10:44:14.0783650Z LL | #![deny(unreachable_patterns)]
2020-01-19T10:44:14.0783707Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-01-19T10:44:14.0783732Z 
2020-01-19T10:44:14.0783768Z error: unreachable pattern
2020-01-19T10:44:14.0783990Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:13:9
2020-01-19T10:44:14.0784048Z    |
2020-01-19T10:44:14.0784087Z LL |         (2,) => {} //~ ERROR unreachable pattern
2020-01-19T10:44:14.0784166Z 
2020-01-19T10:44:14.0784204Z error: unreachable pattern
2020-01-19T10:44:14.0784424Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:19:9
2020-01-19T10:44:14.0784481Z    |
2020-01-19T10:44:14.0784481Z    |
2020-01-19T10:44:14.0784529Z LL |         (1 | 2,) => {} //~ ERROR unreachable pattern
2020-01-19T10:44:14.0784599Z 
2020-01-19T10:44:14.0784651Z error: unreachable pattern
2020-01-19T10:44:14.0784877Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:24:9
2020-01-19T10:44:14.0784963Z    |
---
2020-01-19T10:44:14.0803910Z 
2020-01-19T10:44:14.0803951Z error: unreachable pattern
2020-01-19T10:44:14.0804198Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:27:9
2020-01-19T10:44:14.0804240Z    |
2020-01-19T10:44:14.0804295Z LL |         (2 | 1, 4) => {} //~ ERROR unreachable pattern
2020-01-19T10:44:14.0804360Z 
2020-01-19T10:44:14.0804396Z error: unreachable pattern
2020-01-19T10:44:14.0804697Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:29:9
2020-01-19T10:44:14.0804739Z    |
2020-01-19T10:44:14.0804739Z    |
2020-01-19T10:44:14.0804779Z LL |         (1, 4 | 5) => {} //~ ERROR unreachable pattern
2020-01-19T10:44:14.0804850Z 
2020-01-19T10:44:14.0805022Z error: unreachable pattern
2020-01-19T10:44:14.0805819Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:34:9
2020-01-19T10:44:14.0806016Z    |
2020-01-19T10:44:14.0806016Z    |
2020-01-19T10:44:14.0806062Z LL |         (Some(1),) => {} //~ ERROR unreachable pattern
2020-01-19T10:44:14.0806137Z 
2020-01-19T10:44:14.0806186Z error: unreachable pattern
2020-01-19T10:44:14.0806500Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:35:9
2020-01-19T10:44:14.0806547Z    |
2020-01-19T10:44:14.0806547Z    |
2020-01-19T10:44:14.0806602Z LL |         (None,) => {}    //~ ERROR unreachable pattern
2020-01-19T10:44:14.0806677Z 
2020-01-19T10:44:14.0806719Z error: unreachable pattern
2020-01-19T10:44:14.0806995Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:40:9
2020-01-19T10:44:14.0807042Z    |
2020-01-19T10:44:14.0807042Z    |
2020-01-19T10:44:14.0807088Z LL |         ((1..=4,),) => {} //~ ERROR unreachable pattern
2020-01-19T10:44:14.0807188Z 
2020-01-19T10:44:14.0807230Z error: unreachable pattern
2020-01-19T10:44:14.0807512Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:45:14
2020-01-19T10:44:14.0807561Z    |
2020-01-19T10:44:14.0807561Z    |
2020-01-19T10:44:14.0807607Z LL |         (1 | 1,) => {} //~ ERROR unreachable
2020-01-19T10:44:14.0807686Z 
2020-01-19T10:44:14.0807727Z error: unreachable pattern
2020-01-19T10:44:14.0807981Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:52:15
2020-01-19T10:44:14.0808037Z    |
2020-01-19T10:44:14.0808037Z    |
2020-01-19T10:44:14.0808081Z LL |             | 0] => {} //~ ERROR unreachable
2020-01-19T10:44:14.0808153Z 
2020-01-19T10:44:14.0808209Z error: unreachable pattern
2020-01-19T10:44:14.0808463Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:50:15
2020-01-19T10:44:14.0808633Z    |
---
2020-01-19T10:44:14.0816362Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-19T10:44:14.0816647Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-19T10:44:14.0816788Z 
2020-01-19T10:44:14.0816924Z 
2020-01-19T10:44:14.0819041Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-19T10:44:14.0819579Z 
2020-01-19T10:44:14.0819687Z 
2020-01-19T10:44:14.0819810Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-19T10:44:14.0819952Z Build completed unsuccessfully in 1:01:50
2020-01-19T10:44:14.0819952Z Build completed unsuccessfully in 1:01:50
2020-01-19T10:44:14.0849616Z == clock drift check ==
2020-01-19T10:44:14.0871344Z   local time: Sun Jan 19 10:44:14 UTC 2020
2020-01-19T10:44:14.3873795Z   network time: Sun, 19 Jan 2020 10:44:14 GMT
2020-01-19T10:44:14.3875093Z == end clock drift check ==
2020-01-19T10:44:14.8106591Z 
2020-01-19T10:44:14.8205783Z ##[error]Bash exited with code '1'.
2020-01-19T10:44:14.8219423Z ##[section]Finishing: Run build
2020-01-19T10:44:14.8240563Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67668/merge to s
2020-01-19T10:44:14.8242269Z Task         : Get sources
2020-01-19T10:44:14.8242325Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-19T10:44:14.8242381Z Version      : 1.0.0
2020-01-19T10:44:14.8242417Z Author       : Microsoft
2020-01-19T10:44:14.8242417Z Author       : Microsoft
2020-01-19T10:44:14.8242471Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-19T10:44:14.8242515Z ==============================================================================
2020-01-19T10:44:15.2517867Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-19T10:44:15.2556803Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67668/merge to s
2020-01-19T10:44:15.2681998Z Cleaning up task key
2020-01-19T10:44:15.2682842Z Start cleaning up orphan processes.
2020-01-19T10:44:15.2808314Z Terminate orphan process: pid (3619) (python)
2020-01-19T10:44:15.3063506Z ##[section]Finishing: Finalize Job
