rust
fn foo<const N: usize>() {
    for chunk in get_slice().array_chunks() {
        //                   ^^^^^^^^^^^^ error: can't prove that N != 0
        do_whatever(chunk);
    }
}

fn main() {
    foo::<3>();
}
