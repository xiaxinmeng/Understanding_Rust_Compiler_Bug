
---
---- result.rs - result::Result<T, E>::as_deref (line 1157) stdout ----
error[E0658]: use of unstable library feature 'inner_deref'
 --> result.rs:1160:14
  |
6 | assert_eq!(x.as_deref(), y);
  |
  |
  = note: see issue #50264 <***/issues/50264> for more information
  = help: add `#![feature(inner_deref)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'inner_deref'
  --> result.rs:1164:14
   |
   |
10 | assert_eq!(x.as_deref(), y);
   |
   |
   = note: see issue #50264 <***/issues/50264> for more information
   = help: add `#![feature(inner_deref)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- result.rs - result::Result<T, E>::as_deref_mut (line 1191) stdout ----
error[E0308]: mismatched types
 --> result.rs:1193:40
  |
5 | let y: Result<&mut str, &mut u32> = Ok("HELLO");
  |                                        ^^^^^^^ types differ in mutability
  = note: expected mutable reference `&mut str`
                     found reference `&'static str`

error[E0658]: use of unstable library feature 'inner_deref'
error[E0658]: use of unstable library feature 'inner_deref'
 --> result.rs:1194:14
  |
6 | assert_eq!(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
  |
  |
  = note: see issue #50264 <***/issues/50264> for more information
  = help: add `#![feature(inner_deref)]` to the crate attributes to enable
error[E0308]: mismatched types
 --> result.rs:1197:41
  |
  |
9 | let y: Result<&mut str, &mut u32> = Err(&42);
  |                                         ^^^ types differ in mutability
  = note: expected mutable reference `&mut u32`
                     found reference `&u32`

error[E0658]: use of unstable library feature 'inner_deref'
error[E0658]: use of unstable library feature 'inner_deref'
  --> result.rs:1198:14
   |
10 | assert_eq!(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
   |
   |
   = note: see issue #50264 <***/issues/50264> for more information
   = help: add `#![feature(inner_deref)]` to the crate attributes to enable
error: aborting due to 4 previous errors
