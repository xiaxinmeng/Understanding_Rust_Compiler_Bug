Rust
pub trait CoerceSized<Target> {
    type SizeSource: ?Sized;
    type SizeTarget: Unsize<Self::SizeSource>;
}
