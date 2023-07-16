rust
pub trait Tr { fn f() where Self: Eq, Self: Eq; } // `Self: Eq` not deduped
pub trait Super {}
pub trait Sub: Super { fn f() where Self: Super; } // `Self: Super` not deduped
