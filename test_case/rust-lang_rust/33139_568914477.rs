rust
fn assert_sync<T: Sync>(_t: T) {}

fn main() {
    let f: &(dyn for<'a> Sync + Fn(&'a u32) -> &'a u32) = &|x| x;
    assert_sync(f);
}

