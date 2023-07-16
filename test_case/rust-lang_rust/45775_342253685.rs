rust
macro_rules! m {
    ($pat: pat, $ty: ty) => {
        trait Tr {
            fn f($pat: $ty); // changed to have no body so #35203 is triggered
        }
    }
}

m!(x, u8); // OK
m!(mut x, u8); // warning #35203
m!(&x, &u8); // error
m!((x, y), (u8, u8)): // error
