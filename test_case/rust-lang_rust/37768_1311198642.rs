
error[E0521]: borrowed data escapes outside of associated function
  --> src/lib.rs:10:9
   |
9  |       pub fn foo(&self) {
   |                  -----
   |                  |
   |                  `self` is a reference that is only valid in the associated function body
   |                  let's call the lifetime of this reference `'1`
10 | /         thread::spawn(move || {
11 | |             println!("something {}",&self.a);
12 | |         });
   | |          ^
   | |          |
   | |__________`self` escapes the associated function body here
   |            argument requires that `'1` must outlive `'static`
