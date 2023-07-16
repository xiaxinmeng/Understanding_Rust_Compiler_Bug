rust
trait CollectionFamily {
    type Member<T>;
}
fn floatify() {
    Box::new(Family) as &dyn CollectionFamily<Member=usize>
}

struct Family;
