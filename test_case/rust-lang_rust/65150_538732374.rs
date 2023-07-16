plain
2019-10-06T09:13:14.3446736Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-06T09:13:14.3630369Z ##[command]git config gc.auto 0
2019-10-06T09:13:14.3707199Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-06T09:13:14.3769595Z ##[command]git config --get-all http.proxy
2019-10-06T09:13:14.3916478Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65150/merge:refs/remotes/pull/65150/merge
---
2019-10-06T10:18:14.7826374Z .................................................................................................... 1500/9109
2019-10-06T10:18:22.0968297Z .................................................................................................... 1600/9109
2019-10-06T10:18:31.8515125Z .................................................................................................... 1700/9109
2019-10-06T10:18:41.6677943Z ........i...............i........................................................................... 1800/9109
2019-10-06T10:18:49.2465529Z ...................................................................................................i 1900/9109
2019-10-06T10:19:06.2143983Z iiii................................................................................................ 2000/9109
2019-10-06T10:19:15.6257581Z .................................................................................................... 2200/9109
2019-10-06T10:19:18.4681641Z .................................................................................................... 2300/9109
2019-10-06T10:19:25.1656868Z .................................................................................................... 2400/9109
2019-10-06T10:19:31.1146722Z .................................................................................................... 2500/9109
---
2019-10-06T10:22:35.1518965Z ........................................................................................i........... 4700/9109
2019-10-06T10:22:43.4193831Z ....i............................................................................................... 4800/9109
2019-10-06T10:22:54.5356554Z .................................................................................................... 4900/9109
2019-10-06T10:23:00.6503144Z .................................................................................................... 5000/9109
2019-10-06T10:23:13.5208656Z ..................................................................................ii.ii............. 5100/9109
2019-10-06T10:23:23.4770771Z .................................................................................................... 5300/9109
2019-10-06T10:23:34.2090909Z .................................................................................................... 5400/9109
2019-10-06T10:23:41.4356551Z ................................................i................................................... 5500/9109
2019-10-06T10:23:48.8634046Z .................................................................................................... 5600/9109
2019-10-06T10:23:48.8634046Z .................................................................................................... 5600/9109
2019-10-06T10:23:59.8440687Z .................................................................................................... 5700/9109
2019-10-06T10:24:08.2722677Z .............................................ii...i..ii...........i................................. 5800/9109
2019-10-06T10:24:35.4016827Z .................................................................................................... 6000/9109
2019-10-06T10:24:45.1750114Z .................................................................................................... 6100/9109
2019-10-06T10:24:45.1750114Z .................................................................................................... 6100/9109
2019-10-06T10:24:57.0056056Z ...................................................i..ii............................................ 6200/9109
2019-10-06T10:25:23.0807196Z .................................................................................................... 6400/9109
2019-10-06T10:25:25.3999098Z ...........i........................................................................................ 6500/9109
2019-10-06T10:25:27.7952151Z ....................................................................................i............... 6600/9109
2019-10-06T10:25:30.7267496Z .................................................................................................... 6700/9109
---
2019-10-06T10:29:55.0283372Z 
2019-10-06T10:29:55.0283552Z 73 error[E0308]: mismatched types
2019-10-06T10:29:55.0283963Z 74   --> $DIR/if-no-match-bindings.rs:26:11
2019-10-06T10:29:55.0284175Z 75    |
2019-10-06T10:29:55.0284514Z - 26 |     while &true {}
2019-10-06T10:29:55.0284689Z + LL |     while &true {}
2019-10-06T10:29:55.0285012Z 78    |           |
2019-10-06T10:29:55.0285012Z 78    |           |
2019-10-06T10:29:55.0285156Z 79    |           expected bool, found &bool
2019-10-06T10:29:55.0285434Z 85 error[E0308]: mismatched types
2019-10-06T10:29:55.0285784Z 86   --> $DIR/if-no-match-bindings.rs:27:11
2019-10-06T10:29:55.0285982Z 87    |
2019-10-06T10:29:55.0285982Z 87    |
2019-10-06T10:29:55.0286316Z - 27 |     while &mut true {}
2019-10-06T10:29:55.0287240Z + LL |     while &mut true {}
2019-10-06T10:29:55.0287712Z 90    |           |
2019-10-06T10:29:55.0287712Z 90    |           |
2019-10-06T10:29:55.0287877Z 91    |           expected bool, found &mut bool
2019-10-06T10:29:55.0288172Z 
2019-10-06T10:29:55.0288334Z The actual stderr differed from the expected stderr.
2019-10-06T10:29:55.0288911Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-no-match-bindings/if-no-match-bindings.stderr
2019-10-06T10:29:55.0288911Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-no-match-bindings/if-no-match-bindings.stderr
2019-10-06T10:29:55.0289414Z To update references, rerun the tests and pass the `--bless` flag
2019-10-06T10:29:55.0290185Z To only update this specific test, also pass `--test-args if/if-no-match-bindings.rs`
2019-10-06T10:29:55.0290584Z error: 1 errors occurred comparing output.
2019-10-06T10:29:55.0290747Z status: exit code: 1
2019-10-06T10:29:55.0290747Z status: exit code: 1
2019-10-06T10:29:55.0292102Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/if/if-no-match-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-no-match-bindings" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-no-match-bindings/auxiliary" "-A" "unused"
2019-10-06T10:29:55.0292771Z ------------------------------------------
2019-10-06T10:29:55.0292934Z 
2019-10-06T10:29:55.0293292Z ------------------------------------------
2019-10-06T10:29:55.0293492Z stderr:
2019-10-06T10:29:55.0293492Z stderr:
2019-10-06T10:29:55.0293824Z ------------------------------------------
2019-10-06T10:29:55.0293998Z error[E0308]: mismatched types
2019-10-06T10:29:55.0294373Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:18:8
2019-10-06T10:29:55.0294551Z    |
2019-10-06T10:29:55.0294717Z LL |     if b_ref() {} //~ ERROR mismatched types [E0308]
2019-10-06T10:29:55.0294998Z    |        |
2019-10-06T10:29:55.0294998Z    |        |
2019-10-06T10:29:55.0295152Z    |        expected bool, found &bool
2019-10-06T10:29:55.0295296Z    |        help: consider dereferencing the borrow: `*b_ref()`
2019-10-06T10:29:55.0295609Z    = note: expected type `bool`
2019-10-06T10:29:55.0295751Z               found type `&bool`
2019-10-06T10:29:55.0295870Z 
2019-10-06T10:29:55.0296024Z error[E0308]: mismatched types
2019-10-06T10:29:55.0296024Z error[E0308]: mismatched types
2019-10-06T10:29:55.0297334Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:19:8
2019-10-06T10:29:55.0297630Z    |
2019-10-06T10:29:55.0297823Z LL |     if b_mut_ref() {} //~ ERROR mismatched types [E0308]
2019-10-06T10:29:55.0298145Z    |        |
2019-10-06T10:29:55.0298145Z    |        |
2019-10-06T10:29:55.0298323Z    |        expected bool, found &mut bool
2019-10-06T10:29:55.0298488Z    |        help: consider dereferencing the borrow: `*b_mut_ref()`
2019-10-06T10:29:55.0298822Z    = note: expected type `bool`
2019-10-06T10:29:55.0298983Z               found type `&mut bool`
2019-10-06T10:29:55.0299118Z 
2019-10-06T10:29:55.0299296Z error[E0308]: mismatched types
2019-10-06T10:29:55.0299296Z error[E0308]: mismatched types
2019-10-06T10:29:55.0299717Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:20:8
2019-10-06T10:29:55.0299918Z    |
2019-10-06T10:29:55.0300112Z LL |     if &true {} //~ ERROR mismatched types [E0308]
2019-10-06T10:29:55.0300430Z    |        |
2019-10-06T10:29:55.0300430Z    |        |
2019-10-06T10:29:55.0300607Z    |        expected bool, found &bool
2019-10-06T10:29:55.0300781Z    |        help: consider dereferencing the borrow: `*&true`
2019-10-06T10:29:55.0301113Z    = note: expected type `bool`
2019-10-06T10:29:55.0301271Z               found type `&bool`
2019-10-06T10:29:55.0301404Z 
2019-10-06T10:29:55.0301602Z error[E0308]: mismatched types
2019-10-06T10:29:55.0301602Z error[E0308]: mismatched types
2019-10-06T10:29:55.0302335Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:21:8
2019-10-06T10:29:55.0302534Z    |
2019-10-06T10:29:55.0302679Z LL |     if &mut true {} //~ ERROR mismatched types [E0308]
2019-10-06T10:29:55.0302976Z    |        |
2019-10-06T10:29:55.0302976Z    |        |
2019-10-06T10:29:55.0303117Z    |        expected bool, found &mut bool
2019-10-06T10:29:55.0303271Z    |        help: consider dereferencing the borrow: `*&mut true`
2019-10-06T10:29:55.0303562Z    = note: expected type `bool`
2019-10-06T10:29:55.0303699Z               found type `&mut bool`
2019-10-06T10:29:55.0303835Z 
2019-10-06T10:29:55.0304160Z error[E0308]: mismatched types
2019-10-06T10:29:55.0304160Z error[E0308]: mismatched types
2019-10-06T10:29:55.0304561Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:24:11
2019-10-06T10:29:55.0309268Z    |
2019-10-06T10:29:55.0309332Z LL |     while b_ref() {} //~ ERROR mismatched types [E0308]
2019-10-06T10:29:55.0309443Z    |           |
2019-10-06T10:29:55.0309443Z    |           |
2019-10-06T10:29:55.0309490Z    |           expected bool, found &bool
2019-10-06T10:29:55.0309539Z    |           help: consider dereferencing the borrow: `*b_ref()`
2019-10-06T10:29:55.0309645Z    = note: expected type `bool`
2019-10-06T10:29:55.0309689Z               found type `&bool`
2019-10-06T10:29:55.0309719Z 
2019-10-06T10:29:55.0309780Z error[E0308]: mismatched types
2019-10-06T10:29:55.0309780Z error[E0308]: mismatched types
2019-10-06T10:29:55.0310325Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:25:11
2019-10-06T10:29:55.0310390Z    |
2019-10-06T10:29:55.0310437Z LL |     while b_mut_ref() {} //~ ERROR mismatched types [E0308]
2019-10-06T10:29:55.0310557Z    |           |
2019-10-06T10:29:55.0310557Z    |           |
2019-10-06T10:29:55.0310603Z    |           expected bool, found &mut bool
2019-10-06T10:29:55.0310672Z    |           help: consider dereferencing the borrow: `*b_mut_ref()`
2019-10-06T10:29:55.0310760Z    = note: expected type `bool`
2019-10-06T10:29:55.0310823Z               found type `&mut bool`
2019-10-06T10:29:55.0310856Z 
2019-10-06T10:29:55.0310898Z error[E0308]: mismatched types
2019-10-06T10:29:55.0310898Z error[E0308]: mismatched types
2019-10-06T10:29:55.0311186Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:26:11
2019-10-06T10:29:55.0311254Z    |
2019-10-06T10:29:55.0311301Z LL |     while &true {} //~ ERROR mismatched types [E0308]
2019-10-06T10:29:55.0311408Z    |           |
2019-10-06T10:29:55.0311408Z    |           |
2019-10-06T10:29:55.0311461Z    |           expected bool, found &bool
2019-10-06T10:29:55.0311511Z    |           help: consider dereferencing the borrow: `*&true`
2019-10-06T10:29:55.0311614Z    = note: expected type `bool`
2019-10-06T10:29:55.0311666Z               found type `&bool`
2019-10-06T10:29:55.0311694Z 
2019-10-06T10:29:55.0311752Z error[E0308]: mismatched types
2019-10-06T10:29:55.0311752Z error[E0308]: mismatched types
2019-10-06T10:29:55.0312002Z   --> /checkout/src/test/ui/if/if-no-match-bindings.rs:27:11
2019-10-06T10:29:55.0312050Z    |
2019-10-06T10:29:55.0312113Z LL |     while &mut true {} //~ ERROR mismatched types [E0308]
2019-10-06T10:29:55.0312203Z    |           |
2019-10-06T10:29:55.0312203Z    |           |
2019-10-06T10:29:55.0312248Z    |           expected bool, found &mut bool
2019-10-06T10:29:55.0312316Z    |           help: consider dereferencing the borrow: `*&mut true`
2019-10-06T10:29:55.0312403Z    = note: expected type `bool`
2019-10-06T10:29:55.0312473Z               found type `&mut bool`
2019-10-06T10:29:55.0312503Z 
2019-10-06T10:29:55.0312546Z error: aborting due to 8 previous errors
---
2019-10-06T10:29:55.0313468Z 
2019-10-06T10:29:55.0313700Z ---- [ui] ui/rfc-2497-if-let-chains/disallowed-positions.rs stdout ----
2019-10-06T10:29:55.0313743Z diff of stderr:
2019-10-06T10:29:55.0313769Z 
2019-10-06T10:29:55.0313828Z 513 LL | #![feature(let_chains)] // Avoid inflating `.stderr` with overzealous gates in this test.
2019-10-06T10:29:55.0313921Z 515 
2019-10-06T10:29:55.0314146Z - warning: unnecessary parentheses around `if` condition
2019-10-06T10:29:55.0314341Z -   --> $DIR/disallowed-positions.rs:51:8
2019-10-06T10:29:55.0314502Z -    |
2019-10-06T10:29:55.0314502Z -    |
2019-10-06T10:29:55.0314708Z - LL |     if (true || let 0 = 0) {}
2019-10-06T10:29:55.0314926Z -    |        ^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-06T10:29:55.0315287Z -    = note: `#[warn(unused_parens)]` on by default
2019-10-06T10:29:55.0315598Z - 
2019-10-06T10:29:55.0315805Z - warning: unnecessary parentheses around `while` condition
2019-10-06T10:29:55.0316001Z -   --> $DIR/disallowed-positions.rs:115:11
2019-10-06T10:29:55.0316001Z -   --> $DIR/disallowed-positions.rs:115:11
2019-10-06T10:29:55.0316182Z -    |
2019-10-06T10:29:55.0316738Z - LL |     while (true || let 0 = 0) {}
2019-10-06T10:29:55.0317075Z -    |           ^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-06T10:29:55.0317280Z - 
2019-10-06T10:29:55.0317519Z - wrning: unnecessary parentheses around `let` head expression
2019-10-06T10:29:55.0317925Z -    |
2019-10-06T10:29:55.0317925Z -    |
2019-10-06T10:29:55.0318191Z - LL |     if let Range { start: _, end: _ } = (true..true || false) { }
2019-10-06T10:29:55.0318619Z -    |                                         ^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-06T10:29:55.0319143Z - 
2019-10-06T10:29:55.0319454Z - warning: unnecessary parentheses around `let` head expression
2019-10-06T10:29:55.0320033Z -    |
2019-10-06T10:29:55.0320033Z -    |
2019-10-06T10:29:55.0320397Z - LL |     if let Range { start: _, end: _ } = (true..true && false) { }
2019-10-06T10:29:55.0320776Z -    |                                         ^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-06T10:29:55.0320959Z - 
2019-10-06T10:29:55.0321207Z - warning: unnecessary parentheses around `let` head expression
2019-10-06T10:29:55.0321615Z -    |
2019-10-06T10:29:55.0321615Z -    |
2019-10-06T10:29:55.0321874Z - LL |     while let Range { start: _, end: _ } = (true..true || false) { }
2019-10-06T10:29:55.0322162Z -    |                                            ^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-06T10:29:55.0322348Z - 
2019-10-06T10:29:55.0322596Z - warning: unnecessary parentheses around `let` head expression
2019-10-06T10:29:55.0323018Z -    |
2019-10-06T10:29:55.0323018Z -    |
2019-10-06T10:29:55.0323265Z - LL |     while let Range { start: _, end: _ } = (true..true && false) { }
2019-10-06T10:29:55.0323563Z -    |                                            ^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-06T10:29:55.0323793Z 554 error[E0308]: mismatched types
2019-10-06T10:29:55.0324034Z 555   --> $DIR/disallowed-positions.rs:32:8
2019-10-06T10:29:55.0324081Z 556    |
2019-10-06T10:29:55.0324110Z 
2019-10-06T10:29:55.0324110Z 
2019-10-06T10:29:55.0324242Z 
2019-10-06T10:29:55.0324303Z The actual stderr differed from the expected stderr.
2019-10-06T10:29:55.0324618Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/disallowed-positions.stderr
2019-10-06T10:29:55.0324867Z To update references, rerun the tests and pass the `--bless` flag
2019-10-06T10:29:55.0325169Z To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/disallowed-positions.rs`
2019-10-06T10:29:55.0325256Z error: 1 errors occurred comparing output.
2019-10-06T10:29:55.0325315Z status: exit code: 1
2019-10-06T10:29:55.0325315Z status: exit code: 1
2019-10-06T10:29:55.0326071Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/auxiliary" "-A" "unused"
2019-10-06T10:29:55.0326804Z ------------------------------------------
2019-10-06T10:29:55.0326851Z 
2019-10-06T10:29:55.0327161Z ------------------------------------------
2019-10-06T10:29:55.0327383Z stderr:
2019-10-06T10:29:55.0327383Z stderr:
2019-10-06T10:29:55.0327639Z ------------------------------------------
2019-10-06T10:29:55.0327713Z error: expected one of `,` or `>`, found `&&`
2019-10-06T10:29:55.0328034Z    |
2019-10-06T10:29:55.0328034Z    |
2019-10-06T10:29:55.0328102Z LL |         true && let 1 = 1 //~ ERROR expected one of `,` or `>`, found `&&`
2019-10-06T10:29:55.0328155Z    |              ^^ expected one of `,` or `>` here
2019-10-06T10:29:55.0328232Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0328512Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:32:9
2019-10-06T10:29:55.0328565Z    |
2019-10-06T10:29:55.0328565Z    |
2019-10-06T10:29:55.0328718Z LL |     if &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0328842Z    |
2019-10-06T10:29:55.0328842Z    |
2019-10-06T10:29:55.0329149Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0329237Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0329319Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0330049Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:35:9
2019-10-06T10:29:55.0330133Z    |
2019-10-06T10:29:55.0330133Z    |
2019-10-06T10:29:55.0330176Z LL |     if !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0330270Z    |
2019-10-06T10:29:55.0330270Z    |
2019-10-06T10:29:55.0330500Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0330564Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0330649Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0330878Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:36:9
2019-10-06T10:29:55.0330929Z    |
2019-10-06T10:29:55.0330929Z    |
2019-10-06T10:29:55.0330989Z LL |     if *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0331070Z    |
2019-10-06T10:29:55.0331070Z    |
2019-10-06T10:29:55.0331561Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0331617Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0331693Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0332084Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:38:9
2019-10-06T10:29:55.0332134Z    |
2019-10-06T10:29:55.0332134Z    |
2019-10-06T10:29:55.0332414Z LL |     if -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0332532Z    |
2019-10-06T10:29:55.0332532Z    |
2019-10-06T10:29:55.0332806Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0332890Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0332972Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0333256Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:46:9
2019-10-06T10:29:55.0333329Z    |
2019-10-06T10:29:55.0333329Z    |
2019-10-06T10:29:55.0333380Z LL |     if (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0333491Z    |
2019-10-06T10:29:55.0333491Z    |
2019-10-06T10:29:55.0333768Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0333828Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0333933Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0334276Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:50:16
2019-10-06T10:29:55.0334346Z    |
2019-10-06T10:29:55.0334346Z    |
2019-10-06T10:29:55.0334543Z LL |     if true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0334637Z    |
2019-10-06T10:29:55.0334637Z    |
2019-10-06T10:29:55.0335274Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0335326Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0335410Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0335645Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:51:17
2019-10-06T10:29:55.0335689Z    |
2019-10-06T10:29:55.0335689Z    |
2019-10-06T10:29:55.0335749Z LL |     if (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0335919Z    |
2019-10-06T10:29:55.0335919Z    |
2019-10-06T10:29:55.0360910Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0361068Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0361167Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0361503Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:52:25
2019-10-06T10:29:55.0361555Z    |
2019-10-06T10:29:55.0361555Z    |
2019-10-06T10:29:55.0361604Z LL |     if true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0361712Z    |
2019-10-06T10:29:55.0361712Z    |
2019-10-06T10:29:55.0361956Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0362029Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0362113Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0362360Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:53:25
2019-10-06T10:29:55.0362424Z    |
2019-10-06T10:29:55.0362424Z    |
2019-10-06T10:29:55.0362480Z LL |     if true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0362583Z    |
2019-10-06T10:29:55.0362583Z    |
2019-10-06T10:29:55.0362823Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0362877Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0362965Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0363207Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:56:12
2019-10-06T10:29:55.0363252Z    |
2019-10-06T10:29:55.0363252Z    |
2019-10-06T10:29:55.0363322Z LL |     if x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0363406Z    |
2019-10-06T10:29:55.0363406Z    |
2019-10-06T10:29:55.0363763Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0363822Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0363888Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0364135Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:59:15
2019-10-06T10:29:55.0364177Z    |
2019-10-06T10:29:55.0364177Z    |
2019-10-06T10:29:55.0364220Z LL |     if true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0364315Z    |
2019-10-06T10:29:55.0364315Z    |
2019-10-06T10:29:55.0364537Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0364722Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0364916Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0365186Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:61:11
2019-10-06T10:29:55.0365347Z    |
2019-10-06T10:29:55.0365347Z    |
2019-10-06T10:29:55.0365610Z LL |     if ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0365724Z    |
2019-10-06T10:29:55.0365724Z    |
2019-10-06T10:29:55.0366027Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0366084Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0366180Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0366716Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:63:9
2019-10-06T10:29:55.0366843Z    |
2019-10-06T10:29:55.0366843Z    |
2019-10-06T10:29:55.0366898Z LL |     if (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0367169Z    |
2019-10-06T10:29:55.0367169Z    |
2019-10-06T10:29:55.0367559Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0367621Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0367734Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0368023Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:67:8
2019-10-06T10:29:55.0368074Z    |
2019-10-06T10:29:55.0368074Z    |
2019-10-06T10:29:55.0368140Z LL |     if let Range { start: _, end: _ } = true..true && false {}
2019-10-06T10:29:55.0368241Z    |
2019-10-06T10:29:55.0368241Z    |
2019-10-06T10:29:55.0368518Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0368597Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0368687Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0368989Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:71:8
2019-10-06T10:29:55.0369042Z    |
2019-10-06T10:29:55.0369042Z    |
2019-10-06T10:29:55.0369091Z LL |     if let Range { start: _, end: _ } = true..true || false {}
2019-10-06T10:29:55.0369216Z    |
2019-10-06T10:29:55.0369216Z    |
2019-10-06T10:29:55.0369491Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0369566Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0369648Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0370241Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:78:8
2019-10-06T10:29:55.0370302Z    |
2019-10-06T10:29:55.0370302Z    |
2019-10-06T10:29:55.0370343Z LL |     if let Range { start: F, end } = F..|| true {}
2019-10-06T10:29:55.0370451Z    |
2019-10-06T10:29:55.0370451Z    |
2019-10-06T10:29:55.0370678Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0370729Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0370819Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0371048Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:86:8
2019-10-06T10:29:55.0371090Z    |
2019-10-06T10:29:55.0371090Z    |
2019-10-06T10:29:55.0371147Z LL |     if let Range { start: true, end } = t..&&false {}
2019-10-06T10:29:55.0371226Z    |
2019-10-06T10:29:55.0371226Z    |
2019-10-06T10:29:55.0371466Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0371517Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0371590Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0371837Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:92:19
2019-10-06T10:29:55.0371880Z    |
2019-10-06T10:29:55.0371924Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0371924Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0372088Z    |                   ^^^^^^^^^^^^^^^
2019-10-06T10:29:55.0372125Z    |
2019-10-06T10:29:55.0372374Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0372440Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0372507Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0372732Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:96:12
2019-10-06T10:29:55.0372793Z    |
2019-10-06T10:29:55.0372793Z    |
2019-10-06T10:29:55.0372835Z LL |     while &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0373002Z    |
2019-10-06T10:29:55.0373002Z    |
2019-10-06T10:29:55.0373254Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0373304Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0373397Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0373629Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:99:12
2019-10-06T10:29:55.0373689Z    |
2019-10-06T10:29:55.0373689Z    |
2019-10-06T10:29:55.0373732Z LL |     while !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0373809Z    |
2019-10-06T10:29:55.0373809Z    |
2019-10-06T10:29:55.0374049Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0374099Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0374189Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0374419Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:100:12
2019-10-06T10:29:55.0374461Z    |
2019-10-06T10:29:55.0374461Z    |
2019-10-06T10:29:55.0374518Z LL |     while *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0374602Z    |
2019-10-06T10:29:55.0374602Z    |
2019-10-06T10:29:55.0374823Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0374888Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0374954Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0375197Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:102:12
2019-10-06T10:29:55.0375241Z    |
2019-10-06T10:29:55.0375241Z    |
2019-10-06T10:29:55.0375460Z LL |     while -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0375558Z    |
2019-10-06T10:29:55.0375558Z    |
2019-10-06T10:29:55.0376025Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0376080Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0376856Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0377213Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:110:12
2019-10-06T10:29:55.0377286Z    |
2019-10-06T10:29:55.0377286Z    |
2019-10-06T10:29:55.0377336Z LL |     while (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0377446Z    |
2019-10-06T10:29:55.0377446Z    |
2019-10-06T10:29:55.0377699Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0377756Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0377852Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0378120Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:114:19
2019-10-06T10:29:55.0378168Z    |
2019-10-06T10:29:55.0378168Z    |
2019-10-06T10:29:55.0378234Z LL |     while true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0378469Z    |
2019-10-06T10:29:55.0378469Z    |
2019-10-06T10:29:55.0378775Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0378834Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0378910Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0379189Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:115:20
2019-10-06T10:29:55.0379238Z    |
2019-10-06T10:29:55.0379238Z    |
2019-10-06T10:29:55.0379288Z LL |     while (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0379399Z    |
2019-10-06T10:29:55.0379399Z    |
2019-10-06T10:29:55.0379844Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0380050Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0380233Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0380486Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:116:28
2019-10-06T10:29:55.0380546Z    |
2019-10-06T10:29:55.0380546Z    |
2019-10-06T10:29:55.0380591Z LL |     while true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0380690Z    |
2019-10-06T10:29:55.0380690Z    |
2019-10-06T10:29:55.0380913Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0380963Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0381052Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0381282Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:117:28
2019-10-06T10:29:55.0381340Z    |
2019-10-06T10:29:55.0381340Z    |
2019-10-06T10:29:55.0381385Z LL |     while true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0381473Z    |
2019-10-06T10:29:55.0381473Z    |
2019-10-06T10:29:55.0381712Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0381762Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0381844Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0382072Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:120:15
2019-10-06T10:29:55.0382115Z    |
2019-10-06T10:29:55.0382115Z    |
2019-10-06T10:29:55.0382173Z LL |     while x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0382260Z    |
2019-10-06T10:29:55.0382260Z    |
2019-10-06T10:29:55.0382482Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0382549Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0382622Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0382869Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:123:18
2019-10-06T10:29:55.0382912Z    |
2019-10-06T10:29:55.0382912Z    |
2019-10-06T10:29:55.0382955Z LL |     while true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0383052Z    |
2019-10-06T10:29:55.0383052Z    |
2019-10-06T10:29:55.0383273Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0383338Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0383411Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0383637Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:125:14
2019-10-06T10:29:55.0383696Z    |
2019-10-06T10:29:55.0383696Z    |
2019-10-06T10:29:55.0383738Z LL |     while ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0383916Z    |
2019-10-06T10:29:55.0383916Z    |
2019-10-06T10:29:55.0384163Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0384213Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0384297Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0384527Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:127:12
2019-10-06T10:29:55.0384569Z    |
2019-10-06T10:29:55.0384569Z    |
2019-10-06T10:29:55.0384629Z LL |     while (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0384780Z    |
2019-10-06T10:29:55.0384780Z    |
2019-10-06T10:29:55.0385050Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0385100Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0385174Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0385420Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:131:11
2019-10-06T10:29:55.0385462Z    |
2019-10-06T10:29:55.0385462Z    |
2019-10-06T10:29:55.0385503Z LL |     while let Range { start: _, end: _ } = true..true && false {}
2019-10-06T10:29:55.0385604Z    |
2019-10-06T10:29:55.0385604Z    |
2019-10-06T10:29:55.0385826Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0385892Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0385965Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0386705Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:135:11
2019-10-06T10:29:55.0386788Z    |
2019-10-06T10:29:55.0386788Z    |
2019-10-06T10:29:55.0386835Z LL |     while let Range { start: _, end: _ } = true..true || false {}
2019-10-06T10:29:55.0386959Z    |
2019-10-06T10:29:55.0386959Z    |
2019-10-06T10:29:55.0387266Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0387323Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0387418Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0387680Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:142:11
2019-10-06T10:29:55.0387745Z    |
2019-10-06T10:29:55.0387745Z    |
2019-10-06T10:29:55.0387793Z LL |     while let Range { start: F, end } = F..|| true {}
2019-10-06T10:29:55.0387894Z    |
2019-10-06T10:29:55.0387894Z    |
2019-10-06T10:29:55.0388166Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0388222Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0388323Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0388584Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:150:11
2019-10-06T10:29:55.0388633Z    |
2019-10-06T10:29:55.0388633Z    |
2019-10-06T10:29:55.0388696Z LL |     while let Range { start: true, end } = t..&&false {}
2019-10-06T10:29:55.0388791Z    |
2019-10-06T10:29:55.0388791Z    |
2019-10-06T10:29:55.0389043Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0389118Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0389203Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0389482Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:156:22
2019-10-06T10:29:55.0389532Z    |
2019-10-06T10:29:55.0389582Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0389582Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0389793Z    |                      ^^^^^^^^^^^^^^^
2019-10-06T10:29:55.0389836Z    |
2019-10-06T10:29:55.0390121Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0390196Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0390274Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0390860Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:170:6
2019-10-06T10:29:55.0390919Z    |
2019-10-06T10:29:55.0390919Z    |
2019-10-06T10:29:55.0390961Z LL |     &let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0391156Z    |
2019-10-06T10:29:55.0391156Z    |
2019-10-06T10:29:55.0391414Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0391464Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0394198Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0394633Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:172:6
2019-10-06T10:29:55.0394680Z    |
2019-10-06T10:29:55.0394680Z    |
2019-10-06T10:29:55.0394748Z LL |     !let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0394825Z    |
2019-10-06T10:29:55.0394825Z    |
2019-10-06T10:29:55.0395072Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0395124Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0395189Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0395450Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:173:6
2019-10-06T10:29:55.0395492Z    |
2019-10-06T10:29:55.0395492Z    |
2019-10-06T10:29:55.0395533Z LL |     *let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0395634Z    |
2019-10-06T10:29:55.0395634Z    |
2019-10-06T10:29:55.0395858Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0395924Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0395991Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0399253Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:175:6
2019-10-06T10:29:55.0399364Z    |
2019-10-06T10:29:55.0399364Z    |
2019-10-06T10:29:55.0399711Z LL |     -let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0399824Z    |
2019-10-06T10:29:55.0399824Z    |
2019-10-06T10:29:55.0400098Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0400155Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0400248Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0400522Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:183:6
2019-10-06T10:29:55.0400571Z    |
2019-10-06T10:29:55.0400571Z    |
2019-10-06T10:29:55.0400635Z LL |     (let 0 = 0)?; //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0400722Z    |
2019-10-06T10:29:55.0400722Z    |
2019-10-06T10:29:55.0400994Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0401051Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0401252Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0401634Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:187:13
2019-10-06T10:29:55.0401680Z    |
2019-10-06T10:29:55.0401680Z    |
2019-10-06T10:29:55.0401724Z LL |     true || let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0402012Z    |
2019-10-06T10:29:55.0402012Z    |
2019-10-06T10:29:55.0402393Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0402462Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0402529Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0402894Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:188:14
2019-10-06T10:29:55.0402941Z    |
2019-10-06T10:29:55.0402941Z    |
2019-10-06T10:29:55.0402986Z LL |     (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0403088Z    |
2019-10-06T10:29:55.0403088Z    |
2019-10-06T10:29:55.0403418Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0403479Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0403570Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0403963Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:189:22
2019-10-06T10:29:55.0404027Z    |
2019-10-06T10:29:55.0404027Z    |
2019-10-06T10:29:55.0404076Z LL |     true && (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0404243Z    |
2019-10-06T10:29:55.0404243Z    |
2019-10-06T10:29:55.0404488Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0404542Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0404633Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0404892Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:192:9
2019-10-06T10:29:55.0404939Z    |
2019-10-06T10:29:55.0404939Z    |
2019-10-06T10:29:55.0405006Z LL |     x = let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0405093Z    |
2019-10-06T10:29:55.0405093Z    |
2019-10-06T10:29:55.0405364Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0405419Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0405493Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0405763Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:194:12
2019-10-06T10:29:55.0405809Z    |
2019-10-06T10:29:55.0405809Z    |
2019-10-06T10:29:55.0405855Z LL |     true..(let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0406375Z    |
2019-10-06T10:29:55.0406375Z    |
2019-10-06T10:29:55.0406698Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0406790Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0406868Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0407142Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:195:8
2019-10-06T10:29:55.0407210Z    |
2019-10-06T10:29:55.0407210Z    |
2019-10-06T10:29:55.0407258Z LL |     ..(let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0407364Z    |
2019-10-06T10:29:55.0407364Z    |
2019-10-06T10:29:55.0407934Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-06T10:29:55.0408000Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-06T10:29:55.0408102Z error: `let` expressions are not supported here
2019-10-06T10:29:55.0409267Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:196:6
2019-10-06T10:29:55.0409342Z    |
2019-10-06T10:29:55.0409342Z    |
2019-10-06T10:29:55.0409434Z LL |     (let 0 = 0)..; //~ ERROR `let` expressions are not supported here
2019-10-06T10:29:55.0409522Z    |
2019-10-06T10:29:55.0409522Z    |
2019-10-06T10:29:55.0409912Z    = note: only supported directly in conditions of `if`- and `while`-expressions
---
2019-10-06T10:29:55.0489902Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-06T10:29:55.0490217Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-06T10:29:55.0490285Z 
2019-10-06T10:29:55.0490313Z 
2019-10-06T10:29:55.0492282Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-06T10:29:55.0492618Z 
2019-10-06T10:29:55.0492649Z 
2019-10-06T10:29:55.0492719Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-06T10:29:55.0492770Z Build completed unsuccessfully in 1:09:17
2019-10-06T10:29:55.0492770Z Build completed unsuccessfully in 1:09:17
2019-10-06T10:29:55.0493164Z == clock drift check ==
2019-10-06T10:29:55.0493325Z   local time: Sun Oct  6 10:29:55 UTC 2019
2019-10-06T10:29:55.2083775Z   network time: Sun, 06 Oct 2019 10:29:55 GMT
2019-10-06T10:29:55.2083951Z == end clock drift check ==
2019-10-06T10:29:56.6525278Z ##[error]Bash exited with code '1'.
2019-10-06T10:29:56.6597573Z ##[section]Starting: Checkout
2019-10-06T10:29:56.6600150Z ==============================================================================
2019-10-06T10:29:56.6600256Z Task         : Get sources
2019-10-06T10:29:56.6600310Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
