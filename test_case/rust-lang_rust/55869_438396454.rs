rust
fn counter() -> impl Iterator<Item=i32> {
    let mut state = 0;
    std::iter::repeat(())
        .scan((), move |(), ()| {
            state += 1;
            if state < 6 {
                Some(state)
            } else {
                None
            }
        })
}
