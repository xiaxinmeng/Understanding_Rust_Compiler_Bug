plain
travis_time:end:24cd9108:start=1545754790056409666,finish=1545754794583465919,duration=4527056253
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:39] 
[01:05:39] running 118 tests
[01:06:02] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:06:05] ......iii.i.....ii
[01:06:05] 
[01:06:05]  finished in 26.818
[01:06:05] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc-ui
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:34:49] 
[01:34:49] running 12 tests
ust add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[Uniooon::X]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:4:13\n   |\nLL |      //! , [Uniooon::X] and [Qux::Z].\n   |             ^^^^^^^^^^ cannot be resolved, ignoring\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:34:55] {"message":"`[Qux::Z]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":95,"byte_end":101,"line_start":4,"line_end":4,"column_start":30,"column_end":36,"is_primary":true,"text":[{"text":"     //! , [Uniooon::X] and [Qux::Z].","highlight_start":30,"highlight_end":36}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[Qux::Z]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:4:30\n   |\nLL |      //! , [Uniooon::X] and [Qux::Z].\n   |                              ^^^^^^ cannot be resolved, ignoring\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:34:55] {"message":"`[Uniooon::X]` cannot be resolved, ignoring it...","code":{"code":"in":[],"rendered":null}],"rendered":"warning: `[error]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:57:30\n   |\nLL |  * time to introduce a link [error]\n   |                              ^^^^^ cannot be resolved, ignoring\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:34:55] {"message":"`[error]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":781,"byte_end":811,"line_start":61,"line_end":61,"column_start":1,"column_end":31,"is_primary":true,"text":[{"text":"#[doc = \"single line [error]\"]","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the link appears in this line:\n\nsingle line [error]\n             ^^^^^","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:61:1\n   |\nLL | #[doc = \"single line [error]\"]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: the link appears in this line:\n           \n           single line [error]\n                        ^^^^^\n   = help: to escape `[` and `]` characters, just add '\\' before them r `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[BarB]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:20:9\n   |\nLL |  * bar [BarB] bar\n   |         ^^^^ cannot be resolved, ignoring\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:34:55] {"message":"`[BarC]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":333,"byte_end":337,"line_start":27,"line_end":27,"column_start":6,"column_end":10,"is_primary":true,"text":[{"text":"bar [BarC] bar","highlight_start":6,"highlight_end":10}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[BarC]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:27:6\n   |\nLL | bar [BarC] bar\n   |      ^^^^ cannot be resolved, ignoring\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:34:55] {"message":"`[BarD]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","bytelinux-gnu/stage0-bootstrap-tools/release
129876 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
121508 ./obj/build/bootstrap/debug/incremental/bootstrap-3br8tst6gztgx
121508 ./obj/build/bootstrap/debug/incremental/bootstrap-3br8tst6gztgx
121504 ./obj/build/bootstrap/debug/incremental/bootstrap-3br8tst6gztgx/s-f7xcjmwho4-19xhyzj-3k2japuzqtv9h
115352 ./src/llvm/test/CodeGen
112716 ./obj/build/x86_64-unknown-linux-gnu/stage1
112696 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
111160 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
