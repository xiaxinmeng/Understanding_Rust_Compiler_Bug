rust
trait Trait {
    type Item;
    
    fn do_work(&self) -> &Self::Item;
}

trait Control {
    type Assoc;
    
    fn convert(&self) -> &Self::Assoc;
}

// This one doesn't work
impl<T, D> Trait for D
where
    D: Control<Assoc = T>,
    T: Trait,
{
    type Item = T::Item;

    fn do_work(&self) -> &Self::Item {
        self.convert().do_work()
    }
}

// This one works
impl<D> Trait for D
where
    D: Control,
    D::Assoc: Trait,
{
    type Item = <D::Assoc as Trait>::Item;

    fn do_work(&self) -> &Self::Item {
        self.convert().do_work()
    }
}
