
fn select2<A, AP: SelectPort<A>, B, BP: SelectPort<B>>(AP, BP) -> Either<(A, BP), (AP, B)>
