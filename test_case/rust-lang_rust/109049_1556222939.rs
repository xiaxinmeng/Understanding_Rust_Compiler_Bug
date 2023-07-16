plain
........................................................................................ 3256/4153
...........iii.......................................................................... 3344/4153
........................................................................................ 3432/4153
........................................................................................ 3520/4153
.......................................................................F..F...........F. 3608/4153
F....................................................................................... 3696/4153
.....i....................i........................i........................i........... 3872/4153
.............i........................i........................i.....................i.. 3960/4153
......................i........................i........................i............... 4048/4153
.........i.............................................................................. 4136/4153
---
---- src/slice/mod.rs - slice::[T]::rsplit_array_mut (line 1904) stdout ----
error[E0308]: mismatched types
 --> src/slice/mod.rs:1908:5
  |
7 | let (left, right) = v.rsplit_array_mut::<4>();
  |     ^^^^^^^^^^^^^   ------------------------- this expression has type `Option<(&mut [{integer}], &mut [{integer}; 4])>`
  |     |
  |     expected `Option<(&mut [{integer}], ...)>`, found `(_, _)`
  |
  = note: expected enum `Option<(&mut [{integer}], &mut [{integer}; 4])>`
            found tuple `(_, _)`
help: try wrapping the pattern in `Some`
  |
7 | let Some((left, right)) = v.rsplit_array_mut::<4>();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
---- src/slice/mod.rs - slice::[T]::rsplit_array_ref (line 1856) stdout ----
error[E0308]: mismatched types
 --> src/slice/mod.rs:1862:8
  |
9 |    let (left, right) = v.rsplit_array_ref::<0>();
  |        ^^^^^^^^^^^^^   ------------------------- this expression has type `Option<(&[{integer}], &[{integer}; 0])>`
  |        |
  |        expected `Option<(&[{integer}], &[...; 0])>`, found `(_, _)`
  |
  = note: expected enum `Option<(&[{integer}], &[{integer}; 0])>`
            found tuple `(_, _)`
help: try wrapping the pattern in `Some`
  |
9 |    let Some((left, right)) = v.rsplit_array_ref::<0>();

error[E0308]: mismatched types
  --> src/slice/mod.rs:1868:9
   |
   |
15 |     let (left, right) = v.rsplit_array_ref::<2>();
   |         ^^^^^^^^^^^^^   ------------------------- this expression has type `Option<(&[{integer}], &[{integer}; 2])>`
   |         |
   |         expected `Option<(&[{integer}], &[...; 2])>`, found `(_, _)`
   |
   = note: expected enum `Option<(&[{integer}], &[{integer}; 2])>`
             found tuple `(_, _)`
help: try wrapping the pattern in `Some`
   |
15 |     let Some((left, right)) = v.rsplit_array_ref::<2>();

error[E0308]: mismatched types
  --> src/slice/mod.rs:1874:9
   |
   |
21 |     let (left, right) = v.rsplit_array_ref::<6>();
   |         ^^^^^^^^^^^^^   ------------------------- this expression has type `Option<(&[{integer}], &[{integer}; 6])>`
   |         |
   |         expected `Option<(&[{integer}], &[...; 6])>`, found `(_, _)`
   |
   = note: expected enum `Option<(&[{integer}], &[{integer}; 6])>`
             found tuple `(_, _)`
help: try wrapping the pattern in `Some`
   |
21 |     let Some((left, right)) = v.rsplit_array_ref::<6>();

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
---- src/slice/mod.rs - slice::[T]::split_array_mut (line 1819) stdout ----
error[E0308]: mismatched types
 --> src/slice/mod.rs:1823:5
  |
7 | let (left, right) = v.split_array_mut::<2>();
  |     ^^^^^^^^^^^^^   ------------------------ this expression has type `Option<(&mut [{integer}; 2], &mut [{integer}])>`
  |     |
  |     expected `Option<(&mut [{integer}; 2], ...)>`, found `(_, _)`
  |
  = note: expected enum `Option<(&mut [{integer}; 2], &mut [{integer}])>`
            found tuple `(_, _)`
help: try wrapping the pattern in `Some`
  |
7 | let Some((left, right)) = v.split_array_mut::<2>();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
---- src/slice/mod.rs - slice::[T]::split_array_ref (line 1771) stdout ----
error[E0308]: mismatched types
 --> src/slice/mod.rs:1777:8
  |
9 |    let (left, right) = v.split_array_ref::<0>();
  |        ^^^^^^^^^^^^^   ------------------------ this expression has type `Option<(&[{integer}; 0], &[{integer}])>`
  |        |
  |        expected `Option<(&[{integer}; 0], &[...])>`, found `(_, _)`
  |
  = note: expected enum `Option<(&[{integer}; 0], &[{integer}])>`
            found tuple `(_, _)`
help: try wrapping the pattern in `Some`
  |
9 |    let Some((left, right)) = v.split_array_ref::<0>();

error[E0308]: mismatched types
  --> src/slice/mod.rs:1783:9
   |
   |
15 |     let (left, right) = v.split_array_ref::<2>();
   |         ^^^^^^^^^^^^^   ------------------------ this expression has type `Option<(&[{integer}; 2], &[{integer}])>`
   |         |
   |         expected `Option<(&[{integer}; 2], &[...])>`, found `(_, _)`
   |
   = note: expected enum `Option<(&[{integer}; 2], &[{integer}])>`
             found tuple `(_, _)`
help: try wrapping the pattern in `Some`
   |
15 |     let Some((left, right)) = v.split_array_ref::<2>();

error[E0308]: mismatched types
  --> src/slice/mod.rs:1789:9
   |
   |
21 |     let (left, right) = v.split_array_ref::<6>();
   |         ^^^^^^^^^^^^^   ------------------------ this expression has type `Option<(&[{integer}; 6], &[{integer}])>`
   |         |
   |         expected `Option<(&[{integer}; 6], &[...])>`, found `(_, _)`
   |
   = note: expected enum `Option<(&[{integer}; 6], &[{integer}])>`
             found tuple `(_, _)`
help: try wrapping the pattern in `Some`
   |
21 |     let Some((left, right)) = v.split_array_ref::<6>();

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
---
    src/slice/mod.rs - slice::[T]::split_array_ref (line 1771)

test result: FAILED. 4108 passed; 4 failed; 41 ignored; 0 measured; 0 filtered out; finished in 60.69s

error: doctest failed, to rerun pass `-p core --doc`
