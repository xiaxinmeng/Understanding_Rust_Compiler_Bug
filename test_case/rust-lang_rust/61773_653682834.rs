rust
// All three can be disabled, ICE still triggers
// #![feature(type_alias_impl_trait)]  // Can be enabled or not, ICE will trigger
// #![feature(impl_trait_existential_types)]  // Can be enabled or not, ICE will trigger
// #![feature(trait_alias)]  // Can be enabled or not, ICE will trigger

struct S<T, I>(T, I);
trait UsedAlias<'a, T> = Iterator<Item = &'a UsedTypeAlias<T>>;
type UsedTypeAlias<T> = (usize, T); 
trait OtherTraitAlias<T> = where for<'r> &'r Self: IntoIterator<Item = UsedTypeAlias<T>>;
trait Trait<T> {
    type Associated: OtherTraitAlias<T>;
    fn into(self) -> Self::Associated;
}

// local definition, not used. Uncommenting this line will make compilation proceed to the next error.
// type Unused<T> = impl UsedAlias<T>;
impl<'a, T, I> Trait<T> for S<T, I> where T: Default, I: UsedAlias<T> {
    type Associated = S<T, impl UsedAlias<'a, T>>; // Line also necessary for the ICE
    fn into(self) -> Self::Associated { S(T::default(), vec![(0_usize, T::default())].iter()) }
}

fn other() {
    let _: i32 = 0_f64;
}
