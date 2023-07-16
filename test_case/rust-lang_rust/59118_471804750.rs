rust
trait SelfWhere<One, Two> = Assoc<Two> where Assoc<Two>::Out: Assoc<One>;
