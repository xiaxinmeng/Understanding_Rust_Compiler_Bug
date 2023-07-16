
trait TraitA<Type> {
    fn other(&self) -> Type;
}

trait TraitB<Type>: TraitA<Type>
    where Type: TraitB<Type>
{
    fn stuff(&self) {
        self.other().stuff()
    }
}
