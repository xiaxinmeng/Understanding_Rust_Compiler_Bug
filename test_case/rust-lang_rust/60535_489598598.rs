rust
async fn foo(ref mut x: A) { ... }

// desugaring (current master)
fn foo(ref x: A) -> ... { //~ ERROR temporary value dropped while borrowed [E0716]
    async move {
        let ref x = x;
        ...
    }
}

// desugaring (this PR)
fn foo(__arg0: A) -> ... {
    async move {
        let mut __arg0 = __arg0; // here always mutable binding
        let ref x = __arg0;
        ...
    }
}
