 rust
trait Expr : PartialEq<<Self as Expr>::Item> {
    //^ error: illegal recursive type; insert an enum or struct in the cycle, if this is desired
    type Item = Expr;
}

fn main() {}
