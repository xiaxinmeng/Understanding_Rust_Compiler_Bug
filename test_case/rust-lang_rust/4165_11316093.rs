
pub trait Number: NumConv {
    static pure fn from<T:NumConv>(n: T) -> self;
}
