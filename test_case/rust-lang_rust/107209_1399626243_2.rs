rust
trait ExampleTrait {
    type AssociatedType;
}

impl<T, const N: usize> ExampleTrait for [T; N]
{
    type AssociatedType = T;
}

pub struct Happy<T> {
    arr: [T; MYSIZE],
}

impl<T> ExampleTrait for Happy<T>
where
    [T; 10]: ExampleTrait,
    <[T; 10] as ExampleTrait>::AssociatedType: Clone,
    [T; 10]: ExampleTrait,
    <[T; 10] as ExampleTrait>::AssociatedType: Clone,
{
    type AssociatedType = T;
}

pub const MYSIZE: usize = 10;
pub struct Sad<T> {
    arr: [T; MYSIZE],
}

impl<T> ExampleTrait for Sad<T>
where
    [T; MYSIZE]: ExampleTrait,
    <[T; MYSIZE] as ExampleTrait>::AssociatedType: Clone,
    [T; MYSIZE]: ExampleTrait, // 💣
    <[T; MYSIZE] as ExampleTrait>::AssociatedType: Clone,
{
    type AssociatedType = T;
}
