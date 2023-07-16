rust
trait SelfWhere<One, Two> = Assoc<Two> where <Self as Assoc<Two>>::Out: Assoc<One>;
