plain
.................................................................................................... 2800/3723
.................................................................................................... 2900/3723
...........iii...................................................................................... 3000/3723
.................................................................................................... 3100/3723
..........FF....F..F..FF.F....F..................................................................... 3200/3723
....................................................i............................................... 3400/3723
....i................i.....................i.....................i.....................i............ 3500/3723
.........i................................i.....................i.....................i............. 3600/3723
........i.....................i..................................................................... 3700/3723
........i.....................i..................................................................... 3700/3723
.......................
failures:

---- src/slice/iter.rs - slice::iter::RSplitNInclusive (line 1296) stdout ----
error[E0658]: use of unstable library feature 'split_inclusive_variants'
  |
  |
5 | let mut iter = slice.rsplitn_inclusive(|num| num % 3 == 0);
  |
  = help: add `#![feature(split_inclusive_variants)]` to the crate attributes to enable

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> src/slice/iter.rs:1298:22
     |
5    | let mut iter = slice.rsplitn_inclusive(|num| num % 3 == 0);
     |                      |
     |                      expected 2 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/library/core/src/slice/mod.rs:2370:12
     |
2370 |     pub fn rsplitn_inclusive<F>(&self, n: usize, pred: F) -> RSplitNInclusive<'_, T, F>

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0061, E0658.
Some errors have detailed explanations: E0061, E0658.
For more information about an error, try `rustc --explain E0061`.
Couldn't compile the test.
---- src/slice/iter.rs - slice::iter::RSplitNInclusiveMut (line 1296) stdout ----
error[E0658]: use of unstable library feature 'split_inclusive_variants'
  |
  |
5 | let mut iter = slice.rsplitn_inclusive(|num| num % 3 == 0);
  |
  = help: add `#![feature(split_inclusive_variants)]` to the crate attributes to enable

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> src/slice/iter.rs:1298:22
     |
5    | let mut iter = slice.rsplitn_inclusive(|num| num % 3 == 0);
     |                      |
     |                      expected 2 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/library/core/src/slice/mod.rs:2370:12
     |
2370 |     pub fn rsplitn_inclusive<F>(&self, n: usize, pred: F) -> RSplitNInclusive<'_, T, F>

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0061, E0658.
Some errors have detailed explanations: E0061, E0658.
For more information about an error, try `rustc --explain E0061`.
Couldn't compile the test.
---- src/slice/iter.rs - slice::iter::RSplitNLeftInclusive (line 1296) stdout ----
error[E0658]: use of unstable library feature 'split_inclusive_variants'
  |
  |
5 | let mut iter = slice.rsplitn_left_inclusive(|num| num % 3 == 0);
  |
  = help: add `#![feature(split_inclusive_variants)]` to the crate attributes to enable

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> src/slice/iter.rs:1298:22
     |
5    | let mut iter = slice.rsplitn_left_inclusive(|num| num % 3 == 0);
     |                      |
     |                      expected 2 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/library/core/src/slice/mod.rs:2436:12
     |
2436 |     pub fn rsplitn_left_inclusive<F>(&self, n: usize, pred: F) -> RSplitNLeftInclusive<'_, T, F>

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0061, E0658.
Some errors have detailed explanations: E0061, E0658.
For more information about an error, try `rustc --explain E0061`.
Couldn't compile the test.
---- src/slice/iter.rs - slice::iter::RSplitNLeftInclusiveMut (line 1296) stdout ----
error[E0658]: use of unstable library feature 'split_inclusive_variants'
  |
  |
5 | let mut iter = slice.rsplitn_left_inclusive(|num| num % 3 == 0);
  |
  = help: add `#![feature(split_inclusive_variants)]` to the crate attributes to enable

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> src/slice/iter.rs:1298:22
     |
5    | let mut iter = slice.rsplitn_left_inclusive(|num| num % 3 == 0);
     |                      |
     |                      expected 2 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/library/core/src/slice/mod.rs:2436:12
     |
2436 |     pub fn rsplitn_left_inclusive<F>(&self, n: usize, pred: F) -> RSplitNLeftInclusive<'_, T, F>

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0061, E0658.
Some errors have detailed explanations: E0061, E0658.
For more information about an error, try `rustc --explain E0061`.
Couldn't compile the test.
---- src/slice/iter.rs - slice::iter::SplitNInclusive (line 1296) stdout ----
error[E0658]: use of unstable library feature 'split_inclusive_variants'
  |
  |
5 | let mut iter = slice.splitn_inclusive(|num| num % 3 == 0);
  |
  = help: add `#![feature(split_inclusive_variants)]` to the crate attributes to enable

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> src/slice/iter.rs:1298:22
     |
5    | let mut iter = slice.splitn_inclusive(|num| num % 3 == 0);
     |                      |
     |                      expected 2 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/library/core/src/slice/mod.rs:2246:12
     |
2246 |     pub fn splitn_inclusive<F>(&self, n: usize, pred: F) -> SplitNInclusive<'_, T, F>

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0061, E0658.
Some errors have detailed explanations: E0061, E0658.
For more information about an error, try `rustc --explain E0061`.
Couldn't compile the test.
---- src/slice/iter.rs - slice::iter::SplitNInclusiveMut (line 1296) stdout ----
error[E0658]: use of unstable library feature 'split_inclusive_variants'
  |
  |
5 | let mut iter = slice.splitn_inclusive_mut(|num| num % 3 == 0);
  |
  = help: add `#![feature(split_inclusive_variants)]` to the crate attributes to enable

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> src/slice/iter.rs:1298:22
     |
5    | let mut iter = slice.splitn_inclusive_mut(|num| num % 3 == 0);
     |                      |
     |                      expected 2 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/library/core/src/slice/mod.rs:2272:12
     |
2272 |     pub fn splitn_inclusive_mut<F>(&mut self, n: usize, pred: F) -> SplitNInclusiveMut<'_, T, F>

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0061, E0658.
Some errors have detailed explanations: E0061, E0658.
For more information about an error, try `rustc --explain E0061`.
Couldn't compile the test.
---- src/slice/iter.rs - slice::iter::SplitNLeftInclusive (line 1296) stdout ----
error[E0658]: use of unstable library feature 'split_inclusive_variants'
  |
  |
5 | let mut iter = slice.splitn_left_inclusive(|num| num % 3 == 0);
  |
  = help: add `#![feature(split_inclusive_variants)]` to the crate attributes to enable

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> src/slice/iter.rs:1298:22
     |
5    | let mut iter = slice.splitn_left_inclusive(|num| num % 3 == 0);
     |                      |
     |                      expected 2 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/library/core/src/slice/mod.rs:2312:12
     |
2312 |     pub fn splitn_left_inclusive<F>(&self, n: usize, pred: F) -> SplitNLeftInclusive<'_, T, F>

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0061, E0658.
Some errors have detailed explanations: E0061, E0658.
For more information about an error, try `rustc --explain E0061`.
Couldn't compile the test.
---- src/slice/iter.rs - slice::iter::SplitNLeftInclusiveMut (line 1296) stdout ----
error[E0658]: use of unstable library feature 'split_inclusive_variants'
  |
  |
5 | let mut iter = slice.splitn_left_inclusive_mut(|num| num % 3 == 0);
  |
  = help: add `#![feature(split_inclusive_variants)]` to the crate attributes to enable

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> src/slice/iter.rs:1298:22
     |
5    | let mut iter = slice.splitn_left_inclusive_mut(|num| num % 3 == 0);
     |                      |
     |                      expected 2 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/library/core/src/slice/mod.rs:2337:12
     |
2337 |     pub fn splitn_left_inclusive_mut<F>(

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0061, E0658.
Some errors have detailed explanations: E0061, E0658.
For more information about an error, try `rustc --explain E0061`.
Couldn't compile the test.

failures:
    src/slice/iter.rs - slice::iter::RSplitNInclusive (line 1296)
    src/slice/iter.rs - slice::iter::RSplitNInclusiveMut (line 1296)
    src/slice/iter.rs - slice::iter::RSplitNLeftInclusive (line 1296)
    src/slice/iter.rs - slice::iter::RSplitNLeftInclusiveMut (line 1296)
    src/slice/iter.rs - slice::iter::SplitNInclusive (line 1296)
    src/slice/iter.rs - slice::iter::SplitNInclusiveMut (line 1296)
    src/slice/iter.rs - slice::iter::SplitNLeftInclusive (line 1296)
    src/slice/iter.rs - slice::iter::SplitNLeftInclusiveMut (line 1296)
test result: FAILED. 3683 passed; 8 failed; 32 ignored; 0 measured; 0 filtered out; finished in 67.01s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:21:59
