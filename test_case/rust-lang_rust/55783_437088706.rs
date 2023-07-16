plain
travis_time:end:1618a93e:start=1541694684574630381,finish=1541694685736809147,duration=1162178766
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:54:03] .................................................................................................... 100/4999
[00:54:07] .................................................................................................... 200/4999
[00:54:09] ........................................................................ii...................ii..... 300/4999
[00:54:12] ...........................................................................................iii...... 400/4999
[00:54:15] ..iiiiiiii.iii...........................iii...........................................i...........i 500/4999
[00:54:22] .................................................................................................... 700/4999
[00:54:29] .....................................................................i...........i.................. 800/4999
[00:54:32] ........................................................................................iiii.i...... 900/4999
[00:54:35] ...........ii.iiii.................................................................................. 1000/4999
---
[00:55:12] .................................................................................................... 2200/4999
[00:55:16] .................................................................................................... 2300/4999
[00:55:20] .................................................................................................... 2400/4999
[00:55:24] .................................................................................................... 2500/4999
[00:55:28] ...................................................................iiiiiiiii........................ 2600/4999
[00:55:35] ...............................ii................................................................... 2800/4999
[00:55:38] .................................................................................................... 2900/4999
[00:55:42] .................................................................................................... 3000/4999
[00:55:45] ..........................i......................................................................... 3100/4999
---
[00:58:02] .................................................................................................... 700/2881
[00:58:13] .................................................................................................... 800/2881
[00:58:23] .................................................................................................... 900/2881
[00:58:38] .................................................................................................... 1000/2881
[00:58:51] ......................................................F............................................. 1100/2881
[00:59:09] .................................................................................................... 1300/2881
[00:59:22] ............................................................................i....................... 1400/2881
[00:59:34] .................................................................................................... 1500/2881
[00:59:47] .............................................i...................................................... 1600/2881
---
[01:02:57] failures:
[01:02:57] 
[01:02:57] ---- [run-pass] run-pass/issues/issue-13494.rs stdout ----
[01:02:57] normalized stderr:
[01:02:57] warning: use of deprecated item 'std::sync::mpsc::Select': channel selection will be removed in a future release
[01:02:57]    |
[01:02:57]    |
[01:02:57] LL | /         select! {
[01:02:57] LL | |             _ = rx2.recv() => (),
[01:02:57] LL | |             _ = rcv.recv() => ()
[01:02:57] LL | |         }
[01:02:57]    |
[01:02:57]    = note: #[warn(deprecated)] on by default
[01:02:57]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:02:57] 
[01:02:57] 
[01:02:57] warning: use of deprecated item 'std::sync::mpsc::Select': channel selection will be removed in a future release
[01:02:57]    |
[01:02:57]    |
[01:02:57] LL | /         select! {
[01:02:57] LL | |             _ = rx2.recv() => (),
[01:02:57] LL | |             _ = rcv.recv() => ()
[01:02:57] LL | |         }
[01:02:57]    |
[01:02:57]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:02:57] 
[01:02:57] 
[01:02:57] warning: use of deprecated item 'std::sync::mpsc::Select::new': channel selection will be removed in a future release
[01:02:57]    |
[01:02:57]    |
[01:02:57] LL | /         select! {
[01:02:57] LL | |             _ = rx2.recv() => (),
[01:02:57] LL | |             _ = rcv.recv() => ()
[01:02:57] LL | |         }
[01:02:57]    |
[01:02:57]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:02:57] 
[01:02:57] 
[01:02:57] warning: use of deprecated item 'std::sync::mpsc::Select::handle': channel selection will be removed in a future release
[01:02:57]    |
[01:02:57]    |
[01:02:57] LL | /         select! {
[01:02:57] LL | |             _ = rx2.recv() => (),
[01:02:57] LL | |             _ = rcv.recv() => ()
[01:02:57] LL | |         }
[01:02:57]    |
[01:02:57]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:02:57] 
[01:02:57] 
[01:02:57] warning: use of deprecated item '<std::sync::mpsc::Handle<'rx, T>>::add': channel selection will be removed in a future release
[01:02:57]    |
[01:02:57]    |
[01:02:57] LL | /         select! {
[01:02:57] LL | |             _ = rx2.recv() => (),
[01:02:57] LL | |             _ = rcv.recv() => ()
[01:02:57] LL | |         }
[01:02:57]    |
[01:02:57]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:02:57] 
[01:02:57] 
[01:02:57] warning: use of deprecated item 'std::sync::mpsc::Select::wait': channel selection will be removed in a future release
[01:02:57]    |
[01:02:57]    |
[01:02:57] LL | /         select! {
[01:02:57] LL | |             _ = rx2.recv() => (),
[01:02:57] LL | |             _ = rcv.recv() => ()
[01:02:57] LL | |         }
[01:02:57]    |
[01:02:57]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:02:57] 
[01:02:57] 
[01:02:57] warning: use of deprecated item '<std::sync::mpsc::Handle<'rx, T>>::id': channel selection will be removed in a future release
[01:02:57]    |
[01:02:57]    |
[01:02:57] LL | /         select! {
[01:02:57] LL | |             _ = rx2.recv() => (),
[01:02:57] LL | |             _ = rcv.recv() => ()
[01:02:57] LL | |         }
[01:02:57]    |
[01:02:57]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:02:57] 
[01:02:57] 
[01:02:57] warning: use of deprecated item '<std::sync::mpsc::Handle<'rx, T>>::recv': channel selection will be removed in a future release
[01:02:57]    |
[01:02:57]    |
[01:02:57] LL |             _ = rx2.recv() => (),
[01:02:57] 
[01:02:57] 
[01:02:57] warning: use of deprecated item '<std::sync::mpsc::Handle<'rx, T>>::recv': channel selection will be removed in a future release
[01:02:57]    |
[01:02:57]    |
[01:02:57] LL |             _ = rcv.recv() => ()
[01:02:57] 
[01:02:57] 
[01:02:57] 
[01:02:57] 
[01:02:57] 
[01:02:57] The actual stderr differed from the expected stderr.
[01:02:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-13494/issue-13494.stderr
[01:02:57] To update references, rerun the tests and pass the `--bless` flag
[01:02:57] To only update this specific test, also pass `--test-args issues/issue-13494.rs`
[01:02:57] error: 1 errors occurred comparing output.
[01:02:57] status: exit code: 0
[01:02:57] status: exit code: 0
[01:02:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-13494.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-13494/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-13494/auxiliary"
[01:02:57] ------------------------------------------
[01:02:57] 
[01:02:57] ------------------------------------------
[01:02:57] stderr:
[01:02:57] stderr:
[01:02:57] ------------------------------------------
[01:02:57] {"message":"use of deprecated item 'std::sync::mpsc::Select': channel selection will be removed in a future release","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"<::std::macros::select macros>","byte_start":138,"byte_end":151,"line_start":4,"line_end":4,"column_start":51,"column_end":64,"is_primary":true,"text":[{"text":"use $ crate :: sync :: mpsc :: Select ; let sel = Select :: new (  ) ; $ (","highlight_start":51,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass/issues/issue-13494.rs","byte_start":1117,"byte_end":1203,"line_start":37,"line_end":40,"column_start":9,"column_end":10,"is_primary":false,"text":[{"text":"        select! {","highlight_start":9,"highlight_end":18},{"text":"            _ = rx2.recv() => (),","highlight_start":1,"highlight_end":34},{"text":"            _ = rcv.recv() => ()","highlight_start":1,"highlight_end":33},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"select!","def_site_span":{"file_name":"<::std::macros::select macros>","byte_start":0,"byte_end":382,"line_start":1,"line_end":8,"column_start":1,"column_end":27,"is_primary":false,"text":[{"text":"( $ ( $ name : pat = $ rx : ident . $ meth : ident (  ) => $ code : expr ) , +","highlight_start":1,"highlight_end":79},{"text":") => (","highlight_start":1,"highlight_end":7},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"use $ crate :: sync :: mpsc :: Select ; let sel = Select :: new (  ) ; $ (","highlight_start":1,"highlight_end":75},{"text":"let mut $ rx = sel . handle ( & $ rx ) ; ) + unsafe {","highlight_start":1,"highlight_end":54},{"text":"$ ( $ rx . add (  ) ; ) + } let ret = sel . wait (  ) ; $ (","highlight_start":1,"highlight_end":60},{"text":"if ret == $ rx . id (  ) { let $ name = $ rx . $ meth (  ) ; $ code } else ) +","highlight_start":1,"highlight_end":79},{"text":"{ unreachable ! (  ) } } )","highlight_start":1,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"#[warn(deprecated)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: use of deprecated item 'std::sync::mpsc::Select': channel selection will be removed in a future release\n  --> /checkout/src/test/run-pass/issues/issue-13494.rs:37:9\n   |\nLL | /         select! {\nLL | |             _ = rx2.recv() => (),\nLL | |             _ = rcv.recv() => ()\nLL | |         }\n   | |_________^\n   |\n   = note: #[warn(deprecated)] on by default\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:02:57] {"message":"use of deprecated item 'std::sync::mpsc::Select': channel selection will be removed in a future release","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"<::std::macros::select macros>","byte_start":92,"byte_end":125,"line_start":4,"line_end":4,"column_start":5,"column_end":38,"is_primary":true,"text":[{"text":"use $ crate :: sync :: mpsc :: Select ; let sel = Select :: new (  ) ; $ (","highlight_start":5,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass/issues/issue-13494.rs","byte_start":1117,"byte_end":1203,"line_start":37,"line_end":40,"column_start":9,"column_end":10,"is_primary":false,"text":[{"text":"        select! {","highlight_start":9,"highlight_end":18},{"text":"            _ = rx2.recv() => (),","highlight_start":1,"highlight_end":34},{"text":"            _ = rcv.recv() => ()","highlight_start":1,"highlight_end":33},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"select!","def_site_span":{"file_name":"<::std::macros::select macros>","byte_start":0,"byte_end":382,"line_start":1,"line_end":8,"column_start":1,"column_end":27,"is_primary":false,"text":[{"text":"( $ ( $ name : pat = $ rx : ident . $ meth : ident (  ) => $ code : expr ) , +","highlight_start":1,"highlight_end":79},{"text":") => (","highlight_start":1,"highlight_end":7},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"use $ crate :: sync :: mpsc :: Select ; let sel = Select :: new (  ) ; $ (","highlight_start":1,"highlight_end":75},{"text":"let mut $ rx = sel . handle ( & $ rx ) ; ) + unsafe {","highlight_start":1,"highlight_end":54},{"text":"$ ( $ rx . add (  ) ; ) + } let ret = sel . wait (  ) ; $ (","highlight_start":1,"highlight_end":60},{"text":"if ret == $ rx . id (  ) { let $ name = $ rx . $ meth (  ) ; $ code } else ) +","highlight_start":1,"highlight_end":79},{"text":"{ unreachable ! (  ) } } )","highlight_start":1,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"warning: use of deprecated item 'std::sync::mpsc::Select': channel selection will be removed in a future release\n  --> /checkout/src/test/run-pass/issues/issue-13494.rs:37:9\n   |\nLL | /         select! {\nLL | |             _ = rx2.recv() => (),\nLL | |             _ = rcv.recv() => ()\nLL | |         }\n   | |_________^\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:02:57] {"message":"use of deprecated item 'std::sync::mpsc::Select::new': channel selection will be removed in a future release","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"<::std::macros::select macros>","byte_start":138,"byte_end":151,"line_start":4,"line_end":4,"column_start":51,"column_end":64,"is_primary":true,"text":[{"text":"use $ crate :: sync :: mpsc :: Select ; let sel = Select :: new (  ) ; $ (","highlight_start":51,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass/issues/issue-13494.rs","byte_start":1117,"byte_end":1203,"line_start":37,"line_end":40,"column_start":9,"column_end":10,"is_primary":false,"text":[{"text":"        select! {","highlight_start":9,"highlight_end":18},{"text":"            _ = rx2.recv() => (),","highlight_start":1,"highlight_end":34},{"text":"            _ = rcv.recv() => ()","highlight_start":1,"highlight_end":33},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"select!","def_site_span":{"file_name":"<::std::macros::select macros>","byte_start":0,"byte_end":382,"line_start":1,"line_end":8,"column_start":1,"column_end":27,"is_primary":false,"text":[{"text":"( $ ( $ name : pat = $ rx : ident . $ meth : ident (  ) => $ code : expr ) , +","highlight_start":1,"highlight_end":79},{"text":") => (","highlight_start":1,"highlight_end":7},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"use $ crate :: sync :: mpsc :: Select ; let sel = Select :: new (  ) ; $ (","highlight_start":1,"highlight_end":75},{"text":"let mut $ rx = sel . handle ( & $ rx ) ; ) + unsafe {","highlight_start":1,"highlight_end":54},{"text":"$ ( $ rx . add (  ) ; ) + } let ret = sel . wait (  ) ; $ (","highlight_start":1,"highlight_end":60},{"text":"if ret == $ rx . id (  ) { let $ name = $ rx . $ meth (  ) ; $ code } else ) +","highlight_start":1,"highlight_end":79},{"text":"{ unreachable ! (  ) } } )","highlight_start":1,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"warning: use of deprecated item 'std::sync::mpsc::Select::new': channel selection will be removed in a future release\n  --> /checkout/src/test/run-pass/issues/issue-13494.rs:37:9\n   |\nLL | /         select! {\nLL | |             _ = rx2.recv() => (),\nLL | |             _ = rcv.recv() => ()\nLL | |         }\n   | |_________^\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:02:57] {"message":"use of deprecated item 'std::sync::mpsc::Select::handle': channel selection will be removed in a future release","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"<::std::macros::select macros>","byte_start":184,"byte_end":190,"line_start":5,"line_end":5,"column_start":22,"column_end":28,"is_primary":true,"text":[{"text":"let mut $ rx = sel . handle ( & $ rx ) ; ) + unsafe {","highlight_start":22,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass/issues/issue-13494.rs","byte_start":1117,"byte_end":1203,"line_start":37,"line_end":40,"column_start":9,"column_end":10,"is_primary":false,"text":[{"text":"        select! {","highlight_start":9,"highlight_end":18},{"text":"            _ = rx2.recv() => (),","highlight_start":1,"highlight_end":34},{"text":"            _ = rcv.recv() => ()","highlight_start":1,"highlight_end":33},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"select!","def_site_span":{"file_name":"<::std::macros::select macros>","byte_start":0,"byte_end":382,"line_start":1,"line_end":8,"column_start":1,"column_end":27,"is_primary":false,"text":[{"text":"( $ ( $ name : pat = $ rx : ident . $ meth : ident (  ) => $ code : expr ) , +","highlight_start":1,"highlight_end":79},{"text":") => (","highlight_start":1,"highlight_end":7},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"use $ crate :: sync :: mpsc :: Select ; let sel = Select :: new (  ) ; $ (","highlight_start":1,"highlight_end":75},{"text":"let mut $ rx = sel . handle ( & $ rx ) ; ) + unsafe {","highlight_start":1,"highlight_end":54},{"text":"$ ( $ rx . add (  ) ; ) + } let ret = sel . wait (  ) ; $ (","highlight_start":1,"highlight_end":60},{"text":"if ret == $ rx . id (  ) { let $ name = $ rx . $ meth (  ) ; $ code } else ) +","highlight_start":1,"highlight_end":79},{"text":"{ unreachable ! (  ) } } )","highlight_start":1,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"warning: use of deprecated item 'std::sync::mpsc::Select::handle': channel selection will be removed in a future release\n  --> /checkout/src/test/run-pass/issues/issue-13494.rs:37:9\n   |\nLL | /         select! {\nLL | |             _ = rx2.recv() => (),\nLL | |             _ = rcv.recv() => ()\nLL | |         }\n   | |_________^\n   |\n   = note: this> (","highlight_start":1,"highlight_end":7},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"use $ crate :: sync :: mpsc :: Select ; let sel = Select :: new (  ) ; $ (","highlight_start":1,"highlight_end":75},{"text":"let mut $ rx = sel . handle ( & $ rx ) ; ) + unsafe {","highlight_start":1,"highlight_end":54},{"text":"$ ( $ rx . add (  ) ; ) + } let ret = sel . wait (  ) ; $ (","highlight_start":1,"highlight_end":60},{"text":"if ret == $ rx . id (  ) { let $ name = $ rx . $ meth (  ) ; $ code } else ) +","highlight_start":1,"highlight_end":79},{"text":"{ unreachable ! (  ) } } )","highlight_start":1,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"warning: use of deprecated item '<std::sync::mpsc::Handle<'rx, T>>::add': channel selection will be removed in a future release\n  --> /checkout/src/test/run-pass/issues/issue-13494.rs:37:9\n   |\nLL | /         select! {\nLL | |             _ = rx2.recv() => (),\nLL | |             _ = rcv.recv() => ()\nLL | |         }\n   | |_________^\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:02:57] {"message":"use of deprecated item 'std::sync::mpsc::Select::wait': channel selection will be removed in a future release","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"<::std::macros::select macros>","byte_start":261,"byte_end":265,"line_start":6,"line_end":6,"column_start":45,"column_end":49,"is_primary":true,"text":[{"text":"$ ( $ rx . add (  ) ; ) + } let ret = sel . wait (  ) ; $ (","highlight_start":45,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass/issues/issue-13494.rs","byte_start":1117,"byte_end":1203,"line_start":37,"line_end":40,"column_start":9,"column_end":10,"is_primary":false,"text":[{"text":"        select! {","highlight_start":9,"highlight_end":18},{"text":"            _ = rx2.recv() => (),","highlight_start":1,"highlight_end":34},{"text":"            _ = rcv.recv() => ()","highlight_start":1,"highlight_end":33},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"select!","def_site_span":{"file_name":"<::std::macros::select macros>","byte_start":0,"byte_end":382,"line_start":1,"line_end":8,"column_start":1,"column_end":27,"is_primary":false,"text":[{"text":"( $ ( $ name : pat = $ rx : ident . $ meth : ident (  ) => $ code : expr ) , +","highlight_start":1,"highlight_end":79},{"text":") => (","highlight_start":1,"highlight_end":7},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"use $ crate :: sync :: mpsc :: Select ; let sel = Select :: new (  ) ; $ (","highlight_start":1,"highlight_end":75},{"text":"let mut $ rx = sel . handle ( & $ rx ) ; ) + unsafe {","highlight_start":1,"highlight_end":54},{"text":"$ ( $ rx . add (  ) ; ) + } let ret = sel . wait (  ) ; $ (","highlight_start":1,"highlight_end":60},{"text":"if ret == $ rx . id (  ) { let $ name = $ rx . $ meth (  ) ; $ code } else ) +","highlight_start":1,"highlight_end":79},{"text":"{ unreachable ! (  ) } } )","highlight_start":1,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"warning: use of deprecated item 'std::sync::mpsc::Select::wait': channel selection will be removed in a future release\n  --> /checkout/src/test/run-pass/issues/issue-13494.rs:37:9\n   |\nLL | /         select! {\nLL | |             _ = rx2.recv() => (),\nLL | |             _ = rcv.recv() => ()\nLL | |         }\n   | |_________^\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:02:57] {"message":"use of deprecated item '<std::sync::mpsc::Handle<'rx, T>>::id': channel selection will be removed in a future release","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"<::std::macros::select macros>","byte_start":294,"byte_end":296,"line_start":7,"line_end":7,"column_start":18,"column_end":20,"is_primary":true,"text":[{"text":"if ret == $ rx . id (  ) { let $ name = $ rx . $ meth (  ) ; $ code } else ) +","highlight_start":18,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass/issues/issue-13494.rs","byte_start":1117,"byte_end":1203,"line_start":37,"line_end":40,"column_start":9,"column_end":10,"is_primary":false,"text":[{"text":"        select! {","highlight_start":9,"highlight_end":18},{"text":"            _ = rx2.recv() => (),","highlight_start":1,"highlight_end":34},{"text":"            _ = rcv.recv() => ()","highlight_start":1,"highlight_end":33},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"select!","def_site_span":{"file_name":"<::std::macros::select macros>","byte_start":0,"byte_end":382,"line_start":1,"line_end":8,"column_start":1,"column_end":27,"is_primary":false,"text":[{"text":"( $ ( $ name : pat = $ rx : ident . $ meth : ident (  ) => $ code : expr ) , +","highlight_start":1,"highlight_end":79},{"text":") => (","highlight_start":1,"highlight_end":7},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"use $ crate :: sync :: mpsc :: Select ; let sel = Select :: new (  ) ; $ (","highlight_start":1,"highlight_end":75},{"text":"let mut $ rx = sel . handle ( & $ rx ) ; ) + unsafe {","highlight_start":1,"highlight_end":54},{"text":"$ ( $ rx . add (  ) ; ) + } let ret = sel . wait (  ) ; $ (","highlight_start":1,"highlight_end":60},{"text":"if ret == $ rx . id (  ) { let $ name = $ rx . $ meth (  ) ; $ code } else ) +","highlight_start":1,"highlight_end":79},{"text":"{ unreachable ! (  ) } } )","highlight_start":1,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"warning: use of deprecated item '<std::sync::mpsc::Handle<'rx, T>>::id': channel selection will be removed in a future release\n  --> /checkout/src/test/run-pass/issues/issue-13494.rs:37:9\n   |\nLL | /         select! {\nLL | |             _ = rx2.recv() => (),\nLL | |             _ = rcv.recv() => ()\nLL | |         }\n   | |_________^\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:02:57] {"message":"use of deprecated item '<std::sync::mpsc::Handle<'rx, T>>::recv': channel selection will be removed in a future release","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-13494.rs","byte_start":1147,"byte_end":1151,"line_start":38,"line_end":38,"column_start":21,"column_end":25,"is_primary":true,"text":[{"text":"            _ = rx2.recv() => (),","highlight_start":21,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: use of deprecated item '<std::sync::mpsc::Handle<'rx, T>>::recv': channel selection will be removed in a future release\n  --> /checkout/src/test/run-pass/issues/issue-13494.rs:38:21\n   |\nLL |             _ = rx2.recv() => (),\n   |                     ^^^^\n\n"}
[01:02:57] {"message":"use of deprecated item '<std::sync::mpsc::Handle<'rx, T>>::recv': channel selection will be removed in a future release","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-13494.rs","byte_start":1181,"byte_end":1185,"line_start":39,"line_end":39,"column_start":21,"column_end":25,"is_primary":true,"text":[{"text":"            _ = rcv.recv() => ()","highlight_start":21,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicabilit-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:57] 
[01:02:57] 
[01:02:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:57] Build completed unsuccessfully in 0:12:44
[01:02:57] Build completed unsuccessfully in 0:12:44
[01:02:57] make: *** [check] Error 1
[01:02:57] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04ee27b8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:29b34506:start=1541698475994947291,finish=1541698476001096399,duration=6149108
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00686318
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:167049cb
travis_time:start:167049cb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23377320
$ dmesg | grep -i kill
