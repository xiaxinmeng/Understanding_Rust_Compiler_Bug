
warning[E0505]: cannot move out of `_` because it is borrowed
  --> src\main.rs:18:13
   |
15 |     fn get_mut(&mut self, id: usize) -> Result<&mut u8, Option<&mut State>> {
   |                - let's call the lifetime of this reference `'1`
16 |         match self.vec.get_mut(id) {
17 |             Some(State { ref mut val, ref xy }) if true => Ok(val),
   |                          -----------                       ------- returning this value requires that borrow lasts for `'1`
   |                          |
   |                          borrow of value occurs here
18 |             other => Err(other),
   |             ^^^^^ move out of value occurs here
   |
   = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
   = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
