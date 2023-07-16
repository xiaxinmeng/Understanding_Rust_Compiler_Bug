
failures:

---- slice::[T]::ends_with_1 stdout ----
	error[E0282]: unable to infer enough type information about `T`
 --> <anon>:7:11
  |
7 | assert!(v.ends_with(&[]));
  |           ^^^^^^^^^ cannot infer type for `T`
  |
  = note: type annotations or generic parameter binding required

error: aborting due to previous error(s)

thread 'slice::[T]::ends_with_1' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:203
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- slice::[T]::iter_mut_0 stdout ----
	error[E0277]: the trait bound `&mut [{integer}; 3]: std::cmp::PartialEq<[{integer}; 3]>` is not satisfied
 --> <anon>:8:1
  |
8 | assert_eq!(x, [3, 4, 6]);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::cmp::PartialEq<[{integer}; 3]>` is not implemented for `&mut [{integer}; 3]`
  |
  = help: the following implementations were found:
            <[A] as std::cmp::PartialEq<[B]>>
            <[A; 0] as std::cmp::PartialEq<[B; 0]>>
            <[B] as std::cmp::PartialEq<[A; 0]>>
            <[A; 0] as std::cmp::PartialEq<[B]>>
          and 228 others
  = note: this error originates in a macro outside of the current crate

error: aborting due to previous error(s)

thread 'slice::[T]::iter_mut_0' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:203

---- slice::[T]::starts_with_1 stdout ----
	error[E0282]: unable to infer enough type information about `T`
 --> <anon>:7:11
  |
7 | assert!(v.starts_with(&[]));
  |           ^^^^^^^^^^^ cannot infer type for `T`
  |
  = note: type annotations or generic parameter binding required

error: aborting due to previous error(s)

thread 'slice::[T]::starts_with_1' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:203
