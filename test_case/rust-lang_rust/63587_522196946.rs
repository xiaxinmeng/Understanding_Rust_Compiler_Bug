plain
2019-08-16T23:46:51.8113123Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-16T23:46:51.8328423Z ##[command]git config gc.auto 0
2019-08-16T23:46:51.8392552Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-16T23:46:51.8457807Z ##[command]git config --get-all http.proxy
2019-08-16T23:46:51.8615318Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63587/merge:refs/remotes/pull/63587/merge
---
2019-08-16T23:47:27.2907445Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-16T23:47:27.2907471Z 
2019-08-16T23:47:27.2907669Z   git checkout -b <new-branch-name>
2019-08-16T23:47:27.2907693Z 
2019-08-16T23:47:27.2907734Z HEAD is now at 61a2e724c Merge 6e955fa3049045dfba5ecbf64b4d29435252c6e2 into bdfd698f37184da42254a03ed466ab1f90e6fb6c
2019-08-16T23:47:27.3061538Z ##[section]Finishing: Checkout
2019-08-16T23:47:27.3068951Z ##[section]Starting: Decide whether to run this job
2019-08-16T23:47:27.3071648Z Task         : Bash
2019-08-16T23:47:27.3071689Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-16T23:47:27.3071730Z Version      : 3.151.3
2019-08-16T23:47:27.3071771Z Author       : Microsoft Corporation
2019-08-16T23:47:27.3071771Z Author       : Microsoft Corporation
2019-08-16T23:47:27.3071834Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-08-16T23:47:27.3071896Z ==============================================================================
2019-08-16T23:47:27.4704929Z Generating script.
2019-08-16T23:47:27.4739711Z ========================== Starting Command Output ===========================
2019-08-16T23:47:27.4765399Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fc536e64-9289-4928-9207-26960b65af47.sh
2019-08-16T23:47:27.6756361Z Executing the job since submodules are updated
2019-08-16T23:47:27.6853922Z ##[section]Finishing: Decide whether to run this job
2019-08-16T23:47:27.6864946Z ==============================================================================
2019-08-16T23:47:27.6865071Z Task         : Bash
2019-08-16T23:47:27.6865124Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-16T23:47:27.6865174Z Version      : 3.151.3
---
2019-08-17T02:02:18.7827084Z failures:
2019-08-17T02:02:18.7829244Z 
2019-08-17T02:02:18.7846170Z ---- [ui] ui/assertions_on_constants.rs stdout ----
2019-08-17T02:02:18.7846717Z normalized stderr:
2019-08-17T02:02:18.7847445Z error: `assert!(true)` will be optimized out by the compiler
2019-08-17T02:02:18.7847878Z    |
2019-08-17T02:02:18.7847878Z    |
2019-08-17T02:02:18.7847920Z LL |     assert!(true);
2019-08-17T02:02:18.7848835Z    |
2019-08-17T02:02:18.7849252Z    = note: `-D clippy::assertions-on-constants` implied by `-D warnings`
2019-08-17T02:02:18.7849513Z    = help: remove it
2019-08-17T02:02:18.7849634Z 
2019-08-17T02:02:18.7849634Z 
2019-08-17T02:02:18.7849691Z error: `assert!(false)` should probably be replaced
2019-08-17T02:02:18.7850262Z    |
2019-08-17T02:02:18.7850262Z    |
2019-08-17T02:02:18.7850308Z LL |     assert!(false);
2019-08-17T02:02:18.7850392Z    |
2019-08-17T02:02:18.7850392Z    |
2019-08-17T02:02:18.7850457Z    = help: use `panic!()` or `unreachable!()`
2019-08-17T02:02:18.7850488Z 
2019-08-17T02:02:18.7850545Z error: `assert!(true)` will be optimized out by the compiler
2019-08-17T02:02:18.7850903Z    |
2019-08-17T02:02:18.7850903Z    |
2019-08-17T02:02:18.7850947Z LL |     assert!(true, "true message");
2019-08-17T02:02:18.7851054Z    |
2019-08-17T02:02:18.7851095Z    = help: remove it
2019-08-17T02:02:18.7851125Z 
2019-08-17T02:02:18.7851125Z 
2019-08-17T02:02:18.7851168Z error: `assert!(false)` should probably be replaced
2019-08-17T02:02:18.7851610Z    |
2019-08-17T02:02:18.7851610Z    |
2019-08-17T02:02:18.7851652Z LL |     assert!(false, "false message");
2019-08-17T02:02:18.7852381Z    |
2019-08-17T02:02:18.7852381Z    |
2019-08-17T02:02:18.7852427Z    = help: use `panic!()` or `unreachable!()`
2019-08-17T02:02:18.7852479Z 
2019-08-17T02:02:18.7852680Z error: `assert!(true)` will be optimized out by the compiler
2019-08-17T02:02:18.7853301Z    |
2019-08-17T02:02:18.7853390Z LL |     assert!(B);
2019-08-17T02:02:18.7853479Z    |     ^^^^^^^^^^^
2019-08-17T02:02:18.7853543Z    |
2019-08-17T02:02:18.7853543Z    |
2019-08-17T02:02:18.7853583Z    = help: remove it
2019-08-17T02:02:18.7853611Z 
2019-08-17T02:02:18.7853653Z error: `assert!(false)` should probably be replaced
2019-08-17T02:02:18.7854012Z    |
2019-08-17T02:02:18.7854052Z LL |     assert!(C);
2019-08-17T02:02:18.7854328Z    |     ^^^^^^^^^^^
2019-08-17T02:02:18.7854413Z    |
2019-08-17T02:02:18.7854413Z    |
2019-08-17T02:02:18.7854490Z    = help: use `panic!()` or `unreachable!()`
2019-08-17T02:02:18.7854662Z error: aborting due to 6 previous errors
2019-08-17T02:02:18.7854717Z 
2019-08-17T02:02:18.7854773Z 
2019-08-17T02:02:18.7854820Z 
2019-08-17T02:02:18.7854820Z 
2019-08-17T02:02:18.7854861Z expected stderr:
2019-08-17T02:02:18.7855071Z error: `assert!(true)` will be optimized out by the compiler
2019-08-17T02:02:18.7855625Z    |
2019-08-17T02:02:18.7855625Z    |
2019-08-17T02:02:18.7855665Z LL |     assert!(true);
2019-08-17T02:02:18.7855950Z    |
2019-08-17T02:02:18.7856291Z    = note: `-D clippy::assertions-on-constants` implied by `-D warnings`
2019-08-17T02:02:18.7856343Z    = help: remove it
2019-08-17T02:02:18.7856394Z 
2019-08-17T02:02:18.7856394Z 
2019-08-17T02:02:18.7856624Z error: `assert!(false)` should probably be replaced
2019-08-17T02:02:18.7857323Z    |
2019-08-17T02:02:18.7857323Z    |
2019-08-17T02:02:18.7857412Z LL |     assert!(false);
2019-08-17T02:02:18.7857595Z    |
2019-08-17T02:02:18.7857595Z    |
2019-08-17T02:02:18.7857673Z    = help: use `panic!()` or `unreachable!()`
2019-08-17T02:02:18.7857704Z 
2019-08-17T02:02:18.7857767Z error: `assert!(true)` will be optimized out by the compiler
2019-08-17T02:02:18.7859758Z    |
2019-08-17T02:02:18.7859758Z    |
2019-08-17T02:02:18.7860040Z LL |     assert!(true, "true message");
2019-08-17T02:02:18.7860398Z    |
2019-08-17T02:02:18.7860467Z    = help: remove it
2019-08-17T02:02:18.7860648Z 
2019-08-17T02:02:18.7860648Z 
2019-08-17T02:02:18.7860819Z error: `assert!(false)` should probably be replaced
2019-08-17T02:02:18.7861267Z    |
2019-08-17T02:02:18.7861267Z    |
2019-08-17T02:02:18.7861310Z LL |     assert!(false, "false message");
2019-08-17T02:02:18.7861418Z    |
2019-08-17T02:02:18.7861418Z    |
2019-08-17T02:02:18.7861462Z    = help: use `panic!()` or `unreachable!()`
2019-08-17T02:02:18.7861639Z 
2019-08-17T02:02:18.7862102Z error: `assert!(true)` will be optimized out by the compiler
2019-08-17T02:02:18.7862652Z    |
2019-08-17T02:02:18.7862924Z LL |     assert!(B);
2019-08-17T02:02:18.7863019Z    |     ^^^^^^^^^^^
2019-08-17T02:02:18.7863073Z    |
2019-08-17T02:02:18.7863073Z    |
2019-08-17T02:02:18.7863135Z    = help: remove it
2019-08-17T02:02:18.7863163Z 
2019-08-17T02:02:18.7863362Z error: `assert!(false)` should probably be replaced
2019-08-17T02:02:18.7863953Z    |
2019-08-17T02:02:18.7864042Z LL |     assert!(C);
2019-08-17T02:02:18.7864128Z    |     ^^^^^^^^^^^
2019-08-17T02:02:18.7864171Z    |
2019-08-17T02:02:18.7864171Z    |
2019-08-17T02:02:18.7883923Z    = help: use `panic!()` or `unreachable!()`
2019-08-17T02:02:18.7883987Z 
2019-08-17T02:02:18.7884304Z error: `assert!(true)` will be optimized out by the compiler
2019-08-17T02:02:18.7884924Z    |
2019-08-17T02:02:18.7884924Z    |
2019-08-17T02:02:18.7884967Z LL |     debug_assert!(true);
2019-08-17T02:02:18.7885066Z    |
2019-08-17T02:02:18.7885106Z    = help: remove it
2019-08-17T02:02:18.7885456Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-08-17T02:02:18.7885523Z 
2019-08-17T02:02:18.7885523Z 
2019-08-17T02:02:18.7885572Z error: aborting due to 7 previous errors
2019-08-17T02:02:18.7885600Z 
2019-08-17T02:02:18.7885623Z 
2019-08-17T02:02:18.7885657Z 
2019-08-17T02:02:18.7885697Z diff of stderr:
2019-08-17T02:02:18.7885722Z 
2019-08-17T02:02:18.7886023Z  error: `assert!(true)` will be optimized out by the compiler
2019-08-17T02:02:18.7894878Z     |
2019-08-17T02:02:18.7894878Z     |
2019-08-17T02:02:18.7894975Z  LL |     assert!(true);
2019-08-17T02:02:18.7895167Z     |
2019-08-17T02:02:18.7896000Z     = note: `-D clippy::assertions-on-constants` implied by `-D warnings`
2019-08-17T02:02:18.7896075Z     = help: remove it
2019-08-17T02:02:18.7896113Z  
2019-08-17T02:02:18.7896113Z  
2019-08-17T02:02:18.7896158Z  error: `assert!(false)` should probably be replaced
2019-08-17T02:02:18.7896565Z     |
2019-08-17T02:02:18.7896565Z     |
2019-08-17T02:02:18.7896608Z  LL |     assert!(false);
2019-08-17T02:02:18.7896699Z     |
2019-08-17T02:02:18.7896699Z     |
2019-08-17T02:02:18.7896740Z     = help: use `panic!()` or `unreachable!()`
2019-08-17T02:02:18.7897800Z  
2019-08-17T02:02:18.7900186Z  error: `assert!(true)` will be optimized out by the compiler
2019-08-17T02:02:18.7900751Z     |
2019-08-17T02:02:18.7900751Z     |
2019-08-17T02:02:18.7900798Z  LL |     assert!(true, "true message");
2019-08-17T02:02:18.7900897Z     |
2019-08-17T02:02:18.7900960Z     = help: remove it
2019-08-17T02:02:18.7901000Z  
2019-08-17T02:02:18.7901000Z  
2019-08-17T02:02:18.7901045Z  error: `assert!(false)` should probably be replaced
2019-08-17T02:02:18.7901366Z     |
2019-08-17T02:02:18.7901366Z     |
2019-08-17T02:02:18.7901663Z  LL |     assert!(false, "false message");
2019-08-17T02:02:18.7902199Z     |
2019-08-17T02:02:18.7902199Z     |
2019-08-17T02:02:18.7902294Z     = help: use `panic!()` or `unreachable!()`
2019-08-17T02:02:18.7902361Z  
2019-08-17T02:02:18.7902451Z  error: `assert!(true)` will be optimized out by the compiler
2019-08-17T02:02:18.7903306Z     |
2019-08-17T02:02:18.7903391Z  LL |     assert!(B);
2019-08-17T02:02:18.7903465Z     |     ^^^^^^^^^^^
2019-08-17T02:02:18.7903525Z     |
2019-08-17T02:02:18.7903525Z     |
2019-08-17T02:02:18.7903600Z     = help: remove it
2019-08-17T02:02:18.7903640Z  
2019-08-17T02:02:18.7903732Z  error: `assert!(false)` should probably be replaced
2019-08-17T02:02:18.7904448Z     |
2019-08-17T02:02:18.7904559Z  LL |     assert!(C);
2019-08-17T02:02:18.7904602Z     |     ^^^^^^^^^^^
2019-08-17T02:02:18.7904640Z     |
2019-08-17T02:02:18.7904640Z     |
2019-08-17T02:02:18.7904843Z     = help: use `panic!()` or `unreachable!()`
2019-08-17T02:02:18.7904948Z  
2019-08-17T02:02:18.7905253Z -error: `assert!(true)` will be optimized out by the compiler
2019-08-17T02:02:18.7905926Z -   |
2019-08-17T02:02:18.7905926Z -   |
2019-08-17T02:02:18.7906371Z -LL |     debug_assert!(true);
2019-08-17T02:02:18.7907258Z -   |
2019-08-17T02:02:18.7907685Z -   = help: remove it
2019-08-17T02:02:18.7908733Z -   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-08-17T02:02:18.7909253Z -
---
2019-08-17T02:02:18.7911746Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base' 'assertions_on_constants.rs'
2019-08-17T02:02:18.7911830Z 
2019-08-17T02:02:18.7912074Z error: 1 errors occurred comparing output.
2019-08-17T02:02:18.7912188Z status: exit code: 1
2019-08-17T02:02:18.7914013Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/assertions_on_constants.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base/assertions_on_constants.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-982af386409bbff7.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-78c997e47ed3a519.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-0752014f02199afe.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-56202a0d93acfa8d/out/test_build_base/assertions_on_constants.stage-id.aux" "-A" "unused"
2019-08-17T02:02:18.7914814Z ------------------------------------------
2019-08-17T02:02:18.7915071Z 
2019-08-17T02:02:18.7915427Z ------------------------------------------
2019-08-17T02:02:18.7915682Z stderr:
2019-08-17T02:02:18.7915682Z stderr:
2019-08-17T02:02:18.7915992Z ------------------------------------------
2019-08-17T02:02:18.7918653Z {"message":"`assert!(true)` will be optimized out by the compiler","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":135,"byte_end":149,"line_start":9,"line_end":9,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    assert!(true);","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":135,"byte_end":149,"line_start":9,"line_end":9,"column_start":5,"column_end":19,"is_primary":false,"text":[{"text":"    assert!(true);","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"<::core::macros::builtin::assert macros>","byte_start":0,"byte_end":99,"line_start":1,"line_end":2,"column_start":1,"column_end":44,"is_primary":false,"text":[{"text":"($ cond : expr) => ({ }) ; ($ cond : expr ,) => ({ }) ;","highlight_start":1,"highlight_end":56},{"text":"($ cond : expr , $ ($ arg : tt) +) => ({ })","highlight_start":1,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::assertions-on-constants` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove it","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(true)` will be optimized out by the compiler\n  --> tests/ui/assertions_on_constants.rs:9:5\n   |\nLL |     assert!(true);\n   |     ^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::assertions-on-constants` implied by `-D warnings`\n   = help: remove it\n\n"}
2019-08-17T02:02:18.7921490Z {"message":"`assert!(false)` should probably be replaced","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":154,"byte_end":169,"line_start":10,"line_end":10,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    assert!(false);","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":154,"byte_end":169,"line_start":10,"line_end":10,"column_start":5,"column_end":20,"is_primary":false,"text":[{"text":"    assert!(false);","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"<::core::macros::builtin::assert macros>","byte_start":0,"byte_end":99,"line_start":1,"line_end":2,"column_start":1,"column_end":44,"is_primary":false,"text":[{"text":"($ cond : expr) => ({ }) ; ($ cond : expr ,) => ({ }) ;","highlight_start":1,"highlight_end":56},{"text":"($ cond : expr , $ ($ arg : tt) +) => ({ })","highlight_start":1,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"use `panic!()` or `unreachable!()`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(false)` should probably be replaced\n  --> tests/ui/assertions_on_constants.rs:10:5\n   |\nLL |     assert!(false);\n   |     ^^^^^^^^^^^^^^^\n   |\n   = help: use `panic!()` or `unreachable!()`\n\n"}
2019-08-17T02:02:18.7924685Z {"message":"`assert!(true)` will be optimized out by the compiler","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":174,"byte_end":204,"line_start":11,"line_end":11,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    assert!(true, \"true message\");","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":174,"byte_end":204,"line_start":11,"line_end":11,"column_start":5,"column_end":35,"is_primary":false,"text":[{"text":"    assert!(true, \"true message\");","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"<::core::macros::builtin::assert macros>","byte_start":0,"byte_end":99,"line_start":1,"line_end":2,"column_start":1,"column_end":44,"is_primary":false,"text":[{"text":"($ cond : expr) => ({ }) ; ($ cond : expr ,) => ({ }) ;","highlight_start":1,"highlight_end":56},{"text":"($ cond : expr , $ ($ arg : tt) +) => ({ })","highlight_start":1,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"remove it","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(true)` will be optimized out by the compiler\n  --> tests/ui/assertions_on_constants.rs:11:5\n   |\nLL |     assert!(true, \"true message\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: remove it\n\n"}
2019-08-17T02:02:18.7928604Z {"message":"`assert!(false)` should probably be replaced","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":209,"byte_end":241,"line_start":12,"line_end":12,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    assert!(false, \"false message\");","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":209,"byte_end":241,"line_start":12,"line_end":12,"column_start":5,"column_end":37,"is_primary":false,"text":[{"text":"    assert!(false, \"false message\");","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"<::core::macros::builtin::assert macros>","byte_start":0,"byte_end":99,"line_start":1,"line_end":2,"column_start":1,"column_end":44,"is_primary":false,"text":[{"text":"($ cond : expr) => ({ }) ; ($ cond : expr ,) => ({ }) ;","highlight_start":1,"highlight_end":56},{"text":"($ cond : expr , $ ($ arg : tt) +) => ({ })","highlight_start":1,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"use `panic!()` or `unreachable!()`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(false)` should probably be replaced\n  --> tests/ui/assertions_on_constants.rs:12:5\n   |\nLL |     assert!(false, \"false message\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: use `panic!()` or `unreachable!()`\n\n"}
2019-08-17T02:02:18.7931384Z {"message":"`assert!(true)` will be optimized out by the compiler","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":273,"byte_end":284,"line_start":15,"line_end":15,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    assert!(B);","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":273,"byte_end":284,"line_start":15,"line_end":15,"column_start":5,"column_end":16,"is_primary":false,"text":[{"text":"    assert!(B);","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"<::core::macros::builtin::assert macros>","byte_start":0,"byte_end":99,"line_start":1,"line_end":2,"column_start":1,"column_end":44,"is_primary":false,"text":[{"text":"($ cond : expr) => ({ }) ; ($ cond : expr ,) => ({ }) ;","highlight_start":1,"highlight_end":56},{"text":"($ cond : expr , $ ($ arg : tt) +) => ({ })","highlight_start":1,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"remove it","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(true)` will be optimized out by the compiler\n  --> tests/ui/assertions_on_constants.rs:15:5\n   |\nLL |     assert!(B);\n   |     ^^^^^^^^^^^\n   |\n   = help: remove it\n\n"}
2019-08-17T02:02:18.7934208Z {"message":"`assert!(false)` should probably be replaced","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":317,"byte_end":328,"line_start":18,"line_end":18,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    assert!(C);","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":317,"byte_end":328,"line_start":18,"line_end":18,"column_start":5,"column_end":16,"is_primary":false,"text":[{"text":"    assert!(C);","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"<::core::macros::builtin::assert macros>","byte_start":0,"byte_end":99,"line_start":1,"line_end":2,"column_start":1,"column_end":44,"is_primary":false,"text":[{"text":"($ cond : expr) => ({ }) ; ($ cond : expr ,) => ({ }) ;","highlight_start":1,"highlight_end":56},{"text":"($ cond : expr , $ ($ arg : tt) +) => ({ })","highlight_start":1,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"use `panic!()` or `unreachable!()`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(false)` should probably be replaced\n  --> tests/ui/assertions_on_constants.rs:18:5\n   |\nLL |     assert!(C);\n   |     ^^^^^^^^^^^\n   |\n   = help: use `panic!()` or `unreachable!()`\n\n"}
2019-08-17T02:02:18.7934832Z 
2019-08-17T02:02:18.7935138Z ------------------------------------------
2019-08-17T02:02:18.7935192Z 
2019-08-17T02:02:18.7935546Z thread '[ui] ui/assertions_on_constants.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
---
2019-08-17T02:15:03.0109478Z    Compiling rustc-ap-syntax_pos v546.0.0
2019-08-17T02:16:10.8102373Z    Compiling rustc-ap-rustc_errors v546.0.0
2019-08-17T02:21:33.7256127Z    Compiling racer v2.1.25
2019-08-17T02:27:27.6325744Z     Finished release [optimized] target(s) in 25m 08s
2019-08-17T02:27:27.6753920Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2019-08-17T02:27:27.6754424Z 
2019-08-17T02:27:27.6754698Z the following dependencies are duplicated although they have the same features enabled:
2019-08-17T02:27:27.6755931Z the following dependencies have different features:
2019-08-17T02:27:27.6757329Z   url 2.1.0 (registry+https://github.com/rust-lang/crates.io-index)
2019-08-17T02:27:27.6758673Z     `rls` additionally enabled features {"serde"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liburl-c2632d236f82822a.rlib"
2019-08-17T02:27:27.6760850Z     `clippy-driver` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liburl-d0557b06f98d57d5.rlib"
2019-08-17T02:27:27.6762151Z 
2019-08-17T02:27:27.6763688Z to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
2019-08-17T02:27:27.6768495Z thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:198:13
2019-08-17T02:27:27.6826683Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --no-fail-fast src/doc/book src/doc/nomicon src/doc/reference src/doc/rust-by-example src/doc/embedded-book src/doc/edition-guide src/doc/rustc-guide src/tools/clippy src/tools/rls src/tools/rustfmt src/tools/miri
2019-08-17T02:27:27.6827199Z Build completed unsuccessfully in 2:35:15
2019-08-17T02:27:27.6894815Z {"rustfmt":"test-pass","reference":"test-pass","clippy-driver":"test-fail","edition-guide":"test-pass","book":"test-pass","nomicon":"test-pass","embedded-book":"test-pass","rustc-guide":"test-fail","rls":"test-pass","miri":"test-pass","rust-by-example":"test-pass","rustbook":"test-fail"}
2019-08-17T02:27:27.6898144Z Verifying status of book...
---
2019-08-17T02:27:27.6983074Z Verifying status of rustfmt...
2019-08-17T02:27:27.6999404Z Verifying status of clippy-driver...
2019-08-17T02:27:27.7013481Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-08-17T02:27:27.7022752Z 
2019-08-17T02:27:27.7033486Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-08-17T02:27:27.7033611Z 
2019-08-17T02:27:27.7033993Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-08-17T02:27:27.7034108Z commit another update.
2019-08-17T02:27:27.7034143Z 
2019-08-17T02:27:27.7034462Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-08-17T02:27:27.7034768Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-08-17T02:27:27.7034850Z proper steps.
2019-08-17T02:27:27.7045031Z   local time: Sat Aug 17 02:27:27 UTC 2019
2019-08-17T02:27:27.8661991Z   network time: Sat, 17 Aug 2019 02:27:27 GMT
2019-08-17T02:27:27.8665017Z == end clock drift check ==
2019-08-17T02:27:27.8665017Z == end clock drift check ==
2019-08-17T02:27:30.0727707Z ##[error]Bash exited with code '3'.
2019-08-17T02:27:30.0773706Z ##[section]Starting: Checkout
2019-08-17T02:27:30.0775492Z ==============================================================================
2019-08-17T02:27:30.0775543Z Task         : Get sources
2019-08-17T02:27:30.0775607Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
