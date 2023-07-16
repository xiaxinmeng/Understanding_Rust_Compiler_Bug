rust
#[must_use]
struct S { ... }

impl Trait for S { ... }

fn foo() -> impl Trait {
}
