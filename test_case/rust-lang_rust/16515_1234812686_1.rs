
error: Undefined Behavior: not granting access to tag <3104> because incompatible item [Unique for <3105>] is protected by call 944
  --> src/main.rs:6:1
   |
6  | fn noalias(_: &mut u32, _: &mut u32) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not granting access to tag <3104> because incompatible item [Unique for <3105>] is protected by call 944
   |
   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
help: <3104> was created by a SharedReadWrite retag at offsets [0x0..0x4]
  --> src/main.rs:10:17
   |
10 |     noalias(r1, r2);
   |                 ^^
help: <3104> was protected due to <3095> which was created here
  --> src/main.rs:10:13
   |
10 |     noalias(r1, r2);
   |             ^^
help: this protector is live for this call
  --> src/main.rs:6:1
   |
6  | fn noalias(_: &mut u32, _: &mut u32) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: backtrace:
   = note: inside `noalias` at src/main.rs:6:1
note: inside `use_foo` at src/main.rs:10:5
  --> src/main.rs:10:5
   |
10 |     noalias(r1, r2);
   |     ^^^^^^^^^^^^^^^
note: inside `main` at src/main.rs:18:9
  --> src/main.rs:18:9
   |
18 |         use_foo(std::mem::transmute(&mut pair));
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
