
trait Float: Num, FuzzyEq {}

trait Integer: Num {}

trait Signed: Num, Neg<self> {}

trait Unsigned: Num {}
