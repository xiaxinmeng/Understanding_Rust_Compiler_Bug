plain
2019-11-12T21:26:03.8103783Z    Compiling panic_unwind v0.0.0 (D:\a\1\s\src\libpanic_unwind)
2019-11-12T21:26:03.8103902Z error: variable does not need to be mutable
2019-11-12T21:26:03.8104178Z    --> src\libpanic_unwind\seh.rs:225:9
2019-11-12T21:26:03.8104263Z     |
2019-11-12T21:26:03.8104449Z 225 |     let mut ptrs_ptr = ptrs.as_mut_ptr();
2019-11-12T21:26:03.8104869Z     |         |
2019-11-12T21:26:03.8105018Z     |         help: remove this `mut`
2019-11-12T21:26:03.8105114Z     |
2019-11-12T21:26:03.8105236Z     = note: `-D unused-mut` implied by `-D warnings`
---
2019-11-12T21:26:03.8107113Z   local time: Tue Nov 12 21:26:03 CUT 2019
2019-11-12T21:26:03.8107195Z   network time: Tue, 12 Nov 2019 21:26:03 GMT
2019-11-12T21:26:03.8107287Z == end clock drift check ==
2019-11-12T21:26:03.8107335Z 
2019-11-12T21:26:03.8741721Z ##[error]Bash exited with code '1'.
2019-11-12T21:26:03.8877452Z ##[section]Starting: Checkout
2019-11-12T21:26:03.8991456Z ==============================================================================
2019-11-12T21:26:03.8991570Z Task         : Get sources
2019-11-12T21:26:03.8991654Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
