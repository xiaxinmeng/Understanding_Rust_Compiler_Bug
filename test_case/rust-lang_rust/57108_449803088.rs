plain
travis_time:end:15dfc5e7:start=1545709359333376857,finish=1545709413593476679,duration=54260099822
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
[01:05:00] 
[01:05:00] running 118 tests
[01:05:23] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:05:26] ......iii.i.....ii
[01:05:26] 
[01:05:26]  finished in 26.106
[01:05:26] travis_fold:end:test_debuginfo

---
[01:33:28] running 12 tests
 link appears in this line:
[01:33:33] - 
[01:33:33] +            
[01:33:33] 160            bar [BarD] bar
[01:33:33] 161                 ^^^^
[01:33:33] 162    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:33:33] 171    | ------------------------------- in this macro invocation
[01:33:33] 172    |
[01:33:33] 173    = note: the link appears in this line:
[01:33:33] - 
[01:33:33] - 
[01:33:33] +            
[01:33:33] 175            bar [BarF] bar
[01:33:33] 176                 ^^^^
[01:33:33] 177    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:33:33] 
[01:33:33] The actual stderr differed from the expected stderr.
[01:33:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/intra-links-warning.stderr
[01:33:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/intra-links-warning.stderr
[01:33:33] To update references, rerun the tests and pass the `--bless` flag
[01:33:33] To only update this specific test, also pass `--test-args intra-links-warning.rs`
[01:33:33] error: 1 errors occurred comparing output.
[01:33:33] status: exit code: 0
[01:33:33] status: exit code: 0
[01:33:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-warning.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/auxitest/rustdoc-ui/intra-links-warning.rs","byte_start":891,"byte_end":939,"line_start":67,"line_end":69,"column_start":1,"column_end":12,"is_primary":true,"text":[{"text":"/// Item docs.","highlight_start":1,"highlight_end":15},{"text":"#[doc=\"Hello there!\"]","highlight_start":1,"highlight_end":22},{"text":"/// [error]","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the link appears in this line:\n\n [error]\n  ^^^^^","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:67:1\n   |\nLL | / /// Item docs.\nLL | | #[doc=\"Hello there!\"]\nLL | | /// [error]\n   | |___________^\n   |\n   = note: the link appears in this line:\n           \n            [error]\n             ^^^^^\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:33:33] {"message":"`[error1]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":969,"byte_end":975,"line_start":73,"line_end":73,"column_start":11,"column_end":17,"is_primary":true,"text":[{"text":"/// docs [error1]","highlight_start":11,"highlight_end":17}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error1]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:73:11\n   |\nLL | /// docs [error1]\n   |           ^^^^^^ cannot be resolved, ignoring\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:33:33] {"message":"`[error2]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":988,"byte_end":994,"line_start":75,"line_end":75,"column_start":11,"column_end":17,"is_primary":true,"text":[{"text":"/// docs [error2]","highlight_start":11,"highlight_end":17}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[error2]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:75:11\n   |\nLL | /// docs [error2]\n   |           ^^^^^^ cannot be resolved, ignoring\n   |\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:33:33] {"message":"93,"byte_end":624,"line_start":47,"line_end":47,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"f!(\"Foo\\nbar [BarF] bar\\nbaz\");","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"f!","def_site_span":{"file_name":"/checkout/src/test/rustdoc-ui/intra-links-warning.rs","byte_start":507,"byte_end":592,"line_start":41,"line_end":46,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! f {","highlight_start":1,"highlight_end":17},{"text":"    ($f:expr) => {","highlight_start":1,"highlight_end":19},{"text":"        #[doc = $f]","highlight_start":1,"highlight_end":20},{"text":"        pub fn f() {}","highlight_start":1,"highlight_end":22},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"the link appears in this line:\n\nbar [BarF] bar\n     ^^^^","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `[BarF]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:43:9\n   |\nLL |         #[doc = $f]\n   |         ^^^^^^^^^^^\n...\nLL | f!(\"Foo\\nbar [BarF] bar\\nbaz\");\n   | ------------------------------- in this macro invocation\n   |\n   = note: the link appears in this CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1e33eeec
travis_time:start:1e33eeec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12fc3a83
$ dmesg | grep -i kill
