rust
pub struct MyThing {
  pub x: i32,
  pub y: i32,
  pub z: i32,
  pub name: String,
  ... many fields ...
}

impl const Default for MyThing { ... }

pub const EMPTY: MyThing = MyThing::default()
