plain

   Doc-tests std

running 1201 tests
iFF....................F.............F.....F.....................F.................................. 100/1201
..............................iii......i......i...i.........i....................................... 300/1201
.................................................................................................... 400/1201
...............................................................................................i.... 500/1201
i................................................................i.................................. 600/1201
---
..............................iiiiii.................................i.............................. 1200/1201
.
failures:

---- src/collections/hash/map.rs - collections::hash::map::Drain (line 1383) stdout ----
error[E0596]: cannot borrow `map` as mutable, as it is not declared as mutable
  |
6 | let map = HashMap::from([
6 | let map = HashMap::from([
  |     --- help: consider changing this to be mutable: `mut map`
9 | let iter = map.drain();
  |            ^^^^^^^^^^^ cannot borrow as mutable

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
---- src/collections/hash/map.rs - collections::hash::map::DrainFilter (line 1412) stdout ----
error[E0596]: cannot borrow `map` as mutable, as it is not declared as mutable
   |
8  | let map = HashMap::from([
8  | let map = HashMap::from([
   |     --- help: consider changing this to be mutable: `mut map`
...
11 | let iter = map.drain_filter(|_k, v| *v % 2 == 0);
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/collections/hash/map.rs - collections::hash::map::HashMap<K,V,S>::iter_mut (line 431) stdout ----
error[E0596]: cannot borrow `map` as mutable, as it is not declared as mutable
   |
6  | let map = HashMap::from([
6  | let map = HashMap::from([
   |     --- help: consider changing this to be mutable: `mut map`
...
13 | for (_, val) in map.iter_mut() {
   |                 ^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/collections/hash/map.rs - collections::hash::map::HashMap<K,V,S>::values_mut (line 380) stdout ----
error[E0596]: cannot borrow `map` as mutable, as it is not declared as mutable
   |
6  | let map = HashMap::from([
6  | let map = HashMap::from([
   |     --- help: consider changing this to be mutable: `mut map`
12 | for val in map.values_mut() {
   |            ^^^^^^^^^^^^^^^^ cannot borrow as mutable

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
---- src/collections/hash/map.rs - collections::hash::map::IterMut (line 1246) stdout ----
error[E0596]: cannot borrow `map` as mutable, as it is not declared as mutable
  |
6 | let map = HashMap::from([
6 | let map = HashMap::from([
  |     --- help: consider changing this to be mutable: `mut map`
...
9 | let iter = map.iter_mut();
  |            ^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/collections/hash/map.rs - collections::hash::map::ValuesMut (line 1439) stdout ----
error[E0596]: cannot borrow `map` as mutable, as it is not declared as mutable
  |
6 | let map = HashMap::from([
6 | let map = HashMap::from([
  |     --- help: consider changing this to be mutable: `mut map`
...
9 | let iter_values = map.values_mut();
  |                   ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:27:26
