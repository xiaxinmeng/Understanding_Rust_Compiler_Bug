rust
pub trait Constant: From<<Self as Constant>::Value> + Into<<Self as Constant>::Value> + Copy + Eq + Ord {
    type Value;
}
