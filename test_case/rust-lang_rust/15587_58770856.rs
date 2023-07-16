 rust
{
    use std::slice::BoxedSlice;
    use std::boxed::HEAP;
    let xs = box (HEAP) [$SOMETHING];
    xs.into_vec()
}
