rust
type T = u8;

const L: usize = 4096;

#[bench]
fn std_vec(b: &mut Bencher) {
    let mut vec = Vec::<T>::with_capacity(L);

    b.iter(|| unsafe {
        for i in 0..L {
            vec.push_within_capacity(i as _).unwrap();
        }

        vec.set_len(0);
    });
}

#[bench]
fn const_vec(b: &mut Bencher) {
    static mut VEC: const_collections::vec::Vec<T, L> = const_collections::vec::Vec::new();

    b.iter(|| unsafe {
        for i in 0..L {
            VEC.push(i as _).unwrap();
        }

        VEC.set_len(0);
    });
}
