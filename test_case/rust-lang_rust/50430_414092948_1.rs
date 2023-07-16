
error[E0446]: private type `hardware::Hardware` in public interface
  --> src/main.rs:5:9
   |
5  | /         pub fn get_hardware() -> Hardware {
6  | |             Hardware { foo: 5 }
7  | |         }
   | |_________^ can't leak private type
...
11 |       struct Hardware {
   |       - `hardware::Hardware` declared as private
