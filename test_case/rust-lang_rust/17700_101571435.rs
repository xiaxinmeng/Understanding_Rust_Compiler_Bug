
17700.rs:6:3: 8:4 error: method `bar` has an incompatible type for trait: expected type parameter, found a different type parameter [E0053]
17700.rs:6   fn bar<B>(&self, b: B) -> Option<B> {
17700.rs:7     Some(b)
17700.rs:8   }
error: aborting due to previous error
