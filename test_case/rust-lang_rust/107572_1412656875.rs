rust
trait GAT {
    //             >  < added a bound
    type Assoc<'a>: 'a;
}

fn foo<A: GAT>(f: impl for<'a> FnOnce(<A as GAT>::Assoc<'a>) -> <A as GAT>::Assoc<'a>) {}
