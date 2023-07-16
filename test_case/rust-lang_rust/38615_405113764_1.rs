
error: cannot infer an appropriate lifetime
  --> src/main.rs:12:9
   |
10 |     fn gen(& mut self) -> impl Generator<Yield=i32,Return=()> { 
   |                           ----------------------------------- this return type evaluates to the `'static` lifetime...
11 |  // fn gen<'a>(&'a mut self) -> impl Generator<Yield=i32,Return=()> + 'a { 
12 |         move || { for _i in 0..10 { self.x = self.x + 1; yield self.x; } return (); } 
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...but this borrow...
   |
note: ...can't outlive the anonymous lifetime #1 defined on the method body at 10:5
  --> src/main.rs:10:5
   |
10 | /     fn gen(& mut self) -> impl Generator<Yield=i32,Return=()> { 
11 | |  // fn gen<'a>(&'a mut self) -> impl Generator<Yield=i32,Return=()> + 'a { 
12 | |         move || { for _i in 0..10 { self.x = self.x + 1; yield self.x; } return (); } 
13 | |     }
   | |_____^
help: you can add a constraint to the return type to make it last less than `'static` and match the anonymous lifetime #1 defined on the method body at 10:5
   |
10 |     fn gen(& mut self) -> impl Generator<Yield=i32,Return=()> + '_ { 
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
