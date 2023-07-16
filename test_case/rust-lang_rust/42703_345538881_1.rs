
5 | fn foo<'a>(&self, x: &'a i32) -> &i32 
  |                      -------     ----
  |                      |
  |                      this parameter and the return type are declared with different lifetimes...
6 |         if true {x} else {&self}
  |                  ^ ...but data from `x` is returned here
