plain
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0277]: the trait bound `compile::Std: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:42:33
    |
42  |             first(builder.cache.all::<compile::Std>()),
    |                                 ^^^ the trait `Copy` is not implemented for `compile::Std`
note: required by a bound in `Cache::all`
   --> src/bootstrap/cache.rs:273:25
    |
    |
273 |     pub fn all<S: Ord + Copy + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                         ^^^^ required by this bound in `Cache::all`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
  --> src/bootstrap/builder/tests.rs:44:17
   |
44 |                 compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a },
   |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
  --> src/bootstrap/builder/tests.rs:45:17
   |
45 |                 compile::Std { compiler: Compiler { host: a, stage: 1 }, target: a },
   |                 ^^^^^^^^^^^^ missing `tail_args`
error[E0277]: the trait bound `compile::Rustc: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:57:33
    |
    |
57  |             first(builder.cache.all::<compile::Rustc>()),
    |                                 ^^^ the trait `Copy` is not implemented for `compile::Rustc`
note: required by a bound in `Cache::all`
   --> src/bootstrap/cache.rs:273:25
    |
    |
273 |     pub fn all<S: Ord + Copy + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                         ^^^^ required by this bound in `Cache::all`

error[E0063]: missing field `tail_args` in initializer of `compile::Rustc`
  --> src/bootstrap/builder/tests.rs:58:15
   |
58 |             &[compile::Rustc { compiler: Compiler { host: a, stage: 0 }, target: a },]
   |               ^^^^^^^^^^^^^^ missing `tail_args`
error[E0277]: the trait bound `compile::Std: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:71:33
    |
    |
71  |             first(builder.cache.all::<compile::Std>()),
    |                                 ^^^ the trait `Copy` is not implemented for `compile::Std`
note: required by a bound in `Cache::all`
   --> src/bootstrap/cache.rs:273:25
    |
    |
273 |     pub fn all<S: Ord + Copy + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                         ^^^^ required by this bound in `Cache::all`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
  --> src/bootstrap/builder/tests.rs:72:15
   |
72 |             &[compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a },]
   |               ^^^^^^^^^^^^ missing `tail_args`
error[E0277]: the trait bound `compile::Rustc: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:81:31
    |
    |
81  |         assert!(builder.cache.all::<compile::Rustc>().is_empty());
    |                               ^^^ the trait `Copy` is not implemented for `compile::Rustc`
note: required by a bound in `Cache::all`
   --> src/bootstrap/cache.rs:273:25
    |
    |
273 |     pub fn all<S: Ord + Copy + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                         ^^^^ required by this bound in `Cache::all`
error[E0277]: the trait bound `compile::Std: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:100:33
    |
    |
100 |             first(builder.cache.all::<compile::Std>()),
    |                                 ^^^ the trait `Copy` is not implemented for `compile::Std`
note: required by a bound in `Cache::all`
   --> src/bootstrap/cache.rs:273:25
    |
    |
273 |     pub fn all<S: Ord + Copy + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                         ^^^^ required by this bound in `Cache::all`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:102:17
    |
102 |                 compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:103:17
    |
103 |                 compile::Std { compiler: Compiler { host: a, stage: 1 }, target: a },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:104:17
    |
104 |                 compile::Std { compiler: Compiler { host: a, stage: 0 }, target: b },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:105:17
    |
105 |                 compile::Std { compiler: Compiler { host: a, stage: 1 }, target: b },
    |                 ^^^^^^^^^^^^ missing `tail_args`
error[E0277]: the trait bound `compile::Rustc: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:124:33
    |
    |
124 |             first(builder.cache.all::<compile::Rustc>()),
    |                                 ^^^ the trait `Copy` is not implemented for `compile::Rustc`
note: required by a bound in `Cache::all`
   --> src/bootstrap/cache.rs:273:25
    |
    |
273 |     pub fn all<S: Ord + Copy + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                         ^^^^ required by this bound in `Cache::all`

error[E0063]: missing field `tail_args` in initializer of `compile::Rustc`
   --> src/bootstrap/builder/tests.rs:126:17
    |
126 |                 compile::Rustc { compiler: Compiler { host: a, stage: 0 }, target: a },
    |                 ^^^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Rustc`
   --> src/bootstrap/builder/tests.rs:127:17
    |
127 |                 compile::Rustc { compiler: Compiler { host: a, stage: 0 }, target: b },
    |                 ^^^^^^^^^^^^^^ missing `tail_args`
error[E0277]: the trait bound `compile::Std: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:260:33
    |
    |
260 |             first(builder.cache.all::<compile::Std>()),
    |                                 ^^^ the trait `Copy` is not implemented for `compile::Std`
note: required by a bound in `Cache::all`
   --> src/bootstrap/cache.rs:273:25
    |
    |
273 |     pub fn all<S: Ord + Copy + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                         ^^^^ required by this bound in `Cache::all`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:262:17
    |
262 |                 compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:263:17
    |
263 |                 compile::Std { compiler: Compiler { host: a, stage: 1 }, target: a },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:264:17
    |
264 |                 compile::Std { compiler: Compiler { host: a, stage: 2 }, target: a },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:265:17
    |
265 |                 compile::Std { compiler: Compiler { host: a, stage: 1 }, target: b },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:266:17
    |
266 |                 compile::Std { compiler: Compiler { host: a, stage: 2 }, target: b },
    |                 ^^^^^^^^^^^^ missing `tail_args`
error[E0277]: the trait bound `compile::Rustc: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:288:33
    |
    |
288 |             first(builder.cache.all::<compile::Rustc>()),
    |                                 ^^^ the trait `Copy` is not implemented for `compile::Rustc`
note: required by a bound in `Cache::all`
   --> src/bootstrap/cache.rs:273:25
    |
    |
273 |     pub fn all<S: Ord + Copy + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                         ^^^^ required by this bound in `Cache::all`

error[E0063]: missing field `tail_args` in initializer of `compile::Rustc`
   --> src/bootstrap/builder/tests.rs:290:17
    |
290 |                 compile::Rustc { compiler: Compiler { host: a, stage: 0 }, target: a },
    |                 ^^^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Rustc`
   --> src/bootstrap/builder/tests.rs:291:17
    |
291 |                 compile::Rustc { compiler: Compiler { host: a, stage: 1 }, target: b },
    |                 ^^^^^^^^^^^^^^ missing `tail_args`
error[E0277]: the trait bound `compile::Std: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:383:33
    |
    |
383 |             first(builder.cache.all::<compile::Std>()),
    |                                 ^^^ the trait `Copy` is not implemented for `compile::Std`
note: required by a bound in `Cache::all`
   --> src/bootstrap/cache.rs:273:25
    |
    |
273 |     pub fn all<S: Ord + Copy + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                         ^^^^ required by this bound in `Cache::all`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:385:17
    |
385 |                 compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:386:17
    |
386 |                 compile::Std { compiler: Compiler { host: a, stage: 1 }, target: a },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:387:17
    |
387 |                 compile::Std { compiler: Compiler { host: a, stage: 2 }, target: a },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:388:17
    |
388 |                 compile::Std { compiler: Compiler { host: a, stage: 1 }, target: b },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:389:17
    |
389 |                 compile::Std { compiler: Compiler { host: a, stage: 2 }, target: b },
    |                 ^^^^^^^^^^^^ missing `tail_args`
error[E0277]: the trait bound `compile::Std: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:417:33
    |
    |
417 |             first(builder.cache.all::<compile::Std>()),
    |                                 ^^^ the trait `Copy` is not implemented for `compile::Std`
note: required by a bound in `Cache::all`
   --> src/bootstrap/cache.rs:273:25
    |
    |
273 |     pub fn all<S: Ord + Copy + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                         ^^^^ required by this bound in `Cache::all`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:419:17
    |
419 |                 compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:420:17
    |
420 |                 compile::Std { compiler: Compiler { host: a, stage: 1 }, target: a },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:421:17
    |
421 |                 compile::Std { compiler: Compiler { host: a, stage: 2 }, target: a },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:422:17
    |
422 |                 compile::Std { compiler: Compiler { host: a, stage: 1 }, target: b },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:423:17
    |
423 |                 compile::Std { compiler: Compiler { host: a, stage: 2 }, target: b },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:424:17
    |
424 |                 compile::Std { compiler: Compiler { host: a, stage: 2 }, target: c },
    |                 ^^^^^^^^^^^^ missing `tail_args`
error[E0277]: the trait bound `compile::Rustc: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:429:33
    |
    |
429 |             first(builder.cache.all::<compile::Rustc>()),
    |                                 ^^^ the trait `Copy` is not implemented for `compile::Rustc`
note: required by a bound in `Cache::all`
   --> src/bootstrap/cache.rs:273:25
    |
    |
273 |     pub fn all<S: Ord + Copy + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                         ^^^^ required by this bound in `Cache::all`

error[E0063]: missing field `tail_args` in initializer of `compile::Rustc`
   --> src/bootstrap/builder/tests.rs:431:17
    |
431 |                 compile::Rustc { compiler: Compiler { host: a, stage: 0 }, target: a },
    |                 ^^^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Rustc`
   --> src/bootstrap/builder/tests.rs:432:17
    |
432 |                 compile::Rustc { compiler: Compiler { host: a, stage: 1 }, target: a },
    |                 ^^^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Rustc`
   --> src/bootstrap/builder/tests.rs:433:17
    |
433 |                 compile::Rustc { compiler: Compiler { host: a, stage: 2 }, target: a },
    |                 ^^^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Rustc`
   --> src/bootstrap/builder/tests.rs:434:17
    |
434 |                 compile::Rustc { compiler: Compiler { host: a, stage: 1 }, target: b },
    |                 ^^^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Rustc`
   --> src/bootstrap/builder/tests.rs:435:17
    |
435 |                 compile::Rustc { compiler: Compiler { host: a, stage: 2 }, target: b },
    |                 ^^^^^^^^^^^^^^ missing `tail_args`
error[E0277]: the trait bound `compile::Std: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:451:33
    |
    |
451 |             first(builder.cache.all::<compile::Std>()),
    |                                 ^^^ the trait `Copy` is not implemented for `compile::Std`
note: required by a bound in `Cache::all`
   --> src/bootstrap/cache.rs:273:25
    |
    |
273 |     pub fn all<S: Ord + Copy + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                         ^^^^ required by this bound in `Cache::all`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:453:17
    |
453 |                 compile::Std { compiler: Compiler { host: a, stage: 0 }, target: a },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:454:17
    |
454 |                 compile::Std { compiler: Compiler { host: a, stage: 1 }, target: a },
    |                 ^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Std`
   --> src/bootstrap/builder/tests.rs:455:17
    |
455 |                 compile::Std { compiler: Compiler { host: a, stage: 2 }, target: c },
    |                 ^^^^^^^^^^^^ missing `tail_args`
error[E0277]: the trait bound `compile::Rustc: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:467:33
    |
    |
467 |             first(builder.cache.all::<compile::Rustc>()),
    |                                 ^^^ the trait `Copy` is not implemented for `compile::Rustc`
note: required by a bound in `Cache::all`
   --> src/bootstrap/cache.rs:273:25
    |
    |
273 |     pub fn all<S: Ord + Copy + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                         ^^^^ required by this bound in `Cache::all`

error[E0063]: missing field `tail_args` in initializer of `compile::Rustc`
   --> src/bootstrap/builder/tests.rs:469:17
    |
469 |                 compile::Rustc { compiler: Compiler { host: a, stage: 0 }, target: a },
    |                 ^^^^^^^^^^^^^^ missing `tail_args`

error[E0063]: missing field `tail_args` in initializer of `compile::Rustc`
   --> src/bootstrap/builder/tests.rs:470:17
    |
470 |                 compile::Rustc { compiler: Compiler { host: a, stage: 1 }, target: a },
    |                 ^^^^^^^^^^^^^^ missing `tail_args`
Some errors have detailed explanations: E0063, E0277.
For more information about an error, try `rustc --explain E0063`.
error: could not compile `bootstrap` due to 51 previous errors
Build completed unsuccessfully in 0:35:31
