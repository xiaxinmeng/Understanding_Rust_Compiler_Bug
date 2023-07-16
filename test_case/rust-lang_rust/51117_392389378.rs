

error[E0499]: cannot borrow `*get` as mutable more than once at a time
  --> src/main.rs:12:17
   |
11 |             Some(mod_name) => {
   |                  -------- first mutable borrow occurs here
12 |                 get.take();
   |                 ^^^ second mutable borrow occurs here
13 |                 println!("{:?}", mod_name);
   |                                  -------- borrow later used here

