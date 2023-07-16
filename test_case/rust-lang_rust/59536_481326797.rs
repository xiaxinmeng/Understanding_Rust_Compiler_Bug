plain
travis_time:end:04b2d3d2:start=1554827093121868238,finish=1554827170268856774,duration=77146988536
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
$ export CI_JOB_NAME=$TRAVIS_JOB_NAME
$ export IMAGE=x86_64-gnu-llvm-6.0
$ export RUST_BACKTRACE=1
$ bash -c 'echo $BASH_VERSION'
---
[00:06:16]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:30]    Compiling synstructure v0.10.1
[00:06:47]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:32]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:42] error[E0119]: conflicting implementations of trait `rustc_serialize::SpecializedDecoder<&[traits::specialize::specialization_graph::Graph]>` for type `ty::query::on_disk_cache::CacheDecoder<'_, '_, '_>`:
[00:07:42]     | 
[00:07:42]    ::: src/librustc/ty/query/on_disk_cache.rs:587:1
[00:07:42]     |
[00:07:42] 587 |    implement_ty_decoder!( CacheDecoder<'a, 'tcx, 'x> );
[00:07:42]    --> src/librustc/ty/codec.rs:309:9
[00:07:42]     |
[00:07:42]     |
[00:07:42] 298 | /  macro_rules! impl_arena_allocatable_decoder {
[00:07:42] 299 | |      ([]$args:tt) => {};
[00:07:42] 300 | |      ([decode $(, $attrs:ident)*]
[00:07:42] 301 | |       [[$DecoderName:ident [$($typaram:tt),*]], [$name:ident: $ty:ty], $tcx:lifetime]) => {
[00:07:42] ...   |
[00:07:42] 309 | |          impl<$($typaram),*> SpecializedDecoder<&$tcx [$ty]> for $DecoderName<$($typaram),*> {
[00:07:42] ...   |
[00:07:42] 318 | |      };
[00:07:42] 319 | |  }
[00:07:42] 319 | |  }
[00:07:42]     | |__- in this expansion of `impl_arena_allocatable_decoder!` (#4)
[00:07:42] ...
[00:07:42] 322 | /  macro_rules! impl_arena_allocatable_decoders {
[00:07:42] 323 | |      ($args:tt, [$($a:tt $name:ident: $ty:ty,)*], $tcx:lifetime) => {
[00:07:42] 324 | |          $(
[00:07:42] 325 | |              impl_arena_allocatable_decoder!($a [$args, [$name: $ty], $tcx]);
[00:07:42] 326 | |          )*
[00:07:42] 327 | |      }
[00:07:42] 328 | |  }
[00:07:42] 328 | |  }
[00:07:42]     | |__- in this expansion of `impl_arena_allocatable_decoders!` (#3)
[00:07:42] ...
[00:07:42] 331 | /  macro_rules! implement_ty_decoder {
[00:07:42] 332 | |      ($DecoderName:ident <$($typaram:tt),*>) => {
[00:07:42] 333 | |          mod __ty_decoder_impl {
[00:07:42] 334 | |              use super::$DecoderName;
[00:07:42] ...   |
[00:07:42] 379 | |              arena_types!(impl_arena_allocatable_decoders, [$DecoderName [$($typaram),*]], 'tcx);
[00:07:42] ...   |
[00:07:42] 464 | |      }
[00:07:42] 465 | |  }
[00:07:42] 465 | |  }
[00:07:42]     | |__- in this expansion of `implement_ty_decoder!` (#1)
[00:07:42]    ::: src/librustc/arena.rs:10:1
[00:07:42]     |
[00:07:42]     |
[00:07:42] 10  |  / macro_rules! arena_types {
[00:07:42] 11  |  |     ($macro:path, $args:tt, $tcx:lifetime) => (
[00:07:42] 12  |  |         $macro!($args, [
[00:07:42]     |  |_________-
[00:07:42] 13  | ||             [] vtable_method: Option<(
[00:07:42] 14  | ||                 rustc::hir::def_id::DefId,
[00:07:42] 15  | ||                 rustc::ty::subst::SubstsRef<$tcx>
[00:07:42] ...   ||
[00:07:42] 18  | ||             [decode] specialization_graph: rustc::traits::specialization_graph::Graph,
[00:07:42] 19  | ||         ], $tcx);
[00:07:42]     | ||_________________- in this macro invocation (#3)
[00:07:42] 21  |  | }
[00:07:42] 21  |  | }
[00:07:42]     |  |_- in this expansion of `arena_types!` (#2)
[00:07:42]     = note: conflicting implementation in crate `serialize`:
[00:07:42]     = note: conflicting implementation in crate `serialize`:
[00:07:42]             - impl<D, T> rustc_serialize::SpecializedDecoder<T> for D
[00:07:42]               where D: rustc_serialize::Decoder, T: rustc_serialize::UseSpecializedDecodable;
[00:07:42]     = note: upstream crates may add new impl of trait `rustc_serialize::Decodable` for type `[traits::specialize::specialization_graph::Graph]` in future versions
[00:07:43] error: aborting due to previous error
[00:07:43] 
[00:07:43] For more information about this error, try `rustc --explain E0119`.
[00:07:43] error: Could not compile `rustc`.
---
travis_time:end:04da1cb0:start=1554827657331324974,finish=1554827657336388094,duration=5063120
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0cf34e02
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ded6eea
travis_time:start:0ded6eea
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file 
