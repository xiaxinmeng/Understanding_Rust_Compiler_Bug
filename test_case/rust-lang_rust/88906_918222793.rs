plain
   Compiling rustc-demangle v0.1.21
error: unknown start of token: `
 --> library/alloc/src/boxed.rs:3:6
  |
3 | 5/! [`Box<T>`], casually referred to as a 'box', provides the simplest form of
  |
  |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |
3 | 5/! ['Box<T>`], casually referred to as a 'box', provides the simplest form of

error: unknown start of token: `
 --> library/alloc/src/boxed.rs:3:13
  |
  |
3 | 5/! [`Box<T>`], casually referred to as a 'box', provides the simplest form of
  |
  |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |
3 | 5/! [`Box<T>'], casually referred to as a 'box', provides the simplest form of


error: character literal may only contain one codepoint
  |
  |
3 | 5/! [`Box<T>`], casually referred to as a 'box', provides the simplest form of
  |
  |
help: if you meant to write a `str` literal, use double quotes
  |
3 | 5/! [`Box<T>`], casually referred to as a "box", provides the simplest form of

error: expected item, found `5`
 --> library/alloc/src/boxed.rs:3:1
  |
  |
3 | 5/! [`Box<T>`], casually referred to as a 'box', provides the simplest form of

error[E0432]: unresolved import `crate::boxed::Box`
  --> library/alloc/src/collections/btree/node.rs:40:5
   |
   |
40 | use crate::boxed::Box;
   |     ^^^^^^^^^^^^^^^^^ no `Box` in `boxed`
error[E0432]: unresolved import `crate::boxed::Box`
  --> library/alloc/src/collections/linked_list.rs:24:5
   |
24 | use crate::boxed::Box;
24 | use crate::boxed::Box;
   |     ^^^^^^^^^^^^^^^^^ no `Box` in `boxed`
error[E0432]: unresolved import `crate::boxed::Box`
  --> library/alloc/src/prelude/v1.rs:10:9
   |
10 | pub use crate::boxed::Box;
10 | pub use crate::boxed::Box;
   |         ^^^^^^^^^^^^^^^^^ no `Box` in `boxed`
error[E0432]: unresolved import `crate::boxed::Box`
  --> library/alloc/src/raw_vec.rs:15:5
   |
15 | use crate::boxed::Box;
15 | use crate::boxed::Box;
   |     ^^^^^^^^^^^^^^^^^ no `Box` in `boxed`
error[E0432]: unresolved import `crate::boxed::Box`
   --> library/alloc/src/rc.rs:246:5
    |
246 | use crate::boxed::Box;
246 | use crate::boxed::Box;
    |     ^^^^^^^^^^^^^^^^^ no `Box` in `boxed`
error[E0432]: unresolved import `crate::boxed::Box`
   --> library/alloc/src/slice.rs:100:5
    |
100 | use crate::boxed::Box;
100 | use crate::boxed::Box;
    |     ^^^^^^^^^^^^^^^^^ no `Box` in `boxed`
error[E0432]: unresolved import `crate::boxed::Box`
   --> library/alloc/src/slice.rs:155:9
    |
155 |     use crate::boxed::Box;
155 |     use crate::boxed::Box;
    |         ^^^^^^^^^^^^^^^^^ no `Box` in `boxed`
error[E0432]: unresolved import `crate::boxed::Box`
  --> library/alloc/src/str.rs:38:5
   |
38 | use crate::boxed::Box;
38 | use crate::boxed::Box;
   |     ^^^^^^^^^^^^^^^^^ no `Box` in `boxed`
error[E0432]: unresolved import `crate::boxed::Box`
  --> library/alloc/src/string.rs:67:5
   |
67 | use crate::boxed::Box;
67 | use crate::boxed::Box;
   |     ^^^^^^^^^^^^^^^^^ no `Box` in `boxed`
error[E0432]: unresolved import `crate::boxed::Box`
  --> library/alloc/src/sync.rs:36:5
   |
36 | use crate::boxed::Box;
36 | use crate::boxed::Box;
   |     ^^^^^^^^^^^^^^^^^ no `Box` in `boxed`
error[E0432]: unresolved import `crate::boxed::Box`
  --> library/alloc/src/vec/mod.rs:74:5
   |
74 | use crate::boxed::Box;
74 | use crate::boxed::Box;
   |     ^^^^^^^^^^^^^^^^^ no `Box` in `boxed`
error[E0432]: unresolved import `crate::boxed::Box`
 --> library/alloc/src/vec/is_zero.rs:1:5
  |
1 | use crate::boxed::Box;
1 | use crate::boxed::Box;
  |     ^^^^^^^^^^^^^^^^^ no `Box` in `boxed`
error: specializing impl repeats parameter `T`
    --> library/alloc/src/rc.rs:1896:1
     |
     |
1896 | / impl<T, const N: usize> TryFrom<Rc<[T]>> for Rc<[T; N]> {
1897 | |     type Error = Rc<[T]>;
1898 | |
1899 | |     fn try_from(boxed_slice: Rc<[T]>) -> Result<Self, Self::Error> {
1905 | |     }
1906 | | }
     | |_^


error: specializing impl repeats parameter `T`
    --> library/alloc/src/sync.rs:2486:1
     |
2486 | / impl<T, const N: usize> TryFrom<Arc<[T]>> for Arc<[T; N]> {
2487 | |     type Error = Arc<[T]>;
2488 | |
2489 | |     fn try_from(boxed_slice: Arc<[T]>) -> Result<Self, Self::Error> {
2495 | |     }
2496 | | }
     | |_^

