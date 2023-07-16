
error[E0382]: use of moved value
   --> scratchpad.rs:116:20
    |
116 |     while let Some(_foo) = val { //~ ERROR use of moved value
    |                    ^^^^ value moved here, in previous iteration of loop
117 |         if true {
118 |             val = None;
    |             ---------- this reinitialization might get skipped
    |
    = note: move occurs because value has type `Struct`, which does not implement the `Copy` trait

error[E0382]: use of moved value: `foo`
   --> scratchpad.rs:132:14
    |
126 |     let mut foo = Some(Struct);
    |         ------- move occurs because `foo` has type `Option<Struct>`, which does not implement the `Copy` trait
127 |     let _x = foo.unwrap();
    |                  -------- `foo` moved due to this method call
128 |     if true {
129 |         foo = Some(Struct);
    |         ------------------ this reinitialization might get skipped
...
132 |     let _y = foo; //~ ERROR use of moved value: `foo`
    |              ^^^ value used here after move
    |
note: this function takes ownership of the receiver `self`, which moves `foo`
   --> /home/paul/projects/rust/library/core/src/option.rs:383:25
    |
383 |     pub const fn unwrap(self) -> T {
    |                         ^^^^

