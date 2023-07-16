text
--- Standard Error ---

   Compiling playground v0.0.1 (/playground)
warning[E0597]: `obj` does not live long enough
  --> src/main.rs:19:16
   |
19 |         filter(&obj);
   |         ------ ^^^^ borrowed value does not live long enough
   |         |
   |         borrow later used here
20 |         println!("{:?}", obj); // okay
21 |     }
   |     - `obj` dropped here while still borrowed
   |
   = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
   = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future

    Finished dev [unoptimized + debuginfo] target(s) in 1.48s
     Running `target/debug/playground`

--- Standard Output ---

start
Some("Hello")
Some("world")
Some("!")
finish
