rust
pub struct GenericType<T>(T);

pub trait Trait1 {
    type Assoc1;
}

trait Trait2 {
    type Assoc2;
}

impl<T: Trait1> Trait2 for GenericType<T> {
    type Assoc2 = T::Assoc1;
}

trait Anything {}

trait MyIterator {
    type MyItem;
    fn my_nth() where Self::MyItem: Anything;
}

impl<T: Trait1> MyIterator for GenericType<T>
where
    <Self as Trait2>::Assoc2: MyIterator,
    // <T as Trait1>::Assoc1: MyIterator, // stop-gap fix
{
    type MyItem = <<Self as Trait2>::Assoc2 as MyIterator>::MyItem;
    fn my_nth() {}
}
