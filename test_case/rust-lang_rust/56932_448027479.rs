plain
travis_time:end:158a2415:start=1545086431224463971,finish=1545086433431261879,duration=2206797908
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:48]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:52] error[E0432]: unresolved import `iter::flatten_compat`
[00:03:52]   --> src/libcore/iter/traits/iterator.rs:16:30
[00:03:52]    |
[00:03:52] 16 | use iter::{Flatten, FlatMap, flatten_compat};
[00:03:52]    |                              ^^^^^^^^^^^^^^ no `flatten_compat` in `iter`
[00:03:52] 
[00:03:52] error[E0432]: unresolved imports `iter::ChainState`, `iter::ZipImpl`
[00:03:52]    |
[00:03:52]    |
[00:03:52] 19 | use iter::{ChainState, FromIterator, ZipImpl};
[00:03:52]    |            ^^^^^^^^^^                ^^^^^^^ no `ZipImpl` in `iter`
[00:03:52]    |            no `ChainState` in `iter`
[00:03:52] 
[00:03:52] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:52]     --> src/libcore/iter/adapters.rs:1733:21
[00:03:52]     --> src/libcore/iter/adapters.rs:1733:21
[00:03:52]      |
[00:03:52] 1733 |                     LoopState::from_try(fold(acc, x))
[00:03:52]      |                     ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:52] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:52]     --> src/libcore/iter/adapters.rs:1736:21
[00:03:52]      |
[00:03:52]      |
[00:03:52] 1736 |                     LoopState::Break(Try::from_ok(acc))
[00:03:52]      |                     ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:53] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:53]     --> src/libcore/iter/adapters.rs:1875:29
[00:03:53]      |
[00:03:53]      |
[00:03:53] 1875 |                 if n == 0 { LoopState::Break(r) }
[00:03:53]      |                             ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:53] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:53]     --> src/libcore/iter/adapters.rs:1876:24
[00:03:53]      |
[00:03:53]      |
[00:03:53] 1876 |                 else { LoopState::from_try(r) }
[00:03:53]      |                        ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:53] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:53]     --> src/libcore/iter/adapters.rs:1957:30
[00:03:53]      |
[00:03:53]      |
[00:03:53] 1957 |                 if *n == 0 { LoopState::Break(r) }
[00:03:53]      |                              ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:53] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:53]     --> src/libcore/iter/adapters.rs:1958:24
[00:03:53]      |
[00:03:53]      |
[00:03:53] 1958 |                 else { LoopState::from_try(r) }
[00:03:53]      |                        ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:53] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:53]     --> src/libcore/iter/adapters.rs:2025:25
[00:03:53]      |
[00:03:53]      |
[00:03:53] 2025 |                 None => LoopState::Break(Try::from_ok(acc)),
[00:03:53]      |                         ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:53] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:53]     --> src/libcore/iter/adapters.rs:2026:28
[00:03:53]      |
[00:03:53]      |
[00:03:53] 2026 |                 Some(x) => LoopState::from_try(fold(acc, x)),
[00:03:53]      |                            ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:53] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:112:9
[00:03:53]     |
[00:03:53]     |
[00:03:53] 112 | impl<I> FusedIterator for Rev<I>
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::FusedIterator;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:113:14
[00:03:53]     |
[00:03:53] 113 |     where I: FusedIterator + DoubleEndedIterator {}
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::FusedIterator;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:116:16
[00:03:53]     |
[00:03:53] 116 | unsafe impl<I> TrustedLen for Rev<I>
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::TrustedLen;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:117:14
[00:03:53]     |
[00:03:53] 117 |     where I: TrustedLen + DoubleEndedIterator {}
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::TrustedLen;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:195:20
[00:03:53]     |
[00:03:53] 195 | impl<'a, I, T: 'a> FusedIterator for Cloned<I>
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::FusedIterator;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:196:14
[00:03:53]     |
[00:03:53] 196 |     where I: FusedIterator<Item=&'a T>, T: Clone
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::FusedIterator;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:226:27
[00:03:53]     |
[00:03:53] 226 | unsafe impl<'a, I, T: 'a> TrustedLen for Cloned<I>
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::TrustedLen;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:227:14
[00:03:53]     |
[00:03:53] 227 |     where I: TrustedLen<Item=&'a T>,
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::TrustedLen;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53]    Compiling compiler_builtins v0.1.2
[00:03:53]    Compiling cmake v0.1.33
[00:03:53]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:53] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:270:9
[00:03:53]     |
[00:03:53] 270 | impl<I> FusedIterator for Cycle<I> where I: Clone + Iterator {}
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::FusedIterator;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:604:12
[00:03:53]     |
[00:03:53] 604 | impl<A, B> FusedIterator for Chain<A, B>
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::FusedIterator;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:605:14
[00:03:53]     |
[00:03:53] 605 |     where A: FusedIterator,
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::FusedIterator;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:606:14
[00:03:53]     |
[00:03:53] 606 |           B: FusedIterator<Item=A::Item>,
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::FusedIterator;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:610:19
[00:03:53]     |
[00:03:53] 610 | unsafe impl<A, B> TrustedLen for Chain<A, B>
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::TrustedLen;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:611:14
[00:03:53]     |
[00:03:53] 611 |     where A: TrustedLen, B: TrustedLen<Item=A::Item>,
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::TrustedLen;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:611:29
[00:03:53]     |
[00:03:53] 611 |     where A: TrustedLen, B: TrustedLen<Item=A::Item>,
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::TrustedLen;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:864:12
[00:03:53]     |
[00:03:53] 864 | impl<A, B> FusedIterator for Zip<A, B>
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::FusedIterator;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:53] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:53]    --> src/libcore/iter/adapters.rs:865:14
[00:03:53]     |
[00:03:53] 865 |     where A: FusedIterator, B: FusedIterator, {}
[00:03:53] help: possible candidate is found in another module, you can import it into scope
[00:03:53]     |
[00:03:53] 11  | use iter::traits::marker::FusedIterator;
[00:03:53]     |
[00:03:53]     |
[00:03:53] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]    --> src/libcore/iter/adapters.rs:865:32
[00:03:54]     |
[00:03:54] 865 |     where A: FusedIterator, B: FusedIterator, {}
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]     |
[00:03:54] 11  | use iter::traits::marker::FusedIterator;
[00:03:54]     |
[00:03:54]     |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:54]    --> src/libcore/iter/adapters.rs:868:19
[00:03:54]     |
[00:03:54] 868 | unsafe impl<A, B> TrustedLen for Zip<A, B>
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]     |
[00:03:54] 11  | use iter::traits::marker::TrustedLen;
[00:03:54]     |
[00:03:54]     |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:54]    --> src/libcore/iter/adapters.rs:869:14
[00:03:54]     |
[00:03:54] 869 |     where A: TrustedLen, B: TrustedLen,
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]     |
[00:03:54] 11  | use iter::traits::marker::TrustedLen;
[00:03:54]     |
[00:03:54]     |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:54]    --> src/libcore/iter/adapters.rs:869:29
[00:03:54]     |
[00:03:54] 869 |     where A: TrustedLen, B: TrustedLen,
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]     |
[00:03:54] 11  | use iter::traits::marker::TrustedLen;
[00:03:54]     |
[00:03:54]     |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1006:30
[00:03:54]      |
[00:03:54] 1006 | impl<B, I: FusedIterator, F> FusedIterator for Map<I, F>
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1006:12
[00:03:54]      |
[00:03:54] 1006 | impl<B, I: FusedIterator, F> FusedIterator for Map<I, F>
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1010:22
[00:03:54]      |
[00:03:54] 1010 | unsafe impl<B, I, F> TrustedLen for Map<I, F>
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::TrustedLen;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1011:14
[00:03:54]      |
[00:03:54] 1011 |     where I: TrustedLen,
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::TrustedLen;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1155:27
[00:03:54]      |
[00:03:54] 1155 | impl<I: FusedIterator, P> FusedIterator for Filter<I, P>
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1155:9
[00:03:54]      |
[00:03:54] 1155 | impl<I: FusedIterator, P> FusedIterator for Filter<I, P>
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1265:30
[00:03:54]      |
[00:03:54] 1265 | impl<B, I: FusedIterator, F> FusedIterator for FilterMap<I, F>
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1265:12
[00:03:54]      |
[00:03:54] 1265 | impl<B, I: FusedIterator, F> FusedIterator for FilterMap<I, F>
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1420:9
[00:03:54]      |
[00:03:54] 1420 | impl<I> FusedIterator for Enumerate<I> where I: FusedIterator {}
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1420:49
[00:03:54]      |
[00:03:54] 1420 | impl<I> FusedIterator for Enumerate<I> where I: FusedIterator {}
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1423:16
[00:03:54]      |
[00:03:54] 1423 | unsafe impl<I> TrustedLen for Enumerate<I>
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::TrustedLen;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1424:14
[00:03:54]      |
[00:03:54] 1424 |     where I: TrustedLen,
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::TrustedLen;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1533:24
[00:03:54]      |
[00:03:54] 1533 | impl<I: FusedIterator> FusedIterator for Peekable<I> {}
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1533:9
[00:03:54]      |
[00:03:54] 1533 | impl<I: FusedIterator> FusedIterator for Peekable<I> {}
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1661:12
[00:03:54]      |
[00:03:54] 1661 | impl<I, P> FusedIterator for SkipWhile<I, P>
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1662:14
[00:03:54]      |
[00:03:54] 1662 |     where I: FusedIterator, P: FnMut(&I::Item) -> bool {}
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1744:12
[00:03:54]      |
[00:03:54] 1744 | impl<I, P> FusedIterator for TakeWhile<I, P>
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1745:14
[00:03:54]      |
[00:03:54] 1745 |     where I: FusedIterator, P: FnMut(&I::Item) -> bool {}
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1883:9
[00:03:54]      |
[00:03:54] 1883 | impl<I> FusedIterator for Skip<I> where I: FusedIterator {}
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1883:44
[00:03:54]      |
[00:03:54] 1883 | impl<I> FusedIterator for Skip<I> where I: FusedIterator {}
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1968:9
[00:03:54]      |
[00:03:54] 1968 | impl<I> FusedIterator for Take<I> where I: FusedIterator {}
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1968:44
[00:03:54]      |
[00:03:54] 1968 | impl<I> FusedIterator for Take<I> where I: FusedIterator {}
[00:03:54] help: possible candidate is found in another module, you can import it into scope
[00:03:54]      |
[00:03:54] 11   | use iter::traits::marker::FusedIterator;
[00:03:54]      |
[00:03:54]      |
[00:03:54] 
[00:03:54] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:54]     --> src/libcore/iter/adapters.rs:1971:28
[00:03:54]      |
[00:03:54] 1971 | unsafe impl<I: TrustedLen> TrustedLen for Take<I> {}
---
[00:04:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:00] expected success, got: exit code: 101
[00:04:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:00] Build completed unsuccessfully in 0:00:14
[00:04:00] Makefile:28: recipe for target 'all' failed
[00:04:00] make: *** [all] Error 1
1226368 ./obj
1226328 ./obj/build
1119404 ./src
566780 ./obj/build/x86_64-unknown-linux-gnu
---
187088 ./obj/build/cache/2018-12-09
160388 ./obj/build/bootstrap/debug/incremental
153276 ./src/tools/clang
144288 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144284 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7oqz62uwp-f7htl7-2sxk29axi4qzn
111164 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107420 ./src/tools/lldb
95104 ./src/tools/clang/test
89964 ./src/llvm-emscripten/test/CodeGen
