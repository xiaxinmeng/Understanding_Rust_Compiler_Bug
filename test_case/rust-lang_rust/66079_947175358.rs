rust
#![register_tool(my_tooll)]  // An silly mistake.

#[my_tooll::check_something]
fn target_function() {
  // ...
}
