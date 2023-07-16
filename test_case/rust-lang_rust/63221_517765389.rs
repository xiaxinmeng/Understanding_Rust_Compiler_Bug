rust
#[inert]
mac! { struct S; }

=>

mac! { #[inert] struct S; }

=>

#[inert]
pub struct S;
impl S { ... }
