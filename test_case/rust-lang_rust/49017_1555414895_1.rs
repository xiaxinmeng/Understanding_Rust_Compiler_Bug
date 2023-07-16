rust
pub trait Outer {
    type InnerType where Self::InnerType: Inner;
}
