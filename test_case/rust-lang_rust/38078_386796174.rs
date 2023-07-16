
trait OneTrait {
    type Ty: AnotherTrait<Out=Self::Oy>;
    type Oy;
    fn project() -> Self::Oy;
}

trait AnotherTrait {
    type Out;
}
