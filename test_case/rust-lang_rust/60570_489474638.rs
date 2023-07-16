rust
pub fn zip<A, B, IA, IB>(a: IA, b: IB) -> impl Iterator<Item=(A, B)>
where
    IA: IntoIterator<Item=A>,
    IB: IntoIterator<Item=B>,
{ ... }
