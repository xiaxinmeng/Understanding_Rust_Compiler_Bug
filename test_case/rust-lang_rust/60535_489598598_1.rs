rust
async fn foo(mut x: A) { ... }

// desugaring (current master)
fn foo(mut x: A) -> ... { //~ WARNING variable does not need to be mutable
    async move {
        let mut x = x;
        ...
    }
}

// desugaring (this PR)
fn foo(x: A) -> ... { // removed mutability
    async move {
        let mut x = x;
        ...
    }
}
