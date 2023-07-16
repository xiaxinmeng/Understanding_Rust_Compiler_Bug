 rust
// added to prelude
trait Sum {
    type Output;
    fn sum(self) -> Output;
}

// or 

trait Sum {
    type Output;
    fn sum<I: Iterator>(i: I) -> Output;
}

trait Iterator {
    fn sum<S: Sum>(self) -> S::Output where SomeCrazyWhereClause { 
        S::sum(self) 
    }
}
