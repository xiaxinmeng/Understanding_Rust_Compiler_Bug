rs
fn my_iter<A, I: Iterator<Item=A>>(iter: I) -> impl Iterator<Item=A> {
    gen_to_iter(move || {
        for value in iter {
            yield value;
        }
    })
}
