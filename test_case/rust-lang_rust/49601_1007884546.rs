rust
pub trait Helper<Other>: IntoIterator<Item=<Self as Helper<Other>>::Item> {
    type Item: MyTrait<T = Other>;
}

impl<U, Other> Helper<Other> for U where U: IntoIterator, U::Item: MyTrait<T = Other> {
    type Item = <Self as IntoIterator>::Item;
}

pub fn foo<T>(_: T)
where
    for<'x> &'x T: Helper<&'x i32>
{}
