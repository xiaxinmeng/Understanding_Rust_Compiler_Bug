
error[E0391]: cycle detected when const-evaluating + checking `<impl at src/main.rs:4:1: 7:3>::{{constant}}#0`
 --> src/main.rs:6:18
  |
6 |     Self:FooImpl<{N==0}>
  |                  ^^^^^^
  |
note: ...which requires const-evaluating + checking `<impl at src/main.rs:4:1: 7:3>::{{constant}}#0`...
 --> src/main.rs:6:18
  |
6 |     Self:FooImpl<{N==0}>
  |                  ^^^^^^
note: ...which requires const-evaluating `<impl at src/main.rs:4:1: 7:3>::{{constant}}#0`...
 --> src/main.rs:6:18
  |
6 |     Self:FooImpl<{N==0}>
  |                  ^^^^^^
note: ...which requires type-checking `<impl at src/main.rs:4:1: 7:3>::{{constant}}#0`...
 --> src/main.rs:6:18
  |
6 |     Self:FooImpl<{N==0}>
  |                  ^^^^^^
note: ...which requires processing `<impl at src/main.rs:4:1: 7:3>::{{constant}}#0`...
 --> src/main.rs:6:18
  |
6 |     Self:FooImpl<{N==0}>
  |                  ^^^^^^
  = note: ...which again requires const-evaluating + checking `<impl at src/main.rs:4:1: 7:3>::{{constant}}#0`, completing the cycle
note: cycle used when processing `<impl at src/main.rs:4:1: 7:3>`
 --> src/main.rs:4:1
  |
4 | / impl<const N: usize> Foo for [(); N]
5 | | where
6 | |     Self:FooImpl<{N==0}>
7 | | {}
  | |__^

error: aborting due to previous error
