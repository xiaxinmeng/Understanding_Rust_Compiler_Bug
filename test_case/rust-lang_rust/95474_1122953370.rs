rust
trait Mirror<'a> {
    type Item;
}

impl<'a, T> Mirror<'a> for T {
    type Item = T;
}

trait AnotherTrait { 
    type Blah;
}

impl<'a> AnotherTrait for <u32 as Mirror<'a>>::Item {
    type Blah = &'a u32;
}
