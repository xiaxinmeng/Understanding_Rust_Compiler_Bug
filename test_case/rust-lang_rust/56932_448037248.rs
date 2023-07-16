plain
travis_time:end:0955e99c:start=1545088825812351678,finish=1545088826832701701,duration=1020350023
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:57]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:02:59] error[E0432]: unresolved import `iter::flatten_compat`
[00:02:59]   --> src/libcore/iter/traits/iterator.rs:16:30
[00:02:59]    |
[00:02:59] 16 | use iter::{Flatten, FlatMap, flatten_compat};
[00:02:59]    |                              ^^^^^^^^^^^^^^ no `flatten_compat` in `iter`
[00:02:59] 
[00:02:59] error[E0432]: unresolved imports `iter::ChainState`, `iter::ZipImpl`
[00:02:59]    |
[00:02:59]    |
[00:02:59] 19 | use iter::{ChainState, FromIterator, ZipImpl};
[00:02:59]    |            ^^^^^^^^^^                ^^^^^^^ no `ZipImpl` in `iter`
[00:02:59]    |            no `ChainState` in `iter`
[00:02:59] 
[00:02:59] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:02:59]     --> src/libcore/iter/adapters.rs:1733:21
[00:02:59]     --> src/libcore/iter/adapters.rs:1733:21
[00:02:59]      |
[00:02:59] 1733 |                     LoopState::from_try(fold(acc, x))
[00:02:59]      |                     ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:02:59] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:02:59]     --> src/libcore/iter/adapters.rs:1736:21
[00:02:59]      |
[00:02:59]      |
[00:02:59] 1736 |                     LoopState::Break(Try::from_ok(acc))
[00:02:59]      |                     ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:02:59] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:02:59]     --> src/libcore/iter/adapters.rs:1875:29
[00:02:59]      |
[00:02:59]      |
[00:02:59] 1875 |                 if n == 0 { LoopState::Break(r) }
[00:02:59]      |                             ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:02:59] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:02:59]     --> src/libcore/iter/adapters.rs:1876:24
[00:02:59]      |
[00:02:59]      |
[00:02:59] 1876 |                 else { LoopState::from_try(r) }
[00:02:59]      |                        ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:02:59] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:02:59]     --> src/libcore/iter/adapters.rs:1957:30
[00:02:59]      |
[00:02:59]      |
[00:02:59] 1957 |                 if *n == 0 { LoopState::Break(r) }
[00:02:59]      |                              ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:02:59] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:02:59]     --> src/libcore/iter/adapters.rs:1958:24
[00:02:59]      |
[00:02:59]      |
[00:02:59] 1958 |                 else { LoopState::from_try(r) }
[00:02:59]      |                        ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:00] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:00]     --> src/libcore/iter/adapters.rs:2025:25
[00:03:00]      |
[00:03:00]      |
[00:03:00] 2025 |                 None => LoopState::Break(Try::from_ok(acc)),
[00:03:00]      |                         ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:00] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:00]     --> src/libcore/iter/adapters.rs:2026:28
[00:03:00]      |
[00:03:00]      |
[00:03:00] 2026 |                 Some(x) => LoopState::from_try(fold(acc, x)),
[00:03:00]      |                            ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:00]    Compiling compiler_builtins v0.1.2
[00:03:00]    Compiling cmake v0.1.33
[00:03:00]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:00] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:00] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:00]    --> src/libcore/iter/adapters.rs:112:9
[00:03:00]     |
[00:03:00] 112 | impl<I> FusedIterator for Rev<I>
[00:03:00] help: possible candidate is found in another module, you can import it into scope
[00:03:00]     |
[00:03:00] 11  | use iter::traits::marker::FusedIterator;
[00:03:00]     |
[00:03:00]     |
[00:03:00] 
[00:03:00] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:00]    --> src/libcore/iter/adapters.rs:113:14
[00:03:00]     |
[00:03:00] 113 |     where I: FusedIterator + DoubleEndedIterator {}
[00:03:00] help: possible candidate is found in another module, you can import it into scope
[00:03:00]     |
[00:03:00] 11  | use iter::traits::marker::FusedIterator;
[00:03:00]     |
[00:03:00]     |
[00:03:00] 
[00:03:00] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:00]    --> src/libcore/iter/adapters.rs:116:16
[00:03:00]     |
[00:03:00] 116 | unsafe impl<I> TrustedLen for Rev<I>
[00:03:00] help: possible candidate is found in another module, you can import it into scope
[00:03:00]     |
[00:03:00] 11  | use iter::traits::marker::TrustedLen;
[00:03:00]     |
[00:03:00]     |
[00:03:00] 
[00:03:00] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:00]    --> src/libcore/iter/adapters.rs:117:14
[00:03:00]     |
[00:03:00] 117 |     where I: TrustedLen + DoubleEndedIterator {}
[00:03:00] help: possible candidate is found in another module, you can import it into scope
[00:03:00]     |
[00:03:00] 11  | use iter::traits::marker::TrustedLen;
[00:03:00]     |
[00:03:00]     |
[00:03:00] 
[00:03:00] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:00]    --> src/libcore/iter/adapters.rs:195:20
[00:03:00]     |
[00:03:00] 195 | impl<'a, I, T: 'a> FusedIterator for Cloned<I>
[00:03:00] help: possible candidate is found in another module, you can import it into scope
[00:03:00]     |
[00:03:00] 11  | use iter::traits::marker::FusedIterator;
[00:03:00]     |
[00:03:00]     |
[00:03:00] 
[00:03:00] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:00]    --> src/libcore/iter/adapters.rs:196:14
[00:03:00]     |
[00:03:00] 196 |     where I: FusedIterator<Item=&'a T>, T: Clone
[00:03:00] help: possible candidate is found in another module, you can import it into scope
[00:03:00]     |
[00:03:00] 11  | use iter::traits::marker::FusedIterator;
[00:03:00]     |
[00:03:00]     |
[00:03:00] 
[00:03:00] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:00]    --> src/libcore/iter/adapters.rs:226:27
[00:03:00]     |
[00:03:00] 226 | unsafe impl<'a, I, T: 'a> TrustedLen for Cloned<I>
[00:03:00] help: possible candidate is found in another module, you can import it into scope
[00:03:00]     |
[00:03:00] 11  | use iter::traits::marker::TrustedLen;
[00:03:00]     |
[00:03:00]     |
[00:03:00] 
[00:03:00] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:00]    --> src/libcore/iter/adapters.rs:227:14
[00:03:00]     |
[00:03:00] 227 |     where I: TrustedLen<Item=&'a T>,
[00:03:00] help: possible candidate is found in another module, you can import it into scope
[00:03:00]     |
[00:03:00] 11  | use iter::traits::marker::TrustedLen;
[00:03:00]     |
[00:03:00]     |
[00:03:00] 
[00:03:00] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:00]    --> src/libcore/iter/adapters.rs:270:9
[00:03:00]     |
[00:03:00] 270 | impl<I> FusedIterator for Cycle<I> where I: Clone + Iterator {}
[00:03:00] help: possible candidate is found in another module, you can import it into scope
[00:03:00]     |
[00:03:00] 11  | use iter::traits::marker::FusedIterator;
[00:03:00]     |
[00:03:00]     |
[00:03:00] 
[00:03:00] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:00]    --> src/libcore/iter/adapters.rs:604:12
[00:03:00]     |
[00:03:00] 604 | impl<A, B> FusedIterator for Chain<A, B>
[00:03:00] help: possible candidate is found in another module, you can import it into scope
[00:03:00]     |
[00:03:00] 11  | use iter::traits::marker::FusedIterator;
[00:03:00]     |
[00:03:00]     |
[00:03:00] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]    --> src/libcore/iter/adapters.rs:605:14
[00:03:01]     |
[00:03:01] 605 |     where A: FusedIterator,
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]     |
[00:03:01] 11  | use iter::traits::marker::FusedIterator;
[00:03:01]     |
[00:03:01]     |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]    --> src/libcore/iter/adapters.rs:606:14
[00:03:01]     |
[00:03:01] 606 |           B: FusedIterator<Item=A::Item>,
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]     |
[00:03:01] 11  | use iter::traits::marker::FusedIterator;
[00:03:01]     |
[00:03:01]     |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:01]    --> src/libcore/iter/adapters.rs:610:19
[00:03:01]     |
[00:03:01] 610 | unsafe impl<A, B> TrustedLen for Chain<A, B>
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]     |
[00:03:01] 11  | use iter::traits::marker::TrustedLen;
[00:03:01]     |
[00:03:01]     |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:01]    --> src/libcore/iter/adapters.rs:611:14
[00:03:01]     |
[00:03:01] 611 |     where A: TrustedLen, B: TrustedLen<Item=A::Item>,
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]     |
[00:03:01] 11  | use iter::traits::marker::TrustedLen;
[00:03:01]     |
[00:03:01]     |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:01]    --> src/libcore/iter/adapters.rs:611:29
[00:03:01]     |
[00:03:01] 611 |     where A: TrustedLen, B: TrustedLen<Item=A::Item>,
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]     |
[00:03:01] 11  | use iter::traits::marker::TrustedLen;
[00:03:01]     |
[00:03:01]     |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]    --> src/libcore/iter/adapters.rs:864:12
[00:03:01]     |
[00:03:01] 864 | impl<A, B> FusedIterator for Zip<A, B>
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]     |
[00:03:01] 11  | use iter::traits::marker::FusedIterator;
[00:03:01]     |
[00:03:01]     |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]    --> src/libcore/iter/adapters.rs:865:14
[00:03:01]     |
[00:03:01] 865 |     where A: FusedIterator, B: FusedIterator, {}
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]     |
[00:03:01] 11  | use iter::traits::marker::FusedIterator;
[00:03:01]     |
[00:03:01]     |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]    --> src/libcore/iter/adapters.rs:865:32
[00:03:01]     |
[00:03:01] 865 |     where A: FusedIterator, B: FusedIterator, {}
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]     |
[00:03:01] 11  | use iter::traits::marker::FusedIterator;
[00:03:01]     |
[00:03:01]     |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:01]    --> src/libcore/iter/adapters.rs:868:19
[00:03:01]     |
[00:03:01] 868 | unsafe impl<A, B> TrustedLen for Zip<A, B>
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]     |
[00:03:01] 11  | use iter::traits::marker::TrustedLen;
[00:03:01]     |
[00:03:01]     |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:01]    --> src/libcore/iter/adapters.rs:869:14
[00:03:01]     |
[00:03:01] 869 |     where A: TrustedLen, B: TrustedLen,
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]     |
[00:03:01] 11  | use iter::traits::marker::TrustedLen;
[00:03:01]     |
[00:03:01]     |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:01]    --> src/libcore/iter/adapters.rs:869:29
[00:03:01]     |
[00:03:01] 869 |     where A: TrustedLen, B: TrustedLen,
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]     |
[00:03:01] 11  | use iter::traits::marker::TrustedLen;
[00:03:01]     |
[00:03:01]     |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1006:30
[00:03:01]      |
[00:03:01] 1006 | impl<B, I: FusedIterator, F> FusedIterator for Map<I, F>
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1006:12
[00:03:01]      |
[00:03:01] 1006 | impl<B, I: FusedIterator, F> FusedIterator for Map<I, F>
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1010:22
[00:03:01]      |
[00:03:01] 1010 | unsafe impl<B, I, F> TrustedLen for Map<I, F>
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::TrustedLen;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1011:14
[00:03:01]      |
[00:03:01] 1011 |     where I: TrustedLen,
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::TrustedLen;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1155:27
[00:03:01]      |
[00:03:01] 1155 | impl<I: FusedIterator, P> FusedIterator for Filter<I, P>
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1155:9
[00:03:01]      |
[00:03:01] 1155 | impl<I: FusedIterator, P> FusedIterator for Filter<I, P>
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1265:30
[00:03:01]      |
[00:03:01] 1265 | impl<B, I: FusedIterator, F> FusedIterator for FilterMap<I, F>
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1265:12
[00:03:01]      |
[00:03:01] 1265 | impl<B, I: FusedIterator, F> FusedIterator for FilterMap<I, F>
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1420:9
[00:03:01]      |
[00:03:01] 1420 | impl<I> FusedIterator for Enumerate<I> where I: FusedIterator {}
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1420:49
[00:03:01]      |
[00:03:01] 1420 | impl<I> FusedIterator for Enumerate<I> where I: FusedIterator {}
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1423:16
[00:03:01]      |
[00:03:01] 1423 | unsafe impl<I> TrustedLen for Enumerate<I>
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::TrustedLen;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1424:14
[00:03:01]      |
[00:03:01] 1424 |     where I: TrustedLen,
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::TrustedLen;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1533:24
[00:03:01]      |
[00:03:01] 1533 | impl<I: FusedIterator> FusedIterator for Peekable<I> {}
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1533:9
[00:03:01]      |
[00:03:01] 1533 | impl<I: FusedIterator> FusedIterator for Peekable<I> {}
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1661:12
[00:03:01]      |
[00:03:01] 1661 | impl<I, P> FusedIterator for SkipWhile<I, P>
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1662:14
[00:03:01]      |
[00:03:01] 1662 |     where I: FusedIterator, P: FnMut(&I::Item) -> bool {}
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1744:12
[00:03:01]      |
[00:03:01] 1744 | impl<I, P> FusedIterator for TakeWhile<I, P>
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1745:14
[00:03:01]      |
[00:03:01] 1745 |     where I: FusedIterator, P: FnMut(&I::Item) -> bool {}
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1883:9
[00:03:01]      |
[00:03:01] 1883 | impl<I> FusedIterator for Skip<I> where I: FusedIterator {}
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1883:44
[00:03:01]      |
[00:03:01] 1883 | impl<I> FusedIterator for Skip<I> where I: FusedIterator {}
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1968:9
[00:03:01]      |
[00:03:01] 1968 | impl<I> FusedIterator for Take<I> where I: FusedIterator {}
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1968:44
[00:03:01]      |
[00:03:01] 1968 | impl<I> FusedIterator for Take<I> where I: FusedIterator {}
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01]      |
[00:03:01] 11   | use iter::traits::marker::FusedIterator;
[00:03:01]      |
[00:03:01]      |
[00:03:01] 
[00:03:01] error[E0405]: cannot find trait `TrustedLen` in this scope
[00:03:01]     --> src/libcore/iter/adapters.rs:1971:28
[00:03:01]      |
[00:03:01] 1971 | unsafe impl<I: TrustedLen> TrustedLen for Take<I> {}
---
[00:03:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:07] expected success, got: exit code: 101
[00:03:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:07] Build completed unsuccessfully in 0:00:15
[00:03:07] make: *** [all] Error 1
[00:03:07] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ed19c14
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 17 23:23:43 UTC 2018
---
travis_time:end:09ea80c8:start=1545089023727492192,finish=1545089023731221529,duration=3729337
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e404aec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b15c1de
travis_time:start:0b15c1de
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3506ceb6
$ dmesg | grep -i kill
