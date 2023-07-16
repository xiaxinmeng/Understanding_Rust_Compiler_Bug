
error[E0382]: use of moved value: `foo`
   --> scratchpad.rs:148:14
    |
136 |     let mut foo = Some(Struct);
    |         ------- move occurs because `foo` has type `Option<Struct>`, which does not implement the `Copy` trait
137 |     let _x = foo.unwrap();
    |                  -------- `foo` moved due to this method call
...
148 |     let _y = foo; //~ ERROR use of moved value: `foo`
    |              ^^^ value used here after move
    |
note: these reinitializations might get skipped
   --> scratchpad.rs:141:9
    |
141 |         foo = Some(Struct);
    |         ^^^^^^^^^^^^^^^^^^
142 |     } else if 2 + 2 == 3 {
143 |         foo = Some(Struct);
    |         ^^^^^^^^^^^^^^^^^^
144 |     } else if 2 + 2 == 5 {
145 |         foo = Some(Struct);
    |         ^^^^^^^^^^^^^^^^^^
note: this function takes ownership of the receiver `self`, which moves `foo`
   --> /home/paul/projects/rust/library/core/src/option.rs:383:25
    |
383 |     pub const fn unwrap(self) -> T {
    |                         ^^^^
