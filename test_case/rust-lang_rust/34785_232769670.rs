 rust
mod std { mod intrinsics { extern "rust-intrinsic" { fn discriminant<I, O>(v: &I) -> O } } }

#[derive(HasDiscriminant)]
enum E { ... } 

// expands to
// unsafe impl ::std::mem::HasDiscriminant for E {
//     type T = <whatever is appropriate, but how>;
//     unsafe fn discriminant(&self) -> Self::T { ::std::intrinsics::discriminant(self) }
// }
