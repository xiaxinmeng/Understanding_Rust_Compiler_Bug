plain
travis_time:end:12ca660e:start=1541129290789811539,finish=1541129348751515096,duration=57961703557
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:51:05] .................................................................................................... 100/4984
[00:51:08] .................................................................................................... 200/4984
[00:51:11] ...........................................................................................ii....... 300/4984
[00:51:14] .........................................................................................iii........ 400/4984
[00:51:17] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4984
[00:51:24] .................................................................................................... 700/4984
[00:51:31] ..................................................................i...........i..................... 800/4984
[00:51:34] ....................................................................................iiiii........... 900/4984
[00:51:38] .................................................................................................... 1000/4984
---
[00:52:16] .................................................................................................... 2200/4984
[00:52:21] .................................................................................................... 2300/4984
[00:52:25] .................................................................................................... 2400/4984
[00:52:29] .................................................................................................... 2500/4984
[00:52:32] ...................................................................iiiiiiiii........................ 2600/4984
[00:52:40] ..................ii................................................................................ 2800/4984
[00:52:42] .................................................................................................... 2900/4984
[00:52:47] .................................................................................................... 3000/4984
[00:52:50] .............i...................................................................................... 3100/4984
---
[00:54:48] .................................................................................................... 600/2874
[00:55:03] .................................................................................................... 700/2874
[00:55:14] .................................................................................................... 800/2874
[00:55:23] .................................................................................................... 900/2874
[00:55:37] ............................F..F.................................................................... 1000/2874
[00:55:59] .................................................................................................... 1200/2874
[00:56:08] .................................................................................................... 1300/2874
[00:56:20] .........................................................................i.......................... 1400/2874
[00:56:32] .................................................................................................... 1500/2874
[00:56:32] .................................................................................................... 1500/2874
[00:56:45] ..........................................i......................................................... 1600/2874
[00:57:01] .................................................................................................... 1700/2874
[00:57:13] .........................F.......................................................................... 1800/2874
[00:57:35] .......................................i............................................................ 2000/2874
[00:57:57] .................................................................................................... 2100/2874
[00:58:05] .................................................................................................... 2200/2874
[00:58:21] .ii.....................................................................i....i...................... 2300/2874
[00:58:21] .ii.....................................................................i....i...................... 2300/2874
[00:58:36] ...............i.................................................................................... 2400/2874
[00:58:50] .................................................................................................... 2500/2874
[00:59:15] .................................................................................................... 2600/2874
[00:59:25] .................................................................................................... 2700/2874
[00:59:34] .................................................................................................... 2800/2874
rait/example-calendar.normal/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait/example-calendar.normal/auxiliary"
[00:59:45] ------------------------------------------
[00:59:45] 
[00:59:45] ------------------------------------------
[00:59:45] stderr:
[00:59:45] stderr:
[00:59:45] ------------------------------------------
[00:59:45] {"message":"unused `std::result::Result` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"<::core::macros::write macros>","byte_start":43,"byte_end":96,"line_start":2,"line_end":2,"column_start":1,"column_end":54,"is_primary":true,"text":[{"text":"$ dst . write_fmt ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass/impl-trait/example-calendar.rs","byte_start":8168,"byte_end":8187,"line_start":313,"line_end":313,"column_start":13,"column_end":32,"is_primary":false,"text":[{"text":"            write!(s, \"{}\", e);","highlight_start":13,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"<::core::macros::write macros>","byte_start":0,"byte_end":98,"line_start":1,"line_end":2,"column_start":1,"column_end":56,"is_primary":false,"text":[{"text":"( $ dst : expr , $ ( $ arg : tt ) * ) => (","highlight_start":1,"sed
[00:59:45]    |
[00:59:45]    |
[00:59:45] LL |     write!(&mut w, ""); // should coerce
[00:59:45]    |
[00:59:45]    |
[00:59:45]    = note: this `Result` may be an `Err` variant, which should be handled
[00:59:45] 
[00:59:45] 
[00:59:45] warning: unused `std::result::Result` that must be used
[00:59:45]    |
[00:59:45]    |
[00:59:45] LL |         write!(&mut s, "test");
[00:59:45]    |
[00:59:45]    |
[00:59:45]    = note: this `Result` may be an `Err` variant, which should be handled
[00:59:45] 
[00:59:45] 
[00:59:45] 
[00:59:45] 
[00:59:45] 
[00:59:45] The actual stderr differed from the expected stderr.
[00:59:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/colorful-write-macros/colorful-write-macros.stderr
[00:59:45] To update references, rerun the tests and pass the `--bless` flag
[00:59:45] To only update this specific test, also pass `--test-args macros/colorful-write-macros.rs`
[00:59:45] error: 1 errors occurred comparing output.
[00:59:45] status: exit code: 0
[00:59:45] status: exit code: 0
[00:59:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/macros/colorful-write-macros.rs" "--target=x86ghlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass/macros/colorful-write-macros.rs","byte_start":884,"byte_end":917,"line_start":35,"line_end":35,"column_start":5,"column_end":38,"is_primary":false,"text":[{"text":"    write!(&mut w as &mut Write, \"\");","highlight_start":5,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"<::core::macros::write macros>","byte_start":0,"byte_end":98,"line_start":1,"line_end":2,"column_start":1,"column_end":56,"is_primary":false,"text":[{"text":"( $ dst : expr , $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":43},{"text":"$ dst . write_fmt ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"this `Result` may be an `Err` variant, which should be handled","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused `std::result::Result` that must be used\n  --> /checkout/src/test/run-pass/macros/colorful-write-macros.rs:35:5\n   |\nLL |     write!(&mut w as &mut Write, \"\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this `Result` may be an `Err` variant, which should be handled\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:59:45] {s/colorful-write-macros.rs:36:5\n   |\nLL |     write!(&mut w, \"\"); // should coerce\n   |     ^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this `Result` may be an `Err` variant, which should be handled\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:59:45] {"message":"unused `std::result::Result` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"<::core::macros::write macros>","byte_start":43,"byte_end":96,"line_start":2,"line_end":2,"column_start":1,"column_end":54,"is_primary":true,"text":[{"text":"$ dst . write_fmt ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass/macros/colorful-write-macros.rs","byte_start":1044,"byte_end":1067,"line_start":42,"line_end":42,"column_start":9,"column_end":32,"is_primary":false,"text":[{"text":"        write!(&mut s, \"test\");","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"<::core::macros::write macros>","byte_start":0,"byte_end":98,"line_start":1,"line_end":2,"column_start":1,"column_end":56,"is_primary":false,"text":[{"text":"( $ dst : expr , $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":43},{"text":"$ dst . write_fmt ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":56}],"labe
