plain
[00:03:15]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:03:16]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:03:17]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:03:17]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:03:20] error[E0277]: the trait bound `for<'r> str::IsNotEmpty: ops::function::FnMut<(&'r &[u8],)>` is not satisfied
[00:03:20]     --> libcore/str/mod.rs:2557:14
[00:03:20]      |
[00:03:20] 2557 |             .filter(IsNotEmpty)
[00:03:20]      |              ^^^^^^ the trait `for<'r> ops::function::FnMut<(&'r &[u8],)>` is not implemented for `str::IsNotEmpty`
[00:03:20]      |
[00:03:20]      = help: the following implementations were found:
[00:03:20]                <str::IsNotEmpty as ops::function::FnMut<(&'a &'b str,)>>
[00:03:20] 
[00:03:20] error[E0599]: no method named `map` found for type `iter::Filter<slice::Split<'_, u8, str::IsAsciiWhitespace>, str::IsNotEmpty>` in the current scope
[00:03:20]     --> libcore/str/mod.rs:2558:14
[00:03:20]      |
[00:03:20] 2558 |             .map(BytesToStr);
[00:03:20]      | 
[00:03:20]     ::: libcore/iter/mod.rs:1438:1
[00:03:20]      |
[00:03:20]      |
[00:03:20] 1438 | pub struct Filter<I, P> {
[00:03:20]      | ----------------------- method `map` not found for this
[00:03:20]      |
[00:03:20]      = note: the method `map` exists but the following trait bounds were not satisfied:
[00:03:20]              `iter::Filter<slice::Split<'_, u8, str::IsAsciiWhitespace>, str::IsNotEmpty> : iter::iterator::Iterator`
[00:03:20]              `&mut iter::Filter<slice::Split<'_, u8, str::IsAsciiWhitespace>, str::IsNotEmpty> : iter::iterator::Iterator`
[00:03:20]      = help: items from traits can only be used if the trait is implemented and in scope
[00:03:20]      = note: the following trait defines an item `map`, perhaps you need to implement it:
[00:03:20]              candidate #1: `iter::iterator::Iterator`
[00:03:20] error[E0308]: mismatched types
[00:03:20]     --> libcore/str/mod.rs:4431:9
[00:03:20]      |
[00:03:20]      |
[00:03:20] 4430 |     fn size_hint(&self) -> (usize, Option<usize>) {
[00:03:20]      |                            ---------------------- expected `(usize, option::Option<usize>)` because of return type
[00:03:20] 4431 |         self.inner.next()
[00:03:20]      |         ^^^^^^^^^^^^^^^^^ expected tuple, found enum `option::Option`
[00:03:20]      |
[00:03:20]      = note: expected type `(usize, option::Option<usize>)`
[00:03:20]                 found type `option::Option<&str>`
[00:03:20] 
[00:03:20] error[E0599]: no method named `next` found for type `iter::Map<iter::Filter<slice::Split<'a, u8, str::IsAsciiWhitespace>, str::IsNotEmpty>, str::BytesToStr>` in the current scope
[00:03:20]     --> libcore/str/mod.rs:4452:20
[00:03:20]      |
[00:03:20] 4452 |         self.inner.next()
[00:03:20]      | 
[00:03:20]     ::: libcore/iter/mod.rs:1327:1
[00:03:20]      |
[00:03:20]      |
[00:03:20] 1327 | pub struct Map<I, F> {
[00:03:20]      | -------------------- method `next` not found for this
[00:03:20]      |
[00:03:20]      = note: the method `next` exists but the following trait bounds were not satisfied:
[00:03:20]              `iter::Map<iter::Filter<slice::Split<'a, u8, str::IsAsciiWhitespace>, str::IsNotEmpty>, str::BytesToStr> : iter::iterator::Iterator`
[00:03:20]      = help: items from traits can only be used if the trait is implemented and in scope
[00:03:20]      = note: the following traits define an item `next`, perhaps you need to implement one of them:
[00:03:20]              candidate #1: `iter::iterator::Iterator`
[00:03:20]              candidate #2: `iter::ZipImpl`
[00:03:20]              candidate #3: `str::pattern::Searcher`
[00:03:20] 
[00:03:20] error[E0599]: no method named `next` found for type `iter::Map<iter::Filter<slice::Split<'a, u8, str::IsAsciiWhitespace>, str::IsNotEmpty>, str::BytesToStr>` in the current scope
[00:03:20]     --> libcore/str/mod.rs:4457:20
[00:03:20]      |
[00:03:20] 4457 |         self.inner.next()
[00:03:20]      | 
[00:03:20]     ::: libcore/iter/mod.rs:1327:1
[00:03:20]      |
[00:03:20]      |
[00:03:20] 1327 | pub struct Map<I, F> {
[00:03:20]      | -------------------- method `next` not found for this
[00:03:20]      |
[00:03:20]      = note: the method `next` exists but the following trait bounds were not satisfied:
[00:03:20]              `iter::Map<iter::Filter<slice::Split<'a, u8, str::IsAsciiWhitespace>, str::IsNotEmpty>, str::BytesToStr> : iter::iterator::Iterator`
[00:03:20]      = help: items from traits can only be used if the trait is implemented and in scope
[00:03:20]      = note: the following traits define an item `next`, perhaps you need to implement one of them:
[00:03:20]              candidate #1: `iter::iterator::Iterator`
[00:03:20]              candidate #2: `iter::ZipImpl`
[00:03:20]              candidate #3: `str::pattern::Searcher`
[00:03:20] 
[00:03:20] error[E0599]: no method named `next_back` found for type `iter::Map<iter::Filter<slice::Split<'a, u8, str::IsAsciiWhitespace>, str::IsNotEmpty>, str::BytesToStr>` in the current scope
[00:03:20]     --> libcore/str/mod.rs:4465:20
[00:03:20]      |
[00:03:20] 4465 |         self.inner.next_back()
[00:03:20]      | 
[00:03:20]     ::: libcore/iter/mod.rs:1327:1
[00:03:20]      |
[00:03:20]      |
[00:03:20] 1327 | pub struct Map<I, F> {
[00:03:20]      | -------------------- method `next_back` not found for this
[00:03:20]      |
[00:03:20]      = note: the method `next_back` exists but the following trait bounds were not satisfied:
[00:03:20]              `iter::Map<iter::Filter<slice::Split<'a, u8, str::IsAsciiWhitespace>, str::IsNotEmpty>, str::BytesToStr> : iter::traits::DoubleEndedIterator`
[00:03:20]      = help: items from traits can only be used if the trait is implemented and in scope
[00:03:20]      = note: the following traits define an item `next_back`, perhaps you need to implement one of them:
[00:03:20]              candidate #1: `iter::traits::DoubleEndedIterator`
[00:03:20]              candidate #2: `iter::ZipImpl`
[00:03:20]              candidate #3: `str::pattern::ReverseSearcher`
[00:03:21] error: aborting due to 6 previous errors
[00:03:21] 
[00:03:21] Some errors occurred: E0277, E0308, E0599.
[00:03:21] For more information about an error, try `rustc --explain E0277`.
[00:03:21] For more information about an error, try `rustc --explain E0277`.
[00:03:21] error: Could not compile `core`.
[00:03:21] 
[00:03:21] Caused by:
[00:03:21]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=fb1e36473ec4786e -C extra-filename=-fb1e36473ec4786e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:21] warning: build failed, waiting for other jobs to finish...
[00:03:28] error: build failed
[00:03:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:28] expected success, got: exit code: 101
[00:03:28] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:28] travis_fold:end:stage0-std

[00:03:28] travis_time:end:stage0-std:start=1525556273311935592,finish=1525556299947011943,duration=26635076351


[00:03:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:28] Build completed unsuccessfully in 0:00:28
[00:03:28] Makefile:79: recipe for target 'tidy' failed
[00:03:28] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18fe1f41
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
