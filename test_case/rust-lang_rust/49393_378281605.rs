plain
Resolving deltas: 100% (609943/609943), completed with 4836 local objects.
---
[00:00:44] configure: rust.quiet-tests     := True
---
[00:58:29] ..........................................................................i.........................
[00:58:38] .................i..................................................................................
---
[00:59:21] .............................................................................................i......
[00:59:30] .................................................................i..................................
---
[01:00:45] .............................................i......................................................
---
[01:06:04] .............................i......................................................................
[01:06:24] ..............................................................i.....................................
[01:06:43] ...............................................i....................................................
[01:07:09] ....................................................................................................
[01:07:38] ....................................................................................................
[01:08:06] ....................................................................................................
[01:08:38] .i................................................................................................i.
[01:08:56] ........................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[01:09:27] ............................................................
[01:10:07] ....................................................................................................
[01:10:55] ...............................................................ii...................................
[01:11:34] ..........................i....................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[01:11:58] ................................i.ii.................
[01:12:52] .......................................................................................iiiiiii......
---
[01:15:56] ....................................i...............................................................
[01:16:06] ....................................................................................................
[01:16:16] ..................i............................................................ii.iii...............
[01:16:25] ....................................................................................................
[01:16:35] ........i..............................i............................................................
[01:16:45] ....................................................................................................
[01:16:53] .....................i..............................................................................
---
[01:18:08] ..............i.....................................................................................
[01:18:20] .................i..ii..............................................................................
[01:18:32] ....................................................................................................
[01:18:45] ....................................................................................................
[01:18:56] ....................................................................................i...............
[01:19:08] ..............................i.....................................................................
---
[01:19:50] ...........................i........................................................................
[01:19:52] ....................................................................i...............................
[01:19:53] ................i.......................................................
---
[01:20:13] ...........i........................
---
[01:20:50] i...i..ii....i.............ii........iii......i..i...i...ii..i..i..ii.....
---
[01:20:54] i.......i......................i......
---
[01:21:45] iiii.......i..i........i..i.i.............i..........iiii...........i...i..........ii.i.i.......ii..
[01:21:46] ....ii...
---
[01:35:15] ...i................................................................................................
---
[01:37:58] ......................................i.............................................................
[01:38:29] ....................................................................................................
[01:38:58] .............................................i......................................................
---
[01:41:01] ..........................................................ii........................................
---
[01:42:33] ..............................................................i.....................................
---
[01:45:44] .......................................................F..F...........F..F...........F..F...........
[01:46:06] F..F...........F..F...........F..F..................................................................
---
[01:46:52] 6 | use std::num::wrapping;
[01:46:52]   |     ^^^^^^^^^^^^^^^^^^ no `wrapping` in `num`. Did you mean to use `Wrapping`?
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:689:9
[01:46:52]   |
[01:46:52] 8 | assert!(Wrapping(16).is_power_of_two());
[01:46:52]   |         ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:690:10
[01:46:52]   |
[01:46:52] 9 | assert!(!Wrapping(10).is_power_of_two());
[01:46:52]   |          ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:691:10
[01:46:52]    |
[01:46:52] 10 | assert!(!Wrapping(0).is_power_of_two());
[01:46:52]    |          ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
---
[01:46:52]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:46:52]
[01:46:52] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:46:52] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:46:52]
[01:46:52] ---- num/wrapping.rs - num::wrapping::Wrapping<u128>::next_power_of_two (line 707) stdout ----
[01:46:52]  error[E0432]: unresolved import `std::num::wrapping`
[01:46:52]  --> num/wrapping.rs:710:5
[01:46:52]   |
[01:46:52] 6 | use std::num::wrapping;
[01:46:52]   |     ^^^^^^^^^^^^^^^^^^ no `wrapping` in `num`. Did you mean to use `Wrapping`?
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:712:12
[01:46:52]   |
[01:46:52] 8 | assert_eq!(Wrapping(2).next_power_of_two(), Wrapping(2));
[01:46:52]   |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:712:45
[01:46:52]   |
[01:46:52] 8 | assert_apping;
[01:46:52]    |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:714:50
[01:46:52]    |
[01:46:52] 10 | assert_eq!(Wrapping(200_u8).next_power_of_two(), Wrapping(0));
[01:46:52]    |                                                  ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
---
[01:46:52]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:46:52]
[01:46:52] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:46:52]
[01:46:52] ---- num/wrapping.rs - num::wrapping::Wrapping<u16>::is_power_of_two (line 684) stdout ----
[01:46:52]  error[E0432]: unresolved import `std::num::wrapping`
[01:46:52]  --> num/wrapping.rs:687:5
[01:46:52]   |
[01:46:52] 6 | use std::num::wrapping;
[01:46:52]   |     ^^^^^^^^^^^^^^^^^^ no `wrapping` in `num`. Did you mean to use `Wrapping`?
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:689:9
[01:46:52]   |
[01:46:52] 8 | assert!(Wrapping(16).is_power_of_two());
[01:46:52]   |         ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:690:10
[01:46:52]   |
[01:46:52] 9 | assert!(!Wrapping(10).is_power_of_two());
[01:46:52]   |          ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:691:10
[01:46:52]    |
[01:46:52] 10 | assert!(!Wrapping(0).is_power_of_two());
[01:46:52]    |          ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
---
[01:46:52]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:46:52]
[01:46:52] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:46:52]
[01:46:52] ---- num/wrapping.rs - num::wrapping::Wrapping<u16>::next_power_of_two (line 707) stdout ----
[01:46:52]  error[E0432]: unresolved import `std::num::wrapping`
[01:46:52]  --> num/wrapping.rs:710:5
[01:46:52]   |
[01:46:52] 6 | use std::num::wrapping;
[01:46:52]   |     ^^^^^^^^^^^^^^^^^^ no `wrapping` in `num`. Did you mean to use `Wrapping`?
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:712:12
[01:46:52]   |
[01:46:52] 8 | assert_eq!(Wrapping(2).next_power_of_two(), Wrapping(2));
[01:46:52]   |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:712:45
[01:46:52]   |
[01:46:52] 8 | assert_eq!(Wrapping(2).next_power_of_two(), Wrapping(2));
[01:46:52]   |                                             ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:713:12
[01:46:52]   |
[01:46:52] 9 | assert_eq!(Wrapping(3).next_power_of_two(), Wrapping(4));
[01:46:52]   |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:713:45
[01:46:52]   |
[01:46:52] 9 | assert_eq!(Wrapping(3).next_power_of_two(), Wrapping(4));
[01:46:52]   |                                             ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:714:12
[01:46:52]    |
[01:46:52] 10 | assert_eq!(Wrapping(200_u8).next_power_of_two(), Wrapping(0));
[01:46:52]    |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]    |
[01:46:52] 5  | use std::num::Wrapping;
[01:46:52]    |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:714:50
[01:46:52]    |
[01:46:52] 10 | assert_eq!(Wrapping(200_u8).next_power_of_two(), Wrapping(0));
[01:46:52]    |                                                  ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
---
[01:46:52]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:46:52]
[01:46:52] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:46:52]
[01:46:52] ---- num/wrapping.rs - num::wrapping::Wrapping<u32>::is_power_of_two (line 684) stdout ----
[01:46:52]  error[E0432]: unresolved import `std::num::wrapping`
[01:46:52]  --> num/wrapping.rs:687:5
[01:46:52]   |
[01:46:52] 6 | use std::num::wrapping;
[01:46:52]   |     ^^^^^^^^^^^^^^^^^^ no `wrapping` in `num`. Did you mean to use `Wrapping`?
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:689:9
[01:46:52]   |
[01:46:52] 8 | assert!(Wrapping(16).is_power_of_two());
[01:46:52]   |         ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:690:10
[01:46:52]   |
[01:46:52] 9 | assert!(!Wrapping(10).is_power_of_two());
[01:46:52]   |          ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:691:10
[01:46:52]    |
[01:46:52] 10 | assert!(!Wrapping(0).is_power_of_two());
[01:46:52]    |          ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
---
[01:46:52]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:46:52]
[01:46:52] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:46:52]
[01:46:52] ---- num/wrapping.rs - num::wrapping::Wrapping<u32>::next_power_of_two (line 707) stdout ----
[01:46:52]  error[E0432]: unresolved import `std::num::wrapping`
[01:46:52]  --> num/wrapping.rs:710:5
[01:46:52]   |
[01:46:52] 6 | use std::num::wrapping;
[01:46:52]   |     ^^^^^^^^^^^^^^^^^^ no `wrapping` in `num`. Did you mean to use `Wrapping`?
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:712:12
[01:46:52]   |
[01:46:52] 8 | assert_eq!(Wrapping(2).next_power_of_two(), Wrapping(2));
[01:46:52]   |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:712:45
[01:46:52]   |
[01:46:52] 8 | assert_eq!(Wrapping(2).next_power_of_two(), Wrapping(2));
[01:46:52]   |                                             ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:713:12
[01:46:52]   |
[01:46:52] 9 | assert_eq!(Wrapping(3).next_power_of_two(), Wrapping(4));
[01:46:52]   |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:713:45
[01:46:52]   |
[01:46:52] 9 | assert_eq!(Wrapping(3).next_power_of_two(), Wrapping(4));
[01:46:52]   |                                             ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:714:12
[01:46:52]    |
[01:46:52] 10 | assert_eq!(Wrapping(200_u8).next_power_of_two(), Wrapping(0));
[01:46:52]    |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]    |
[01:46:52] 5  | use std::num::Wrapping;
[01:46:52]    |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:714:50
[01:46:52]    |
[01:46:52] 10 | assert_eq!(Wrapping(200_u8).next_power_of_two(), Wrapping(0));
[01:46:52]    |                                                  ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
---
[01:46:52]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:46:52]
[01:46:52] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:46:52]
[01:46:52] ---- num/wrapping.rs - num::wrapping::Wrapping<u64>::is_power_of_two (line 684) stdout ----
[01:46:52]  error[E0432]: unresolved import `std::num::wrapping`
[01:46:52]  --> num/wrapping.rs:687:5
[01:46:52]   |
[01:46:52] 6 | use std::num::wrapping;
[01:46:52]   |     ^^^^^^^^^^^^^^^^^^ no `wrapping` in `num`. Did you mean to use `Wrapping`?
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:689:9
[01:46:52]   |
[01:46:52] 8 | assert!(Wrapping(16).is_power_of_two());
[01:46:52]   |         ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:690:10
[01:46:52]   |
[01:46:52] 9 | assert!(!Wrapping(10).is_power_of_two());
[01:46:52]   |          ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:691:10
[01:46:52]    |
[01:46:52] 10 | assert!(!Wrapping(0).is_power_of_two());
[01:46:52]    |          ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
---
[01:46:52]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:46:52]
[01:46:52] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:46:52]
[01:46:52] ---- num/wrapping.rs - num::wrapping::Wrapping<u64>::next_power_of_two (line 707) stdout ----
[01:46:52]  error[E0432]: unresolved import `std::num::wrapping`
[01:46:52]  --> num/wrapping.rs:710:5
[01:46:52]   |
[01:46:52] 6 | use std::num::wrapping;
[01:46:52]   |     ^^^^^^^^^^^^^^^^^^ no `wrapping` in `num`. Did you mean to use `Wrapping`?
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:712:12
[01:46:52]   |
[01:46:52] 8 | assert_eq!(Wrapping(2).next_power_of_two(), Wrapping(2));
[01:46:52]   |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:712:45
[01:46:52]   |
[01:46:52] 8 | assert_eq!(Wrapping(2).next_power_of_two(), Wrapping(2));
[01:46:52]   |                                             ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:713:12
[01:46:52]   |
[01:46:52] 9 | assert_eq!(Wrapping(3).next_power_of_two(), Wrapping(4));
[01:46:52]   |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:713:45
[01:46:52]   |
[01:46:52] 9 | assert_eq!(Wrapping(3).next_power_of_two(), Wrapping(4));
[01:46:52]   |                                             ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:714:12
[01:46:52]    |
[01:46:52] 10 | assert_eq!(Wrapping(200_u8).next_power_of_two(), Wrapping(0));
[01:46:52]    |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]    |
[01:46:52] 5  | use std::num::Wrapping;
[01:46:52]    |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:714:50
[01:46:52]    |
[01:46:52] 10 | assert_eq!(Wrapping(200_u8).next_power_of_two(), Wrapping(0));
[01:46:52]    |                                                  ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
---
[01:46:52]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:46:52]
[01:46:52] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:46:52]
[01:46:52] ---- num/wrapping.rs - num::wrapping::Wrapping<u8>::is_power_of_two (line 684) stdout ----
[01:46:52]  error[E0432]: unresolved import `std::num::wrapping`
[01:46:52]  --> num/wrapping.rs:687:5
[01:46:52]   |
[01:46:52] 6 | use std::num::wrapping;
[01:46:52]   |     ^^^^^^^^^^^^^^^^^^ no `wrapping` in `num`. Did you mean to use `Wrapping`?
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:689:9
[01:46:52]   |
[01:46:52] 8 | assert!(Wrapping(16).is_power_of_two());
[01:46:52]   |         ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:690:10
[01:46:52]   |
[01:46:52] 9 | assert!(!Wrapping(10).is_power_of_nction `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:712:12
[01:46:52]   |
[01:46:52] 8 | assert_eq!(Wrapping(2).next_power_of_two(), Wrapping(2));
[01:46:52]   |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:712:45
[01:46:52]   |
[01:46:52] 8 | assert_eq!(Wrapping(2).next_power_of_two(), Wrapping(2));
[01:46:52]   |                                             ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:713:12
[01:46:52]   |
[01:46:52] 9 | assert_eq!(Wrapping(3).next_power_of_two(), Wrapping(4));
[01:46:52]   |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:713:45
[01:46:52]   |
[01:46:52] 9 | assert_eq!(Wrapping(3).next_power_of_two(), Wrapping(4));
[01:46:52]   |                                             ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:714:12
[01:46:52]    |
[01:46:52] 10 | assert_eq!(Wrapping(200_u8).next_power_of_two(), Wrapping(0));
[01:46:52]    |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]    |
[01:46:52] 5  | use std::num::Wrapping;
[01:46:52]    |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:714:50
[01:46:52]    |
[01:46:52] 10 | assert_eq!(Wrapping(200_u8).next_power_of_two(), Wrapping(0));
[01:46:52]    |                                                  ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
---
[01:46:52]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:46:52]
[01:46:52] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:46:52]
[01:46rror: unused import: `std::num::wrapping`
[01:46:52]  --> num/wrapping.rs:687:5
---
[01:46:52]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:46:52]
[01:46:52] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:46:52]
[01:46:52] ---- num/wrapping.rs - num::wrapping::Wrapping<usize>::next_power_of_two (line 707) stdout ----
[01:46:52]  error[E0432]: unresolved import `std::num::wrapping`
[01:46:52]  --> num/wrapping.rs:710:5
[01:46:52]   |
[01:46:52] 6 | use std::num::wrapping;
[01:46:52]   |     ^^^^^^^^^^^^^^^^^^ no `wrapping` in `num`. Did you mean to use `Wrapping`?
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:712:12
[01:46:52]   |
[01:46:52] 8 | assert_eq!(Wrapping(2).next_power_of_two(), Wrapping(2));
[01:46:52]   |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:712:45
[01:46:52]   |
[01:46:52] 8 | assert_eq!(Wrapping(2).next_power_of_two(), Wrapping(2));
[01:46:52]   |                                             ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:713:12
[01:46:52]   |
[01:46:52] 9 | assert_eq!(Wrapping(3).next_power_of_two(), Wrapping(4));
[01:46:52]   |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]  --> num/wrapping.rs:713:45
[01:46:52]   |
[01:46:52] 9 | assert_eq!(Wrapping(3).next_power_of_two(), Wrapping(4));
[01:46:52]   |                                             ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]   |
[01:46:52] 5 | use std::num::Wrapping;
[01:46:52]   |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/wrapping.rs:714:12
[01:46:52]    |
[01:46:52] 10 | assert_eq!(Wrapping(200_u8).next_power_of_two(), Wrapping(0));
[01:46:52]    |            ^^^^^^^^ not found in this scope
[01:46:52] help: possible candidate is found in another module, you can import it into scope
[01:46:52]    |
[01:46:52] 5  | use std::num::Wrapping;
[01:46:52]    |
[01:46:52]
[01:46:52] error[E0425]: cannot find function `Wrapping` in this scope
[01:46:52]   --> num/     num/wrapping.rs - num::wrapping::Wrapping<u64>::next_power_of_two (line 707)
---
[01:46:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:46:52] expected success, got: exit code: 101
[01:46:52]
[01:46:52]
[01:46:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:46:52] Build completed unsuccessfully in 0:49:26
[01:46:52] Makefile:58: recipe for target 'check' failed
[01:46:52] make: *** [check] Error 1
