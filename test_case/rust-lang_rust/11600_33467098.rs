
 gdb prog1 
GNU gdb (GDB) 7.6.1
...
Reading symbols from /home/asamoilov/work/projects/rust-sandbox/my-rust/prog1...done.
(gdb) l
1   // **********************************************
2   //prog1.rs <http://prog1.rs>
3   use std::rand::{task_rng, Rng};
4   fn main() {
5       let names = ["Alice", "Bob", "Carol"];
6       for name in names.iter() {
7           let v = task_rng().shuffle(~[1,2,3]);
8           for num in v.iter() {
9               println!("{:s} says: {:d}", *name, *num);
10          }
