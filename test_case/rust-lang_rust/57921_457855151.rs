rust
#![register_tools(servo, another_tool)]

#[servo::must_root]
#[another_tool::another_attr]
fn foo() {}
