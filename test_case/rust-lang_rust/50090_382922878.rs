rust
// Example 1:
if let S { x: _x, y: 2 } = (S { x: 1, y: 2 }) { println!("Ok"); }
// Example 2:
if (T {}) == (T {}) { println!("Ok"); }
