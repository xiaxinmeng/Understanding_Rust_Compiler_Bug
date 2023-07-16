rs
impl<K> Unpin for Foo<K>
where
    Inner<K>: Unpin
