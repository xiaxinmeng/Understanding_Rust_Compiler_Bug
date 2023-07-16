plain
travis_time:end:066f1090:start=1556168974224929740,finish=1556168974981869426,duration=756939686
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:08:20]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:22] error: argument never used
[00:08:22]     --> src/librustc/mir/mod.rs:2187:65
[00:08:22]      |
[00:08:22] 2187 |                         write!(fmt, " as {})", projection.base, name)?;
[00:08:22]      |                                     ---------                   ^^^^ argument never used
[00:08:22]      |                                     formatting specifier missing
[00:08:22] 
[00:08:22] error: argument never used
[00:08:22]     --> src/librustc/mir/mod.rs:2190:75
[00:08:22]     --> src/librustc/mir/mod.rs:2190:75
[00:08:22]      |
[00:08:22] 2190 |                         write!(fmt, " as variant#{:?})", projection.base, index)?;
[00:08:22]      |                                     -------------------                   ^^^^^ argument never used
[00:08:22]      |                                     formatting specifier missing
[00:08:22] 
[00:08:22] 
[00:08:45] error[E0277]: the trait bound `mir::PlaceProjectionsIter<'_, '_>: std::iter::DoubleEndedIterator` is not satisfied
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2184 |             for projection in place_projections.rev() {
[00:08:45]      |                                                 ^^^ the trait `std::iter::DoubleEndedIterator` is not implemented for `mir::PlaceProjectionsIter<'_, '_>`
[00:08:45] 
[00:08:45] error[E0277]: the trait bound `mir::PlaceProjectionsIter<'_, '_>: std::iter::DoubleEndedIterator` is not satisfied
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2184 |             for projection in place_projections.rev() {
[00:08:45]      |                               ^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::iter::DoubleEndedIterator` is not implemented for `mir::PlaceProjectionsIter<'_, '_>`
[00:08:45]      |
[00:08:45]      = note: required because of the requirements on the impl of `std::iter::Iterator` for `std::iter::Rev<mir::PlaceProjectionsIter<'_, '_>>`
[00:08:45] error[E0277]: `mir::Place<'_>` doesn't implement `std::fmt::Display`
[00:08:45]     --> src/librustc/mir/mod.rs:2187:48
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2187 |                           write!(fmt, " as {})", projection.base, name)?;
[00:08:45]      |                           |                      |
[00:08:45]      |                           |                      `mir::Place<'_>` cannot be formatted with the default formatter
[00:08:45]      |                           in this macro invocation
[00:08:45]      | 
[00:08:45]      | 
[00:08:45]     ::: <::core::macros::write macros>:1:1
[00:08:45]      |
[00:08:45] 1    | / ( $ dst : expr , $ ( $ arg : tt ) * ) => (
[00:08:45] 2    | | $ dst . write_fmt ( format_args ! ( $ ( $ arg ) * ) ) )
[00:08:45]      |
[00:08:45]      = help: the trait `std::fmt::Display` is not implemented for `mir::Place<'_>`
[00:08:45]      = help: the trait `std::fmt::Display` is not implemented for `mir::Place<'_>`
[00:08:45]      = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
[00:08:45]      = note: required by `std::fmt::Display::fmt`
[00:08:45] 
[00:08:45] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2162 |                     write!(fmt, "{:?}", id)?;
[00:08:45]      |                     |
[00:08:45]      |                     cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                     cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                     in this expansion of `desugaring of `?``
[00:08:45]      |
[00:08:45]      = help: the trait `std::ops::Try` is not implemented for `()`
[00:08:45]      = note: required by `std::ops::Try::from_error`
[00:08:45] 
[00:08:45] 
[00:08:45] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
[00:08:45]      |
[00:08:45] 2165 | /                     write!(
[00:08:45] 2166 | |                         fmt,
[00:08:45] 2166 | |                         fmt,
[00:08:45] 2167 | |                         "({}: {:?})",
[00:08:45] 2168 | |                         ty::tls::with(|tcx| tcx.def_path_str(*def_id)),
[00:08:45] 2170 | |                     )?;
[00:08:45]      | |                      ^
[00:08:45]      | |                      |
[00:08:45]      | |                      cannot use the `?` operator in a function that returns `()`
[00:08:45]      | |                      cannot use the `?` operator in a function that returns `()`
[00:08:45]      | |______________________in this expansion of `desugaring of `?``
[00:08:45]      |
[00:08:45]      = help: the trait `std::ops::Try` is not implemented for `()`
[00:08:45]      = note: required by `std::ops::Try::from_error`
[00:08:45] 
[00:08:45] 
[00:08:45] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
[00:08:45]      |
[00:08:45] 2175 | /                     write!(
[00:08:45] 2176 | |                         fmt,
[00:08:45] 2176 | |                         fmt,
[00:08:45] 2177 | |                         "({:?}: {:?})",
[00:08:45] 2178 | |                         promoted,
[00:08:45] 2180 | |                     )?;
[00:08:45]      | |                      ^
[00:08:45]      | |                      |
[00:08:45]      | |                      cannot use the `?` operator in a function that returns `()`
[00:08:45]      | |                      cannot use the `?` operator in a function that returns `()`
[00:08:45]      | |______________________in this expansion of `desugaring of `?``
[00:08:45]      |
[00:08:45]      = help: the trait `std::ops::Try` is not implemented for `()`
[00:08:45]      = note: required by `std::ops::Try::from_error`
[00:08:45] 
[00:08:45] 
[00:08:45] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2187 |                         write!(fmt, " as {})", projection.base, name)?;
[00:08:45]      |                         |
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         in this expansion of `desugaring of `?``
[00:08:45]      |
[00:08:45]      = help: the trait `std::ops::Try` is not implemented for `()`
[00:08:45]      = note: required by `std::ops::Try::from_error`
[00:08:45] 
[00:08:45] 
[00:08:45] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2190 |                         write!(fmt, " as variant#{:?})", projection.base, index)?;
[00:08:45]      |                         |
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         in this expansion of `desugaring of `?``
[00:08:45]      |
[00:08:45]      = help: the trait `std::ops::Try` is not implemented for `()`
[00:08:45]      = note: required by `std::ops::Try::from_error`
[00:08:45] 
[00:08:45] 
[00:08:45] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2193 |                         write!(fmt, "(*{:?})", projection.base)?;
[00:08:45]      |                         |
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         in this expansion of `desugaring of `?``
[00:08:45]      |
[00:08:45]      = help: the trait `std::ops::Try` is not implemented for `()`
[00:08:45]      = note: required by `std::ops::Try::from_error`
[00:08:45] 
[00:08:45] 
[00:08:45] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2196 |                         write!(fmt, "({:?}.{:?}: {:?})", projection.base, field.index(), ty)?;
[00:08:45]      |                         |
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         in this expansion of `desugaring of `?``
[00:08:45]      |
[00:08:45]      = help: the trait `std::ops::Try` is not implemented for `()`
[00:08:45]      = note: required by `std::ops::Try::from_error`
[00:08:45] 
[00:08:45] 
[00:08:45] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2199 |                         write!(fmt, "{:?}[{:?}]", projection.base, index)?;
[00:08:45]      |                         |
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         in this expansion of `desugaring of `?``
[00:08:45]      |
[00:08:45]      = help: the trait `std::ops::Try` is not implemented for `()`
[00:08:45]      = note: required by `std::ops::Try::from_error`
[00:08:45] 
[00:08:45] 
[00:08:45] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2206 |                         write!(fmt, "{:?}[{:?} of {:?}]", projection.base, offset, min_length)?;
[00:08:45]      |                         |
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         in this expansion of `desugaring of `?``
[00:08:45]      |
[00:08:45]      = help: the trait `std::ops::Try` is not implemented for `()`
[00:08:45]      = note: required by `std::ops::Try::from_error`
[00:08:45] 
[00:08:45] 
[00:08:45] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2213 |                         write!(fmt, "{:?}[-{:?} of {:?}]", projection.base, offset, min_length)?;
[00:08:45]      |                         |
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         in this expansion of `desugaring of `?``
[00:08:45]      |
[00:08:45]      = help: the trait `std::ops::Try` is not implemented for `()`
[00:08:45]      = note: required by `std::ops::Try::from_error`
[00:08:45] 
[00:08:45] 
[00:08:45] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2216 |                         write!(fmt, "{:?}[{:?}:]", projection.base, from)?;
[00:08:45]      |                         |
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         in this expansion of `desugaring of `?``
[00:08:45]      |
[00:08:45]      = help: the trait `std::ops::Try` is not implemented for `()`
[00:08:45]      = note: required by `std::ops::Try::from_error`
[00:08:45] 
[00:08:45] 
[00:08:45] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2219 |                         write!(fmt, "{:?}[:-{:?}]", projection.base, to)?;
[00:08:45]      |                         |
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         in this expansion of `desugaring of `?``
[00:08:45]      |
[00:08:45]      = help: the trait `std::ops::Try` is not implemented for `()`
[00:08:45]      = note: required by `std::ops::Try::from_error`
[00:08:45] 
[00:08:45] 
[00:08:45] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
[00:08:45]      |
[00:08:45]      |
[00:08:45] 2222 |                         write!(fmt, "{:?}[{:?}:-{:?}]", projection.base, from, to)?;
[00:08:45]      |                         |
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         cannot use the `?` operator in a function that returns `()`
[00:08:45]      |                         in this expansion of `desugaring of `?``
[00:08:45]      |
[00:08:45]      = help: the trait `std::ops::Try` is not implemented for `()`
[00:08:45]      = note: required by `std::ops::Try::from_error`
[00:08:45] 
[00:08:45] 
[00:08:46] error[E0308]: mismatched types
[00:08:46]     --> src/librustc/mir/mod.rs:2159:9
[00:08:46]      |
[00:08:46] 2158 |       fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
[00:08:46]      |                                                 ----------- expected `std::result::Result<(), std::fmt::Error>` because of return type
[00:08:46] 2159 | /         self.iterate(|place_base, place_projections| {
[00:08:46] 2160 | |             match place_base {
[00:08:46] 2161 | |                 PlaceBase::Local(id) => {
[00:08:46] 2162 | |                     write!(fmt, "{:?}", id)?;
[00:08:46] 2226 | |
[00:08:46] 2227 | |         })
[00:08:46]      | |__________^ expected enum `std::result::Result`, found ()
[00:08:46]      |
---
travis_time:end:0311ffac:start=1556169523463622055,finish=1556169523468411765,duration=4789710
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15945bae
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3b1b4894
travis_time:start:3b1b4894
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/na
