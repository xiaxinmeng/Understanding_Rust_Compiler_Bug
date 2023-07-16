rust
   Compiling playground v0.0.1 (/playground)
warning[E0506]: cannot assign to `greeting` because it is borrowed
  --> src/main.rs:28:5
   |
26 |     let res = scope(|s| s.spawn(|| &greeting));
   |                     ---             -------- borrow occurs due to use in closure
   |                     |
   |                     borrow of `greeting` occurs here
27 | 
28 |     greeting = "DEALLOCATED".to_string();
   |     ^^^^^^^^ assignment to borrowed `greeting` occurs here
...
31 |     println!("thread result: {:?}", res);
   |                                     --- borrow later used here
   |
   = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
           It represents potential unsoundness in your code.
           This warning will become a hard error in the future.

warning[E0505]: cannot move out of `greeting` because it is borrowed
  --> src/main.rs:29:10
   |
26 |     let res = scope(|s| s.spawn(|| &greeting));
   |                     ---             -------- borrow occurs due to use in closure
   |                     |
   |                     borrow of `greeting` occurs here
...
29 |     drop(greeting);
   |          ^^^^^^^^ move out of `greeting` occurs here
30 | 
31 |     println!("thread result: {:?}", res);
   |                                     --- borrow later used here
   |
   = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
           It represents potential unsoundness in your code.
           This warning will become a hard error in the future.

    Finished release [optimized] target(s) in 1.13s
     Running `target/release/playground`
