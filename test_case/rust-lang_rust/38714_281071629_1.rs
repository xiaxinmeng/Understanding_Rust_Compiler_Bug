
rustc 1.15.1 (021bd294c 2017-02-08)
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in function call due to conflicting requirements
  --> <anon>:15:27
   |
15 |             UsizeRef{ a: &vec[0] }
   |                           ^^^^^^
   |
help: consider using an explicit lifetime parameter as shown: fn main()
  --> <anon>:12:1
   |
12 |   fn main() {
   |  _^ starting here...
13 | |     //Does not compile
14 | |     let a: RefTo = Box::new(|vec: &Vec<usize>| {
15 | |             UsizeRef{ a: &vec[0] }
16 | |         }
17 | |     );
18 | | }
   | |_^ ...ending here

error: aborting due to previous error
