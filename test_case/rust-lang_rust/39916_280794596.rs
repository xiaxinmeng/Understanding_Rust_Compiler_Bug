Rust
fn fuzz_once<A: Arbitrary, F: FnOnce<A>>(target: F, data: &[u8])
        -> <F as FnOnce<A>>::Target {
    let mut g = FuzzGen::new(data);
    target.call_once(<A as Arbitrary>::arbitrary(&mut g))
}

fn fuzz_mut<A: Arbitrary, F: FnMut<A>>(target: &mut F, data: &[u8])
        -> <F as FnOnce<A>>::Target {
    let mut g = FuzzGen::new(data);
    target.call_mut(<A as Arbitrary>::arbitrary(&mut g))
}

fn fuzz<A: Arbitrary, F: Fn<A>>(target: &F, data: &[u8])
        -> <F as FnOnce<A>>::Target {
    let mut g = FuzzGen::new(data);
    target.call(<A as Arbitrary>::arbitrary(&mut g))
}
