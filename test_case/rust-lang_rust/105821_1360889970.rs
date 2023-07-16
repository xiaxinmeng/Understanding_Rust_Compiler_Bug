rs
#![allow(incomplete_features)]
#![feature(adt_const_params, generic_const_exprs)]

const fn str_to_arr<const FOO: &'static str>() -> [u8; FOO.len()] {
    let mut out = [0; FOO.len()];
    let bytes = FOO.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        out[i] = bytes[i];
        i += 1;
    }
    out
}

const fn concat_strs<const A: &'static str, const B: &'static str>() -> &'static str
where
    [(); A.len()]:,
    [(); B.len()]:,
    [(); A.len() + B.len()]:,
{
    struct Inner<const A: &'static str, const B: &'static str>;
    impl<const A: &'static str, const B: &'static str> Inner<A, B>
    where
        [(); A.len()]:,
        [(); B.len()]:,
        [(); A.len() + B.len()]:,
    {
        const ARRA: [u8; A.len()] = str_to_arr::<A>();
        const ARRB: [u8; B.len()] = str_to_arr::<B>();
        const ARRJOIN: [u8; A.len() + B.len()] = {
            let mut out = [0; A.len() + B.len()];
            let mut i = 0;
            while i < A.len() {
                out[i] = Self::ARRA[i];
                i += 1;
            }
            while i - A.len() < B.len() {
                out[i] = Self::ARRB[i - A.len()];
                i += 1;
            }
            out
        };
        const STRJOIN: &str = unsafe { std::str::from_utf8_unchecked(&Self::ARRJOIN) };
    }
    
    Inner::<A, B>::STRJOIN
}

const TESTL: &str = "Hello, ";
const TESTR: &str = "world!";
const TESTJOIN: &str = concat_strs::<TESTL, TESTR>();

fn main() {
    println!("'{TESTL}'");
    println!("'{TESTR}'");
    println!("'{TESTJOIN}'");
}
