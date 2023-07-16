rust
trait Trait {
    type Ty;
}
impl<'a, 'b> Trait for &'a &'b () {
    type Ty = () where 'b: 'a;
}
