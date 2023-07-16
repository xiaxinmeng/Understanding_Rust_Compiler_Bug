plain
travis_time:end:27230b80:start=1540592126364655246,finish=1540592185337873655,duration=58973218409
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:19:36]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:19:43]    Compiling compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
[00:19:43]    Compiling cmake v0.1.33
[00:19:43]    Compiling alloc_jemalloc v0.0.0 (/checkout/src/liballoc_jemalloc)
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2582 | where I::Item: IntoIterator {
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2581 | pub struct Flatten<I: Iterator>
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2582 | where I::Item: IntoIterator {
[00:19:46]      |       ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0221]: ambiguous associated type `Item` in bounds of `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2583 |     inner: FlattenCompat<I, <I::Item as IntoIterator>::IntoIter>,
[00:19:46]      |                              ^^^^^^^ ambiguous associated type `Item`
[00:19:46]     ::: libcore/iter/traits.rs:225:5
[00:19:46]      |
[00:19:46] 225  |     type Item;
[00:19:46] 225  |     type Item;
[00:19:46]      |     ---------- ambiguous `Item` from `iter::traits::IntoIterator`
[00:19:46]     ::: libcore/iter/iterator.rs:104:5
[00:19:46]      |
[00:19:46] 104  |     type Item;
[00:19:46] 104  |     type Item;
[00:19:46]      |     ---------- ambiguous `Item` from `iter::iterator::Iterator`
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `P`
[00:19:46]    --> libcore/pin.rs:121:5
[00:19:46]     |
[00:19:46] 121 |     P::Target: Unpin,
[00:19:46]     |
[00:19:46]     |
[00:19:46]     = note: ...which again requires computing the bounds for type parameter `P`, completing the cycle
[00:19:46] note: cycle used when processing `pin::<impl at libcore/pin.rs:119:1: 132:2>`
[00:19:46]    --> libcore/pin.rs:121:5
[00:19:46]     |
[00:19:46] 121 |     P::Target: Unpin,
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Target` not found for `P`
[00:19:46]    --> libcore/pin.rs:121:5
[00:19:46]     |
[00:19:46] 121 |     P::Target: Unpin,
[00:19:46]     |     ^^^^^^^^^ associated type `Target` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `P`
[00:19:46]    --> libcore/pin.rs:288:5
[00:19:46]     |
[00:19:46] 288 |     P::Target: Unpin
[00:19:46]     |
[00:19:46]     |
[00:19:46]     = note: ...which again requires computing the bounds for type parameter `P`, completing the cycle
[00:19:46] note: cycle used when processing `pin::<impl at libcore/pin.rs:286:1: 293:2>`
[00:19:46]    --> libcore/pin.rs:288:5
[00:19:46]     |
[00:19:46] 288 |     P::Target: Unpin
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Target` not found for `P`
[00:19:46]    --> libcore/pin.rs:288:5
[00:19:46]     |
[00:19:46] 288 |     P::Target: Unpin
[00:19:46]     |     ^^^^^^^^^ associated type `Target` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2600 |           FProj: FnMut(&I::Item) -> B,
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::iterator::select_fold1`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2600 |           FProj: FnMut(&I::Item) -> B,
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2600 |           FProj: FnMut(&I::Item) -> B,
[00:19:46]      |                         ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2601 |           FCmp: FnMut(&B, &I::Item, &B, &I::Item) -> bool
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::iterator::select_fold1`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2600 |           FProj: FnMut(&I::Item) -> B,
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2601 |           FCmp: FnMut(&B, &I::Item, &B, &I::Item) -> bool
[00:19:46]      |                            ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2601 |           FCmp: FnMut(&B, &I::Item, &B, &I::Item) -> bool
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::iterator::select_fold1`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2600 |           FProj: FnMut(&I::Item) -> B,
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2601 |           FCmp: FnMut(&B, &I::Item, &B, &I::Item) -> bool
[00:19:46]      |                                          ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `A`
[00:19:46]     |
[00:19:46]     |
[00:19:46] 854 |     B: Iterator<Item = A::Item>
[00:19:46]     |
[00:19:46]     |
[00:19:46]     = note: ...which again requires computing the bounds for type parameter `A`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:852:1: 989:2>`
[00:19:46]     |
[00:19:46]     |
[00:19:46] 854 |     B: Iterator<Item = A::Item>
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `A`
[00:19:46]     |
[00:19:46]     |
[00:19:46] 854 |     B: Iterator<Item = A::Item>
[00:19:46]     |                        ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0221]: ambiguous associated type `Item` in bounds of `A`
[00:19:46]     |
[00:19:46]     |
[00:19:46] 854 |     B: Iterator<Item = A::Item>
[00:19:46]     |                        ^^^^^^^ ambiguous associated type `Item`
[00:19:46]    ::: libcore/iter/iterator.rs:104:5
[00:19:46]     |
[00:19:46] 104 |     type Item;
[00:19:46]     |     ----------
[00:19:46]     |     ----------
[00:19:46]     |     |
[00:19:46]     |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:46]     |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `A`
[00:19:46]     |
[00:19:46]     |
[00:19:46] 994 |     B: DoubleEndedIterator<Item=A::Item>,
[00:19:46]     |
[00:19:46]     |
[00:19:46]     = note: ...which again requires computing the bounds for type parameter `A`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:992:1: 1049:2>`
[00:19:46]     |
[00:19:46]     |
[00:19:46] 994 |     B: DoubleEndedIterator<Item=A::Item>,
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `A`
[00:19:46]     |
[00:19:46]     |
[00:19:46] 994 |     B: DoubleEndedIterator<Item=A::Item>,
[00:19:46]     |                                 ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0221]: ambiguous associated type `Item` in bounds of `A`
[00:19:46]     |
[00:19:46]     |
[00:19:46] 994 |     B: DoubleEndedIterator<Item=A::Item>,
[00:19:46]     |                                 ^^^^^^^ ambiguous associated type `Item`
[00:19:46]    ::: libcore/iter/iterator.rs:104:5
[00:19:46]     |
[00:19:46] 104 |     type Item;
[00:19:46]     |     ----------
[00:19:46]     |     ----------
[00:19:46]     |     |
[00:19:46]     |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:46]     |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `A`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1055 |           B: FusedIterator<Item=A::Item>,
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `A`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1053:1: 1056:3>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1055 |           B: FusedIterator<Item=A::Item>,
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `A`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1055 |           B: FusedIterator<Item=A::Item>,
[00:19:46]      |                                 ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0221]: ambiguous associated type `Item` in bounds of `A`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1055 |           B: FusedIterator<Item=A::Item>,
[00:19:46]      |                                 ^^^^^^^ ambiguous associated type `Item`
[00:19:46]     ::: libcore/iter/iterator.rs:104:5
[00:19:46]      |
[00:19:46] 104  |     type Item;
[00:19:46]      |     ----------
[00:19:46]      |     ----------
[00:19:46]      |     |
[00:19:46]      |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:46]      |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `A`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1060 |     where A: TrustedLen, B: TrustedLen<Item=A::Item>,
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `A`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1059:1: 1061:3>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1060 |     where A: TrustedLen, B: TrustedLen<Item=A::Item>,
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `A`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1060 |     where A: TrustedLen, B: TrustedLen<Item=A::Item>,
[00:19:46]      |                                             ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0221]: ambiguous associated type `Item` in bounds of `A`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1060 |     where A: TrustedLen, B: TrustedLen<Item=A::Item>,
[00:19:46]      |                                             ^^^^^^^ ambiguous associated type `Item`
[00:19:46]     ::: libcore/iter/iterator.rs:104:5
[00:19:46]      |
[00:19:46] 104  |     type Item;
[00:19:46]      |     ----------
[00:19:46]      |     ----------
[00:19:46]      |     |
[00:19:46]      |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:46]      |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1389 | impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1389:1: 1415:2>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1389 | impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1389 | impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
[00:19:46]      |                                                               ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1419 |     F: FnMut(I::Item) -> B,
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1418:1: 1439:2>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1419 |     F: FnMut(I::Item) -> B,
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1419 |     F: FnMut(I::Item) -> B,
[00:19:46]      |              ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1443 |     where F: FnMut(I::Item) -> B
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1442:1: 1452:2>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1443 |     where F: FnMut(I::Item) -> B
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1443 |     where F: FnMut(I::Item) -> B
[00:19:46]      |                    ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1456 |     where F: FnMut(I::Item) -> B {}
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1455:1: 1456:36>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1456 |     where F: FnMut(I::Item) -> B {}
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1456 |     where F: FnMut(I::Item) -> B {}
[00:19:46]      |                    ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1461 |           F: FnMut(I::Item) -> B {}
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1459:1: 1461:36>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1461 |           F: FnMut(I::Item) -> B {}
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1461 |           F: FnMut(I::Item) -> B {}
[00:19:46]      |                    ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1466 |           F: FnMut(I::Item) -> B,
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1464:1: 1473:2>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1466 |           F: FnMut(I::Item) -> B,
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1466 |           F: FnMut(I::Item) -> B,
[00:19:46]      |                    ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1500 | impl<I: Iterator, P> Iterator for Filter<I, P> where P: FnMut(&I::Item) -> bool {
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1500:1: 1562:2>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1500 | impl<I: Iterator, P> Iterator for Filter<I, P> where P: FnMut(&I::Item) -> bool {
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1500 | impl<I: Iterator, P> Iterator for Filter<I, P> where P: FnMut(&I::Item) -> bool {
[00:19:46]      |                                                                ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1566 |     where P: FnMut(&I::Item) -> bool,
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1565:1: 1601:2>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1566 |     where P: FnMut(&I::Item) -> bool,
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1566 |     where P: FnMut(&I::Item) -> bool,
[00:19:46]      |                     ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1605 |     where P: FnMut(&I::Item) -> bool {}
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1604:1: 1605:40>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1605 |     where P: FnMut(&I::Item) -> bool {}
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1605 |     where P: FnMut(&I::Item) -> bool {}
[00:19:46]      |                     ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1633 |     where F: FnMut(I::Item) -> Option<B>,
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1632:1: 1674:2>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1633 |     where F: FnMut(I::Item) -> Option<B>,
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1633 |     where F: FnMut(I::Item) -> Option<B>,
[00:19:46]      |                    ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1678 |     where F: FnMut(I::Item) -> Option<B>,
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1677:1: 1711:2>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1678 |     where F: FnMut(I::Item) -> Option<B>,
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1678 |     where F: FnMut(I::Item) -> Option<B>,
[00:19:46]      |                    ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1715 |     where F: FnMut(I::Item) -> Option<B> {}
[00:19:46]      |
[00:19:46]      |
[00:19:46]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:46] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1714:1: 1715:44>`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1715 |     where F: FnMut(I::Item) -> Option<B> {}
[00:19:46] 
[00:19:46] 
[00:19:46] error[E0220]: associated type `Item` not found for `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 1715 |     where F: FnMut(I::Item) -> Option<B> {}
[00:19:46]      |                    ^^^^^^^ associated type `Item` not found
[00:19:46] 
[00:19:46] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:46]      |
[00:19:46]      |
[00:19:46] 2071 |     where P: FnMut(&I::Item) -> bool
[00:19:46]      |
[00:19:46]      |
---
151412 ./src/tools/clang
149644 ./obj/build/bootstrap/debug/incremental
149112 ./src/llvm-emscripten/test
134188 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0
134184 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0/s-f63ecq6kb1-1hj3nk-ycmuivyxm2px
107668 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
95516 ./obj/build/x86_64-unknown-linux-gnu/stage1
95496 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
