plain
travis_time:end:0275546d:start=1560302582476494139,finish=1560302585967335180,duration=3490841041
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:15:06] 
[00:15:06] error[E0425]: cannot find value `non_zst_count` in this scope
[00:15:06]     --> src/librustc_typeck/check/mod.rs:1834:77
[00:15:06]      |
[00:15:06] 1834 |     let msg = format!("needs exactly one non-zero-sized field, but has {}", non_zst_count);
[00:15:06] 
[00:15:06] error[E0425]: cannot find value `adt` in this scope
[00:15:06]     --> src/librustc_typeck/check/mod.rs:1840:12
[00:15:06]      |
[00:15:06]      |
[00:15:06] 1840 |         if adt.is_enum() { "the variant of a " } else { "" },
[00:15:06] 
[00:15:06] error[E0425]: cannot find value `adt` in this scope
[00:15:06]     --> src/librustc_typeck/check/mod.rs:1841:9
[00:15:06]      |
[00:15:06]      |
[00:15:06] 1841 |         adt.descr(),
[00:15:06] 
[00:15:06] error[E0425]: cannot find value `span` in this scope
[00:15:06]     --> src/librustc_typeck/check/mod.rs:1898:14
[00:15:06]      |
[00:15:06]      |
[00:15:06] 1898 |         Some(span)
[00:15:06]      |              ^^^^ help: a local variable with a similar name exists: `_span`
[00:15:08] error[E0614]: type `bool` cannot be dereferenced
[00:15:08]     --> src/librustc_typeck/check/mod.rs:1897:85
[00:15:08]      |
[00:15:08]      |
[00:15:08] 1897 |     let non_zst_fields = field_infos.clone().filter_map(|(_span, zst, _align1)| if !*zst {
[00:15:08] 
[00:15:08] error[E0277]: `usize` is not an iterator
[00:15:08]     --> src/librustc_typeck/check/mod.rs:1904:9
[00:15:08]      |
[00:15:08]      |
[00:15:08] 1904 |         bad_non_zero_sized_fields(tcx, non_zst_count, sp);
[00:15:08]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^ `usize` is not an iterator
[00:15:08]      = help: the trait `std::iter::Iterator` is not implemented for `usize`
[00:15:08]      = help: the trait `std::iter::Iterator` is not implemented for `usize`
[00:15:08]      = note: if you want to iterate between `start` until a value `end`, use the exclusive range syntax `start..end` or the inclusive range syntax `start..=end`
[00:15:08] note: required by `check::bad_non_zero_sized_fields`
[00:15:08]     --> src/librustc_typeck/check/mod.rs:1829:1
[00:15:08]      |
[00:15:08] 1829 | / fn bad_non_zero_sized_fields<'a, 'tcx, I: Iterator<Item = Span>>(
[00:15:08] 1831 | |     field_spans: I,
[00:15:08] 1832 | |     sp: Span,
[00:15:08] ...    |
[00:15:08] ...    |
[00:15:08] 1848 | |     err.emit();
[00:15:08]      | |_^
[00:15:08] 
[00:15:12] error: aborting due to 7 previous errors
[00:15:12] 
