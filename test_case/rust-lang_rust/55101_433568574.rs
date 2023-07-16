plain
travis_time:end:08055814:start=1540595096229658178,finish=1540595158920930693,duration=62691272515
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:19:42]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:19:48]    Compiling compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
[00:19:48]    Compiling cmake v0.1.33
[00:19:48]    Compiling alloc_jemalloc v0.0.0 (/checkout/src/liballoc_jemalloc)
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2582 | where I::Item: IntoIterator {
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2581 | pub struct Flatten<I: Iterator>
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2582 | where I::Item: IntoIterator {
[00:19:52]      |       ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0221]: ambiguous associated type `Item` in bounds of `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2583 |     inner: FlattenCompat<I, <I::Item as IntoIterator>::IntoIter>,
[00:19:52]      |                              ^^^^^^^ ambiguous associated type `Item`
[00:19:52]     ::: libcore/iter/traits.rs:225:5
[00:19:52]      |
[00:19:52] 225  |     type Item;
[00:19:52] 225  |     type Item;
[00:19:52]      |     ---------- ambiguous `Item` from `iter::traits::IntoIterator`
[00:19:52]     ::: libcore/iter/iterator.rs:104:5
[00:19:52]      |
[00:19:52] 104  |     type Item;
[00:19:52] 104  |     type Item;
[00:19:52]      |     ---------- ambiguous `Item` from `iter::iterator::Iterator`
[00:19:52]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:19:52]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `P`
[00:19:52]    --> libcore/pin.rs:121:5
[00:19:52]     |
[00:19:52] 121 |     P::Target: Unpin,
[00:19:52]     |
[00:19:52]     |
[00:19:52]     = note: ...which again requires computing the bounds for type parameter `P`, completing the cycle
[00:19:52] note: cycle used when processing `pin::<impl at libcore/pin.rs:119:1: 132:2>`
[00:19:52]    --> libcore/pin.rs:121:5
[00:19:52]     |
[00:19:52] 121 |     P::Target: Unpin,
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Target` not found for `P`
[00:19:52]    --> libcore/pin.rs:121:5
[00:19:52]     |
[00:19:52] 121 |     P::Target: Unpin,
[00:19:52]     |     ^^^^^^^^^ associated type `Target` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `P`
[00:19:52]    --> libcore/pin.rs:288:5
[00:19:52]     |
[00:19:52] 288 |     P::Target: Unpin
[00:19:52]     |
[00:19:52]     |
[00:19:52]     = note: ...which again requires computing the bounds for type parameter `P`, completing the cycle
[00:19:52] note: cycle used when processing `pin::<impl at libcore/pin.rs:286:1: 293:2>`
[00:19:52]    --> libcore/pin.rs:288:5
[00:19:52]     |
[00:19:52] 288 |     P::Target: Unpin
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Target` not found for `P`
[00:19:52]    --> libcore/pin.rs:288:5
[00:19:52]     |
[00:19:52] 288 |     P::Target: Unpin
[00:19:52]     |     ^^^^^^^^^ associated type `Target` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2600 |           FProj: FnMut(&I::Item) -> B,
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::iterator::select_fold1`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2600 |           FProj: FnMut(&I::Item) -> B,
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2600 |           FProj: FnMut(&I::Item) -> B,
[00:19:52]      |                         ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2601 |           FCmp: FnMut(&B, &I::Item, &B, &I::Item) -> bool
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::iterator::select_fold1`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2600 |           FProj: FnMut(&I::Item) -> B,
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2601 |           FCmp: FnMut(&B, &I::Item, &B, &I::Item) -> bool
[00:19:52]      |                            ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2601 |           FCmp: FnMut(&B, &I::Item, &B, &I::Item) -> bool
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::iterator::select_fold1`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2600 |           FProj: FnMut(&I::Item) -> B,
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2601 |           FCmp: FnMut(&B, &I::Item, &B, &I::Item) -> bool
[00:19:52]      |                                          ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `A`
[00:19:52]     |
[00:19:52]     |
[00:19:52] 854 |     B: Iterator<Item = A::Item>
[00:19:52]     |
[00:19:52]     |
[00:19:52]     = note: ...which again requires computing the bounds for type parameter `A`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:852:1: 989:2>`
[00:19:52]     |
[00:19:52]     |
[00:19:52] 854 |     B: Iterator<Item = A::Item>
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `A`
[00:19:52]     |
[00:19:52]     |
[00:19:52] 854 |     B: Iterator<Item = A::Item>
[00:19:52]     |                        ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0221]: ambiguous associated type `Item` in bounds of `A`
[00:19:52]     |
[00:19:52]     |
[00:19:52] 854 |     B: Iterator<Item = A::Item>
[00:19:52]     |                        ^^^^^^^ ambiguous associated type `Item`
[00:19:52]    ::: libcore/iter/iterator.rs:104:5
[00:19:52]     |
[00:19:52] 104 |     type Item;
[00:19:52]     |     ----------
[00:19:52]     |     ----------
[00:19:52]     |     |
[00:19:52]     |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:52]     |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `A`
[00:19:52]     |
[00:19:52]     |
[00:19:52] 994 |     B: DoubleEndedIterator<Item=A::Item>,
[00:19:52]     |
[00:19:52]     |
[00:19:52]     = note: ...which again requires computing the bounds for type parameter `A`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:992:1: 1049:2>`
[00:19:52]     |
[00:19:52]     |
[00:19:52] 994 |     B: DoubleEndedIterator<Item=A::Item>,
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `A`
[00:19:52]     |
[00:19:52]     |
[00:19:52] 994 |     B: DoubleEndedIterator<Item=A::Item>,
[00:19:52]     |                                 ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0221]: ambiguous associated type `Item` in bounds of `A`
[00:19:52]     |
[00:19:52]     |
[00:19:52] 994 |     B: DoubleEndedIterator<Item=A::Item>,
[00:19:52]     |                                 ^^^^^^^ ambiguous associated type `Item`
[00:19:52]    ::: libcore/iter/iterator.rs:104:5
[00:19:52]     |
[00:19:52] 104 |     type Item;
[00:19:52]     |     ----------
[00:19:52]     |     ----------
[00:19:52]     |     |
[00:19:52]     |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:52]     |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `A`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1055 |           B: FusedIterator<Item=A::Item>,
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `A`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1053:1: 1056:3>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1055 |           B: FusedIterator<Item=A::Item>,
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `A`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1055 |           B: FusedIterator<Item=A::Item>,
[00:19:52]      |                                 ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0221]: ambiguous associated type `Item` in bounds of `A`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1055 |           B: FusedIterator<Item=A::Item>,
[00:19:52]      |                                 ^^^^^^^ ambiguous associated type `Item`
[00:19:52]     ::: libcore/iter/iterator.rs:104:5
[00:19:52]      |
[00:19:52] 104  |     type Item;
[00:19:52]      |     ----------
[00:19:52]      |     ----------
[00:19:52]      |     |
[00:19:52]      |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:52]      |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `A`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1060 |     where A: TrustedLen, B: TrustedLen<Item=A::Item>,
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `A`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1059:1: 1061:3>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1060 |     where A: TrustedLen, B: TrustedLen<Item=A::Item>,
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `A`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1060 |     where A: TrustedLen, B: TrustedLen<Item=A::Item>,
[00:19:52]      |                                             ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0221]: ambiguous associated type `Item` in bounds of `A`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1060 |     where A: TrustedLen, B: TrustedLen<Item=A::Item>,
[00:19:52]      |                                             ^^^^^^^ ambiguous associated type `Item`
[00:19:52]     ::: libcore/iter/iterator.rs:104:5
[00:19:52]      |
[00:19:52] 104  |     type Item;
[00:19:52]      |     ----------
[00:19:52]      |     ----------
[00:19:52]      |     |
[00:19:52]      |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:52]      |     ambiguous `Item` from `iter::iterator::Iterator`
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1389 | impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1389:1: 1415:2>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1389 | impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1389 | impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
[00:19:52]      |                                                               ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1419 |     F: FnMut(I::Item) -> B,
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1418:1: 1439:2>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1419 |     F: FnMut(I::Item) -> B,
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1419 |     F: FnMut(I::Item) -> B,
[00:19:52]      |              ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1443 |     where F: FnMut(I::Item) -> B
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1442:1: 1452:2>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1443 |     where F: FnMut(I::Item) -> B
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1443 |     where F: FnMut(I::Item) -> B
[00:19:52]      |                    ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1456 |     where F: FnMut(I::Item) -> B {}
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1455:1: 1456:36>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1456 |     where F: FnMut(I::Item) -> B {}
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1456 |     where F: FnMut(I::Item) -> B {}
[00:19:52]      |                    ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1461 |           F: FnMut(I::Item) -> B {}
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1459:1: 1461:36>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1461 |           F: FnMut(I::Item) -> B {}
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1461 |           F: FnMut(I::Item) -> B {}
[00:19:52]      |                    ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1466 |           F: FnMut(I::Item) -> B,
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1464:1: 1473:2>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1466 |           F: FnMut(I::Item) -> B,
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1466 |           F: FnMut(I::Item) -> B,
[00:19:52]      |                    ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1500 | impl<I: Iterator, P> Iterator for Filter<I, P> where P: FnMut(&I::Item) -> bool {
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1500:1: 1562:2>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1500 | impl<I: Iterator, P> Iterator for Filter<I, P> where P: FnMut(&I::Item) -> bool {
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1500 | impl<I: Iterator, P> Iterator for Filter<I, P> where P: FnMut(&I::Item) -> bool {
[00:19:52]      |                                                                ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1566 |     where P: FnMut(&I::Item) -> bool,
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1565:1: 1601:2>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1566 |     where P: FnMut(&I::Item) -> bool,
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1566 |     where P: FnMut(&I::Item) -> bool,
[00:19:52]      |                     ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1605 |     where P: FnMut(&I::Item) -> bool {}
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1604:1: 1605:40>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1605 |     where P: FnMut(&I::Item) -> bool {}
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1605 |     where P: FnMut(&I::Item) -> bool {}
[00:19:52]      |                     ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1633 |     where F: FnMut(I::Item) -> Option<B>,
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1632:1: 1674:2>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1633 |     where F: FnMut(I::Item) -> Option<B>,
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1633 |     where F: FnMut(I::Item) -> Option<B>,
[00:19:52]      |                    ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1678 |     where F: FnMut(I::Item) -> Option<B>,
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1677:1: 1711:2>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1678 |     where F: FnMut(I::Item) -> Option<B>,
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1678 |     where F: FnMut(I::Item) -> Option<B>,
[00:19:52]      |                    ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1715 |     where F: FnMut(I::Item) -> Option<B> {}
[00:19:52]      |
[00:19:52]      |
[00:19:52]      = note: ...which again requires computing the bounds for type parameter `I`, completing the cycle
[00:19:52] note: cycle used when processing `iter::<impl at libcore/iter/mod.rs:1714:1: 1715:44>`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1715 |     where F: FnMut(I::Item) -> Option<B> {}
[00:19:52] 
[00:19:52] 
[00:19:52] error[E0220]: associated type `Item` not found for `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 1715 |     where F: FnMut(I::Item) -> Option<B> {}
[00:19:52]      |                    ^^^^^^^ associated type `Item` not found
[00:19:52] 
[00:19:52] error[E0391]: cycle detected when computing the bounds for type parameter `I`
[00:19:52]      |
[00:19:52]      |
[00:19:52] 2071 |     where P: FnMut(&I::Item) -> bool
[00:19:52]      |
---
151412 ./src/tools/clang
149644 ./obj/build/bootstrap/debug/incremental
149112 ./src/llvm-emscripten/test
134188 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0
134184 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0/s-f63fpqzhtn-1rgwtcb-ycmuivyxm2px
107668 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
95520 ./obj/build/x86_64-unknown-linux-gnu/stage1
95500 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
---
travis_time:end:020f7669:start=1540596366299976952,finish=1540596366304672617,duration=4695665
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16523f04
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ef0d678
travis_time:start:0ef0d678
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or dir
