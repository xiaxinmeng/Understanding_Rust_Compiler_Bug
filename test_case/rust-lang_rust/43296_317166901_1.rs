rust
trait SomeTrait {
    type H: Hasher;
    fn do_the_thing(&self) -> Result<Self::H, DoTheThingError>;

    // more items omitted
}
