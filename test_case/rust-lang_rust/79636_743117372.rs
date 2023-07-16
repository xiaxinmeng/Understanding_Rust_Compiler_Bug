rust

fn join<A, M: Monad>(mmx: a!(M:<a!(M:<A>)>)) -> M::Of<A> {
    do_!{
        mx <- mmx;
        x <- mx;
        M::pure(x)
    }
}

// alternative version, using trivial conversion 
// between MT: HasTyCon<Param = T, GetTyCon = M>
//     and M::Of<T>
fn join_alternative<A, M: Monad>(mmx: a!(M:<a!(M:<A>)>)) -> M::Of<A> {
    do_!{
        mx <- mmx;
        mx.as_con_ty()
    }
}

fn main() {
    let xss = vec![vec![1,2,3],vec![],vec![4,5],vec![6]];
    // join can be used on concrete monadic types
    let xs = join(xss);
    print!("{:?}", xs); // [1,2,3,4,5,6]
    
}

// join can be used in a generic context
fn join_twice<A, M: Monad>(mmmx:  a!(M:<a!(M:<a!(M:<A>)>)>)) -> M::Of<A> {
    join(join(mmmx))
}
