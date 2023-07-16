rust
trait Trait<T> {
    type Assoc;
}

trait Other {
    type Bar;
}

impl<T> Other for T {
    type Bar = [T; 1 + 2];
}

fn foo<T, U>() 
where
    <U as Trait<[T; 1 + 2]>>::Assoc: Sized,
    U: Trait<<T as Other>::Bar>,
{

}
