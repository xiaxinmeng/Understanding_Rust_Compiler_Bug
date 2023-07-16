plain
[01:25:12] 
[01:25:12] failures:
[01:25:12] 
[01:25:12] ---- [ui] ui/derive.rs stdout ----
[01:25:12] normalized stderr:
[01:25:12] error: you are implementing `Hash` explicitly but have derived `PartialEq`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 34 | / impl Hash for Bah {
[01:25:12] 35 | |     fn hash<H: Hasher>(&self, _: &mut H) {}
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12] note: `PartialEq` implemented here
[01:25:12]   --> $DIR/derive.rs:31:10
[01:25:12]   --> $DIR/derive.rs:31:10
[01:25:12]    |
[01:25:12] 31 | #[derive(PartialEq)]
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]    |
[01:25:12]    |
[01:25:12] 41 | / impl Clone for Qux {
[01:25:12] 42 | |     fn clone(&self) -> Self { Qux }
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D expl-impl-clone-on-copy` implied by `-D warnings`
[01:25:12] note: consider deriving `Clone` or removing `Copy`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 41 | / impl Clone for Qux {
[01:25:12] 42 | |     fn clone(&self) -> Self { Qux }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]    |
[01:25:12]    |
[01:25:12] 65 | / impl<'a> Clone for Lt<'a> {
[01:25:12] 66 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: consider deriving `Clone` or removing `Copy`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 65 | / impl<'a> Clone for Lt<'a> {
[01:25:12] 66 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]    |
[01:25:12]    |
[01:25:12] 75 | / impl Clone for BigArray {
[01:25:12] 76 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: consider deriving `Clone` or removing `Copy`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 75 | / impl Clone for BigArray {
[01:25:12] 76 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]    |
[01:25:12]    |
[01:25:12] 85 | / impl Clone for FnPtr {
[01:25:12] 86 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: consider deriving `Clone` or removing `Copy`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 85 | / impl Clone for FnPtr {
[01:25:12] 86 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] error: aborting due to 5 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] expected stderr:
[01:25:12] error: you are deriving `Hash` but have implemented `PartialEq` explicitly
[01:25:12]    |
[01:25:12]    |
[01:25:12] 17 | #[derive(Hash)]
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: #[deny(derive_hash_xor_eq)] on by default
[01:25:12] note: `PartialEq` implemented here
[01:25:12]    |
[01:25:12]    |
[01:25:12] 20 | / impl PartialEq for Bar {
[01:25:12] 21 | |     fn eq(&self, _: &Bar) -> bool { true }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are deriving `Hash` but have implemented `PartialEq` explicitly
[01:25:12]    |
[01:25:12]    |
[01:25:12] 24 | #[derive(Hash)]
[01:25:12]    |
[01:25:12] note: `PartialEq` implemented here
[01:25:12]   --> $DIR/derive.rs:27:1
[01:25:12]    |
[01:25:12]    |
[01:25:12] 27 | / impl PartialEq<Baz> for Baz {
[01:25:12] 28 | |     fn eq(&self, _: &Baz) -> bool { true }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are implementing `Hash` explicitly but have derived `PartialEq`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 34 | / impl Hash for Bah {
[01:25:12] 35 | |     fn hash<H: Hasher>(&self, _: &mut H) {}
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12] note: `PartialEq` implemented here
[01:25:12]   --> $DIR/derive.rs:31:10
[01:25:12]   --> $DIR/derive.rs:31:10
[01:25:12]    |
[01:25:12] 31 | #[derive(PartialEq)]
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]    |
[01:25:12]    |
[01:25:12] 41 | / impl Clone for Qux {
[01:25:12] 42 | |     fn clone(&self) -> Self { Qux }
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D expl-impl-clone-on-copy` implied by `-D warnings`
[01:25:12] note: consider deriving `Clone` or removing `Copy`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 41 | / impl Clone for Qux {
[01:25:12] 42 | |     fn clone(&self) -> Self { Qux }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]    |
[01:25:12]    |
[01:25:12] 65 | / impl<'a> Clone for Lt<'a> {
[01:25:12] 66 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: consider deriving `Clone` or removing `Copy`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 65 | / impl<'a> Clone for Lt<'a> {
[01:25:12] 66 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]    |
[01:25:12]    |
[01:25:12] 75 | / impl Clone for BigArray {
[01:25:12] 76 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: consider deriving `Clone` or removing `Copy`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 75 | / impl Clone for BigArray {
[01:25:12] 76 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]    |
[01:25:12]    |
[01:25:12] 85 | / impl Clone for FnPtr {
[01:25:12] 86 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: consider deriving `Clone` or removing `Copy`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 85 | / impl Clone for FnPtr {
[01:25:12] 86 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] error: aborting due to 7 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] diff of stderr:
[01:25:12] 
[01:25:12] -error: you are deriving `Hash` but have implemented `PartialEq` explicitly
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -17 | #[derive(Hash)]
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -   = note: #[deny(derive_hash_xor_eq)] on by default
[01:25:12] -note: `PartialEq` implemented here
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -20 | / impl PartialEq for Bar {
[01:25:12] -21 | |     fn eq(&self, _: &Bar) -> bool { true }
[01:25:12] -22 | | }
[01:25:12] -
[01:25:12] -
[01:25:12] -error: you are deriving `Hash` but have implemented `PartialEq` explicitly
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -24 | #[derive(Hash)]
[01:25:12] -   |
[01:25:12] -note: `PartialEq` implemented here
[01:25:12] -  --> $DIR/derive.rs:27:1
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -27 | / impl PartialEq<Baz> for Baz {
[01:25:12] -28 | |     fn eq(&self, _: &Baz) -> bool { true }
[01:25:12] -29 | | }
[01:25:12] -
[01:25:12] -
[01:25:12]  error: you are implementing `Hash` explicitly but have derived `PartialEq`
[01:25:12]     |
[01:25:12]     |
[01:25:12]  34 | / impl Hash for Bah {
[01:25:12]  35 | |     fn hash<H: Hasher>(&self, _: &mut H) {}
[01:25:12]     | |_^
[01:25:12]     |
[01:25:12]  note: `PartialEq` implemented here
[01:25:12]    --> $DIR/derive.rs:31:10
[01:25:12]    --> $DIR/derive.rs:31:10
[01:25:12]     |
[01:25:12]  31 | #[derive(PartialEq)]
[01:25:12]  
[01:25:12]  
[01:25:12]  error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]     |
[01:25:12]     |
[01:25:12]  41 | / impl Clone for Qux {
[01:25:12]  42 | |     fn clone(&self) -> Self { Qux }
[01:25:12]     | |_^
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: `-D expl-impl-clone-on-copy` implied by `-D warnings`
[01:25:12]  note: consider deriving `Clone` or removing `Copy`
[01:25:12]     |
[01:25:12]     |
[01:25:12]  41 | / impl Clone for Qux {
[01:25:12]  42 | |     fn clone(&self) -> Self { Qux }
[01:25:12]     | |_^
[01:25:12]  
[01:25:12]  
[01:25:12]  error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]     |
[01:25:12]     |
[01:25:12]  65 | / impl<'a> Clone for Lt<'a> {
[01:25:12]  66 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]     | |_^
[01:25:12]     |
[01:25:12]     |
[01:25:12]  note: consider deriving `Clone` or removing `Copy`
[01:25:12]     |
[01:25:12]     |
[01:25:12]  65 | / impl<'a> Clone for Lt<'a> {
[01:25:12]  66 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]     | |_^
[01:25:12]  
[01:25:12]  
[01:25:12]  error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]     |
[01:25:12]     |
[01:25:12]  75 | / impl Clone for BigArray {
[01:25:12]  76 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]     | |_^
[01:25:12]     |
[01:25:12]     |
[01:25:12]  note: consider deriving `Clone` or removing `Copy`
[01:25:12]     |
[01:25:12]     |
[01:25:12]  75 | / impl Clone for BigArray {
[01:25:12]  76 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]     | |_^
[01:25:12]  
[01:25:12]  
[01:25:12]  error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]     |
[01:25:12]     |
[01:25:12]  85 | / impl Clone for FnPtr {
[01:25:12]  86 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]     | |_^
[01:25:12]     |
[01:25:12]     |
[01:25:12]  note: consider deriving `Clone` or removing `Copy`
[01:25:12]     |
[01:25:12]     |
[01:25:12]  85 | / impl Clone for FnPtr {
[01:25:12]  86 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]     | |_^
[01:25:12]  
[01:25:12] -error: aborting due to 7 previous errors
[01:25:12] +error: aborting due to 5 previous errors
---
[01:25:12] 
[01:25:12] ------------------------------------------
[01:25:12] stderr:
[01:25:12] ------------------------------------------
[01:25:12] error: you are implementing `Hash` explicitly but have derived `PartialEq`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 34 | / impl Hash for Bah {
[01:25:12] 35 | |     fn hash<H: Hasher>(&self, _: &mut H) {}
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12] note: `PartialEq` implemented here
[01:25:12]   --> tests/ui/derive.rs:31:10
[01:25:12]   --> tests/ui/derive.rs:31:10
[01:25:12]    |
[01:25:12] 31 | #[derive(PartialEq)]
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]    |
[01:25:12]    |
[01:25:12] 41 | / impl Clone for Qux {
[01:25:12] 42 | |     fn clone(&self) -> Self { Qux }
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D expl-impl-clone-on-copy` implied by `-D warnings`
[01:25:12] note: consider deriving `Clone` or removing `Copy`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 41 | / impl Clone for Qux {
[01:25:12] 42 | |     fn clone(&self) -> Self { Qux }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]    |
[01:25:12]    |
[01:25:12] 65 | / impl<'a> Clone for Lt<'a> {
[01:25:12] 66 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: consider deriving `Clone` or removing `Copy`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 65 | / impl<'a> Clone for Lt<'a> {
[01:25:12] 66 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]    |
[01:25:12]    |
[01:25:12] 75 | / impl Clone for BigArray {
[01:25:12] 76 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: consider deriving `Clone` or removing `Copy`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 75 | / impl Clone for BigArray {
[01:25:12] 76 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are implementing `Clone` explicitly on a `Copy` type
[01:25:12]    |
[01:25:12]    |
[01:25:12] 85 | / impl Clone for FnPtr {
[01:25:12] 86 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: consider deriving `Clone` or removing `Copy`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 85 | / impl Clone for FnPtr {
[01:25:12] 86 | |     fn clone(&self) -> Self { unimplemented!() }
[01:25:12]    | |_^
[01:25:12] 
[01:25:12] error: aborting due to 5 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] ------------------------------------------
[01:25:12] 
[01:25:12] thread '[ui] ui/derive.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.11/src/runtest.rs:2544:9
[01:25:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:25:12] 
[01:25:12] ---- [ui] ui/for_loop.rs stdout ----
[01:25:12] normalized stderr:
[01:25:12] error: for loop over `option`, which is an `Option`. This is more readably written as an `if let` statement.
[01:25:12]    |
[01:25:12] 17 |     for x in option {
[01:25:12]    |              ^^^^^^
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D for-loop-over-option` implied by `-D warnings`
[01:25:12]    = help: consider replacing `for x in option` with `if let Some(x) = option`
[01:25:12] 
[01:25:12] error: for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement.
[01:25:12]    |
[01:25:12] 22 |     for x in result {
[01:25:12]    |              ^^^^^^
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D for-loop-over-result` implied by `-D warnings`
[01:25:12]    = help: consider replacing `for x in result` with `if let Ok(x) = result`
[01:25:12] 
[01:25:12] error: for loop over `option.ok_or("x not found")`, which is a `Result`. This is more readably written as an `if let` statement.
[01:25:12]    |
[01:25:12]    |
[01:25:12] 26 |     for x in option.ok_or("x not found") {
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = help: consider replacing `for x in option.ok_or("x not found")` with `if let Ok(x) = option.ok_or("x not found")`
[01:25:12] 
[01:25:12] error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want
[01:25:12]    |
[01:25:12]    |
[01:25:12] 32 |     for x in v.iter().next() {
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D iter-next-loop` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: for loop over `v.iter().next().and(Some(0))`, which is an `Option`. This is more readably written as an `if let` statement.
[01:25:12]    |
[01:25:12]    |
[01:25:12] 37 |     for x in v.iter().next().and(Some(0)) {
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = help: consider replacing `for x in v.iter().next().and(Some(0))` with `if let Some(x) = v.iter().next().and(Some(0))`
[01:25:12] 
[01:25:12] error: for loop over `v.iter().next().ok_or("x not found")`, which is a `Result`. This is more readably written as an `if let` statement.
[01:25:12]    |
[01:25:12]    |
[01:25:12] 41 |     for x in v.iter().next().ok_or("x not found") {
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = help: consider replacing `for x in v.iter().next().ok_or("x not found")` with `if let Ok(x) = v.iter().next().ok_or("x not found")`
[01:25:12] 
[01:25:12] error: this loop never actually loops
[01:25:12]    |
[01:25:12]    |
[01:25:12] 53 | /     while let Some(x) = option {
[01:25:12] 55 | |         break;
[01:25:12] 56 | |     }
[01:25:12]    | |_____^
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D never-loop` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: this loop never actually loops
[01:25:12]    |
[01:25:12]    |
[01:25:12] 59 | /     while let Ok(x) = result {
[01:25:12] 61 | |         break;
[01:25:12] 62 | |     }
[01:25:12]    | |_____^
[01:25:12] 
[01:25:12] 
[01:25:12] error: the loop variable `i` is only used to index `vec`.
[01:25:12]     |
[01:25:12]     |
[01:25:12] 128 |     for i in 0..=MAX_LEN {
[01:25:12] help: consider using an iterator
[01:25:12]     |
[01:25:12]     |
[01:25:12] 128 |     for <item> in vec.iter().take(MAX_LEN + 1) {
[01:25:12] 
[01:25:12] 
[01:25:12] error: the loop variable `i` is only used to index `vec`.
[01:25:12]     |
[01:25:12]     |
[01:25:12] 136 |     for i in 5..=10 {
[01:25:12] help: consider using an iterator
[01:25:12]     |
[01:25:12]     |
[01:25:12] 136 |     for <item> in vec.iter().take(10 + 1).skip(5) {
[01:25:12] 
[01:25:12] 
[01:25:12] error: this range is empty so this for loop will never run
[01:25:12]     |
[01:25:12]     |
[01:25:12] 152 |     for i in 10..=0 {
[01:25:12] help: consider using the following if you are attempting to iterate over this range in reverse
[01:25:12]     |
[01:25:12]     |
[01:25:12] 152 |     for i in (0...10).rev() {
[01:25:12] 
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]     |
[01:25:12]     |
[01:25:12] 215 |     for _v in vec.iter() {}
[01:25:12]     |               ^^^^^^^^^^ help: to write this more concisely, try: `&vec`
[01:25:12]     |
[01:25:12]     = note: `-D explicit-iter-loop` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]     |
[01:25:12]     |
[01:25:12] 217 |     for _v in vec.iter_mut() {}
[01:25:12]     |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&mut vec`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over containers instead of using explicit iteration methods`
[01:25:12]     |
[01:25:12]     |
[01:25:12] 220 |     for _v in out_vec.into_iter() {}
[01:25:12]     |               ^^^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `out_vec`
[01:25:12]     |
[01:25:12]     = note: `-D explicit-into-iter-loop` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]     |
[01:25:12]     |
[01:25:12] 223 |     for _v in array.into_iter() {}
[01:25:12]     |               ^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&array`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]     |
[01:25:12]     |
[01:25:12] 228 |     for _v in [1, 2, 3].iter() {}
[01:25:12]     |               ^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[1, 2, 3]`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]     |
[01:25:12]     |
[01:25:12] 232 |     for _v in [0; 32].iter() {}
[01:25:12]     |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[0; 32]`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]     |
[01:25:12]     |
[01:25:12] 237 |     for _v in ll.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&ll`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]     |
[01:25:12]     |
[01:25:12] 240 |     for _v in vd.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&vd`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]     |
[01:25:12]     |
[01:25:12] 243 |     for _v in bh.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&bh`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]     |
[01:25:12]     |
[01:25:12] 246 |     for _v in hm.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&hm`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]     |
[01:25:12]     |
[01:25:12] 249 |     for _v in bt.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&bt`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]     |
[01:25:12]     |
[01:25:12] 252 |     for _v in hs.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&hs`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]     |
[01:25:12]     |
[01:25:12] 255 |     for _v in bs.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&bs`
[01:25:12] 
[01:25:12] error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want
[01:25:12]     |
[01:25:12]     |
[01:25:12] 257 |     for _v in vec.iter().next() {}
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are collect()ing an iterator and throwing away the result. Consider using an explicit for loop to exhaust the iterator
[01:25:12]     |
[01:25:12]     |
[01:25:12] 264 |     vec.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>();
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: `-D unused-collect` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: the variable `_index` is used as a loop counter. Consider using `for (_index, item) in &vec.enumerate()` or similar iterators
[01:25:12]     |
[01:25:12]     |
[01:25:12] 269 |     for _v in &vec {
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: `-D explicit-counter-loop` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: the variable `_index` is used as a loop counter. Consider using `for (_index, item) in &vec.enumerate()` or similar iterators
[01:25:12]     |
[01:25:12]     |
[01:25:12] 275 |     for _v in &vec {
[01:25:12] 
[01:25:12] 
[01:25:12] error: you seem to want to iterate on a map's values
[01:25:12]     |
[01:25:12]     |
[01:25:12] 385 |     for (_, v) in &m {
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: `-D for-kv-map` implied by `-D warnings`
[01:25:12] help: use the corresponding method
[01:25:12]     |
[01:25:12] 385 |     for v in m.values() {
[01:25:12] 
[01:25:12] 
[01:25:12] error: you seem to want to iterate on a map's values
[01:25:12]     |
[01:25:12]     |
[01:25:12] 390 |     for (_, v) in &*m {
[01:25:12] help: use the corresponding method
[01:25:12]     |
[01:25:12]     |
[01:25:12] 390 |     for v in (*m).values() {
[01:25:12] 
[01:25:12] 
[01:25:12] error: you seem to want to iterate on a map's values
[01:25:12]     |
[01:25:12]     |
[01:25:12] 398 |     for (_, v) in &mut m {
[01:25:12] help: use the corresponding method
[01:25:12]     |
[01:25:12]     |
[01:25:12] 398 |     for v in m.values_mut() {
[01:25:12] 
[01:25:12] 
[01:25:12] error: you seem to want to iterate on a map's values
[01:25:12]     |
[01:25:12]     |
[01:25:12] 403 |     for (_, v) in &mut *m {
[01:25:12] help: use the corresponding method
[01:25:12]     |
[01:25:12]     |
[01:25:12] 403 |     for v in (*m).values_mut() {
[01:25:12] 
[01:25:12] 
[01:25:12] error: you seem to want to iterate on a map's keys
[01:25:12]     |
[01:25:12]     |
[01:25:12] 409 |     for (k, _value) in rm {
[01:25:12] help: use the corresponding method
[01:25:12]     |
[01:25:12]     |
[01:25:12] 409 |     for k in rm.keys() {
[01:25:12] 
[01:25:12] error: aborting due to 33 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] expected stderr:
[01:25:12] error: for loop over `option`, which is an `Option`. This is more readably written as an `if let` statement.
[01:25:12]    |
[01:25:12] 17 |     for x in option {
[01:25:12]    |              ^^^^^^
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D for-loop-over-option` implied by `-D warnings`
[01:25:12]    = help: consider replacing `for x in option` with `if let Some(x) = option`
[01:25:12] 
[01:25:12] error: for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement.
[01:25:12]    |
[01:25:12] 22 |     for x in result {
[01:25:12]    |              ^^^^^^
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D for-loop-over-result` implied by `-D warnings`
[01:25:12]    = help: consider replacing `for x in result` with `if let Ok(x) = result`
[01:25:12] 
[01:25:12] error: for loop over `option.ok_or("x not found")`, which is a `Result`. This is more readably written as an `if let` statement.
[01:25:12]    |
[01:25:12]    |
[01:25:12] 26 |     for x in option.ok_or("x not found") {
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = help: consider replacing `for x in option.ok_or("x not found")` with `if let Ok(x) = option.ok_or("x not found")`
[01:25:12] 
[01:25:12] error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want
[01:25:12]    |
[01:25:12]    |
[01:25:12] 32 |     for x in v.iter().next() {
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D iter-next-loop` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: for loop over `v.iter().next().and(Some(0))`, which is an `Option`. This is more readably written as an `if let` statement.
[01:25:12]    |
[01:25:12]    |
[01:25:12] 37 |     for x in v.iter().next().and(Some(0)) {
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = help: consider replacing `for x in v.iter().next().and(Some(0))` with `if let Some(x) = v.iter().next().and(Some(0))`
[01:25:12] 
[01:25:12] error: for loop over `v.iter().next().ok_or("x not found")`, which is a `Result`. This is more readably written as an `if let` statement.
[01:25:12]    |
[01:25:12]    |
[01:25:12] 41 |     for x in v.iter().next().ok_or("x not found") {
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = help: consider replacing `for x in v.iter().next().ok_or("x not found")` with `if let Ok(x) = v.iter().next().ok_or("x not found")`
[01:25:12] 
[01:25:12] error: this loop never actually loops
[01:25:12]    |
[01:25:12]    |
[01:25:12] 53 | /     while let Some(x) = option {
[01:25:12] 55 | |         break;
[01:25:12] 56 | |     }
[01:25:12]    | |_____^
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D never-loop` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: this loop never actually loops
[01:25:12]    |
[01:25:12]    |
[01:25:12] 59 | /     while let Ok(x) = result {
[01:25:12] 61 | |         break;
[01:25:12] 62 | |     }
[01:25:12]    | |_____^
[01:25:12] 
[01:25:12] 
[01:25:12] error: the loop variable `i` is only used to index `vec`.
[01:25:12]    |
[01:25:12]    |
[01:25:12] 86 |     for i in 0..vec.len() {
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D needless-range-loop` implied by `-D warnings`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 86 |     for <item> in &vec {
[01:25:12] 
[01:25:12] 
[01:25:12] error: the loop variable `i` is only used to index `vec`.
[01:25:12]    |
[01:25:12]    |
[01:25:12] 95 |     for i in 0..vec.len() {
[01:25:12] help: consider using an iterator
[01:25:12]    |
[01:25:12]    |
[01:25:12] 95 |     for <item> in &vec {
[01:25:12] 
[01:25:12] 
[01:25:12] error: the loop variable `j` is only used to index `STATIC`.
[01:25:12]     |
[01:25:12]     |
[01:25:12] 100 |     for j in 0..4 {
[01:25:12] help: consider using an iterator
[01:25:12]     |
[01:25:12]     |
[01:25:12] 100 |     for <item> in STATIC.iter().take(4) {
[01:25:12] 
[01:25:12] 
[01:25:12] error: the loop variable `j` is only used to index `CONST`.
[01:25:12]     |
[01:25:12]     |
[01:25:12] 104 |     for j in 0..4 {
[01:25:12] help: consider using an iterator
[01:25:12]     |
[01:25:12]     |
[01:25:12] 104 |     for <item> in CONST.iter().take(4) {
[01:25:12] 
[01:25:12] 
[01:25:12] error: the loop variable `i` is used to index `vec`
[01:25:12]     |
[01:25:12]     |
[01:25:12] 108 |     for i in 0..vec.len() {
[01:25:12] help: consider using an iterator
[01:25:12]     |
[01:25:12]     |
[01:25:12] 108 |     for (i, <item>) in vec.iter().enumerate() {
[01:25:12] 
[01:25:12] 
[01:25:12] error: the loop variable `i` is only used to index `vec2`.
[01:25:12]     |
[01:25:12]     |
[01:25:12] 116 |     for i in 0..vec.len() {
[01:25:12] help: consider using an iterator
[01:25:12]     |
[01:25:12]     |
[01:25:12] 116 |     for <item> in vec2.iter().take(vec.len()) {
[01:25:12] 
[01:25:12] 
[01:25:12] error: the loop variable `i` is only used to index `vec`.
[01:25:12]     |
[01:25:12]     |
[01:25:12] 120 |     for i in 5..vec.len() {
[01:25:12] help: consider using an iterator
[01:25:12]     |
[01:25:12]     |
[01:25:12] 120 |     for <item> in vec.iter().skip(5) {
---
[01:25:12] 
[01:25:12] ------------------------------------------
[01:25:12] stderr:
[01:25:12] ------------------------------------------
[01:25:12] error: for loop over `option`, which is an `Option`. This is more readably written as an `if let` statement.
[01:25:12]   --> tests/ui/for_loop.rs:17:14
[01:25:12] 17 |     for x in option {
[01:25:12]    |              ^^^^^^
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D for-loop-over-option` implied by `-D warnings`
[01:25:12]    = help: consider replacing `for x in option` with `if let Some(x) = option`
[01:25:12] 
[01:25:12] error: for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement.
[01:25:12]   --> tests/ui/for_loop.rs:22:14
[01:25:12] 22 |     for x in result {
[01:25:12]    |              ^^^^^^
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D for-loop-over-result` implied by `-D warnings`
[01:25:12]    = help: consider replacing `for x in result` with `if let Ok(x) = result`
[01:25:12] 
[01:25:12] error: for loop over `option.ok_or("x not found")`, which is a `Result`. This is more readably written as an `if let` statement.
[01:25:12]   --> tests/ui/for_loop.rs:26:14
[01:25:12]    |
[01:25:12] 26 |     for x in option.ok_or("x not found") {
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = help: consider replacing `for x in option.ok_or("x not found")` with `if let Ok(x) = option.ok_or("x not found")`
[01:25:12] 
[01:25:12] error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want
[01:25:12]   --> tests/ui/for_loop.rs:32:14
[01:25:12]    |
[01:25:12] 32 |     for x in v.iter().next() {
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D iter-next-loop` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: for loop over `v.iter().next().and(Some(0))`, which is an `Option`. This is more readably written as an `if let` statement.
[01:25:12]   --> tests/ui/for_loop.rs:37:14
[01:25:12]    |
[01:25:12] 37 |     for x in v.iter().next().and(Some(0)) {
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = help: consider replacing `for x in v.iter().next().and(Some(0))` with `if let Some(x) = v.iter().next().and(Some(0))`
[01:25:12] 
[01:25:12] error: for loop over `v.iter().next().ok_or("x not found")`, which is a `Result`. This is more readably written as an `if let` statement.
[01:25:12]   --> tests/ui/for_loop.rs:41:14
[01:25:12]    |
[01:25:12] 41 |     for x in v.iter().next().ok_or("x not found") {
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = help: consider replacing `for x in v.iter().next().ok_or("x not found")` with `if let Ok(x) = v.iter().next().ok_or("x not found")`
[01:25:12] 
[01:25:12] error: this loop never actually loops
[01:25:12]   --> tests/ui/for_loop.rs:53:5
[01:25:12]    |
[01:25:12] 53 | /     while let Some(x) = option {
[01:25:12] 55 | |         break;
[01:25:12] 56 | |     }
[01:25:12]    | |_____^
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D never-loop` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: this loop never actually loops
[01:25:12]   --> tests/ui/for_loop.rs:59:5
[01:25:12]    |
[01:25:12] 59 | /     while let Ok(x) = result {
[01:25:12] 61 | |         break;
[01:25:12] 62 | |     }
[01:25:12]    | |_____^
[01:25:12] 
[01:25:12] 
[01:25:12] error: the loop variable `i` is only used to index `vec`.
[01:25:12]    --> tests/ui/for_loop.rs:128:14
[01:25:12]     |
[01:25:12] 128 |     for i in 0..=MAX_LEN {
[01:25:12] help: consider using an iterator
[01:25:12]     |
[01:25:12]     |
[01:25:12] 128 |     for <item> in vec.iter().take(MAX_LEN + 1) {
[01:25:12] 
[01:25:12] 
[01:25:12] error: the loop variable `i` is only used to index `vec`.
[01:25:12]    --> tests/ui/for_loop.rs:136:14
[01:25:12]     |
[01:25:12] 136 |     for i in 5..=10 {
[01:25:12] help: consider using an iterator
[01:25:12]     |
[01:25:12]     |
[01:25:12] 136 |     for <item> in vec.iter().take(10 + 1).skip(5) {
[01:25:12] 
[01:25:12] 
[01:25:12] error: this range is empty so this for loop will never run
[01:25:12]    --> tests/ui/for_loop.rs:152:14
[01:25:12]     |
[01:25:12] 152 |     for i in 10..=0 {
[01:25:12] help: consider using the following if you are attempting to iterate over this range in reverse
[01:25:12]     |
[01:25:12]     |
[01:25:12] 152 |     for i in (0...10).rev() {
[01:25:12] 
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]    --> tests/ui/for_loop.rs:215:15
[01:25:12]     |
[01:25:12] 215 |     for _v in vec.iter() {}
[01:25:12]     |               ^^^^^^^^^^ help: to write this more concisely, try: `&vec`
[01:25:12]     |
[01:25:12]     = note: `-D explicit-iter-loop` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]    --> tests/ui/for_loop.rs:217:15
[01:25:12]     |
[01:25:12] 217 |     for _v in vec.iter_mut() {}
[01:25:12]     |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&mut vec`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over containers instead of using explicit iteration methods`
[01:25:12]    --> tests/ui/for_loop.rs:220:15
[01:25:12]     |
[01:25:12] 220 |     for _v in out_vec.into_iter() {}
[01:25:12]     |               ^^^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `out_vec`
[01:25:12]     |
[01:25:12]     = note: `-D explicit-into-iter-loop` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]    --> tests/ui/for_loop.rs:223:15
[01:25:12]     |
[01:25:12] 223 |     for _v in array.into_iter() {}
[01:25:12]     |               ^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&array`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]    --> tests/ui/for_loop.rs:228:15
[01:25:12]     |
[01:25:12] 228 |     for _v in [1, 2, 3].iter() {}
[01:25:12]     |               ^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[1, 2, 3]`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]    --> tests/ui/for_loop.rs:232:15
[01:25:12]     |
[01:25:12] 232 |     for _v in [0; 32].iter() {}
[01:25:12]     |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[0; 32]`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]    --> tests/ui/for_loop.rs:237:15
[01:25:12]     |
[01:25:12] 237 |     for _v in ll.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&ll`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]    --> tests/ui/for_loop.rs:240:15
[01:25:12]     |
[01:25:12] 240 |     for _v in vd.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&vd`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]    --> tests/ui/for_loop.rs:243:15
[01:25:12]     |
[01:25:12] 243 |     for _v in bh.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&bh`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]    --> tests/ui/for_loop.rs:246:15
[01:25:12]     |
[01:25:12] 246 |     for _v in hm.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&hm`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]    --> tests/ui/for_loop.rs:249:15
[01:25:12]     |
[01:25:12] 249 |     for _v in bt.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&bt`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]    --> tests/ui/for_loop.rs:252:15
[01:25:12]     |
[01:25:12] 252 |     for _v in hs.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&hs`
[01:25:12] 
[01:25:12] error: it is more idiomatic to loop over references to containers instead of using explicit iteration methods
[01:25:12]    --> tests/ui/for_loop.rs:255:15
[01:25:12]     |
[01:25:12] 255 |     for _v in bs.iter() {}
[01:25:12]     |               ^^^^^^^^^ help: to write this more concisely, try: `&bs`
[01:25:12] 
[01:25:12] error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want
[01:25:12]    --> tests/ui/for_loop.rs:257:15
[01:25:12]     |
[01:25:12] 257 |     for _v in vec.iter().next() {}
[01:25:12] 
[01:25:12] 
[01:25:12] error: you are collect()ing an iterator and throwing away the result. Consider using an explicit for loop to exhaust the iterator
[01:25:12]    --> tests/ui/for_loop.rs:264:5
[01:25:12]     |
[01:25:12] 264 |     vec.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>();
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: `-D unused-collect` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: the variable `_index` is used as a loop counter. Consider using `for (_index, item) in &vec.enumerate()` or similar iterators
[01:25:12]    --> tests/ui/for_loop.rs:269:15
[01:25:12]     |
[01:25:12] 269 |     for _v in &vec {
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: `-D explicit-counter-loop` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: the variable `_index` is used as a loop counter. Consider using `for (_index, item) in &vec.enumerate()` or similar iterators
[01:25:12]    --> tests/ui/for_loop.rs:275:15
[01:25:12]     |
[01:25:12] 275 |     for _v in &vec {
[01:25:12] 
[01:25:12] 
[01:25:12] error: you seem to want to iterate on a map's values
[01:25:12]    --> tests/ui/for_loop.rs:385:19
[01:25:12]     |
[01:25:12] 385 |     for (_, v) in &m {
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: `-D for-kv-map` implied by `-D warnings`
[01:25:12] help: use the corresponding method
[01:25:12]     |
[01:25:12] 385 |     for v in m.values() {
[01:25:12] 
[01:25:12] 
[01:25:12] error: you seem to want to iterate on a map's values
[01:25:12]    --> tests/ui/for_loop.rs:390:19
[01:25:12]     |
[01:25:12] 390 |     for (_, v) in &*m {
[01:25:12] help: use the corresponding method
[01:25:12]     |
[01:25:12]     |
[01:25:12] 390 |     for v in (*m).values() {
[01:25:12] 
[01:25:12] 
[01:25:12] error: you seem to want to iterate on a map's values
[01:25:12]    --> tests/ui/for_loop.rs:398:19
[01:25:12]     |
[01:25:12] 398 |     for (_, v) in &mut m {
[01:25:12] help: use the corresponding method
[01:25:12]     |
[01:25:12]     |
[01:25:12] 398 |     for v in m.values_mut() {
[01:25:12] 
[01:25:12] 
[01:25:12] error: you seem to want to iterate on a map's values
[01:25:12]    --> tests/ui/for_loop.rs:403:19
[01:25:12]     |
[01:25:12] 403 |     for (_, v) in &mut *m {
[01:25:12] help: use the corresponding method
[01:25:12]     |
[01:25:12]     |
[01:25:12] 403 |     for v in (*m).values_mut() {
[01:25:12] 
[01:25:12] 
[01:25:12] error: you seem to want to iterate on a map's keys
[01:25:12]    --> tests/ui/for_loop.rs:409:24
[01:25:12]     |
[01:25:12] 409 |     for (k, _value) in rm {
[01:25:12] help: use the corresponding method
[01:25:12]     |
[01:25:12]     |
[01:25:12] 409 |     for k in rm.keys() {
[01:25:12] 
[01:25:12] error: aborting due to 33 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] ------------------------------------------
[01:25:12] 
[01:25:12] thread '[ui] ui/for_loop.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.11/src/runtest.rs:2544:9
[01:25:12] 
[01:25:12] ---- [ui] ui/matches.rs stdout ----
[01:25:12] normalized stderr:
[01:25:12] error: you seem to be trying to use match for destructuring a single pattern. Consider using `if let`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 21 | /     match ExprNode::Butterflies {
[01:25:12] 22 | |         ExprNode::ExprAddrOf => Some(&NODE),
[01:25:12] 23 | |         _ => { let x = 5; None },
[01:25:12] 24 | |     }
[01:25:12]    | |_____^ help: try this: `if let ExprNode::ExprAddrOf = ExprNode::Butterflies { Some(&NODE) } else { let x = 5; None }`
[01:25:12]    = note: `-D single-match-else` implied by `-D warnings`
[01:25:12] 
[01:25:12] 
[01:25:12] error: you don't need to add `&` to all patterns
[01:25:12]    |
[01:25:12] 30 | /         match v {
[01:25:12] 30 | /         match v {
[01:25:12] 31 | |             &Some(v) => println!("{:?}", v),
[01:25:12] 32 | |             &None => println!("none"),
[01:25:12]    | |_________^
[01:25:12]    |
[01:25:12]    = note: `-D match-ref-pats` implied by `-D warnings`
[01:25:12]    = note: `-D match-ref-pats` implied by `-D warnings`
[01:25:12] help: instead of prefixing all patterns with `&`, you can dereference the expression
[01:25:12] 30 |         match *v {
[01:25:12] 30 |         match *v {
[01:25:12] 31 |             Some(v) => println!("{:?}", v),
[01:25:12] 32 |             None => println!("none"),
[01:25:12] 
[01:25:12] 
[01:25:12] error: you don't need to add `&` to all patterns
[01:25:12]    |
[01:25:12] 40 | /     match tup {
[01:25:12] 40 | /     match tup {
[01:25:12] 41 | |         &(v, 1) => println!("{}", v),
[01:25:12] 42 | |         _ => println!("none"),
[01:25:12]    | |_____^
[01:25:12]    | |_____^
[01:25:12] help: instead of prefixing all patterns with `&`, you can dereference the expression
[01:25:12]    |
[01:25:12] 40 |     match *tup {
[01:25:12] 41 |         (v, 1) => println!("{}", v),
[01:25:12] 
[01:25:12] 
[01:25:12] error: you don't need to add `&` to both the expression and the patterns
[01:25:12]    |
[01:25:12] 46 | /     match &w {
[01:25:12] 46 | /     match &w {
[01:25:12] 47 | |         &Some(v) => println!("{:?}", v),
[01:25:12] 48 | |         &None => println!("none"),
[01:25:12]    | |_____^
[01:25:12] help: try
[01:25:12]    |
[01:25:12] 46 |     match w {
[01:25:12] 46 |     match w {
[01:25:12] 47 |         Some(v) => println!("{:?}", v),
[01:25:12] 48 |         None => println!("none"),
[01:25:12] 
[01:25:12] 
[01:25:12] error: you don't need to add `&` to all patterns
[01:25:12]    |
[01:25:12]    |
[01:25:12] 57 | /     if let &None = a {
[01:25:12] 58 | |         println!("none");
[01:25:12]    | |_____^
[01:25:12]    | |_____^
[01:25:12] help: instead of prefixing all patterns with `&`, you can dereference the expression
[01:25:12] 57 |     if let None = *a {
[01:25:12]    |            ^^^^   ^^
[01:25:12] 
[01:25:12] 
[01:25:12] error: you don't need to add `&` to both the expression and the patterns
[01:25:12]    |
[01:25:12]    |
[01:25:12] 62 | /     if let &None = &b {
[01:25:12] 63 | |         println!("none");
[01:25:12]    | |_____^
[01:25:12] help: try
[01:25:12]    |
[01:25:12] 62 |     if let None = b {
[01:25:12] 62 |     if let None = b {
[01:25:12]    |            ^^^^   ^
[01:25:12] 
[01:25:12] error: some ranges overlap
[01:25:12]    |
[01:25:12]    |
[01:25:12] 71 |         0 ... 10 => println!("0 ... 10"),
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D match-overlapping-arm` implied by `-D warnings`
[01:25:12] note: overlaps with this
[01:25:12]    |
[01:25:12]    |
[01:25:12] 72 |         0 ... 11 => println!("0 ... 11"),
[01:25:12] 
[01:25:12] 
[01:25:12] error: some ranges overlap
[01:25:12]    |
[01:25:12]    |
[01:25:12] 77 |         0 ... 5 => println!("0 ... 5"),
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: overlaps with this
[01:25:12]    |
[01:25:12]    |
[01:25:12] 79 |         FOO ... 11 => println!("0 ... 11"),
[01:25:12] 
[01:25:12] 
[01:25:12] error: some ranges overlap
[01:25:12]    |
[01:25:12]    |
[01:25:12] 85 |         0 ... 5 => println!("0 ... 5"),
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: overlaps with this
[01:25:12]    |
[01:25:12] 84 |         2 => println!("2"),
[01:25:12]    |         ^
[01:25:12] 
[01:25:12] 
[01:25:12] error: some ranges overlap
[01:25:12]    |
[01:25:12]    |
[01:25:12] 91 |         0 ... 2 => println!("0 ... 2"),
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: overlaps with this
[01:25:12]    |
[01:25:12] 90 |         2 => println!("2"),
[01:25:12]    |         ^
[01:25:12] 
[01:25:12] 
[01:25:12] error: some ranges overlap
[01:25:12]     |
[01:25:12]     |
[01:25:12] 114 |         0 .. 11 => println!("0 .. 11"),
[01:25:12]     |
[01:25:12]     |
[01:25:12] note: overlaps with this
[01:25:12]     |
[01:25:12]     |
[01:25:12] 115 |         0 ... 11 => println!("0 ... 11"),
[01:25:12] 
[01:25:12] 
[01:25:12] error: Err(_) will match all errors, maybe not a good idea
[01:25:12]     |
[01:25:12]     |
[01:25:12] 132 |         Err(_) => panic!("err")
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: `-D match-wild-err-arm` implied by `-D warnings`
[01:25:12]     = note: to remove this warning, match each error seperately or use unreachable macro
[01:25:12] 
[01:25:12] error: Err(_) will match all errors, maybe not a good idea
[01:25:12]     |
[01:25:12]     |
[01:25:12] 138 |         Err(_) => {panic!()}
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: to remove this warning, match each error seperately or use unreachable macro
[01:25:12] 
[01:25:12] error: Err(_) will match all errors, maybe not a good idea
[01:25:12]     |
[01:25:12]     |
[01:25:12] 144 |         Err(_) => {panic!();}
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: to remove this warning, match each error seperately or use unreachable macro
[01:25:12] error: use as_ref() instead
[01:25:12]    --> $DIR/matches.rs:212:33
[01:25:12]     |
[01:25:12]     |
[01:25:12] 212 |       let borrowed: Option<&()> = match owned {
[01:25:12] 213 | |         None => None,
[01:25:12] 213 | |         None => None,
[01:25:12] 214 | |         Some(ref v) => Some(v),
[01:25:12] 215 | |     };
[01:25:12]     | |_____^ help: try this: `owned.as_ref()`
[01:25:12]     = note: `-D match-as-ref` implied by `-D warnings`
[01:25:12] 
[01:25:12] 
[01:25:12] error: use as_mut() instead
[01:25:12]     |
[01:25:12]     |
[01:25:12] 218 |       let borrow_mut: Option<&mut ()> = match mut_owned {
[01:25:12] 219 | |         None => None,
[01:25:12] 219 | |         None => None,
[01:25:12] 220 | |         Some(ref mut v) => Some(v),
[01:25:12] 221 | |     };
[01:25:12]     | |_____^ help: try this: `mut_owned.as_mut()`
[01:25:12] error: aborting due to 16 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] expected stderr:
[01:25:12] error: you seem to be trying to use match for destructuring a single pattern. Consider using `if let`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 21 | /     match ExprNode::Butterflies {
[01:25:12] 22 | |         ExprNode::ExprAddrOf => Some(&NODE),
[01:25:12] 23 | |         _ => { let x = 5; None },
[01:25:12] 24 | |     }
[01:25:12]    | |_____^ help: try this: `if let ExprNode::ExprAddrOf = ExprNode::Butterflies { Some(&NODE) } else { let x = 5; None }`
[01:25:12]    = note: `-D single-match-else` implied by `-D warnings`
[01:25:12] 
[01:25:12] 
[01:25:12] error: you don't need to add `&` to all patterns
[01:25:12]    |
[01:25:12] 30 | /         match v {
[01:25:12] 30 | /         match v {
[01:25:12] 31 | |             &Some(v) => println!("{:?}", v),
[01:25:12] 32 | |             &None => println!("none"),
[01:25:12]    | |_________^
[01:25:12]    |
[01:25:12]    = note: `-D match-ref-pats` implied by `-D warnings`
[01:25:12]    = note: `-D match-ref-pats` implied by `-D warnings`
[01:25:12] help: instead of prefixing all patterns with `&`, you can dereference the expression
[01:25:12] 30 |         match *v {
[01:25:12] 30 |         match *v {
[01:25:12] 31 |             Some(v) => println!("{:?}", v),
[01:25:12] 32 |             None => println!("none"),
[01:25:12] 
[01:25:12] 
[01:25:12] error: you don't need to add `&` to all patterns
[01:25:12]    |
[01:25:12] 40 | /     match tup {
---
[01:25:12] 
[01:25:12] ------------------------------------------
[01:25:12] stderr:
[01:25:12] ------------------------------------------
[01:25:12] error: you seem to be trying to use match for destructuring a single pattern. Consider using `if let`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 21 | /     match ExprNode::Butterflies {
[01:25:12] 22 | |         ExprNode::ExprAddrOf => Some(&NODE),
[01:25:12] 23 | |         _ => { let x = 5; None },
[01:25:12] 24 | |     }
[01:25:12]    | |_____^ help: try this: `if let ExprNode::ExprAddrOf = ExprNode::Butterflies { Some(&NODE) } else { let x = 5; None }`
[01:25:12]    = note: `-D single-match-else` implied by `-D warnings`
[01:25:12] 
[01:25:12] 
[01:25:12] error: you don't need to add `&` to all patterns
[01:25:12]    |
[01:25:12] 30 | /         match v {
[01:25:12] 30 | /         match v {
[01:25:12] 31 | |             &Some(v) => println!("{:?}", v),
[01:25:12] 32 | |             &None => println!("none"),
[01:25:12]    | |_________^
[01:25:12]    |
[01:25:12]    = note: `-D match-ref-pats` implied by `-D warnings`
[01:25:12]    = note: `-D match-ref-pats` implied by `-D warnings`
[01:25:12] help: instead of prefixing all patterns with `&`, you can dereference the expression
[01:25:12] 30 |         match *v {
[01:25:12] 30 |         match *v {
[01:25:12] 31 |             Some(v) => println!("{:?}", v),
[01:25:12] 32 |             None => println!("none"),
[01:25:12] 
[01:25:12] 
[01:25:12] error: you don't need to add `&` to all patterns
[01:25:12]    |
[01:25:12] 40 | /     match tup {
[01:25:12] 40 | /     match tup {
[01:25:12] 41 | |         &(v, 1) => println!("{}", v),
[01:25:12] 42 | |         _ => println!("none"),
[01:25:12]    | |_____^
[01:25:12]    | |_____^
[01:25:12] help: instead of prefixing all patterns with `&`, you can dereference the expression
[01:25:12]    |
[01:25:12] 40 |     match *tup {
[01:25:12] 41 |         (v, 1) => println!("{}", v),
[01:25:12] 
[01:25:12] 
[01:25:12] error: you don't need to add `&` to both the expression and the patterns
[01:25:12]    |
[01:25:12] 46 | /     match &w {
[01:25:12] 46 | /     match &w {
[01:25:12] 47 | |         &Some(v) => println!("{:?}", v),
[01:25:12] 48 | |         &None => println!("none"),
[01:25:12]    | |_____^
[01:25:12] help: try
[01:25:12]    |
[01:25:12] 46 |     match w {
[01:25:12] 46 |     match w {
[01:25:12] 47 |         Some(v) => println!("{:?}", v),
[01:25:12] 48 |         None => println!("none"),
[01:25:12] 
[01:25:12] 
[01:25:12] error: you don't need to add `&` to all patterns
[01:25:12]    |
[01:25:12]    |
[01:25:12] 57 | /     if let &None = a {
[01:25:12] 58 | |         println!("none");
[01:25:12]    | |_____^
[01:25:12]    | |_____^
[01:25:12] help: instead of prefixing all patterns with `&`, you can dereference the expression
[01:25:12] 57 |     if let None = *a {
[01:25:12]    |            ^^^^   ^^
[01:25:12] 
[01:25:12] 
[01:25:12] error: you don't need to add `&` to both the expression and the patterns
[01:25:12]    |
[01:25:12]    |
[01:25:12] 62 | /     if let &None = &b {
[01:25:12] 63 | |         println!("none");
[01:25:12]    | |_____^
[01:25:12] help: try
[01:25:12]    |
[01:25:12] 62 |     if let None = b {
[01:25:12] 62 |     if let None = b {
[01:25:12]    |            ^^^^   ^
[01:25:12] 
[01:25:12] error: some ranges overlap
[01:25:12]    |
[01:25:12]    |
[01:25:12] 71 |         0 ... 10 => println!("0 ... 10"),
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D match-overlapping-arm` implied by `-D warnings`
[01:25:12] note: overlaps with this
[01:25:12]    |
[01:25:12]    |
[01:25:12] 72 |         0 ... 11 => println!("0 ... 11"),
[01:25:12] 
[01:25:12] 
[01:25:12] error: some ranges overlap
[01:25:12]    |
[01:25:12]    |
[01:25:12] 77 |         0 ... 5 => println!("0 ... 5"),
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: overlaps with this
[01:25:12]    |
[01:25:12]    |
[01:25:12] 79 |         FOO ... 11 => println!("0 ... 11"),
[01:25:12] 
[01:25:12] 
[01:25:12] error: some ranges overlap
[01:25:12]    |
[01:25:12]    |
[01:25:12] 85 |         0 ... 5 => println!("0 ... 5"),
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: overlaps with this
[01:25:12]    |
[01:25:12] 84 |         2 => println!("2"),
[01:25:12]    |         ^
[01:25:12] 
[01:25:12] 
[01:25:12] error: some ranges overlap
[01:25:12]    |
[01:25:12]    |
[01:25:12] 91 |         0 ... 2 => println!("0 ... 2"),
[01:25:12]    |
[01:25:12]    |
[01:25:12] note: overlaps with this
[01:25:12]    |
[01:25:12] 90 |         2 => println!("2"),
[01:25:12]    |         ^
[01:25:12] 
[01:25:12] 
[01:25:12] error: some ranges overlap
[01:25:12]     |
[01:25:12]     |
[01:25:12] 114 |         0 .. 11 => println!("0 .. 11"),
[01:25:12]     |
[01:25:12]     |
[01:25:12] note: overlaps with this
[01:25:12]     |
[01:25:12]     |
[01:25:12] 115 |         0 ... 11 => println!("0 ... 11"),
[01:25:12] 
[01:25:12] 
[01:25:12] error: Err(_) will match all errors, maybe not a good idea
[01:25:12]     |
[01:25:12]     |
[01:25:12] 132 |         Err(_) => panic!("err")
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: `-D match-wild-err-arm` implied by `-D warnings`
[01:25:12]     = note: to remove this warning, match each error seperately or use unreachable macro
[01:25:12] 
[01:25:12] error: Err(_) will match all errors, maybe not a good idea
[01:25:12]     |
[01:25:12]     |
[01:25:12] 138 |         Err(_) => {panic!()}
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: to remove this warning, match each error seperately or use unreachable macro
[01:25:12] 
[01:25:12] error: Err(_) will match all errors, maybe not a good idea
[01:25:12]     |
[01:25:12]     |
[01:25:12] 144 |         Err(_) => {panic!();}
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: to remove this warning, match each error seperately or use unreachable macro
[01:25:12] error: use as_ref() instead
[01:25:12]    --> tests/ui/matches.rs:212:33
[01:25:12]     |
[01:25:12]     |
[01:25:12] 212 |       let borrowed: Option<&()> = match owned {
[01:25:12] 213 | |         None => None,
[01:25:12] 213 | |         None => None,
[01:25:12] 214 | |         Some(ref v) => Some(v),
[01:25:12] 215 | |     };
[01:25:12]     | |_____^ help: try this: `owned.as_ref()`
[01:25:12]     = note: `-D match-as-ref` implied by `-D warnings`
[01:25:12] 
[01:25:12] 
[01:25:12] error: use as_mut() instead
[01:25:12]     |
[01:25:12]     |
[01:25:12] 218 |       let borrow_mut: Option<&mut ()> = match mut_owned {
[01:25:12] 219 | |         None => None,
[01:25:12] 219 | |         None => None,
[01:25:12] 220 | |         Some(ref mut v) => Some(v),
[01:25:12] 221 | |     };
[01:25:12]     | |_____^ help: try this: `mut_owned.as_mut()`
[01:25:12] error: aborting due to 16 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] ------------------------------------------
---
[01:25:12] normalized stderr:
[01:25:12] 
[01:25:12] 
[01:25:12] expected stderr:
[01:25:12] error: the loop variable `i` is only used to index `ns`.
[01:25:12]   |
[01:25:12] 8 |     for i in 3..10 {
[01:25:12]   |              ^^^^^
[01:25:12]   |
[01:25:12]   |
[01:25:12]   = note: `-D needless-range-loop` implied by `-D warnings`
[01:25:12]   |
[01:25:12]   |
[01:25:12] 8 |     for <item> in ns.iter().take(10).skip(3) {
[01:25:12] 
[01:25:12] 
[01:25:12] error: the loop variable `i` is only used to index `ms`.
[01:25:12]    |
[01:25:12]    |
[01:25:12] 29 |     for i in 0..ms.len() {
[01:25:12] help: consider using an iterator
[01:25:12]    |
[01:25:12]    |
[01:25:12] 29 |     for <item> in &mut ms {
[01:25:12] 
[01:25:12] 
[01:25:12] error: the loop variable `i` is only used to index `ms`.
[01:25:12]    |
[01:25:12]    |
[01:25:12] 35 |     for i in 0..ms.len() {
[01:25:12] help: consider using an iterator
[01:25:12]    |
[01:25:12]    |
[01:25:12] 35 |     for <item> in &mut ms {
[01:25:12] 
[01:25:12] error: aborting due to 3 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] diff of stderr:
[01:25:12] 
[01:25:12] -error: the loop variable `i` is only used to index `ns`.
[01:25:12] - --> $DIR/needless_range_loop.rs:8:14
[01:25:12] -  |
[01:25:12] -8 |     for i in 3..10 {
[01:25:12] -  |              ^^^^^
[01:25:12] -  |
[01:25:12] -  = note: `-D needless-range-loop` implied by `-D warnings`
[01:25:12] -  |
[01:25:12] -  |
[01:25:12] -8 |     for <item> in ns.iter().take(10).skip(3) {
[01:25:12] -  |         ^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:25:12] -
[01:25:12] -error: the loop variable `i` is only used to index `ms`.
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -29 |     for i in 0..ms.len() {
[01:25:12] -help: consider using an iterator
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -29 |     for <item> in &mut ms {
[01:25:12] -
[01:25:12] -
[01:25:12] -error: the loop variable `i` is only used to index `ms`.
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -35 |     for i in 0..ms.len() {
[01:25:12] -help: consider using an iterator
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -35 |     for <item> in &mut ms {
[01:25:12] -
[01:25:12] -error: aborting due to 3 previous errors
[01:25:12] -
[01:25:12] -
---
[01:25:12] normalized stderr:
[01:25:12] error: use of `println!`
[01:25:12]   --> $DIR/print.rs:25:5
[01:25:12]    |
[01:25:12] 25 |     println!("Hello");
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D print-stdout` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: use of `print!`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 26 |     print!("Hello");
[01:25:12] 
[01:25:12] 
[01:25:12] error: use of `print!`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 28 |     print!("Hello {}", "World");
[01:25:12] 
[01:25:12] 
[01:25:12] error: use of `print!`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 30 |     print!("Hello {:?}", "World");
[01:25:12] 
[01:25:12] 
[01:25:12] error: use of `print!`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 32 |     print!("Hello {:#?}", "#orld");
[01:25:12] 
[01:25:12] error: aborting due to 5 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] expected stderr:
[01:25:12] error: use of `Debug`-based formatting
[01:25:12]    |
[01:25:12]    |
[01:25:12] 13 |         write!(f, "{:?}", 43.1415)
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D use-debug` implied by `-D warnings`
[01:25:12] error: use of `println!`
[01:25:12]   --> $DIR/print.rs:25:5
[01:25:12]    |
[01:25:12]    |
[01:25:12] 25 |     println!("Hello");
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D print-stdout` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: use of `print!`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 26 |     print!("Hello");
[01:25:12] 
[01:25:12] 
[01:25:12] error: use of `print!`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 28 |     print!("Hello {}", "World");
[01:25:12] 
[01:25:12] 
[01:25:12] error: use of `print!`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 30 |     print!("Hello {:?}", "World");
[01:25:12] 
[01:25:12] 
[01:25:12] error: use of `Debug`-based formatting
[01:25:12]    |
[01:25:12]    |
[01:25:12] 30 |     print!("Hello {:?}", "World");
[01:25:12] 
[01:25:12] 
[01:25:12] error: use of `print!`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 32 |     print!("Hello {:#?}", "#orld");
[01:25:12] 
[01:25:12] 
[01:25:12] error: use of `Debug`-based formatting
[01:25:12]    |
[01:25:12]    |
[01:25:12] 32 |     print!("Hello {:#?}", "#orld");
[01:25:12] 
[01:25:12] error: aborting due to 8 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] diff of stderr:
[01:25:12] 
[01:25:12] -error: use of `Debug`-based formatting
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -13 |         write!(f, "{:?}", 43.1415)
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -   = note: `-D use-debug` implied by `-D warnings`
[01:25:12]  error: use of `println!`
[01:25:12]    --> $DIR/print.rs:25:5
[01:25:12]     |
[01:25:12]     |
[01:25:12]  25 |     println!("Hello");
[01:25:12]     |
[01:25:12]     |
[01:25:12]     = note: `-D print-stdout` implied by `-D warnings`
[01:25:12]  
[01:25:12]  error: use of `print!`
[01:25:12]     |
[01:25:12]     |
[01:25:12]  26 |     print!("Hello");
[01:25:12]  
[01:25:12]  
[01:25:12]  error: use of `print!`
[01:25:12]     |
[01:25:12]     |
[01:25:12]  28 |     print!("Hello {}", "World");
[01:25:12]  
[01:25:12]  
[01:25:12]  error: use of `print!`
[01:25:12]     |
[01:25:12]     |
[01:25:12]  30 |     print!("Hello {:?}", "World");
[01:25:12]  
[01:25:12]  
[01:25:12] -error: use of `Debug`-based formatting
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -30 |     print!("Hello {:?}", "World");
[01:25:12] -
[01:25:12] -
[01:25:12]  error: use of `print!`
[01:25:12]     |
[01:25:12]     |
[01:25:12]  32 |     print!("Hello {:#?}", "#orld");
[01:25:12]  
[01:25:12]  
[01:25:12] -error: use of `Debug`-based formatting
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -32 |     print!("Hello {:#?}", "#orld");
[01:25:12] -
[01:25:12] -error: aborting due to 8 previous errors
[01:25:12] +error: aborting due to 5 previous errors
[01:25:12]  
---
[01:25:12] ------------------------------------------
[01:25:12] error: use of `println!`
[01:25:12]   --> tests/ui/print.rs:25:5
[01:25:12]    |
[01:25:12] 25 |     println!("Hello");
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D print-stdout` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: use of `print!`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 26 |     print!("Hello");
[01:25:12] 
[01:25:12] 
[01:25:12] error: use of `print!`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 28 |     print!("Hello {}", "World");
[01:25:12] 
[01:25:12] 
[01:25:12] error: use of `print!`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 30 |     print!("Hello {:?}", "World");
[01:25:12] 
[01:25:12] 
[01:25:12] error: use of `print!`
[01:25:12]    |
[01:25:12]    |
[01:25:12] 32 |     print!("Hello {:#?}", "#orld");
[01:25:12] 
[01:25:12] error: aborting due to 5 previous errors
[01:25:12] 
[01:25:12] 
---
[01:25:12] normalized stderr:
[01:25:12] 
[01:25:12] 
[01:25:12] expected stderr:
[01:25:12] error: an inclusive range would be more readable
[01:25:12]    |
[01:25:12]    |
[01:25:12] 10 |     for _ in 0..3+1 { }
[01:25:12]    |              ^^^^^^ help: use: `0..=3`
[01:25:12]    |
[01:25:12]    = note: `-D range-plus-one` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: an inclusive range would be more readable
[01:25:12]    |
[01:25:12]    |
[01:25:12] 13 |     for _ in 0..1+5 { }
[01:25:12]    |              ^^^^^^ help: use: `0..=5`
[01:25:12] 
[01:25:12] error: an inclusive range would be more readable
[01:25:12]    |
[01:25:12]    |
[01:25:12] 16 |     for _ in 1..1+1 { }
[01:25:12]    |              ^^^^^^ help: use: `1..=1`
[01:25:12] 
[01:25:12] error: an inclusive range would be more readable
[01:25:12]    |
[01:25:12]    |
[01:25:12] 22 |     for _ in 0..(1+f()) { }
[01:25:12]    |              ^^^^^^^^^^ help: use: `0..=f()`
[01:25:12] 
[01:25:12] error: an exclusive range would be more readable
[01:25:12]    |
[01:25:12] 26 |     let _ = ..=11-1;
[01:25:12] 26 |     let _ = ..=11-1;
[01:25:12]    |             ^^^^^^^ help: use: `..11`
[01:25:12]    |
[01:25:12]    = note: `-D range-minus-one` implied by `-D warnings`
[01:25:12] 
[01:25:12] error: an exclusive range would be more readable
[01:25:12]    |
[01:25:12]    |
[01:25:12] 27 |     let _ = ..=(11-1);
[01:25:12]    |             ^^^^^^^^^ help: use: `..11`
[01:25:12] 
[01:25:12] error: an inclusive range would be more readable
[01:25:12]    |
[01:25:12]    |
[01:25:12] 28 |     let _ = (f()+1)..(f()+1);
[01:25:12]    |             ^^^^^^^^^^^^^^^^ help: use: `(f()+1)..=f()`
[01:25:12] error: aborting due to 7 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] diff of stderr:
[01:25:12] 
[01:25:12] -error: an inclusive range would be more readable
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -10 |     for _ in 0..3+1 { }
[01:25:12] -   |              ^^^^^^ help: use: `0..=3`
[01:25:12] -   |
[01:25:12] -   = note: `-D range-plus-one` implied by `-D warnings`
[01:25:12] -
[01:25:12] -error: an inclusive range would be more readable
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -13 |     for _ in 0..1+5 { }
[01:25:12] -   |              ^^^^^^ help: use: `0..=5`
[01:25:12] -
[01:25:12] -error: an inclusive range would be more readable
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -16 |     for _ in 1..1+1 { }
[01:25:12] -   |              ^^^^^^ help: use: `1..=1`
[01:25:12] -
[01:25:12] -error: an inclusive range would be more readable
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -22 |     for _ in 0..(1+f()) { }
[01:25:12] -   |              ^^^^^^^^^^ help: use: `0..=f()`
[01:25:12] -
[01:25:12] -error: an exclusive range would be more readable
[01:25:12] -   |
[01:25:12] -26 |     let _ = ..=11-1;
[01:25:12] -26 |     let _ = ..=11-1;
[01:25:12] -   |             ^^^^^^^ help: use: `..11`
[01:25:12] -   |
[01:25:12] -   = note: `-D range-minus-one` implied by `-D warnings`
[01:25:12] -
[01:25:12] -error: an exclusive range would be more readable
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -27 |     let _ = ..=(11-1);
[01:25:12] -   |             ^^^^^^^^^ help: use: `..11`
[01:25:12] -
[01:25:12] -error: an inclusive range would be more readable
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -28 |     let _ = (f()+1)..(f()+1);
[01:25:12] -   |             ^^^^^^^^^^^^^^^^ help: use: `(f()+1)..=f()`
[01:25:12] -error: aborting due to 7 previous errors
[01:25:12] -
[01:25:12] -
[01:25:12] 
[01:25:12] 
[01:25:12] The actual stderr differed from the expected stderr.
[01:25:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base/range_plus_minus_one.stderr
[01:25:12] To update references, run this command from build directory:
[01:25:12] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base' 'range_plus_minus_one.rs'
[01:25:12] 
[01:25:12] error: 1 errors occurred comparing output.
[01:25:12] status: exit code: 0
[01:25:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/range_plus_minus_one.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base/range_plus_minus_one.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base/range_plus_minus_one.stage-id.aux" "-A" "unused"
[01:25:12] ------------------------------------------
[01:25:12] 
[01:25:12] ------------------------------------------
[01:25:12] stderr:
---
[01:25:12] normalized stderr:
[01:25:12] 
[01:25:12] 
[01:25:12] expected stderr:
[01:25:12] error: zero-width space detected
[01:25:12]   |
[01:25:12]   |
[01:25:12] 6 |     print!("Here >​< is a ZWS, and ​another");
[01:25:12]   |
[01:25:12]   |
[01:25:12]   = note: `-D zero-width-space` implied by `-D warnings`
[01:25:12]   = help: Consider replacing the string with:
[01:25:12]           ""Here >/u{200B}< is a ZWS, and /u{200B}another""
[01:25:12] 
[01:25:12] error: non-nfc unicode sequence detected
[01:25:12]    |
[01:25:12]    |
[01:25:12] 12 |     print!("̀àh?");
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D unicode-not-nfc` implied by `-D warnings`
[01:25:12]    = help: Consider replacing the string with:
[01:25:12]            ""̀àh?""
[01:25:12] error: literal non-ASCII character detected
[01:25:12]   --> $DIR/unicode.rs:18:12
[01:25:12]    |
[01:25:12]    |
[01:25:12] 18 |     print!("Üben!");
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D non-ascii-literal` implied by `-D warnings`
[01:25:12]    = help: Consider replacing the string with:
[01:25:12]            ""/u{dc}ben!""
[01:25:12] error: aborting due to 3 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] diff of stderr:
[01:25:12] 
[01:25:12] -error: zero-width space detected
[01:25:12] - --> $DIR/unicode.rs:6:12
[01:25:12] -  |
[01:25:12] -6 |     print!("Here >​< is a ZWS, and ​another");
[01:25:12] -  |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:25:12] -  |
[01:25:12] -  = note: `-D zero-width-space` implied by `-D warnings`
[01:25:12] -  = help: Consider replacing the string with:
[01:25:12] -          ""Here >/u{200B}< is a ZWS, and /u{200B}another""
[01:25:12] -
[01:25:12] -error: non-nfc unicode sequence detected
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -12 |     print!("̀àh?");
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -   = note: `-D unicode-not-nfc` implied by `-D warnings`
[01:25:12] -   = help: Consider replacing the string with:
[01:25:12] -           ""̀àh?""
[01:25:12] -error: literal non-ASCII character detected
[01:25:12] -  --> $DIR/unicode.rs:18:12
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -18 |     print!("Üben!");
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -   = note: `-D non-ascii-literal` implied by `-D warnings`
[01:25:12] -   = help: Consider replacing the string with:
[01:25:12] -           ""/u{dc}ben!""
[01:25:12] -error: aborting due to 3 previous errors
[01:25:12] -
[01:25:12] -
[01:25:12] 
[01:25:12] 
[01:25:12] The actual stderr differed from the expected stderr.
[01:25:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base/unicode.stderr
[01:25:12] To update references, run this command from build directory:
[01:25:12] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base' 'unicode.rs'
[01:25:12] 
[01:25:12] error: 1 errors occurred comparing output.
[01:25:12] status: exit code: 0
[01:25:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/unicode.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base/unicode.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base/unicode.stage-id.aux" "-A" "unused"
[01:25:12] ------------------------------------------
[01:25:12] 
[01:25:12] ------------------------------------------
[01:25:12] stderr:
[01:25:12] stderr:
[01:25:12] ------------------------------------------
[01:25:12] 
[01:25:12] ------------------------------------------
[01:25:12] 
[01:25:12] thread '[ui] ui/unicode.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.11/src/runtest.rs:2544:9
[01:25:12] 
[01:25:12] ---- [ui] ui/unused_io_amount.rs stdout ----
[01:25:12] normalized stderr:
[01:25:12] error: handle written amount returned or use `Write::write_all` instead
[01:25:12]   --> $DIR/unused_io_amount.rs:18:5
[01:25:12]    |
[01:25:12] 18 |     s.write(b"test")?;
[01:25:12] 
[01:25:12] 
[01:25:12] error: handle read amount returned or use `Read::read_exact` instead
[01:25:12]   --> $DIR/unused_io_amount.rs:20:5
[01:25:12]    |
[01:25:12] 20 |     s.read(&mut buf)?;
[01:25:12] 
[01:25:12] 
[01:25:12] error: handle written amount returned or use `Write::write_all` instead
[01:25:12]   --> $DIR/unused_io_amount.rs:25:5
[01:25:12]    |
[01:25:12] 25 |     s.write(b"test").unwrap();
[01:25:12] 
[01:25:12] 
[01:25:12] error: handle read amount returned or use `Read::read_exact` instead
[01:25:12]   --> $DIR/unused_io_amount.rs:27:5
[01:25:12]    |
[01:25:12] 27 |     s.read(&mut buf).unwrap();
[01:25:12] 
[01:25:12] error: aborting due to 4 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] expected stderr:
[01:25:12] error: handle written amount returned or use `Write::write_all` instead
[01:25:12]   --> $DIR/unused_io_amount.rs:11:5
[01:25:12]    |
[01:25:12] 11 |     try!(s.write(b"test"));
[01:25:12]    |
[01:25:12]    |
[01:25:12]    = note: `-D unused-io-amount` implied by `-D warnings`
[01:25:12] 
[01:25:12] 
[01:25:12] error: handle read amount returned or use `Read::read_exact` instead
[01:25:12]   --> $DIR/unused_io_amount.rs:13:5
[01:25:12]    |
[01:25:12] 13 |     try!(s.read(&mut buf));
[01:25:12]    |
[01:25:12]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:25:12] 
[01:25:12] 
[01:25:12] error: handle written amount returned or use `Write::write_all` instead
[01:25:12]   --> $DIR/unused_io_amount.rs:18:5
[01:25:12]    |
[01:25:12] 18 |     s.write(b"test")?;
[01:25:12] 
[01:25:12] 
[01:25:12] error: handle read amount returned or use `Read::read_exact` instead
[01:25:12]   --> $DIR/unused_io_amount.rs:20:5
[01:25:12]    |
[01:25:12] 20 |     s.read(&mut buf)?;
[01:25:12] 
[01:25:12] 
[01:25:12] error: handle written amount returned or use `Write::write_all` instead
[01:25:12]   --> $DIR/unused_io_amount.rs:25:5
[01:25:12]    |
[01:25:12] 25 |     s.write(b"test").unwrap();
[01:25:12] 
[01:25:12] 
[01:25:12] error: handle read amount returned or use `Read::read_exact` instead
[01:25:12]   --> $DIR/unused_io_amount.rs:27:5
[01:25:12]    |
[01:25:12] 27 |     s.read(&mut buf).unwrap();
[01:25:12] 
[01:25:12] error: aborting due to 6 previous errors
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] 
[01:25:12] diff of stderr:
[01:25:12] 
[01:25:12]  error: handle written amount returned or use `Write::write_all` instead
[01:25:12] -  --> $DIR/unused_io_amount.rs:11:5
[01:25:12] -   |
[01:25:12] -11 |     try!(s.write(b"test"));
[01:25:12] -   |
[01:25:12] -   |
[01:25:12] -   = note: `-D unused-io-amount` implied by `-D warnings`
[01:25:12] -
[01:25:12] -
[01:25:12] -error: handle read amount returned or use `Read::read_exact` instead
[01:25:12] -  --> $DIR/unused_io_amount.rs:13:5
[01:25:12] -   |
[01:25:12] -13 |     try!(s.read(&mut buf));
[01:25:12] -   |
[01:25:12] -   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:25:12] -
[01:25:12] -
[01:25:12] -error: handle written amount returned or use `Write::write_all` instead
[01:25:12]    --> $DIR/unused_io_amount.rs:18:5
[01:25:12]     |
[01:25:12]  18 |     s.write(b"test")?;
[01:25:12]  
[01:25:12]  
[01:25:12]  error: handle read amount returned or use `Read::read_exact` instead
[01:25:12]    --> $DIR/unused_io_amount.rs:20:5
[01:25:12]     |
[01:25:12]  20 |     s.read(&mut buf)?;
[01:25:12]  
[01:25:12]  
[01:25:12]  error: handle written amount returned or use `Write::write_all` instead
[01:25:12]    --> $DIR/unused_io_amount.rs:25:5
[01:25:12]     |
[01:25:12]  25 |     s.write(b"test").unwrap();
[01:25:12]  
[01:25:12]  
[01:25:12]  error: handle read amount returned or use `Read::read_exact` instead
[01:25:12]    --> $DIR/unused_io_amount.rs:27:5
[01:25:12]     |
[01:25:12]  27 |     s.read(&mut buf).unwrap();
[01:25:12]  
[01:25:12] -error: aborting due to 6 previous errors
[01:25:12] +error: aborting due to 4 previous errors
[01:25:12]  
---
[01:25:12] 
[01:25:12] ------------------------------------------
[01:25:12] stderr:
[01:25:12] ------------------------------------------
[01:25:12] error: handle written amount returned or use `Write::write_all` instead
[01:25:12]    |
[01:25:12]    |
[01:25:12] 18 |     s.write(b"test")?;
[01:25:12] 
[01:25:12] 
[01:25:12] error: handle read amount returned or use `Read::read_exact` instead
[01:25:12]    |
[01:25:12]    |
[01:25:12] 20 |     s.read(&mut buf)?;
[01:25:12] 
[01:25:12] 
[01:25:12] error: handle written amount returned or use `Write::write_all` instead
[01:25:12]    |
[01:25:12]    |
[01:25:12] 25 |     s.write(b"test").unwrap();
[01:25:12] 
[01:25:12] 
[01:25:12] error: handle read amount returned or use `Read::read_exact` instead
[01:25:12]    |
[01:25:12]    |
[01:25:12] 27 |     s.read(&mut buf).unwrap();
[01:25:12] 
[01:25:12] error: aborting due to 4 previous errors
[01:25:12] 
[01:25:12] 
---
[01:25:12] Verifying status of rustfmt...
[01:25:12] Verifying status of clippy-driver...
[01:25:12] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:25:12] 
[01:25:12] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:25:12] 
[01:25:12] If you do intend to update 'clippy-driver', please check the error messages above and
[01:25:12] commit another update.
[01:25:12] 
[01:25:12] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:25:12] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:25:12] proper steps.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:03490418
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0fd733bc:start=1532103599128617612,finish=1532103599135527867,duration=6910255
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03c4a220
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1460ce71
travis_time:start:1460ce71
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0949d0a6
$ dmesg | grep -i kill
