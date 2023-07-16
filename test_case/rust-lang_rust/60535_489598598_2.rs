rust
async fn foo((mut x, y): (A, A)) { ... }

// desugaring (current master)
fn foo(__arg0: (A, A)) -> ... {
    async move {
        let __arg0 = __arg0;
        let (mut x, y) = __arg0; //~ ERROR cannot borrow `__arg0.0` as mutable, as `__arg0` is not declared as mutable [E0596]
        ...
    }
}

// desugaring (this PR), it is handled in the same way as by-ref binding.
fn foo(__arg0: A) -> ... {
    async move {
        let mut __arg0 = __arg0; // here always mutable binding
        let (mut x, y) = __arg0;
        ...
    }
}
