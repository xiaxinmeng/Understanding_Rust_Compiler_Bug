rust
enum List<T>
where
    // In where clauses, both on the side of types being constrained and on the bounds side.
    Self: PartialOrd<Self>
{
    Nil,
    Cons(T, Box<Self>) // In fields. The usual rules re. infinite types apply.
}
