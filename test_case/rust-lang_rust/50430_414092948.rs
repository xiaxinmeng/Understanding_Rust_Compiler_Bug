
error[E0446]: private type `hardware::Hardware` in public interface
 --> 50430.rs:5:9
  |
5 | /         pub fn get_hardware() -> Hardware {
6 | |             Hardware { foo: 5 }
7 | |         }
  | |_________^ can't leak private type
