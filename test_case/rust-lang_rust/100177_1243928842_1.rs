rust
trait GenericTrait<T> {}

trait ParamToAssoc {
    type Assoc;
}

impl<T, G> ParamToAssoc for G where G: GenericTrait<T> {
    type Assoc = T;
}
