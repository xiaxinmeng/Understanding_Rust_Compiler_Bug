
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'id` due to conflicting requirements
  --> src/compiletest/lcell-02.rs:10:33
   |
10 |             let c1ref1 = owner1.ro(&c1);
   |                                 ^^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined here...
  --> src/compiletest/lcell-02.rs:8:27
   |
8  |           LCellOwner::scope(|mut owner2| {
   |  ___________________________^
9  | |             let c1 = Rc::new(LCell::new(100u32));
10 | |             let c1ref1 = owner1.ro(&c1);
11 | |             let c1ref2 = owner2.ro(&c1);   // Compile error
12 | |             println!("{}", *c1ref2);
13 | |         });
   | |_________^
note: ...but the lifetime must also be valid for the anonymous lifetime #1 defined here...
  --> src/compiletest/lcell-02.rs:7:23
   |
7  |       LCellOwner::scope(|mut owner1| {
   |  _______________________^
8  | |         LCellOwner::scope(|mut owner2| {
9  | |             let c1 = Rc::new(LCell::new(100u32));
10 | |             let c1ref1 = owner1.ro(&c1);
...  |
13 | |         });
14 | |     });
   | |_____^
note: ...so that the types are compatible
  --> src/compiletest/lcell-02.rs:11:33
   |
11 |             let c1ref2 = owner2.ro(&c1);   // Compile error
   |                                 ^^
   = note: expected `&LCellOwner<'_>`
              found `&LCellOwner<'_>`
