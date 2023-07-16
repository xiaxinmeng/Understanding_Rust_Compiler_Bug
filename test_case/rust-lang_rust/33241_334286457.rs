
error: compilation successful
  --> src/main.rs:11:1
   |
11 | / fn main() { //~ ERROR compilation successful
12 | |     let t: &(u8, fmt::Debug) = any();
13 | |     println!("{:?}", &t.1);
14 | | }
   | |_^
