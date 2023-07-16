
error[[E0382]](https://doc.rust-lang.org/stable/error-index.html#E0382): use of moved value: `cs`
  --> src/main.rs:13:21
   |
5  |     let mut cs = S{};
   |         ------ move occurs because `cs` has type `S`, which does not implement the `Copy` trait
...
9  |             cs = S{};
   |             -------- this reinitialization might get skipped
...
13 |             let t = cs;
   |                     ^^ value moved here, in previous iteration of loop

For more information about this error, try `rustc --explain E0382`.
warning: `playground` (bin "playground") generated 2 warnings
error: could not compile `playground` due to previous error; 2 warnings emitted
Standard Output
