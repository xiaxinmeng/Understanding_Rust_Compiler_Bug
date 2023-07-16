text
error[E0623]: lifetime mismatch
   |
LL |     fn b(self: &Box<Foo>, f: &Foo) -> &Foo { f }
   |                              ----     ----   ^ ...but data from `f` is returned here
   |                              |
   |                              this parameter and the return type are declared with different lifetimes...
