rs
pub struct Inner<T>(T);

trait MyTrait {
    type MyItem;
}

trait OtherTrait {}

unsafe impl<T> Sync for Inner<T>
where
    T: MyTrait<MyItem = bool>,
    <T as MyTrait>::MyItem: OtherTrait,
{
}

pub struct Foo<K> {
    inner_field: Inner<K>,
}
