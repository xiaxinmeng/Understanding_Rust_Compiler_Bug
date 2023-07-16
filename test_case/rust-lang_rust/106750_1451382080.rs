rust
// #![feature(type_alias_impl_trait)]

pub fn compute1(data: impl AsRef<[u8]>) -> impl AsRef<[u8]> + 'static {
    [0u8; 4]
}

// type Output = impl AsRef<[u8]> + 'static;

// pub fn compute2(data: impl AsRef<[u8]>) -> Output {
//     [0u8; 4]
// }

pub fn foo() {
    let a: Vec<u8> = vec![0, 1, 2, 3];
    
    let b = compute1(a.as_slice()); // ERROR
    // let b = compute2(a.as_slice()); // ok

    drop(a);
    dbg!(b.as_ref());
}
