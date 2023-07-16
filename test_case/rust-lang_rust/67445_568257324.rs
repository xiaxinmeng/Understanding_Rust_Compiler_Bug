
trait Zone {
    type Alllocator;
    fn allocator() -> Self::Allocator where Self: Default {
        todo!("Zone::allocator() required because {} implements Default", type_name::<Self>())
    }
}
