rust
async fn foo(mut x: A) { ... }

// desugaring (current master)
fn foo(x: A) -> ... { // removed mutability
    async move {
        let mut x = x;
        ...
    }
}

// Probably it should desugar like this
fn foo(__arg0: A) -> ... {
    async move {
        let mut x = __arg0;
        ...
    }
}
