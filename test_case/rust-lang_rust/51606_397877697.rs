plain
[01:18:05] ....................................................................................................
[01:18:20] ....................................................................................................
[01:18:36] ....................................................................................................
[01:18:50] ....................................................................................................
[01:19:10] ..........................F.........................................................................
[01:19:27] failures:
[01:19:27] 
[01:19:27] ---- slice/mod.rs - slice::[T]::group_by_mut (line 848) stdout ----
[01:19:27] error[E0308]: mismatched types
[01:19:27] error[E0308]: mismatched types
[01:19:27]   --> slice/mod.rs:855:1
[01:19:27]    |
[01:19:27] 10 | assert_eq!(iter.next(), Some(&[1, 1, 1][..]));
[01:19:27]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
[01:19:27]    |
[01:19:27]    = note: expected type `std::option::Option<&mut [{integer}]>`
[01:19:27]               found type `std::option::Option<&[{integer}]>`
[01:19:27] 
[01:19:27] error[E0308]: mismatched types
[01:19:27]   --> slice/mod.rs:856:1
[01:19:27]    |
[01:19:27]    |
[01:19:27] 11 | assert_eq!(iter.next(), Some(&[3, 3][..]));
[01:19:27]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
[01:19:27]    |
[01:19:27]    = note: expected type `std::option::Option<&mut [{integer}]>`
[01:19:27]               found type `std::option::Option<&[{integer}]>`
[01:19:27] 
[01:19:27] error[E0308]: mismatched types
[01:19:27]   --> slice/mod.rs:857:1
[01:19:27]    |
[01:19:27]    |
[01:19:27] 12 | assert_eq!(iter.next(), Some(&[2, 2, 2][..]));
[01:19:27]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
[01:19:/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
59904 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
59488 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
56708 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
56704 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release
