
trait U {}
trait T<X: U> {}
trait S<Y> { fn m(x: ~T<Y>) {} }

fn main() {}
