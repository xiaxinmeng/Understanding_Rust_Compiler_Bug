plain
2019-10-06T04:07:13.9862161Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-06T04:07:14.9604568Z ##[command]git config gc.auto 0
2019-10-06T04:07:14.9611363Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-06T04:07:14.9614332Z ##[command]git config --get-all http.proxy
2019-10-06T04:07:14.9617227Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65150/merge:refs/remotes/pull/65150/merge
---
2019-10-06T05:10:53.2966930Z .................................................................................................... 1500/9104
2019-10-06T05:11:00.4149545Z .................................................................................................... 1600/9104
2019-10-06T05:11:09.7563049Z .................................................................................................... 1700/9104
2019-10-06T05:11:19.2404570Z .......i...............i............................................................................ 1800/9104
2019-10-06T05:11:26.7783348Z ..................................................................................................ii 1900/9104
2019-10-06T05:11:43.6155227Z iii................................................................................................. 2000/9104
2019-10-06T05:11:52.6122768Z .................................................................................................... 2200/9104
2019-10-06T05:11:55.3684622Z .................................................................................................... 2300/9104
2019-10-06T05:12:01.8881486Z .................................................................................................... 2400/9104
2019-10-06T05:12:07.7185659Z .................................................................................................... 2500/9104
---
2019-10-06T05:15:07.2870753Z .......................................................................................i............ 4700/9104
2019-10-06T05:15:15.4448259Z ...i................................................................................................ 4800/9104
2019-10-06T05:15:26.4582427Z .................................................................................................... 4900/9104
2019-10-06T05:15:32.2398484Z .................................................................................................... 5000/9104
2019-10-06T05:15:44.8189452Z ................................................................................ii.ii............... 5100/9104
2019-10-06T05:15:54.6261858Z .................................................................................................... 5300/9104
2019-10-06T05:16:04.8625184Z .................................................................................................... 5400/9104
2019-10-06T05:16:11.8650314Z ..............................................i..................................................... 5500/9104
2019-10-06T05:16:18.9557433Z .................................................................................................... 5600/9104
2019-10-06T05:16:18.9557433Z .................................................................................................... 5600/9104
2019-10-06T05:16:29.9052468Z .................................................................................................... 5700/9104
2019-10-06T05:16:37.8659469Z ...........................................ii...i..ii...........i................................... 5800/9104
2019-10-06T05:17:04.3079565Z .................................................................................................... 6000/9104
2019-10-06T05:17:13.9134085Z .................................................................................................... 6100/9104
2019-10-06T05:17:13.9134085Z .................................................................................................... 6100/9104
2019-10-06T05:17:30.1633050Z .................................................i..ii.............................................. 6200/9104
2019-10-06T05:17:54.9107396Z .................................................................................................... 6400/9104
2019-10-06T05:17:57.2288524Z .........i.......................................................................................... 6500/9104
2019-10-06T05:17:59.5627653Z .................................................................................i.................. 6600/9104
2019-10-06T05:18:02.4196051Z .................................................................................................... 6700/9104
---
2019-10-06T05:22:19.6927601Z diff of stderr:
2019-10-06T05:22:19.6927730Z 
2019-10-06T05:22:19.6928159Z 2   --> $DIR/if-no-match-bindings.rs:18:8
2019-10-06T05:22:19.6928377Z 3    |
2019-10-06T05:22:19.6928522Z 4 LL |     if b_ref() {}
2019-10-06T05:22:19.6929740Z -    |        ^^^^^^^ expected bool, found &bool
2019-10-06T05:22:19.6930178Z +    |        |
2019-10-06T05:22:19.6930178Z +    |        |
2019-10-06T05:22:19.6930321Z +    |        expected bool, found &bool
2019-10-06T05:22:19.6930488Z +    |        help: consider dereferencing the borrow: `*b_ref()`
2019-10-06T05:22:19.6930765Z 7    = note: expected type `bool`
2019-10-06T05:22:19.6930920Z 8               found type `&bool`
2019-10-06T05:22:19.6932240Z 
2019-10-06T05:22:19.6933207Z 11   --> $DIR/if-no-match-bindings.rs:19:8
2019-10-06T05:22:19.6933207Z 11   --> $DIR/if-no-match-bindings.rs:19:8
2019-10-06T05:22:19.6933546Z 12    |
2019-10-06T05:22:19.6933699Z 13 LL |     if b_mut_ref() {}
2019-10-06T05:22:19.6934109Z -    |        ^^^^^^^^^^^ expected bool, found &mut bool
2019-10-06T05:22:19.6934580Z +    |        |
2019-10-06T05:22:19.6934580Z +    |        |
2019-10-06T05:22:19.6934722Z +    |        expected bool, found &mut bool
2019-10-06T05:22:19.6934883Z +    |        help: consider dereferencing the borrow: `*b_mut_ref()`
2019-10-06T05:22:19.6935166Z 16    = note: expected type `bool`
2019-10-06T05:22:19.6935321Z 17               found type `&mut bool`
2019-10-06T05:22:19.6935437Z 
2019-10-06T05:22:19.6935828Z 20   --> $DIR/if-no-match-bindings.rs:20:8
2019-10-06T05:22:19.6935828Z 20   --> $DIR/if-no-match-bindings.rs:20:8
2019-10-06T05:22:19.6936037Z 21    |
2019-10-06T05:22:19.6936172Z 22 LL |     if &true {}
2019-10-06T05:22:19.6936559Z -    |        ^^^^^ expected bool, found &bool
2019-10-06T05:22:19.6936923Z +    |        |
2019-10-06T05:22:19.6936923Z +    |        |
2019-10-06T05:22:19.6937059Z +    |        expected bool, found &bool
2019-10-06T05:22:19.6937447Z +    |        help: consider dereferencing the borrow: `*&true`
2019-10-06T05:22:19.6937801Z 25    = note: expected type `bool`
2019-10-06T05:22:19.6937941Z 26               found type `&bool`
2019-10-06T05:22:19.6938078Z 
2019-10-06T05:22:19.6938572Z 29   --> $DIR/if-no-match-bindings.rs:21:8
2019-10-06T05:22:19.6938572Z 29   --> $DIR/if-no-match-bindings.rs:21:8
2019-10-06T05:22:19.6942158Z 30    |
2019-10-06T05:22:19.6942212Z 31 LL |     if &mut true {}
2019-10-06T05:22:19.6942593Z -    |        ^^^^^^^^^ expected bool, found &mut bool
2019-10-06T05:22:19.6942683Z +    |        |
2019-10-06T05:22:19.6942683Z +    |        |
2019-10-06T05:22:19.6942741Z +    |        expected bool, found &mut bool
2019-10-06T05:22:19.6942788Z +    |        help: consider dereferencing the borrow: `*&mut true`
2019-10-06T05:22:19.6942887Z 34    = note: expected type `bool`
2019-10-06T05:22:19.6942930Z 35               found type `&mut bool`
2019-10-06T05:22:19.6942959Z 
2019-10-06T05:22:19.6943206Z 38   --> $DIR/if-no-match-bindings.rs:24:11
2019-10-06T05:22:19.6943206Z 38   --> $DIR/if-no-match-bindings.rs:24:11
2019-10-06T05:22:19.6943267Z 39    |
2019-10-06T05:22:19.6943307Z 40 LL |     while b_ref() {}
2019-10-06T05:22:19.6943543Z -    |           ^^^^^^^ expected bool, found &bool
2019-10-06T05:22:19.6943643Z +    |           |
2019-10-06T05:22:19.6943643Z +    |           |
2019-10-06T05:22:19.6943685Z +    |           expected bool, found &bool
2019-10-06T05:22:19.6943926Z +    |           help: consider dereferencing the borrow: `*b_ref()`
2019-10-06T05:22:19.6944025Z 43    = note: expected type `bool`
2019-10-06T05:22:19.6944066Z 44               found type `&bool`
2019-10-06T05:22:19.6944225Z 
2019-10-06T05:22:19.6944509Z 47   --> $DIR/if-no-match-bindings.rs:25:11
2019-10-06T05:22:19.6944509Z 47   --> $DIR/if-no-match-bindings.rs:25:11
2019-10-06T05:22:19.6944554Z 48    |
2019-10-06T05:22:19.6944595Z 49 LL |     while b_mut_ref() {}
2019-10-06T05:22:19.6944863Z -    |           ^^^^^^^^^^^ expected bool, found &mut bool
2019-10-06T05:22:19.6944965Z +    |           |
2019-10-06T05:22:19.6944965Z +    |           |
2019-10-06T05:22:19.6945027Z +    |           expected bool, found &mut bool
2019-10-06T05:22:19.6945076Z +    |           help: consider dereferencing the borrow: `*b_mut_ref()`
2019-10-06T05:22:19.6945177Z 52    = note: expected type `bool`
2019-10-06T05:22:19.6945221Z 53               found type `&mut bool`
2019-10-06T05:22:19.6945257Z 
2019-10-06T05:22:19.6945490Z 56   --> $DIR/if-no-match-bindings.rs:26:11
2019-10-06T05:22:19.6945490Z 56   --> $DIR/if-no-match-bindings.rs:26:11
2019-10-06T05:22:19.6945549Z 57    |
2019-10-06T05:22:19.6945590Z 58 LL |     while &true {}
2019-10-06T05:22:19.6945821Z -    |           ^^^^^ expected bool, found &bool
2019-10-06T05:22:19.6945925Z +    |           |
2019-10-06T05:22:19.6945925Z +    |           |
2019-10-06T05:22:19.6945968Z +    |           expected bool, found &bool
2019-10-06T05:22:19.6946032Z +    |           help: consider dereferencing the borrow: `*&true`
2019-10-06T05:22:19.6946117Z 61    = note: expected type `bool`
2019-10-06T05:22:19.6946169Z 62               found type `&bool`
2019-10-06T05:22:19.6946213Z 
2019-10-06T05:22:19.6946448Z 65   --> $DIR/if-no-match-bindings.rs:27:11
2019-10-06T05:22:19.6946448Z 65   --> $DIR/if-no-match-bindings.rs:27:11
2019-10-06T05:22:19.6946491Z 66    |
2019-10-06T05:22:19.6946546Z 67 LL |     while &mut true {}
2019-10-06T05:22:19.6946787Z -    |           ^^^^^^^^^ expected bool, found &mut bool
2019-10-06T05:22:19.6946883Z +    |           |
2019-10-06T05:22:19.6946883Z +    |           |
2019-10-06T05:22:19.6946941Z +    |           expected bool, found &mut bool
2019-10-06T05:22:19.6946990Z +    |           help: consider dereferencing the borrow: `*&mut true`
2019-10-06T05:22:19.6947091Z 70    = note: expected type `bool`
2019-10-06T05:22:19.6947135Z 71               found type `&mut bool`
2019-10-06T05:22:19.6947163Z 
2019-10-06T05:22:19.6947188Z 
2019-10-06T05:22:19.6947188Z 
2019-10-06T05:22:19.6947247Z The actual stderr differed from the expected stderr.
2019-10-06T05:22:19.6947678Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-no-match-bindings/if-no-match-bindings.stderr
2019-10-06T05:22:19.6948032Z To update references, rerun the tests and pass the `--bless` flag
2019-10-06T05:22:19.6948360Z To only update this specific test, also pass `--test-args if/if-no-match-bindings.rs`
2019-10-06T05:22:19.6948445Z error: 1 errors occurred comparing output.
2019-10-06T05:22:19.6948516Z status: exit code: 1
2019-10-06T05:22:19.6948516Z status: exit code: 1
2019-10-06T05:22:19.6949605Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/if/if-no-match-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-no-match-bindings" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-no-match-bindings/auxiliary" "-A" "unused"
2019-10-06T05:22:19.6949967Z ------------------------------------------
2019-10-06T05:22:19.6950001Z 
2019-10-06T05:22:19.7005564Z ------------------------------------------
2019-10-06T05:22:19.7005679Z stderr:
2019-10-06T05:22:19.7005679Z stderr:
2019-10-06T05:22:19.7006103Z ------------------------------------------
2019-10-06T05:22:19.7006347Z error[E0308]: mismatched types
2019-10-06T05:22:19.7006684Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:18:8
2019-10-06T05:22:19.7006739Z    |
2019-10-06T05:22:19.7006791Z LL |     if b_ref() {} //~ ERROR mismatched types [E0308]
2019-10-06T05:22:19.7007005Z    |        |
2019-10-06T05:22:19.7007005Z    |        |
2019-10-06T05:22:19.7007048Z    |        expected bool, found &bool
2019-10-06T05:22:19.7007111Z    |        help: consider dereferencing the borrow: `*b_ref()`
2019-10-06T05:22:19.7007194Z    = note: expected type `bool`
2019-10-06T05:22:19.7007237Z               found type `&bool`
2019-10-06T05:22:19.7007299Z 
2019-10-06T05:22:19.7007340Z error[E0308]: mismatched types
2019-10-06T05:22:19.7007340Z error[E0308]: mismatched types
2019-10-06T05:22:19.7007600Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:19:8
2019-10-06T05:22:19.7007662Z    |
2019-10-06T05:22:19.7007706Z LL |     if b_mut_ref() {} //~ ERROR mismatched types [E0308]
2019-10-06T05:22:19.7007797Z    |        |
2019-10-06T05:22:19.7007797Z    |        |
2019-10-06T05:22:19.7007853Z    |        expected bool, found &mut bool
2019-10-06T05:22:19.7007899Z    |        help: consider dereferencing the borrow: `*b_mut_ref()`
2019-10-06T05:22:19.7007996Z    = note: expected type `bool`
2019-10-06T05:22:19.7008037Z               found type `&mut bool`
2019-10-06T05:22:19.7008066Z 
2019-10-06T05:22:19.7008104Z error[E0308]: mismatched types
2019-10-06T05:22:19.7008104Z error[E0308]: mismatched types
2019-10-06T05:22:19.7008365Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:20:8
2019-10-06T05:22:19.7008410Z    |
2019-10-06T05:22:19.7008453Z LL |     if &true {} //~ ERROR mismatched types [E0308]
2019-10-06T05:22:19.7008561Z    |        |
2019-10-06T05:22:19.7008561Z    |        |
2019-10-06T05:22:19.7008602Z    |        expected bool, found &bool
2019-10-06T05:22:19.7008663Z    |        help: consider dereferencing the borrow: `*&true`
2019-10-06T05:22:19.7008743Z    = note: expected type `bool`
2019-10-06T05:22:19.7008792Z               found type `&bool`
2019-10-06T05:22:19.7008838Z 
2019-10-06T05:22:19.7008877Z error[E0308]: mismatched types
2019-10-06T05:22:19.7008877Z error[E0308]: mismatched types
2019-10-06T05:22:19.7009120Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:21:8
2019-10-06T05:22:19.7009614Z    |
2019-10-06T05:22:19.7009661Z LL |     if &mut true {} //~ ERROR mismatched types [E0308]
2019-10-06T05:22:19.7009747Z    |        |
2019-10-06T05:22:19.7009747Z    |        |
2019-10-06T05:22:19.7009807Z    |        expected bool, found &mut bool
2019-10-06T05:22:19.7009854Z    |        help: consider dereferencing the borrow: `*&mut true`
2019-10-06T05:22:19.7012595Z    = note: expected type `bool`
2019-10-06T05:22:19.7012727Z               found type `&mut bool`
2019-10-06T05:22:19.7012758Z 
2019-10-06T05:22:19.7012799Z error[E0308]: mismatched types
2019-10-06T05:22:19.7012799Z error[E0308]: mismatched types
2019-10-06T05:22:19.7013236Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:24:11
2019-10-06T05:22:19.7013286Z    |
2019-10-06T05:22:19.7013330Z LL |     while b_ref() {} //~ ERROR mismatched types [E0308]
2019-10-06T05:22:19.7013819Z    |           |
2019-10-06T05:22:19.7013819Z    |           |
2019-10-06T05:22:19.7013861Z    |           expected bool, found &bool
2019-10-06T05:22:19.7013927Z    |           help: consider dereferencing the borrow: `*b_ref()`
2019-10-06T05:22:19.7014012Z    = note: expected type `bool`
2019-10-06T05:22:19.7014072Z               found type `&bool`
2019-10-06T05:22:19.7014101Z 
2019-10-06T05:22:19.7014140Z error[E0308]: mismatched types
2019-10-06T05:22:19.7014140Z error[E0308]: mismatched types
2019-10-06T05:22:19.7014764Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:25:11
2019-10-06T05:22:19.7014849Z    |
2019-10-06T05:22:19.7014896Z LL |     while b_mut_ref() {} //~ ERROR mismatched types [E0308]
2019-10-06T05:22:19.7015003Z    |           |
2019-10-06T05:22:19.7015003Z    |           |
2019-10-06T05:22:19.7015048Z    |           expected bool, found &mut bool
2019-10-06T05:22:19.7015097Z    |           help: consider dereferencing the borrow: `*b_mut_ref()`
2019-10-06T05:22:19.7019470Z    = note: expected type `bool`
2019-10-06T05:22:19.7019514Z               found type `&mut bool`
2019-10-06T05:22:19.7019546Z 
2019-10-06T05:22:19.7019604Z error[E0308]: mismatched types
2019-10-06T05:22:19.7019604Z error[E0308]: mismatched types
2019-10-06T05:22:19.7020039Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:26:11
2019-10-06T05:22:19.7020089Z    |
2019-10-06T05:22:19.7020134Z LL |     while &true {} //~ ERROR mismatched types [E0308]
2019-10-06T05:22:19.7020240Z    |           |
2019-10-06T05:22:19.7020240Z    |           |
2019-10-06T05:22:19.7020283Z    |           expected bool, found &bool
2019-10-06T05:22:19.7020363Z    |           help: consider dereferencing the borrow: `*&true`
2019-10-06T05:22:19.7020448Z    = note: expected type `bool`
2019-10-06T05:22:19.7020508Z               found type `&bool`
2019-10-06T05:22:19.7020537Z 
2019-10-06T05:22:19.7020577Z error[E0308]: mismatched types
2019-10-06T05:22:19.7020577Z error[E0308]: mismatched types
2019-10-06T05:22:19.7020852Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:27:11
2019-10-06T05:22:19.7020916Z    |
2019-10-06T05:22:19.7020962Z LL |     while &mut true {} //~ ERROR mismatched types [E0308]
2019-10-06T05:22:19.7021067Z    |           |
2019-10-06T05:22:19.7021067Z    |           |
2019-10-06T05:22:19.7021110Z    |           expected bool, found &mut bool
2019-10-06T05:22:19.7021159Z    |           help: consider dereferencing the borrow: `*&mut true`
2019-10-06T05:22:19.7021262Z    = note: expected type `bool`
2019-10-06T05:22:19.7021305Z               found type `&mut bool`
2019-10-06T05:22:19.7021334Z 
2019-10-06T05:22:19.7021403Z error: aborting due to 8 previous errors
---
2019-10-06T05:22:19.7022946Z diff of stderr:
2019-10-06T05:22:19.7022974Z 
2019-10-06T05:22:19.7023211Z 517   --> $DIR/disallowed-positions.rs:32:8
2019-10-06T05:22:19.7023257Z 518    |
2019-10-06T05:22:19.7023315Z 519 LL |     if &let 0 = 0 {}
2019-10-06T05:22:19.7023556Z -    |        ^^^^^^^^^^ expected bool, found &bool
2019-10-06T05:22:19.7023661Z +    |        |
2019-10-06T05:22:19.7023661Z +    |        |
2019-10-06T05:22:19.7023704Z +    |        expected bool, found &bool
2019-10-06T05:22:19.7023903Z +    |        help: consider dereferencing the borrow: `*&let 0 = 0`
2019-10-06T05:22:19.7024022Z 522    = note: expected type `bool`
2019-10-06T05:22:19.7024066Z 523               found type `&bool`
2019-10-06T05:22:19.7024095Z 
2019-10-06T05:22:19.7024390Z 702   --> $DIR/disallowed-positions.rs:96:11
2019-10-06T05:22:19.7024390Z 702   --> $DIR/disallowed-positions.rs:96:11
2019-10-06T05:22:19.7024437Z 703    |
2019-10-06T05:22:19.7024488Z 704 LL |     while &let 0 = 0 {}
2019-10-06T05:22:19.7024730Z -    |           ^^^^^^^^^^ expected bool, found &bool
2019-10-06T05:22:19.7024836Z +    |           |
2019-10-06T05:22:19.7024836Z +    |           |
2019-10-06T05:22:19.7024881Z +    |           expected bool, found &bool
2019-10-06T05:22:19.7024948Z +    |           help: consider dereferencing the borrow: `*&let 0 = 0`
2019-10-06T05:22:19.7025033Z 707    = note: expected type `bool`
2019-10-06T05:22:19.7025610Z 708               found type `&bool`
2019-10-06T05:22:19.7025640Z 
2019-10-06T05:22:19.7025665Z 
2019-10-06T05:22:19.7025665Z 
2019-10-06T05:22:19.7025721Z The actual stderr differed from the expected stderr.
2019-10-06T05:22:19.7026158Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/disallowed-positions.stderr
2019-10-06T05:22:19.7026766Z To update references, rerun the tests and pass the `--bless` flag
2019-10-06T05:22:19.7027169Z To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/disallowed-positions.rs`
2019-10-06T05:22:19.7027726Z error: 1 errors occurred comparing output.
2019-10-06T05:22:19.7027775Z status: exit code: 1
2019-10-06T05:22:19.7027775Z status: exit code: 1
2019-10-06T05:22:19.7028707Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/auxiliary" "-A" "unused"
2019-10-06T05:22:19.7029088Z ------------------------------------------
2019-10-06T05:22:19.7029420Z 
2019-10-06T05:22:19.7030101Z ------------------------------------------
2019-10-06T05:22:19.7030158Z stderr:
2019-10-06T05:22:19.7030158Z stderr:
2019-10-06T05:22:19.7030413Z ------------------------------------------
2019-10-06T05:22:19.7030463Z error: expected one of `,` or `>`, found `&&`
2019-10-06T05:22:19.7030803Z    |
2019-10-06T05:22:19.7030803Z    |
2019-10-06T05:22:19.7030855Z LL |         true && let 1 = 1 //~ ERROR expected one of `,` or `>`, found `&&`
2019-10-06T05:22:19.7030906Z    |              ^^ expected one of `,` or `>` here
2019-10-06T05:22:19.7031010Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7031289Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:32:9
2019-10-06T05:22:19.7031358Z    |
2019-10-06T05:22:19.7031358Z    |
2019-10-06T05:22:19.7031408Z LL |     if &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7031508Z    |
2019-10-06T05:22:19.7031508Z    |
2019-10-06T05:22:19.7031813Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7031872Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7031967Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7032257Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:35:9
2019-10-06T05:22:19.7032306Z    |
2019-10-06T05:22:19.7032306Z    |
2019-10-06T05:22:19.7032505Z LL |     if !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7032740Z    |
2019-10-06T05:22:19.7032740Z    |
2019-10-06T05:22:19.7033053Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7033128Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7033201Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7033489Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:36:9
2019-10-06T05:22:19.7033538Z    |
2019-10-06T05:22:19.7033538Z    |
2019-10-06T05:22:19.7033585Z LL |     if *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7033694Z    |
2019-10-06T05:22:19.7033694Z    |
2019-10-06T05:22:19.7033967Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7034065Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7034141Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7034421Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:38:9
2019-10-06T05:22:19.7034485Z    |
2019-10-06T05:22:19.7034485Z    |
2019-10-06T05:22:19.7034756Z LL |     if -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7034864Z    |
2019-10-06T05:22:19.7034864Z    |
2019-10-06T05:22:19.7035277Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7035335Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7035429Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7035707Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:46:9
2019-10-06T05:22:19.7035755Z    |
2019-10-06T05:22:19.7035755Z    |
2019-10-06T05:22:19.7035818Z LL |     if (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7035904Z    |
2019-10-06T05:22:19.7035904Z    |
2019-10-06T05:22:19.7036204Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7036261Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7036335Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7036632Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:50:16
2019-10-06T05:22:19.7036689Z    |
2019-10-06T05:22:19.7036689Z    |
2019-10-06T05:22:19.7036736Z LL |     if true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7036840Z    |
2019-10-06T05:22:19.7036840Z    |
2019-10-06T05:22:19.7037231Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7037299Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7037369Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7037630Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:51:17
2019-10-06T05:22:19.7037691Z    |
2019-10-06T05:22:19.7037691Z    |
2019-10-06T05:22:19.7037735Z LL |     if (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7037836Z    |
2019-10-06T05:22:19.7037836Z    |
2019-10-06T05:22:19.7038077Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7038254Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7038369Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7038627Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:52:25
2019-10-06T05:22:19.7038688Z    |
2019-10-06T05:22:19.7038688Z    |
2019-10-06T05:22:19.7038737Z LL |     if true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7039287Z    |
2019-10-06T05:22:19.7039287Z    |
2019-10-06T05:22:19.7039791Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7039861Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7039954Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7040259Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:53:25
2019-10-06T05:22:19.7040319Z    |
2019-10-06T05:22:19.7040319Z    |
2019-10-06T05:22:19.7040385Z LL |     if true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7040473Z    |
2019-10-06T05:22:19.7040473Z    |
2019-10-06T05:22:19.7040734Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7040806Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7040879Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7041172Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:56:12
2019-10-06T05:22:19.7041221Z    |
2019-10-06T05:22:19.7041221Z    |
2019-10-06T05:22:19.7041268Z LL |     if x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7041372Z    |
2019-10-06T05:22:19.7041372Z    |
2019-10-06T05:22:19.7041630Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7041803Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7041876Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7042173Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:59:15
2019-10-06T05:22:19.7042240Z    |
2019-10-06T05:22:19.7042240Z    |
2019-10-06T05:22:19.7042287Z LL |     if true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7042500Z    |
2019-10-06T05:22:19.7042500Z    |
2019-10-06T05:22:19.7042760Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7042813Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7042902Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7054059Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:61:11
2019-10-06T05:22:19.7054186Z    |
2019-10-06T05:22:19.7054186Z    |
2019-10-06T05:22:19.7054281Z LL |     if ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7054382Z    |
2019-10-06T05:22:19.7054382Z    |
2019-10-06T05:22:19.7054737Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7054806Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7054899Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7055236Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:63:9
2019-10-06T05:22:19.7055297Z    |
2019-10-06T05:22:19.7055297Z    |
2019-10-06T05:22:19.7055353Z LL |     if (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7055475Z    |
2019-10-06T05:22:19.7055475Z    |
2019-10-06T05:22:19.7055809Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7055902Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7055994Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7056321Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:67:8
2019-10-06T05:22:19.7056395Z    |
2019-10-06T05:22:19.7056395Z    |
2019-10-06T05:22:19.7056450Z LL |     if let Range { start: _, end: _ } = true..true && false {}
2019-10-06T05:22:19.7056576Z    |
2019-10-06T05:22:19.7056576Z    |
2019-10-06T05:22:19.7057115Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7057200Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7057314Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7057685Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:71:8
2019-10-06T05:22:19.7057770Z    |
2019-10-06T05:22:19.7057770Z    |
2019-10-06T05:22:19.7057826Z LL |     if let Range { start: _, end: _ } = true..true || false {}
2019-10-06T05:22:19.7057933Z    |
2019-10-06T05:22:19.7057933Z    |
2019-10-06T05:22:19.7058264Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7058329Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7082400Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7082937Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:78:8
2019-10-06T05:22:19.7083024Z    |
2019-10-06T05:22:19.7083024Z    |
2019-10-06T05:22:19.7083080Z LL |     if let Range { start: F, end } = F..|| true {}
2019-10-06T05:22:19.7083185Z    |
2019-10-06T05:22:19.7083185Z    |
2019-10-06T05:22:19.7083552Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7083852Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7083957Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7084357Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:86:8
2019-10-06T05:22:19.7084416Z    |
2019-10-06T05:22:19.7084416Z    |
2019-10-06T05:22:19.7084481Z LL |     if let Range { start: true, end } = t..&&false {}
2019-10-06T05:22:19.7084587Z    |
2019-10-06T05:22:19.7084587Z    |
2019-10-06T05:22:19.7085014Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7085098Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7085188Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7085994Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:92:19
2019-10-06T05:22:19.7086070Z    |
2019-10-06T05:22:19.7086126Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7086126Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7086198Z    |                   ^^^^^^^^^^^^^^^
2019-10-06T05:22:19.7086247Z    |
2019-10-06T05:22:19.7086644Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7086726Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7086817Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7087155Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:96:12
2019-10-06T05:22:19.7087220Z    |
2019-10-06T05:22:19.7087220Z    |
2019-10-06T05:22:19.7087277Z LL |     while &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7087390Z    |
2019-10-06T05:22:19.7087390Z    |
2019-10-06T05:22:19.7087703Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7087779Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7087880Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7088200Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:99:12
2019-10-06T05:22:19.7088255Z    |
2019-10-06T05:22:19.7088255Z    |
2019-10-06T05:22:19.7088324Z LL |     while !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7088425Z    |
2019-10-06T05:22:19.7088425Z    |
2019-10-06T05:22:19.7088879Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7088961Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7089347Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7089778Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:100:12
2019-10-06T05:22:19.7089853Z    |
2019-10-06T05:22:19.7089853Z    |
2019-10-06T05:22:19.7089909Z LL |     while *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7090021Z    |
2019-10-06T05:22:19.7090021Z    |
2019-10-06T05:22:19.7090340Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7090412Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7090500Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7090820Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:102:12
2019-10-06T05:22:19.7090896Z    |
2019-10-06T05:22:19.7090896Z    |
2019-10-06T05:22:19.7091531Z LL |     while -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7091661Z    |
2019-10-06T05:22:19.7091661Z    |
2019-10-06T05:22:19.7091979Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7092212Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7092322Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7092688Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:110:12
2019-10-06T05:22:19.7092744Z    |
2019-10-06T05:22:19.7092744Z    |
2019-10-06T05:22:19.7092811Z LL |     while (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7092912Z    |
2019-10-06T05:22:19.7092912Z    |
2019-10-06T05:22:19.7093231Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7093308Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7093405Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7093726Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:114:19
2019-10-06T05:22:19.7093781Z    |
2019-10-06T05:22:19.7093781Z    |
2019-10-06T05:22:19.7093855Z LL |     while true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7093957Z    |
2019-10-06T05:22:19.7093957Z    |
2019-10-06T05:22:19.7094269Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7094342Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7094429Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7094761Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:115:20
2019-10-06T05:22:19.7094828Z    |
2019-10-06T05:22:19.7094828Z    |
2019-10-06T05:22:19.7094885Z LL |     while (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7095006Z    |
2019-10-06T05:22:19.7095006Z    |
2019-10-06T05:22:19.7095316Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7095391Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7095492Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7095814Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:116:28
2019-10-06T05:22:19.7095879Z    |
2019-10-06T05:22:19.7095879Z    |
2019-10-06T05:22:19.7095937Z LL |     while true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7096058Z    |
2019-10-06T05:22:19.7096058Z    |
2019-10-06T05:22:19.7096470Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7096544Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7096645Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7096999Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:117:28
2019-10-06T05:22:19.7097067Z    |
2019-10-06T05:22:19.7097067Z    |
2019-10-06T05:22:19.7097134Z LL |     while true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7097239Z    |
2019-10-06T05:22:19.7097239Z    |
2019-10-06T05:22:19.7097562Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7097625Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7097713Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7098048Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:120:15
2019-10-06T05:22:19.7098161Z    |
2019-10-06T05:22:19.7098161Z    |
2019-10-06T05:22:19.7098217Z LL |     while x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7098335Z    |
2019-10-06T05:22:19.7098335Z    |
2019-10-06T05:22:19.7098648Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7098832Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7098921Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7099607Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:123:18
2019-10-06T05:22:19.7099676Z    |
2019-10-06T05:22:19.7099676Z    |
2019-10-06T05:22:19.7099732Z LL |     while true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7099843Z    |
2019-10-06T05:22:19.7099843Z    |
2019-10-06T05:22:19.7100175Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7100243Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7100341Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7100660Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:125:14
2019-10-06T05:22:19.7100732Z    |
2019-10-06T05:22:19.7100732Z    |
2019-10-06T05:22:19.7100789Z LL |     while ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7100905Z    |
2019-10-06T05:22:19.7100905Z    |
2019-10-06T05:22:19.7101216Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7101280Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7101385Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7101713Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:127:12
2019-10-06T05:22:19.7101769Z    |
2019-10-06T05:22:19.7101769Z    |
2019-10-06T05:22:19.7101841Z LL |     while (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7101944Z    |
2019-10-06T05:22:19.7101944Z    |
2019-10-06T05:22:19.7102267Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7102340Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7102428Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7102755Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:131:11
2019-10-06T05:22:19.7102810Z    |
2019-10-06T05:22:19.7102810Z    |
2019-10-06T05:22:19.7102864Z LL |     while let Range { start: _, end: _ } = true..true && false {}
2019-10-06T05:22:19.7102984Z    |
2019-10-06T05:22:19.7102984Z    |
2019-10-06T05:22:19.7103421Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7103512Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7103599Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7103961Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:135:11
2019-10-06T05:22:19.7104041Z    |
2019-10-06T05:22:19.7104041Z    |
2019-10-06T05:22:19.7104095Z LL |     while let Range { start: _, end: _ } = true..true || false {}
2019-10-06T05:22:19.7104216Z    |
2019-10-06T05:22:19.7104216Z    |
2019-10-06T05:22:19.7104527Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7104593Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7104690Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7105016Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:142:11
2019-10-06T05:22:19.7105072Z    |
2019-10-06T05:22:19.7105072Z    |
2019-10-06T05:22:19.7105133Z LL |     while let Range { start: F, end } = F..|| true {}
2019-10-06T05:22:19.7105238Z    |
2019-10-06T05:22:19.7105238Z    |
2019-10-06T05:22:19.7105558Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7105732Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7105835Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7106191Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:150:11
2019-10-06T05:22:19.7106247Z    |
2019-10-06T05:22:19.7106247Z    |
2019-10-06T05:22:19.7106300Z LL |     while let Range { start: true, end } = t..&&false {}
2019-10-06T05:22:19.7106423Z    |
2019-10-06T05:22:19.7106423Z    |
2019-10-06T05:22:19.7106744Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7106821Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7106907Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7107233Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:156:22
2019-10-06T05:22:19.7107298Z    |
2019-10-06T05:22:19.7107356Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7107356Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7107414Z    |                      ^^^^^^^^^^^^^^^
2019-10-06T05:22:19.7107470Z    |
2019-10-06T05:22:19.7107779Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7107843Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7107939Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7108268Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:170:6
2019-10-06T05:22:19.7108335Z    |
2019-10-06T05:22:19.7108335Z    |
2019-10-06T05:22:19.7108389Z LL |     &let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7108489Z    |
2019-10-06T05:22:19.7108489Z    |
2019-10-06T05:22:19.7109178Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7109269Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7109375Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7109701Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:172:6
2019-10-06T05:22:19.7109757Z    |
2019-10-06T05:22:19.7109757Z    |
2019-10-06T05:22:19.7109826Z LL |     !let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7109923Z    |
2019-10-06T05:22:19.7109923Z    |
2019-10-06T05:22:19.7110213Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7110420Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7110515Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7110862Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:173:6
2019-10-06T05:22:19.7110927Z    |
2019-10-06T05:22:19.7110927Z    |
2019-10-06T05:22:19.7110978Z LL |     *let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7111086Z    |
2019-10-06T05:22:19.7111086Z    |
2019-10-06T05:22:19.7111374Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7111445Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7111528Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7111820Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:175:6
2019-10-06T05:22:19.7111881Z    |
2019-10-06T05:22:19.7111881Z    |
2019-10-06T05:22:19.7112169Z LL |     -let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7112275Z    |
2019-10-06T05:22:19.7112275Z    |
2019-10-06T05:22:19.7112561Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7112623Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7112820Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7113149Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:183:6
2019-10-06T05:22:19.7113202Z    |
2019-10-06T05:22:19.7113202Z    |
2019-10-06T05:22:19.7113262Z LL |     (let 0 = 0)?; //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7113356Z    |
2019-10-06T05:22:19.7113356Z    |
2019-10-06T05:22:19.7113657Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7113729Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7113812Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7114121Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:187:13
2019-10-06T05:22:19.7114173Z    |
2019-10-06T05:22:19.7114173Z    |
2019-10-06T05:22:19.7114225Z LL |     true || let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7114345Z    |
2019-10-06T05:22:19.7114345Z    |
2019-10-06T05:22:19.7114631Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7114701Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7114782Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7115073Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:188:14
2019-10-06T05:22:19.7115137Z    |
2019-10-06T05:22:19.7115137Z    |
2019-10-06T05:22:19.7115200Z LL |     (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7115307Z    |
2019-10-06T05:22:19.7115307Z    |
2019-10-06T05:22:19.7115592Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7115652Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7115754Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7116046Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:189:22
2019-10-06T05:22:19.7116097Z    |
2019-10-06T05:22:19.7116097Z    |
2019-10-06T05:22:19.7116166Z LL |     true && (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7116266Z    |
2019-10-06T05:22:19.7116266Z    |
2019-10-06T05:22:19.7116564Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7116725Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7116834Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7117156Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:192:9
2019-10-06T05:22:19.7117209Z    |
2019-10-06T05:22:19.7117209Z    |
2019-10-06T05:22:19.7117273Z LL |     x = let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7117381Z    |
2019-10-06T05:22:19.7117381Z    |
2019-10-06T05:22:19.7117670Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7117744Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7117826Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7118128Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:194:12
2019-10-06T05:22:19.7118181Z    |
2019-10-06T05:22:19.7118181Z    |
2019-10-06T05:22:19.7118241Z LL |     true..(let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7118346Z    |
2019-10-06T05:22:19.7118346Z    |
2019-10-06T05:22:19.7118629Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7118690Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7118885Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7119506Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:195:8
2019-10-06T05:22:19.7119587Z    |
2019-10-06T05:22:19.7119587Z    |
2019-10-06T05:22:19.7119639Z LL |     ..(let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7119743Z    |
2019-10-06T05:22:19.7119743Z    |
2019-10-06T05:22:19.7120088Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7120151Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7120263Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7120559Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:196:6
2019-10-06T05:22:19.7120612Z    |
2019-10-06T05:22:19.7120612Z    |
2019-10-06T05:22:19.7120680Z LL |     (let 0 = 0)..; //~ ERROR `let` expressions are not supported here
2019-10-06T05:22:19.7120785Z    |
2019-10-06T05:22:19.7120785Z    |
2019-10-06T05:22:19.7121087Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7121148Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7121230Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7121535Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:198:6
2019-10-06T05:22:19.7121587Z    |
2019-10-06T05:22:19.7121587Z    |
2019-10-06T05:22:19.7121637Z LL |     (let Range { start: _, end: _ } = true..true || false);
2019-10-06T05:22:19.7121758Z    |
2019-10-06T05:22:19.7121758Z    |
2019-10-06T05:22:19.7122044Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7122119Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7122200Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7122500Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:202:6
2019-10-06T05:22:19.7122567Z    |
2019-10-06T05:22:19.7122613Z LL |     (let true = let true = true);
2019-10-06T05:22:19.7122613Z LL |     (let true = let true = true);
2019-10-06T05:22:19.7122663Z    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-06T05:22:19.7122723Z    |
2019-10-06T05:22:19.7123010Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T05:22:19.7123071Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T05:22:19.7123304Z error: `let` expressions are not supported here
2019-10-06T05:22:19.7123650Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:202:17
2019-10-06T05:22:19.7123705Z    |
2019-10-06T05:22:19.7123769Z LL |     (let true = let true = true);
---
2019-10-06T05:22:20.3002565Z test result: FAILED. 9064 passed; 2 failed; 38 ignored; 0 measured; 0 filtered out
2019-10-06T05:22:20.3002613Z 
2019-10-06T05:22:20.3002643Z 
2019-10-06T05:22:20.3002688Z 
2019-10-06T05:22:20.3004319Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-06T05:22:20.3004623Z 
2019-10-06T05:22:20.3004656Z 
2019-10-06T05:22:20.3004705Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-06T05:22:20.3004763Z Build completed unsuccessfully in 1:07:30
2019-10-06T05:22:20.3004763Z Build completed unsuccessfully in 1:07:30
2019-10-06T05:22:20.3005116Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-06T05:22:20.3005184Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-06T05:22:20.3005255Z == clock drift check ==
2019-10-06T05:22:20.3005409Z   local time: Sun Oct  6 05:22:19 UTC 2019
2019-10-06T05:22:20.3005476Z   network time: Sun, 06 Oct 2019 05:22:19 GMT
2019-10-06T05:22:20.3005530Z == end clock drift check ==
2019-10-06T05:22:20.9545901Z ##[error]Bash exited with code '1'.
2019-10-06T05:22:20.9597930Z ##[section]Starting: Checkout
2019-10-06T05:22:20.9600457Z ==============================================================================
2019-10-06T05:22:20.9600541Z Task         : Get sources
2019-10-06T05:22:20.9600595Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
