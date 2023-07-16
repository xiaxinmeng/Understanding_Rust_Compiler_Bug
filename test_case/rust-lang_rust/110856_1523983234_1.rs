
error[[E0502]](https://doc.rust-lang.org/nightly/error_codes/E0502.html): cannot borrow `x` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:13
  |
3 |     let z = &x;
  |             -- immutable borrow occurs here
4 |     let y = &mut x;
  |             ^^^^^^ mutable borrow occurs here
5 |     println!("{y}{z}");
  |                  --- immutable borrow later used here

note: erroneous constant used
 --> src/main.rs:5:14
  |
5 |     println!("{y}{z}");
  |              ^^^^^^^^
