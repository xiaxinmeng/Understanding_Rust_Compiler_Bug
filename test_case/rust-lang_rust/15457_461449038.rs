
error[E0507]: cannot move out of borrowed content
 --> src/main.rs:7:9
6 |    fn method(&self) -> Option<Vec<u8>> {
  |              ----- `self` is borrowed here
7 |        self.option.map(|x| x)
  |        ^^^^^^^^^^^ --- `map` consumes `self.option` here, moving it
  |        |
  |        cannot move out of borrowed content
