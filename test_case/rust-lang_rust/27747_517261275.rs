rust
impl<T> [T] {
    pub fn concat<Item: ?Sized>(&self) -> <Self as Concat<Item>>::Output
        where Self: Concat<Item>
    {
        Concat::concat(self)
    }

    pub fn join<Separator>(&self, sep: Separator) -> <Self as Join<Separator>>::Output
        where Self: Join<Separator>
    {
        Join::join(self, sep)
    }

    pub fn connect<Separator>(&self, sep: Separator) -> <Self as Join<Separator>>::Output
        where Self: Join<Separator>
    {
        Join::join(self, sep)
    }
}
