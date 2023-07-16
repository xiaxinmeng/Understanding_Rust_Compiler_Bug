
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
  --> src/main.rs:17:9
   |
16 |         let instruction = self.fetch();
   |                           ---- mutable borrow occurs here
17 |         self.execute(instruction);
   |         ^^^^         ----------- mutable borrow later used here
   |         |
   |         immutable borrow occurs here
