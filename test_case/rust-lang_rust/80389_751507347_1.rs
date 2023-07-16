
error[E0623]: lifetime mismatch
 --> src/lib.rs:4:5
  |
3 | pub fn test<'a, 'b>(map: &'a HashMap<&'static (), ()>, key: &'b ()) -> Option<&'a ()> {
  |                                                             ------     --------------
  |                                                             |
  |                                                             this parameter and the return type are declared with different lifetimes...
4 |     map.get(&key)
  |     ^^^^^^^^^^^^^ ...but data from `key` is returned here
