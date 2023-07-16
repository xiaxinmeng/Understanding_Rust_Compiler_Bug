
error[E0446]: restricted type `foo::hardware::Hardware` in public interface
  --> src/main.rs:6:9
   |
6  | /         pub fn get_hardware() -> Hardware {
7  | |             Hardware { foo: 5 }
8  | |         }
   | |_________^ can't leak restricted type
...
12 |       pub(in foo::hardware) struct Hardware {
   |       --------------------- `foo::hardware::Hardware` declared as restricted
