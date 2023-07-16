 rust
use std::sync::Arc;
fn test() {
    let command = Arc::new(Box::new(|&:| {}));
    (*command)();
}
