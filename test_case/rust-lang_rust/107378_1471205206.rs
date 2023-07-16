rust
async fn does_not_compile(mut iter: impl Iterator<Item = &()>) -> &() {
    iter.next().unwrap()
}

async fn compiles<'a>(mut iter: impl Iterator<Item = &'a ()>) -> &'a () {
    iter.next().unwrap()
}

async fn wrong_error(iter: &mut impl Iterator<Item = &()>) -> &() {
    iter.next().unwrap()
}
