rust
pub struct X;

pub trait One<A> {}
pub trait Two<A, B>: One<A> + One<B> {}

impl<A, B> Two<A, B> for X
where
    X: One<A> + One<B>
{}

pub fn foo1<A, B>(_a: A, _b: B)
where
    X: Two<A, B>
{}

// Calls foo1 but we have a different order of types in the trait bound.
// However, the traits are defined so that the order technically does not
// matter: if one order is satisfied, then it's satisfied for all
// possible orderings. This is evidenced by the fact that it works with
// a different order than the trait ordering if we *explicitly* specify
// the types. However, without explicitly specifying types, type inference
// fails.
pub fn foo2<A, B>(a: A, b: B)
where
    // Note different order of parameters
    X: Two<B, A>,
{
    // Fails due to type inference
    foo1(a, b);
    
    // Works when types are explicitly named
    //foo1::<A, B>(a, b);
}
