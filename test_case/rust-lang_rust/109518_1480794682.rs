
struct A;

pub trait TestTrait<T> {}

impl TestTrait<&Series> for A {}

impl <T> TestTrait<T> for A
where 
    T: NumericNative,
{}
