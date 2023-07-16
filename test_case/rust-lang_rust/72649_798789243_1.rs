rust
error[E0382]: use of moved value: `value`
 --> src/lib.rs:5:20
  |
4 |         let value: NonCopy;
  |             ----- move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
5 |         let used = value;
  |                    ^^^^^ value moved here, in previous iteration of loop
  
