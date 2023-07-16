plain
travis_time:end:049979e8:start=1557730395607184103,finish=1557730490872629303,duration=95265445200
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:55:23]    Compiling parking_lot_core v0.4.0
[00:55:29]    Compiling tempfile v3.0.5
[00:55:32]    Compiling parking_lot v0.7.1
[00:55:33]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:55:37] error[E0599]: no method named `peek` found for type `syntax::parse::lexer::StringReader<'a>` in the current scope
[00:55:37]    --> src/librustdoc/html/highlight.rs:236:31
[00:55:37]     |
[00:55:37] 236 |                 if self.lexer.peek().tok != token::Whitespace => Class::RefKeyWord,
[00:55:37] 
[00:55:37] 
[00:55:37] error[E0599]: no method named `peek` found for type `syntax::parse::lexer::StringReader<'a>` in the current scope
[00:55:37]    --> src/librustdoc/html/highlight.rs:259:31
[00:55:37]     |
[00:55:37] 259 |                 if self.lexer.peek().tok.is_ident() {
[00:55:37] 
[00:55:37] 
[00:55:37] error[E0599]: no method named `peek` found for type `syntax::parse::lexer::StringReader<'a>` in the current scope
[00:55:37]    --> src/librustdoc/html/highlight.rs:282:31
[00:55:37]     |
[00:55:37] 282 |                 if self.lexer.peek().tok == token::Not {
[00:55:37] 
[00:55:37] 
[00:55:37] error[E0599]: no method named `peek` found for type `syntax::parse::lexer::StringReader<'a>` in the current scope
[00:55:37]    --> src/librustdoc/html/highlight.rs:284:35
[00:55:37]     |
[00:55:37] 284 |                     if self.lexer.peek().tok == token::OpenDelim(token::Bracket) {
[00:55:37] 
[00:55:37] 
[00:55:37] error[E0599]: no method named `peek` found for type `syntax::parse::lexer::StringReader<'a>` in the current scope
[00:55:37]    --> src/librustdoc/html/highlight.rs:294:31
[00:55:37]     |
[00:55:37] 294 |                 if self.lexer.peek().tok == token::OpenDelim(token::Bracket) {
[00:55:37] 
[00:55:37] 
[00:55:37] error[E0599]: no method named `peek` found for type `syntax::parse::lexer::StringReader<'a>` in the current scope
[00:55:37]    --> src/librustdoc/html/highlight.rs:344:46
[00:55:37]     |
[00:55:37] 344 |                         } else if self.lexer.peek().tok == token::Not {
[00:55:37] 
[00:55:39] error: aborting due to 6 previous errors
[00:55:39] 
[00:55:39] For more information about this error, try `rustc --explain E0599`.
---
travis_time:end:220fe708:start=1557733840427043205,finish=1557733840432512204,duration=5468999
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3b517309
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1563d3cc
travis_time:start:1563d3cc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01ad1f65
$ dmesg | grep -i kill
