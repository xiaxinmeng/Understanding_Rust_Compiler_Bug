plain
2019-08-29T18:06:36.6053823Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-29T18:06:36.6251192Z ##[command]git config gc.auto 0
2019-08-29T18:06:36.6335953Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-29T18:06:36.6394038Z ##[command]git config --get-all http.proxy
2019-08-29T18:06:36.6540590Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63468/merge:refs/remotes/pull/63468/merge
---
2019-08-29T19:09:35.1188165Z .................................................................................................... 1500/8974
2019-08-29T19:09:40.7815218Z .................................................................................................... 1600/8974
2019-08-29T19:09:53.4362683Z ................................................i...............i................................... 1700/8974
2019-08-29T19:10:01.7293772Z .................................................................................................... 1800/8974
2019-08-29T19:10:15.8853652Z .......................................iiiii........................................................ 1900/8974
2019-08-29T19:10:26.5577897Z .................................................................................................... 2100/8974
2019-08-29T19:10:29.1060324Z .................................................................................................... 2200/8974
2019-08-29T19:10:33.1912753Z .................................................................................................... 2300/8974
2019-08-29T19:10:40.5850816Z .................................................................................................... 2400/8974
---
2019-08-29T19:13:38.9421167Z ..........................i...............i.............F........................................... 4700/8974
2019-08-29T19:13:50.7391586Z .................................................................................................... 4800/8974
2019-08-29T19:13:56.9036898Z .................................................................................................... 4900/8974
2019-08-29T19:14:07.6742355Z .................................................................................................... 5000/8974
2019-08-29T19:14:13.2353020Z .......ii.ii........................................................................................ 5100/8974
2019-08-29T19:14:26.0786197Z .................................................................................................... 5300/8974
2019-08-29T19:14:34.2630987Z ......................................................................i............................. 5400/8974
2019-08-29T19:14:41.4168503Z .................................................................................................... 5500/8974
2019-08-29T19:14:48.3120646Z .................................................................................................... 5600/8974
2019-08-29T19:14:48.3120646Z .................................................................................................... 5600/8974
2019-08-29T19:14:58.8094170Z ................................................................ii...i..ii...........i.............. 5700/8974
2019-08-29T19:15:24.6118209Z .................................................................................................... 5900/8974
2019-08-29T19:15:31.4655689Z .................................................................................................... 6000/8974
2019-08-29T19:15:31.4655689Z .................................................................................................... 6000/8974
2019-08-29T19:15:37.5659950Z .................................................................i..ii.............................. 6100/8974
2019-08-29T19:16:06.3978055Z .................................................................................................... 6300/8974
2019-08-29T19:16:08.4775683Z ....................i............................................................................... 6400/8974
2019-08-29T19:16:10.6588483Z ............................................................................................i....... 6500/8974
2019-08-29T19:16:13.3818738Z .................................................................................................... 6600/8974
---
2019-08-29T19:20:11.5869623Z failures:
2019-08-29T19:20:11.5916856Z 
2019-08-29T19:20:11.5921190Z ---- [ui] ui/conditional-compilation/cfg-generic-params.rs stdout ----
2019-08-29T19:20:11.5921513Z 
2019-08-29T19:20:11.5921578Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-29T19:20:11.5921633Z status: exit code: 101
2019-08-29T19:20:11.5922526Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/conditional-compilation/cfg-generic-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-generic-params" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-generic-params/auxiliary" "-A" "unused"
2019-08-29T19:20:11.5923125Z ------------------------------------------
2019-08-29T19:20:11.5923164Z 
2019-08-29T19:20:11.5923388Z ------------------------------------------
2019-08-29T19:20:11.5923455Z stderr:
2019-08-29T19:20:11.5923455Z stderr:
2019-08-29T19:20:11.5923676Z ------------------------------------------
2019-08-29T19:20:11.5924328Z thread 'rustc' panicked at 'adding a def'n for node-id NodeId(3) and data LifetimeNs("") but a previous def'n exists: DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: Misc, disambiguator: 0 } }', src/librustc/hir/map/definitions.rs:455:9
2019-08-29T19:20:11.5924470Z 
2019-08-29T19:20:11.5924517Z error: internal compiler error: unexpected panic
2019-08-29T19:20:11.5924546Z 
2019-08-29T19:20:11.5924611Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-29T19:20:11.5924611Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-29T19:20:11.5924650Z 
2019-08-29T19:20:11.5925052Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-29T19:20:11.5925361Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-29T19:20:11.5925394Z 
2019-08-29T19:20:11.5925394Z 
2019-08-29T19:20:11.5925669Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-29T19:20:11.5925751Z 
2019-08-29T19:20:11.5925960Z ------------------------------------------
2019-08-29T19:20:11.5925992Z 
2019-08-29T19:20:11.5926035Z 
2019-08-29T19:20:11.5926035Z 
2019-08-29T19:20:11.5926275Z ---- [ui] ui/feature-gates/feature-gate-custom_attribute2.rs stdout ----
2019-08-29T19:20:11.5926309Z 
2019-08-29T19:20:11.5926474Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-29T19:20:11.5926549Z status: exit code: 101
2019-08-29T19:20:11.5927338Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-custom_attribute2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-custom_attribute2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-custom_attribute2/auxiliary" "-A" "unused"
2019-08-29T19:20:11.5927680Z ------------------------------------------
2019-08-29T19:20:11.5927713Z 
2019-08-29T19:20:11.5927948Z ------------------------------------------
2019-08-29T19:20:11.5928002Z stderr:
2019-08-29T19:20:11.5928002Z stderr:
2019-08-29T19:20:11.5928214Z ------------------------------------------
2019-08-29T19:20:11.5928664Z thread 'rustc' panicked at 'adding a def'n for node-id NodeId(3) and data LifetimeNs("") but a previous def'n exists: DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: Misc, disambiguator: 0 } }', src/librustc/hir/map/definitions.rs:455:9
2019-08-29T19:20:11.5928781Z 
2019-08-29T19:20:11.5928844Z error: internal compiler error: unexpected panic
2019-08-29T19:20:11.5928874Z 
2019-08-29T19:20:11.5928918Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-29T19:20:11.5928918Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-29T19:20:11.5928947Z 
2019-08-29T19:20:11.5929281Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-29T19:20:11.5929556Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-29T19:20:11.5929608Z 
2019-08-29T19:20:11.5929608Z 
2019-08-29T19:20:11.5929892Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-29T19:20:11.5929955Z 
2019-08-29T19:20:11.5930181Z ------------------------------------------
2019-08-29T19:20:11.5930304Z 
2019-08-29T19:20:11.5930329Z 
2019-08-29T19:20:11.5930329Z 
2019-08-29T19:20:11.5930986Z ---- [ui] ui/issues/issue-49934.rs stdout ----
2019-08-29T19:20:11.5931027Z 
2019-08-29T19:20:11.5931100Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-29T19:20:11.5931149Z status: exit code: 101
2019-08-29T19:20:11.5931881Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-49934.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49934" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49934/auxiliary" "-A" "unused"
2019-08-29T19:20:11.5932217Z ------------------------------------------
2019-08-29T19:20:11.5932280Z 
2019-08-29T19:20:11.5932506Z ------------------------------------------
2019-08-29T19:20:11.5932552Z stderr:
---
2019-08-29T19:20:11.5933253Z    |     ^^^^^^^^^^^^^^^^
2019-08-29T19:20:11.5933297Z    |
2019-08-29T19:20:11.5933367Z    = note: this may become a hard error in a future release
2019-08-29T19:20:11.5933402Z 
2019-08-29T19:20:11.5933824Z thread 'rustc' panicked at 'non-eager expansion without a parent scope', src/libcore/option.rs:1166:5
2019-08-29T19:20:11.5934121Z 
2019-08-29T19:20:11.5934166Z error: internal compiler error: unexpected panic
2019-08-29T19:20:11.5934205Z 
2019-08-29T19:20:11.5934268Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-29T19:20:11.5934268Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-29T19:20:11.5934298Z 
2019-08-29T19:20:11.5934653Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-29T19:20:11.5934957Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-29T19:20:11.5934989Z 
2019-08-29T19:20:11.5934989Z 
2019-08-29T19:20:11.5935501Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-29T19:20:11.5935579Z 
2019-08-29T19:20:11.5935783Z ------------------------------------------
2019-08-29T19:20:11.5935813Z 
2019-08-29T19:20:11.5935837Z 
2019-08-29T19:20:11.5935837Z 
2019-08-29T19:20:11.5936087Z ---- [ui] ui/proc-macro/proc-macro-gates2.rs stdout ----
2019-08-29T19:20:11.5936118Z 
2019-08-29T19:20:11.5936164Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-29T19:20:11.5936227Z status: exit code: 101
2019-08-29T19:20:11.5936925Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/proc-macro-gates2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-gates2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-gates2/auxiliary" "-A" "unused"
2019-08-29T19:20:11.5937238Z ------------------------------------------
2019-08-29T19:20:11.5937270Z 
2019-08-29T19:20:11.5937480Z ------------------------------------------
2019-08-29T19:20:11.5937542Z stderr:
2019-08-29T19:20:11.5937542Z stderr:
2019-08-29T19:20:11.5937746Z ------------------------------------------
2019-08-29T19:20:11.5938006Z thread 'rustc' panicked at 'non-eager expansion without a parent scope', src/libcore/option.rs:1166:5
2019-08-29T19:20:11.5938208Z 
2019-08-29T19:20:11.5938249Z error: internal compiler error: unexpected panic
2019-08-29T19:20:11.5938297Z 
2019-08-29T19:20:11.5938339Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-29T19:20:11.5938339Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-29T19:20:11.5938368Z 
2019-08-29T19:20:11.5938703Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-29T19:20:11.5938971Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-29T19:20:11.5939002Z 
2019-08-29T19:20:11.5939002Z 
2019-08-29T19:20:11.5939287Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-29T19:20:11.5939350Z 
2019-08-29T19:20:11.5939551Z ------------------------------------------
2019-08-29T19:20:11.5939580Z 
2019-08-29T19:20:11.5939622Z 
2019-08-29T19:20:11.5939622Z 
2019-08-29T19:20:11.5939854Z ---- [ui] ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs stdout ----
2019-08-29T19:20:11.5939910Z diff of stderr:
2019-08-29T19:20:11.5939937Z 
2019-08-29T19:20:11.5939999Z + error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5940643Z +   --> $DIR/param-attrs-builtin-attrs.rs:7:9
2019-08-29T19:20:11.5940697Z +    |
2019-08-29T19:20:11.5940764Z + LL |         #[test] a: i32,
2019-08-29T19:20:11.5940850Z + 
2019-08-29T19:20:11.5940850Z + 
2019-08-29T19:20:11.5940898Z + error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5941152Z +   --> $DIR/param-attrs-builtin-attrs.rs:23:5
2019-08-29T19:20:11.5941199Z +    |
2019-08-29T19:20:11.5941242Z + LL |     #[test] a: u32,
2019-08-29T19:20:11.5941454Z + 
2019-08-29T19:20:11.5941454Z + 
2019-08-29T19:20:11.5941500Z + error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5941780Z +   --> $DIR/param-attrs-builtin-attrs.rs:38:5
2019-08-29T19:20:11.5941839Z +    |
2019-08-29T19:20:11.5941883Z + LL |     #[test] a: u32,
2019-08-29T19:20:11.5941986Z + 
2019-08-29T19:20:11.5941986Z + 
2019-08-29T19:20:11.5942032Z + error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5942263Z +   --> $DIR/param-attrs-builtin-attrs.rs:58:9
2019-08-29T19:20:11.5942331Z +    |
2019-08-29T19:20:11.5942374Z + LL |         #[test] a: i32,
2019-08-29T19:20:11.5942478Z + 
2019-08-29T19:20:11.5942478Z + 
2019-08-29T19:20:11.5942525Z + error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5942750Z +   --> $DIR/param-attrs-builtin-attrs.rs:79:9
2019-08-29T19:20:11.5942796Z +    |
2019-08-29T19:20:11.5942859Z + LL |         #[test] a: i32,
2019-08-29T19:20:11.5942951Z + 
2019-08-29T19:20:11.5942951Z + 
2019-08-29T19:20:11.5943018Z + error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5943248Z +   --> $DIR/param-attrs-builtin-attrs.rs:98:9
2019-08-29T19:20:11.5943303Z +    |
2019-08-29T19:20:11.5943365Z + LL |         #[test] a: i32,
2019-08-29T19:20:11.5943451Z + 
2019-08-29T19:20:11.5943451Z + 
2019-08-29T19:20:11.5943497Z + error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5943899Z +   --> $DIR/param-attrs-builtin-attrs.rs:117:9
2019-08-29T19:20:11.5943942Z +    |
2019-08-29T19:20:11.5943982Z + LL |         #[test] a: i32,
2019-08-29T19:20:11.5944080Z + 
2019-08-29T19:20:11.5944080Z + 
2019-08-29T19:20:11.5944123Z + error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5944357Z +   --> $DIR/param-attrs-builtin-attrs.rs:134:9
2019-08-29T19:20:11.5944401Z +    |
2019-08-29T19:20:11.5944448Z + LL |         #[test] a: u32,
2019-08-29T19:20:11.5944547Z + 
2019-08-29T19:20:11.5944591Z 1 error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.5944806Z 2   --> $DIR/param-attrs-builtin-attrs.rs:5:9
2019-08-29T19:20:11.5944954Z 3    |
2019-08-29T19:20:11.5944954Z 3    |
2019-08-29T19:20:11.5944982Z 
2019-08-29T19:20:11.5945023Z 262 LL |         #[no_mangle] b: i32
2019-08-29T19:20:11.5945124Z 264 
2019-08-29T19:20:11.5945124Z 264 
2019-08-29T19:20:11.5945427Z - error[E0658]: the attribute `test` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-29T19:20:11.5945836Z -    |
2019-08-29T19:20:11.5945836Z -    |
2019-08-29T19:20:11.5946030Z - LL |         #[test] a: i32,
2019-08-29T19:20:11.5946399Z -    |
2019-08-29T19:20:11.5946399Z -    |
2019-08-29T19:20:11.5946680Z -    = note: for more information, see ***/issues/29642
2019-08-29T19:20:11.5946941Z -    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-29T19:20:11.5947137Z - 
2019-08-29T19:20:11.5947411Z - error[E0658]: the attribute `test` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-29T19:20:11.5947828Z -    |
2019-08-29T19:20:11.5947828Z -    |
2019-08-29T19:20:11.5948015Z - LL |     #[test] a: u32,
2019-08-29T19:20:11.5948378Z -    |
2019-08-29T19:20:11.5948378Z -    |
2019-08-29T19:20:11.5948647Z -    = note: for more information, see ***/issues/29642
2019-08-29T19:20:11.5948895Z -    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-29T19:20:11.5949081Z - 
2019-08-29T19:20:11.5949357Z - error[E0658]: the attribute `test` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-29T19:20:11.5949760Z -    |
2019-08-29T19:20:11.5949760Z -    |
2019-08-29T19:20:11.5950024Z - LL |     #[test] a: u32,
2019-08-29T19:20:11.5951475Z -    |
2019-08-29T19:20:11.5951475Z -    |
2019-08-29T19:20:11.5951808Z -    = note: for more information, see ***/issues/29642
2019-08-29T19:20:11.5952083Z -    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-29T19:20:11.5952304Z - 
2019-08-29T19:20:11.5952599Z - error[E0658]: the attribute `test` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-29T19:20:11.5959680Z -    |
2019-08-29T19:20:11.5959680Z -    |
2019-08-29T19:20:11.5960010Z - LL |         #[test] a: i32,
2019-08-29T19:20:11.5963887Z -    |
2019-08-29T19:20:11.5963887Z -    |
2019-08-29T19:20:11.5964395Z -    = note: for more information, see ***/issues/29642
2019-08-29T19:20:11.5964648Z -    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-29T19:20:11.5964815Z - 
2019-08-29T19:20:11.5965121Z - error[E0658]: the attribute `test` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-29T19:20:11.5965690Z -    |
2019-08-29T19:20:11.5965690Z -    |
2019-08-29T19:20:11.5965916Z - LL |         #[test] a: i32,
2019-08-29T19:20:11.5966266Z -    |
2019-08-29T19:20:11.5966266Z -    |
2019-08-29T19:20:11.5966548Z -    = note: for more information, see ***/issues/29642
2019-08-29T19:20:11.5966801Z -    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-29T19:20:11.5966972Z - 
2019-08-29T19:20:11.5967267Z - error[E0658]: the attribute `test` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-29T19:20:11.5967651Z -    |
2019-08-29T19:20:11.5967651Z -    |
2019-08-29T19:20:11.5967865Z - LL |         #[test] a: i32,
2019-08-29T19:20:11.5968218Z -    |
2019-08-29T19:20:11.5968218Z -    |
2019-08-29T19:20:11.5968505Z -    = note: for more information, see ***/issues/29642
2019-08-29T19:20:11.5968762Z -    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-29T19:20:11.5968932Z - 
2019-08-29T19:20:11.5969222Z - error[E0658]: the attribute `test` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-29T19:20:11.5969817Z -    |
2019-08-29T19:20:11.5969817Z -    |
2019-08-29T19:20:11.5970029Z - LL |         #[test] a: i32,
2019-08-29T19:20:11.5970869Z -    |
2019-08-29T19:20:11.5970869Z -    |
2019-08-29T19:20:11.5971528Z -    = note: for more information, see ***/issues/29642
2019-08-29T19:20:11.5971806Z -    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-29T19:20:11.5971990Z - 
2019-08-29T19:20:11.5972301Z - error[E0658]: the attribute `test` is currently unknown to the compiler and may have meaning added to it in the future
2019-08-29T19:20:11.5972736Z -    |
2019-08-29T19:20:11.5972736Z -    |
2019-08-29T19:20:11.5972958Z - LL |         #[test] a: u32,
2019-08-29T19:20:11.5973552Z -    |
2019-08-29T19:20:11.5973552Z -    |
2019-08-29T19:20:11.5974110Z -    = note: for more information, see ***/issues/29642
2019-08-29T19:20:11.5975305Z -    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-08-29T19:20:11.5975564Z 337 error: aborting due to 52 previous errors
2019-08-29T19:20:11.5975632Z 338 
2019-08-29T19:20:11.5976052Z - For more information about this error, try `rustc --explain E0658`.
2019-08-29T19:20:11.5976099Z 340 
2019-08-29T19:20:11.5976099Z 340 
2019-08-29T19:20:11.5976200Z 
2019-08-29T19:20:11.5976228Z 
2019-08-29T19:20:11.5976273Z The actual stderr differed from the expected stderr.
2019-08-29T19:20:11.5976746Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs/param-attrs-builtin-attrs.stderr
2019-08-29T19:20:11.5977074Z To update references, rerun the tests and pass the `--bless` flag
2019-08-29T19:20:11.5977360Z To only update this specific test, also pass `--test-args rfc-2565-param-attrs/param-attrs-builtin-attrs.rs`
2019-08-29T19:20:11.5977473Z error: 1 errors occurred comparing output.
2019-08-29T19:20:11.5977517Z status: exit code: 1
2019-08-29T19:20:11.5977517Z status: exit code: 1
2019-08-29T19:20:11.5978461Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs/auxiliary" "-A" "unused"
2019-08-29T19:20:11.5978808Z ------------------------------------------
2019-08-29T19:20:11.5978842Z 
2019-08-29T19:20:11.5979047Z ------------------------------------------
2019-08-29T19:20:11.5979099Z stderr:
2019-08-29T19:20:11.5979099Z stderr:
2019-08-29T19:20:11.5979324Z ------------------------------------------
2019-08-29T19:20:11.5979374Z error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5979685Z    |
2019-08-29T19:20:11.5979685Z    |
2019-08-29T19:20:11.5979727Z LL |         #[test] a: i32,
2019-08-29T19:20:11.5979816Z 
2019-08-29T19:20:11.5979816Z 
2019-08-29T19:20:11.5979859Z error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5980586Z    |
2019-08-29T19:20:11.5980586Z    |
2019-08-29T19:20:11.5980663Z LL |     #[test] a: u32,
2019-08-29T19:20:11.5980737Z 
2019-08-29T19:20:11.5980737Z 
2019-08-29T19:20:11.5980802Z error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5981268Z    |
2019-08-29T19:20:11.5981268Z    |
2019-08-29T19:20:11.5981311Z LL |     #[test] a: u32,
2019-08-29T19:20:11.5981404Z 
2019-08-29T19:20:11.5981404Z 
2019-08-29T19:20:11.5981450Z error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5981813Z    |
2019-08-29T19:20:11.5981813Z    |
2019-08-29T19:20:11.5981857Z LL |         #[test] a: i32,
2019-08-29T19:20:11.5981951Z 
2019-08-29T19:20:11.5981951Z 
2019-08-29T19:20:11.5981996Z error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5982337Z    |
2019-08-29T19:20:11.5982337Z    |
2019-08-29T19:20:11.5982381Z LL |         #[test] a: i32,
2019-08-29T19:20:11.5982454Z 
2019-08-29T19:20:11.5982454Z 
2019-08-29T19:20:11.5982519Z error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5982843Z    |
2019-08-29T19:20:11.5982843Z    |
2019-08-29T19:20:11.5982905Z LL |         #[test] a: i32,
2019-08-29T19:20:11.5982978Z 
2019-08-29T19:20:11.5982978Z 
2019-08-29T19:20:11.5983024Z error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5983360Z    |
2019-08-29T19:20:11.5983360Z    |
2019-08-29T19:20:11.5983402Z LL |         #[test] a: i32,
2019-08-29T19:20:11.5983493Z 
2019-08-29T19:20:11.5983493Z 
2019-08-29T19:20:11.5983726Z error: expected an inert attribute, found an attribute macro
2019-08-29T19:20:11.5984124Z    |
2019-08-29T19:20:11.5984124Z    |
2019-08-29T19:20:11.5984163Z LL |         #[test] a: u32,
2019-08-29T19:20:11.5984258Z 
2019-08-29T19:20:11.5984302Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.5984571Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:5:9
2019-08-29T19:20:11.5984618Z    |
---
2019-08-29T19:20:11.5985277Z    |
2019-08-29T19:20:11.5985335Z LL |         /// Bar
2019-08-29T19:20:11.5985390Z    |         ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.5985421Z 
2019-08-29T19:20:11.5985699Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.5986046Z    |
2019-08-29T19:20:11.5986087Z LL |         #[must_use]
2019-08-29T19:20:11.5986148Z    |         ^^^^^^^^^^^
2019-08-29T19:20:11.5986175Z 
2019-08-29T19:20:11.5986175Z 
2019-08-29T19:20:11.5986220Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.5986479Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:13:9
2019-08-29T19:20:11.5986545Z    |
2019-08-29T19:20:11.5986584Z LL |         /// Baz
2019-08-29T19:20:11.5986630Z    |         ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.5986681Z 
2019-08-29T19:20:11.5986963Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.5987287Z    |
2019-08-29T19:20:11.5987287Z    |
2019-08-29T19:20:11.5987330Z LL |         #[no_mangle] b: i32,
2019-08-29T19:20:11.5987485Z 
2019-08-29T19:20:11.5987550Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.5987828Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:21:5
2019-08-29T19:20:11.5987876Z    |
---
2019-08-29T19:20:11.5988375Z    |
2019-08-29T19:20:11.5988415Z LL |     /// Bar
2019-08-29T19:20:11.5988487Z    |     ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.5988518Z 
2019-08-29T19:20:11.5988794Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.5989127Z    |
2019-08-29T19:20:11.5989168Z LL |     #[must_use]
2019-08-29T19:20:11.5989209Z    |     ^^^^^^^^^^^
2019-08-29T19:20:11.5989255Z 
2019-08-29T19:20:11.5989255Z 
2019-08-29T19:20:11.5989300Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.5989554Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:29:5
2019-08-29T19:20:11.5989621Z    |
2019-08-29T19:20:11.5989661Z LL |     /// Baz
2019-08-29T19:20:11.5989705Z    |     ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.5989735Z 
2019-08-29T19:20:11.5990101Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.5990793Z    |
2019-08-29T19:20:11.5990793Z    |
2019-08-29T19:20:11.5990859Z LL |     #[no_mangle] b: i32,
2019-08-29T19:20:11.5990943Z 
2019-08-29T19:20:11.5990989Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.5991673Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:36:5
2019-08-29T19:20:11.5991729Z    |
---
2019-08-29T19:20:11.5992256Z    |
2019-08-29T19:20:11.5992296Z LL |     /// Bar
2019-08-29T19:20:11.5992352Z    |     ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.5992403Z 
2019-08-29T19:20:11.5992690Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.5993034Z    |
2019-08-29T19:20:11.5993078Z LL |     #[must_use]
2019-08-29T19:20:11.5993121Z    |     ^^^^^^^^^^^
2019-08-29T19:20:11.5993149Z 
2019-08-29T19:20:11.5993149Z 
2019-08-29T19:20:11.5993216Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.5993480Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:44:5
2019-08-29T19:20:11.5993528Z    |
2019-08-29T19:20:11.5993587Z LL |     /// Baz
2019-08-29T19:20:11.5993635Z    |     ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.5993665Z 
2019-08-29T19:20:11.5993957Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.5994454Z    |
2019-08-29T19:20:11.5994454Z    |
2019-08-29T19:20:11.5994495Z LL |     #[no_mangle] b: i32,
2019-08-29T19:20:11.5994701Z 
2019-08-29T19:20:11.5994745Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.5995028Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:53:9
2019-08-29T19:20:11.5995095Z    |
---
2019-08-29T19:20:11.5996259Z    |
2019-08-29T19:20:11.5996298Z LL |         /// Baz
2019-08-29T19:20:11.5996361Z    |         ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.5996390Z 
2019-08-29T19:20:11.5996656Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.5996970Z    |
2019-08-29T19:20:11.5997009Z LL |         #[must_use]
2019-08-29T19:20:11.5997068Z    |         ^^^^^^^^^^^
2019-08-29T19:20:11.5997095Z 
2019-08-29T19:20:11.5997095Z 
2019-08-29T19:20:11.5997210Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.5997742Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:64:9
2019-08-29T19:20:11.5997822Z    |
2019-08-29T19:20:11.5997859Z LL |         /// Qux
2019-08-29T19:20:11.5997912Z    |         ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.5997941Z 
2019-08-29T19:20:11.5998270Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.5998566Z    |
2019-08-29T19:20:11.5998566Z    |
2019-08-29T19:20:11.5998627Z LL |         #[no_mangle] b: i32,
2019-08-29T19:20:11.5998695Z 
2019-08-29T19:20:11.5998756Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.5999003Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:74:9
2019-08-29T19:20:11.5999056Z    |
---
2019-08-29T19:20:11.6000032Z    |
2019-08-29T19:20:11.6000070Z LL |         /// Baz
2019-08-29T19:20:11.6000113Z    |         ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.6000141Z 
2019-08-29T19:20:11.6000815Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.6001288Z    |
2019-08-29T19:20:11.6001352Z LL |         #[must_use]
2019-08-29T19:20:11.6001397Z    |         ^^^^^^^^^^^
2019-08-29T19:20:11.6001425Z 
2019-08-29T19:20:11.6001425Z 
2019-08-29T19:20:11.6001490Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.6001786Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:85:9
2019-08-29T19:20:11.6001836Z    |
2019-08-29T19:20:11.6001878Z LL |         /// Qux
2019-08-29T19:20:11.6001944Z    |         ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.6001975Z 
2019-08-29T19:20:11.6002261Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.6002606Z    |
2019-08-29T19:20:11.6002606Z    |
2019-08-29T19:20:11.6002649Z LL |         #[no_mangle] b: i32,
2019-08-29T19:20:11.6002743Z 
2019-08-29T19:20:11.6002797Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.6003064Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:93:9
2019-08-29T19:20:11.6003132Z    |
---
2019-08-29T19:20:11.6004409Z    |
2019-08-29T19:20:11.6004447Z LL |         /// Baz
2019-08-29T19:20:11.6004511Z    |         ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.6004539Z 
2019-08-29T19:20:11.6004807Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.6005118Z    |
2019-08-29T19:20:11.6005157Z LL |         #[must_use]
2019-08-29T19:20:11.6005215Z    |         ^^^^^^^^^^^
2019-08-29T19:20:11.6005243Z 
2019-08-29T19:20:11.6005243Z 
2019-08-29T19:20:11.6005286Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.6005538Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:104:9
2019-08-29T19:20:11.6005604Z    |
2019-08-29T19:20:11.6005642Z LL |         /// Qux
2019-08-29T19:20:11.6005684Z    |         ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.6005720Z 
2019-08-29T19:20:11.6006007Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.6006299Z    |
2019-08-29T19:20:11.6006299Z    |
2019-08-29T19:20:11.6006359Z LL |         #[no_mangle] b: i32,
2019-08-29T19:20:11.6006427Z 
2019-08-29T19:20:11.6006487Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.6006734Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:112:9
2019-08-29T19:20:11.6006779Z    |
---
2019-08-29T19:20:11.6007860Z    |
2019-08-29T19:20:11.6007898Z LL |         /// Baz
2019-08-29T19:20:11.6007940Z    |         ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.6007969Z 
2019-08-29T19:20:11.6008262Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.6008556Z    |
2019-08-29T19:20:11.6008617Z LL |         #[must_use]
2019-08-29T19:20:11.6008666Z    |         ^^^^^^^^^^^
2019-08-29T19:20:11.6008693Z 
2019-08-29T19:20:11.6008693Z 
2019-08-29T19:20:11.6008735Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.6009004Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:123:9
2019-08-29T19:20:11.6009049Z    |
2019-08-29T19:20:11.6009087Z LL |         /// Qux
2019-08-29T19:20:11.6009148Z    |         ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.6009176Z 
2019-08-29T19:20:11.6009442Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.6009753Z    |
2019-08-29T19:20:11.6009753Z    |
2019-08-29T19:20:11.6009863Z LL |         #[no_mangle] b: i32,
2019-08-29T19:20:11.6009958Z 
2019-08-29T19:20:11.6010001Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.6010701Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:132:9
2019-08-29T19:20:11.6010778Z    |
---
2019-08-29T19:20:11.6011279Z    |
2019-08-29T19:20:11.6011339Z LL |         /// Bar
2019-08-29T19:20:11.6011385Z    |         ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.6011416Z 
2019-08-29T19:20:11.6011712Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.6012055Z    |
2019-08-29T19:20:11.6012106Z LL |         #[must_use]
2019-08-29T19:20:11.6012169Z    |         ^^^^^^^^^^^
2019-08-29T19:20:11.6012197Z 
2019-08-29T19:20:11.6012197Z 
2019-08-29T19:20:11.6012244Z error: documentation comments cannot be applied to function parameters
2019-08-29T19:20:11.6012530Z   --> /checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs:140:9
2019-08-29T19:20:11.6012579Z    |
2019-08-29T19:20:11.6012620Z LL |         /// Baz
2019-08-29T19:20:11.6012666Z    |         ^^^^^^^ doc comments are not allowed here
2019-08-29T19:20:11.6012716Z 
2019-08-29T19:20:11.6013000Z error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
2019-08-29T19:20:11.6013342Z    |
2019-08-29T19:20:11.6013342Z    |
2019-08-29T19:20:11.6013385Z LL |         #[no_mangle] b: i32
2019-08-29T19:20:11.6013460Z 
2019-08-29T19:20:11.6013523Z error: aborting due to 52 previous errors
2019-08-29T19:20:11.6013653Z 
2019-08-29T19:20:11.6013679Z 
---
2019-08-29T19:20:11.6016422Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-29T19:20:11.6016475Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-29T19:20:11.6016515Z 
2019-08-29T19:20:11.6016557Z 
2019-08-29T19:20:11.6017972Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-29T19:20:11.6018238Z 
2019-08-29T19:20:11.6018266Z 
2019-08-29T19:20:11.6018311Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-29T19:20:11.6018357Z Build completed unsuccessfully in 1:06:09
2019-08-29T19:20:11.6018357Z Build completed unsuccessfully in 1:06:09
2019-08-29T19:20:11.6031624Z == clock drift check ==
2019-08-29T19:20:11.6044510Z   local time: Thu Aug 29 19:20:11 UTC 2019
2019-08-29T19:20:11.7606239Z   network time: Thu, 29 Aug 2019 19:20:11 GMT
2019-08-29T19:20:11.7612837Z == end clock drift check ==
2019-08-29T19:20:12.6257690Z ##[error]Bash exited with code '1'.
2019-08-29T19:20:12.6311956Z ##[section]Starting: Checkout
2019-08-29T19:20:12.6313613Z ==============================================================================
2019-08-29T19:20:12.6313727Z Task         : Get sources
2019-08-29T19:20:12.6313795Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
