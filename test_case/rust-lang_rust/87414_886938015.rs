rust
fn foo() -> [(); { |x: u32| { x }; 4 }] { todo!() }
fn bar() { let _: [(); { |x: u32| { x }; 4 }]; }
