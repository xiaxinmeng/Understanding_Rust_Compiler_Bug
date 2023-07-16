plain
2019-10-28T13:17:07.9566449Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-28T13:17:07.9765320Z ##[command]git config gc.auto 0
2019-10-28T13:17:07.9841967Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-28T13:17:07.9890565Z ##[command]git config --get-all http.proxy
2019-10-28T13:17:08.0030780Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65893/merge:refs/remotes/pull/65893/merge
---
2019-10-28T14:12:27.2256979Z .................................................................................................... 1600/9259
2019-10-28T14:12:32.5222190Z .................................................................................................... 1700/9259
2019-10-28T14:12:43.7074747Z ..........................................................i...............i......................... 1800/9259
2019-10-28T14:12:50.5957389Z .................................................................................................... 1900/9259
2019-10-28T14:13:03.9695471Z ................................................iiiii............................................... 2000/9259
2019-10-28T14:13:14.3505026Z .................................................................................................... 2200/9259
2019-10-28T14:13:16.8699359Z .................................................................................................... 2300/9259
2019-10-28T14:13:20.3964386Z .................................................................................................... 2400/9259
2019-10-28T14:13:41.6745580Z .................................................................................................... 2500/9259
---
2019-10-28T14:16:20.5893492Z .................................................i...............i.................................. 4800/9259
2019-10-28T14:16:28.7192576Z .................................................................................................... 4900/9259
2019-10-28T14:16:36.4782226Z .................................................................................................... 5000/9259
2019-10-28T14:16:42.2140561Z .................................................................................................... 5100/9259
2019-10-28T14:16:51.6973783Z ..................................................ii.ii...........i................................. 5200/9259
2019-10-28T14:17:00.5650717Z .................................................................................................... 5400/9259
2019-10-28T14:17:09.1064863Z .................................................................................................... 5500/9259
2019-10-28T14:17:16.5463902Z ....................i............................................................................... 5600/9259
2019-10-28T14:17:22.1566145Z .................................................................................................... 5700/9259
2019-10-28T14:17:22.1566145Z .................................................................................................... 5700/9259
2019-10-28T14:17:32.8976074Z .................................................................................................... 5800/9259
2019-10-28T14:17:44.2780898Z .....ii...i..ii............i........................................................................ 5900/9259
2019-10-28T14:18:04.2664719Z .................................................................................................... 6100/9259
2019-10-28T14:18:09.5374305Z .................................................................................................... 6200/9259
2019-10-28T14:18:09.5374305Z .................................................................................................... 6200/9259
2019-10-28T14:18:22.4797617Z ........................i..ii....................................................................... 6300/9259
2019-10-28T14:18:40.8604165Z ..........................................................................................i......... 6500/9259
2019-10-28T14:18:42.9221102Z .................................................................................................... 6600/9259
2019-10-28T14:18:45.0126923Z .................................................................i.................................. 6700/9259
2019-10-28T14:18:47.7939068Z .................................................................................................... 6800/9259
---
2019-10-28T14:22:34.4594269Z 
2019-10-28T14:22:34.4595118Z ---- [ui] ui/rfc-2497-if-let-chains/feature-gate.rs stdout ----
2019-10-28T14:22:34.4595406Z diff of stderr:
2019-10-28T14:22:34.4595457Z 
2019-10-28T14:22:34.4595856Z 295    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4595913Z 296    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4596355Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4596433Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4596661Z 299   --> $DIR/feature-gate.rs:14:9
2019-10-28T14:22:34.4596701Z 300    |
2019-10-28T14:22:34.4596701Z 300    |
2019-10-28T14:22:34.4596760Z 301 LL |     if (let 0 = 1) {}
2019-10-28T14:22:34.4596785Z 
2019-10-28T14:22:34.4597010Z 304    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4597080Z 305    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4597763Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4597815Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4598068Z 308   --> $DIR/feature-gate.rs:18:11
2019-10-28T14:22:34.4598117Z 309    |
2019-10-28T14:22:34.4598117Z 309    |
2019-10-28T14:22:34.4598160Z 310 LL |     if (((let 0 = 1))) {}
2019-10-28T14:22:34.4598212Z 
2019-10-28T14:22:34.4598475Z 313    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4598534Z 314    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4598815Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4598865Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4599071Z 317   --> $DIR/feature-gate.rs:22:16
2019-10-28T14:22:34.4599134Z 318    |
2019-10-28T14:22:34.4599134Z 318    |
2019-10-28T14:22:34.4599177Z 319 LL |     if true && let 0 = 1 {}
2019-10-28T14:22:34.4599205Z 
2019-10-28T14:22:34.4599475Z 322    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4599533Z 323    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4599823Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4599873Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4600088Z 326   --> $DIR/feature-gate.rs:26:8
2019-10-28T14:22:34.4600134Z 327    |
2019-10-28T14:22:34.4600134Z 327    |
2019-10-28T14:22:34.4600195Z 328 LL |     if let 0 = 1 && true {}
2019-10-28T14:22:34.4600224Z 
2019-10-28T14:22:34.4600474Z 331    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4600548Z 332    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4600971Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4601011Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4601192Z 335   --> $DIR/feature-gate.rs:30:9
2019-10-28T14:22:34.4601230Z 336    |
2019-10-28T14:22:34.4601230Z 336    |
2019-10-28T14:22:34.4601264Z 337 LL |     if (let 0 = 1) && true {}
2019-10-28T14:22:34.4601303Z 
2019-10-28T14:22:34.4601669Z 340    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4601715Z 341    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4601948Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4601988Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4602151Z 344   --> $DIR/feature-gate.rs:34:17
2019-10-28T14:22:34.4602203Z 345    |
2019-10-28T14:22:34.4602203Z 345    |
2019-10-28T14:22:34.4602237Z 346 LL |     if true && (let 0 = 1) {}
2019-10-28T14:22:34.4602260Z 
2019-10-28T14:22:34.4602476Z 349    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4602522Z 350    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4602743Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4602783Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4602956Z 353   --> $DIR/feature-gate.rs:38:9
2019-10-28T14:22:34.4602992Z 354    |
2019-10-28T14:22:34.4602992Z 354    |
2019-10-28T14:22:34.4603044Z 355 LL |     if (let 0 = 1) && (let 0 = 1) {}
2019-10-28T14:22:34.4603067Z 
2019-10-28T14:22:34.4603349Z 358    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4603418Z 359    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4603651Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4603708Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4603874Z 362   --> $DIR/feature-gate.rs:38:24
2019-10-28T14:22:34.4603911Z 363    |
2019-10-28T14:22:34.4603911Z 363    |
2019-10-28T14:22:34.4603946Z 364 LL |     if (let 0 = 1) && (let 0 = 1) {}
2019-10-28T14:22:34.4603986Z 
2019-10-28T14:22:34.4604186Z 367    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4604231Z 368    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4604462Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4604501Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4604669Z 371   --> $DIR/feature-gate.rs:44:8
2019-10-28T14:22:34.4604725Z 372    |
2019-10-28T14:22:34.4604725Z 372    |
2019-10-28T14:22:34.4604763Z 373 LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4604788Z 
2019-10-28T14:22:34.4605009Z 376    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4605057Z 377    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4605284Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4605323Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4605487Z 380   --> $DIR/feature-gate.rs:44:21
2019-10-28T14:22:34.4605543Z 381    |
2019-10-28T14:22:34.4605543Z 381    |
2019-10-28T14:22:34.4605581Z 382 LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4605613Z 
2019-10-28T14:22:34.4605820Z 385    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4605892Z 386    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4606100Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4606163Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4606328Z 389   --> $DIR/feature-gate.rs:44:35
2019-10-28T14:22:34.4606363Z 390    |
2019-10-28T14:22:34.4606363Z 390    |
2019-10-28T14:22:34.4606421Z 391 LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4606448Z 
2019-10-28T14:22:34.4606645Z 394    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4606689Z 395    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4607008Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4607376Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4607748Z 398   --> $DIR/feature-gate.rs:44:48
2019-10-28T14:22:34.4607803Z 399    |
2019-10-28T14:22:34.4607803Z 399    |
2019-10-28T14:22:34.4607853Z 400 LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4607885Z 
2019-10-28T14:22:34.4608159Z 403    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4608216Z 404    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4608494Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4608543Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4608748Z 407   --> $DIR/feature-gate.rs:44:61
2019-10-28T14:22:34.4608812Z 408    |
2019-10-28T14:22:34.4608812Z 408    |
2019-10-28T14:22:34.4608860Z 409 LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4608900Z 
2019-10-28T14:22:34.4609153Z 412    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4609324Z 413    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4609619Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4609687Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4609894Z 416   --> $DIR/feature-gate.rs:56:8
2019-10-28T14:22:34.4609939Z 417    |
2019-10-28T14:22:34.4609939Z 417    |
2019-10-28T14:22:34.4610002Z 418 LL |     if let Range { start: _, end: _ } = (true..true) && false {}
2019-10-28T14:22:34.4610033Z 
2019-10-28T14:22:34.4610284Z 421    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4610359Z 422    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4610619Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4610844Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4611140Z 425   --> $DIR/feature-gate.rs:64:12
2019-10-28T14:22:34.4611176Z 426    |
2019-10-28T14:22:34.4611176Z 426    |
2019-10-28T14:22:34.4611215Z 427 LL |     while (let 0 = 1) {}
2019-10-28T14:22:34.4611255Z 
2019-10-28T14:22:34.4611454Z 430    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4611498Z 431    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4612166Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4612220Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4612427Z 434   --> $DIR/feature-gate.rs:68:14
2019-10-28T14:22:34.4612492Z 435    |
2019-10-28T14:22:34.4612492Z 435    |
2019-10-28T14:22:34.4612536Z 436 LL |     while (((let 0 = 1))) {}
2019-10-28T14:22:34.4612565Z 
2019-10-28T14:22:34.4612832Z 439    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4612901Z 440    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4613163Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4613240Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4613448Z 443   --> $DIR/feature-gate.rs:72:19
2019-10-28T14:22:34.4613493Z 444    |
2019-10-28T14:22:34.4613493Z 444    |
2019-10-28T14:22:34.4613551Z 445 LL |     while true && let 0 = 1 {}
2019-10-28T14:22:34.4613580Z 
2019-10-28T14:22:34.4613829Z 448    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4613902Z 449    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4614160Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4614209Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4614433Z 452   --> $DIR/feature-gate.rs:76:11
2019-10-28T14:22:34.4614479Z 453    |
2019-10-28T14:22:34.4614479Z 453    |
2019-10-28T14:22:34.4614625Z 454 LL |     while let 0 = 1 && true {}
2019-10-28T14:22:34.4614671Z 
2019-10-28T14:22:34.4614949Z 457    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4615015Z 458    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4615450Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4615494Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4615853Z 461   --> $DIR/feature-gate.rs:80:12
2019-10-28T14:22:34.4615907Z 462    |
2019-10-28T14:22:34.4615907Z 462    |
2019-10-28T14:22:34.4615943Z 463 LL |     while (let 0 = 1) && true {}
2019-10-28T14:22:34.4615967Z 
2019-10-28T14:22:34.4616345Z 466    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4616390Z 467    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4616595Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4616684Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4616848Z 470   --> $DIR/feature-gate.rs:84:20
2019-10-28T14:22:34.4616883Z 471    |
2019-10-28T14:22:34.4616883Z 471    |
2019-10-28T14:22:34.4617002Z 472 LL |     while true && (let 0 = 1) {}
2019-10-28T14:22:34.4617031Z 
2019-10-28T14:22:34.4617689Z 475    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4617772Z 476    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4618037Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4618087Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4618310Z 479   --> $DIR/feature-gate.rs:88:12
2019-10-28T14:22:34.4618355Z 480    |
2019-10-28T14:22:34.4618355Z 480    |
2019-10-28T14:22:34.4618400Z 481 LL |     while (let 0 = 1) && (let 0 = 1) {}
2019-10-28T14:22:34.4618446Z 
2019-10-28T14:22:34.4618696Z 484    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4618765Z 485    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4619049Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4619099Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4619305Z 488   --> $DIR/feature-gate.rs:88:27
2019-10-28T14:22:34.4619368Z 489    |
2019-10-28T14:22:34.4619368Z 489    |
2019-10-28T14:22:34.4619413Z 490 LL |     while (let 0 = 1) && (let 0 = 1) {}
2019-10-28T14:22:34.4619442Z 
2019-10-28T14:22:34.4619710Z 493    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4619767Z 494    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4620043Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4620093Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4620300Z 497   --> $DIR/feature-gate.rs:94:11
2019-10-28T14:22:34.4620353Z 498    |
2019-10-28T14:22:34.4620353Z 498    |
2019-10-28T14:22:34.4620419Z 499 LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4620452Z 
2019-10-28T14:22:34.4620877Z 502    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4620937Z 503    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4621146Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4621199Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4621363Z 506   --> $DIR/feature-gate.rs:94:24
2019-10-28T14:22:34.4622043Z 507    |
2019-10-28T14:22:34.4622043Z 507    |
2019-10-28T14:22:34.4622103Z 508 LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4622130Z 
2019-10-28T14:22:34.4622362Z 511    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4622407Z 512    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4622758Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4622805Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4622991Z 515   --> $DIR/feature-gate.rs:94:38
2019-10-28T14:22:34.4623026Z 516    |
2019-10-28T14:22:34.4623026Z 516    |
2019-10-28T14:22:34.4623064Z 517 LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4623090Z 
2019-10-28T14:22:34.4623307Z 520    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4623352Z 521    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4623570Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4623609Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4623772Z 524   --> $DIR/feature-gate.rs:94:51
2019-10-28T14:22:34.4623821Z 525    |
2019-10-28T14:22:34.4623821Z 525    |
2019-10-28T14:22:34.4623867Z 526 LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4623893Z 
2019-10-28T14:22:34.4624158Z 529    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4624226Z 530    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4624451Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4624505Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4624669Z 533   --> $DIR/feature-gate.rs:94:64
2019-10-28T14:22:34.4624703Z 534    |
2019-10-28T14:22:34.4624703Z 534    |
2019-10-28T14:22:34.4624755Z 535 LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4624781Z 
2019-10-28T14:22:34.4624979Z 538    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4625024Z 539    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4625256Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4625295Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4625479Z 542   --> $DIR/feature-gate.rs:106:11
2019-10-28T14:22:34.4625515Z 543    |
2019-10-28T14:22:34.4625515Z 543    |
2019-10-28T14:22:34.4625552Z 544 LL |     while let Range { start: _, end: _ } = (true..true) && false {}
2019-10-28T14:22:34.4625636Z 
2019-10-28T14:22:34.4625840Z 547    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4625885Z 548    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4626103Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4626141Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4626304Z 551   --> $DIR/feature-gate.rs:123:16
2019-10-28T14:22:34.4626355Z 552    |
2019-10-28T14:22:34.4626355Z 552    |
2019-10-28T14:22:34.4626390Z 553 LL |     use_expr!((let 0 = 1 && 0 == 0));
2019-10-28T14:22:34.4626420Z 
2019-10-28T14:22:34.4626638Z 556    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-28T14:22:34.4626690Z 557    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-28T14:22:34.4626911Z - error: `let` expressions are not supported here
2019-10-28T14:22:34.4626952Z + error: `let` expressions are not supported
2019-10-28T14:22:34.4627559Z 560   --> $DIR/feature-gate.rs:126:16
2019-10-28T14:22:34.4627608Z 561    |
2019-10-28T14:22:34.4627608Z 561    |
2019-10-28T14:22:34.4627666Z 562 LL |     use_expr!((let 0 = 1));
2019-10-28T14:22:34.4627721Z 
2019-10-28T14:22:34.4627766Z The actual stderr differed from the expected stderr.
2019-10-28T14:22:34.4628097Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/feature-gate/feature-gate.stderr
2019-10-28T14:22:34.4628097Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/feature-gate/feature-gate.stderr
2019-10-28T14:22:34.4628343Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T14:22:34.4628801Z To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/feature-gate.rs`
2019-10-28T14:22:34.4628889Z error: 1 errors occurred comparing output.
2019-10-28T14:22:34.4628945Z status: exit code: 1
2019-10-28T14:22:34.4628945Z status: exit code: 1
2019-10-28T14:22:34.4629774Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/feature-gate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/feature-gate/auxiliary" "-A" "unused"
2019-10-28T14:22:34.4630124Z ------------------------------------------
2019-10-28T14:22:34.4630169Z 
2019-10-28T14:22:34.4630399Z ------------------------------------------
2019-10-28T14:22:34.4630462Z stderr:
2019-10-28T14:22:34.4630462Z stderr:
2019-10-28T14:22:34.4630686Z ------------------------------------------
2019-10-28T14:22:34.4630965Z error: no rules expected the token `let`
2019-10-28T14:22:34.4631210Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:131:15
2019-10-28T14:22:34.4631251Z    |
2019-10-28T14:22:34.4631288Z LL |     macro_rules! use_expr {
2019-10-28T14:22:34.4632517Z    |     --------------------- when calling this macro
2019-10-28T14:22:34.4632561Z ...
2019-10-28T14:22:34.4632595Z LL |     use_expr!(let 0 = 1);
2019-10-28T14:22:34.4632634Z    |               ^^^ no rules expected this token in macro call
2019-10-28T14:22:34.4632713Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4632947Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:14:9
2019-10-28T14:22:34.4633000Z    |
2019-10-28T14:22:34.4633000Z    |
2019-10-28T14:22:34.4633042Z LL |     if (let 0 = 1) {}
2019-10-28T14:22:34.4633109Z    |
2019-10-28T14:22:34.4633109Z    |
2019-10-28T14:22:34.4633371Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4633425Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4633504Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4633714Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:18:11
2019-10-28T14:22:34.4633752Z    |
2019-10-28T14:22:34.4633752Z    |
2019-10-28T14:22:34.4633799Z LL |     if (((let 0 = 1))) {}
2019-10-28T14:22:34.4633866Z    |
2019-10-28T14:22:34.4633866Z    |
2019-10-28T14:22:34.4634098Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4634142Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4634219Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4634430Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:22:16
2019-10-28T14:22:34.4634468Z    |
2019-10-28T14:22:34.4634468Z    |
2019-10-28T14:22:34.4634501Z LL |     if true && let 0 = 1 {}
2019-10-28T14:22:34.4634591Z    |
2019-10-28T14:22:34.4634591Z    |
2019-10-28T14:22:34.4634985Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4635046Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4635109Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4635335Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:26:8
2019-10-28T14:22:34.4635374Z    |
2019-10-28T14:22:34.4635374Z    |
2019-10-28T14:22:34.4635408Z LL |     if let 0 = 1 && true {}
2019-10-28T14:22:34.4635494Z    |
2019-10-28T14:22:34.4635494Z    |
2019-10-28T14:22:34.4635713Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4635777Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4635949Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4636209Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:30:9
2019-10-28T14:22:34.4636269Z    |
2019-10-28T14:22:34.4636269Z    |
2019-10-28T14:22:34.4636307Z LL |     if (let 0 = 1) && true {}
2019-10-28T14:22:34.4636398Z    |
2019-10-28T14:22:34.4636398Z    |
2019-10-28T14:22:34.4636649Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4636696Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4636778Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4637003Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:34:17
2019-10-28T14:22:34.4637059Z    |
2019-10-28T14:22:34.4637059Z    |
2019-10-28T14:22:34.4637536Z LL |     if true && (let 0 = 1) {}
2019-10-28T14:22:34.4637640Z    |
2019-10-28T14:22:34.4637640Z    |
2019-10-28T14:22:34.4638261Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4638441Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4638546Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4638833Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:38:9
2019-10-28T14:22:34.4638880Z    |
2019-10-28T14:22:34.4638880Z    |
2019-10-28T14:22:34.4638940Z LL |     if (let 0 = 1) && (let 0 = 1) {}
2019-10-28T14:22:34.4639025Z    |
2019-10-28T14:22:34.4639025Z    |
2019-10-28T14:22:34.4641705Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4641784Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4641880Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4642246Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:38:24
2019-10-28T14:22:34.4642328Z    |
2019-10-28T14:22:34.4642328Z    |
2019-10-28T14:22:34.4642372Z LL |     if (let 0 = 1) && (let 0 = 1) {}
2019-10-28T14:22:34.4642486Z    |
2019-10-28T14:22:34.4642486Z    |
2019-10-28T14:22:34.4642793Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4642855Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4642938Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4643405Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:44:8
2019-10-28T14:22:34.4643617Z    |
2019-10-28T14:22:34.4643617Z    |
2019-10-28T14:22:34.4643665Z LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4643769Z    |
2019-10-28T14:22:34.4643769Z    |
2019-10-28T14:22:34.4644372Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4644439Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4644755Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4645183Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:44:21
2019-10-28T14:22:34.4645229Z    |
2019-10-28T14:22:34.4645229Z    |
2019-10-28T14:22:34.4645272Z LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4645373Z    |
2019-10-28T14:22:34.4645373Z    |
2019-10-28T14:22:34.4645787Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4645852Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4645923Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4646151Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:44:35
2019-10-28T14:22:34.4646208Z    |
2019-10-28T14:22:34.4646208Z    |
2019-10-28T14:22:34.4646250Z LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4646672Z    |
2019-10-28T14:22:34.4646672Z    |
2019-10-28T14:22:34.4646975Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4647027Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4647490Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4647789Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:44:48
2019-10-28T14:22:34.4647853Z    |
2019-10-28T14:22:34.4647853Z    |
2019-10-28T14:22:34.4647900Z LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4647995Z    |
2019-10-28T14:22:34.4647995Z    |
2019-10-28T14:22:34.4648292Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4648349Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4648451Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4648812Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:44:61
2019-10-28T14:22:34.4648869Z    |
2019-10-28T14:22:34.4648869Z    |
2019-10-28T14:22:34.4648934Z LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
2019-10-28T14:22:34.4649030Z    |
2019-10-28T14:22:34.4649030Z    |
2019-10-28T14:22:34.4649354Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4649409Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4649503Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4649757Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:56:8
2019-10-28T14:22:34.4649804Z    |
2019-10-28T14:22:34.4649804Z    |
2019-10-28T14:22:34.4649865Z LL |     if let Range { start: _, end: _ } = (true..true) && false {}
2019-10-28T14:22:34.4649968Z    |
2019-10-28T14:22:34.4649968Z    |
2019-10-28T14:22:34.4650266Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4650321Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4650398Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4650853Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:64:12
2019-10-28T14:22:34.4650893Z    |
2019-10-28T14:22:34.4650893Z    |
2019-10-28T14:22:34.4651103Z LL |     while (let 0 = 1) {}
2019-10-28T14:22:34.4651373Z    |
2019-10-28T14:22:34.4651373Z    |
2019-10-28T14:22:34.4651617Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4651681Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4651750Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4652191Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:68:14
2019-10-28T14:22:34.4652235Z    |
2019-10-28T14:22:34.4652235Z    |
2019-10-28T14:22:34.4652281Z LL |     while (((let 0 = 1))) {}
2019-10-28T14:22:34.4652378Z    |
2019-10-28T14:22:34.4652378Z    |
2019-10-28T14:22:34.4652629Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4652696Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4652768Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4653003Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:72:19
2019-10-28T14:22:34.4653062Z    |
2019-10-28T14:22:34.4653062Z    |
2019-10-28T14:22:34.4653102Z LL |     while true && let 0 = 1 {}
2019-10-28T14:22:34.4653200Z    |
2019-10-28T14:22:34.4653200Z    |
2019-10-28T14:22:34.4653762Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4653899Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4654168Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4654430Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:76:11
2019-10-28T14:22:34.4654488Z    |
2019-10-28T14:22:34.4654488Z    |
2019-10-28T14:22:34.4654526Z LL |     while let 0 = 1 && true {}
2019-10-28T14:22:34.4654601Z    |
2019-10-28T14:22:34.4654601Z    |
2019-10-28T14:22:34.4655201Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4655446Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4655529Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4655752Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:80:12
2019-10-28T14:22:34.4655793Z    |
2019-10-28T14:22:34.4655793Z    |
2019-10-28T14:22:34.4655888Z LL |     while (let 0 = 1) && true {}
2019-10-28T14:22:34.4655970Z    |
2019-10-28T14:22:34.4655970Z    |
2019-10-28T14:22:34.4656607Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4657440Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4657542Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4657875Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:84:20
2019-10-28T14:22:34.4657923Z    |
2019-10-28T14:22:34.4657923Z    |
2019-10-28T14:22:34.4657964Z LL |     while true && (let 0 = 1) {}
2019-10-28T14:22:34.4658068Z    |
2019-10-28T14:22:34.4658068Z    |
2019-10-28T14:22:34.4658352Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4658428Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4658510Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4658822Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:88:12
2019-10-28T14:22:34.4658873Z    |
2019-10-28T14:22:34.4658873Z    |
2019-10-28T14:22:34.4658920Z LL |     while (let 0 = 1) && (let 0 = 1) {}
2019-10-28T14:22:34.4659038Z    |
2019-10-28T14:22:34.4659038Z    |
2019-10-28T14:22:34.4659332Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4659407Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4659489Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4659762Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:88:27
2019-10-28T14:22:34.4659832Z    |
2019-10-28T14:22:34.4659832Z    |
2019-10-28T14:22:34.4659878Z LL |     while (let 0 = 1) && (let 0 = 1) {}
2019-10-28T14:22:34.4659989Z    |
2019-10-28T14:22:34.4659989Z    |
2019-10-28T14:22:34.4660282Z    = note: for more information, see ***/issues/53667
2019-10-28T14:22:34.4660349Z    = help: add `#![feature(let_chains)]` to the crate attributes to enable
2019-10-28T14:22:34.4660445Z error[E0658]: `let` expressions in this position are experimental
2019-10-28T14:22:34.4660853Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/feature-gate.rs:94:11
2019-10-28T14:22:34.4660913Z    |
2019-10-28T14:22:34.4660913Z    |
2019-10-28T14:22:34.4660956Z LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
---
2019-10-28T14:22:34.4961574Z 
2019-10-28T14:22:34.4961592Z 
2019-10-28T14:22:34.4961796Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-28T14:22:34.4961840Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-28T14:22:34.4963130Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-28T14:22:34.4963385Z 
2019-10-28T14:22:34.4963409Z 
2019-10-28T14:22:34.4963466Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-28T14:22:34.4963506Z Build completed unsuccessfully in 0:59:05
2019-10-28T14:22:34.4963506Z Build completed unsuccessfully in 0:59:05
2019-10-28T14:22:34.4963541Z == clock drift check ==
2019-10-28T14:22:34.4963644Z   local time: Mon Oct 28 14:22:34 UTC 2019
2019-10-28T14:22:34.7503895Z   network time: Mon, 28 Oct 2019 14:22:34 GMT
2019-10-28T14:22:34.7503971Z == end clock drift check ==
2019-10-28T14:22:35.7944921Z 
2019-10-28T14:22:35.8051423Z ##[error]Bash exited with code '1'.
2019-10-28T14:22:35.8089047Z ##[section]Starting: Checkout
2019-10-28T14:22:35.8090749Z ==============================================================================
2019-10-28T14:22:35.8090804Z Task         : Get sources
2019-10-28T14:22:35.8091028Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
