
error[E0382]: use of moved value: `scary`
  --> src/main.rs:15:18
   |                      
15 |     creep(scary, scary);
   |           -----  ^^^^^ value used here after move
   |           | 
   |           value moved here
   |
   = note: move occurs because `scary` has type `SpookyThing<NotCopyable>`, which does not implement the `Copy` trait
