
[01:28:48] failures:
[01:28:48] 
[01:28:48] ---- /checkout/src/doc/unstable-book/src/language-features/macro-at-most-once-rep.md - macro_at_most_once_rep (line 9) stdout ----
[01:28:48] 	error[E0658]: Using the `?` macro Kleene operator for "at most one" repetition is unstable (see issue #48075)
[01:28:48]  --> /checkout/src/doc/unstable-book/src/language-features/macro-at-most-once-rep.md:11:17
[01:28:48]   |
[01:28:48] 4 |     (something $(,)?) // `?` indicates `,` is "optional"...
[01:28:48]   |                 ^^^
[01:28:48]   |
[01:28:48]   = help: add #![feature(macro_at_most_once_rep)] to the crate attributes to enable
[01:28:48] 
[01:28:48] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:295:13
[01:28:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:28:48] 
[01:28:48] 
[01:28:48] failures:
[01:28:48]     /checkout/src/doc/unstable-book/src/language-features/macro-at-most-once-rep.md - macro_at_most_once_rep (line 9)
