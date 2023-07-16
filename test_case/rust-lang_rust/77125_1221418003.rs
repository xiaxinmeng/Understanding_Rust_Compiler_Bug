rust
trait ConstTypeId {
    const ID: TypeId;
}

impl<T: 'static> ConstTypeId for T {
    const ID: TypeId = TypeId::of::<T>();
}

match foo.type_id {
    u16::ID | u32::ID | u64::ID => do_foo(&foo.data),
    Foo::ID | Bar::ID | Baz::ID => do_bar(&foo.data),
    _ => do_default(&foo.data),
}
