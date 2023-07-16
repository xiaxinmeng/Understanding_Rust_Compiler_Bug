rust
type T = u8;
pub struct T1<A, B, C, D, E, F, G, H>(A, B, C, D, E, F, G, H);

impl<A, B, C, D, E, F, G, H> PartialEq for T1<A, B, C, D, E, F, G, H>
where 
A: PartialEq, B: PartialEq, C: PartialEq, D: PartialEq, E: PartialEq, F: PartialEq, G: PartialEq, H: PartialEq
{
    default fn eq(&self, b:&Self)->bool{
        let a = self;
        a.0 == b.0 &&
        a.1 == b.1 &&
        a.2 == b.2 &&
        a.3 == b.3 &&
        a.4 == b.4 &&
        a.5 == b.5 &&
        a.6 == b.6 &&
        a.7 == b.7
    }
}

impl<A: PartialEq> PartialEq for T1<A,A,A,A,A,A,A,A>
{
    fn eq(&self, b:&Self)->bool{
        assert_eq!(
            std::mem::size_of::<Self>(),
            std::mem::size_of::<[A;8]>()
        );
        unsafe{
            let a: &[A;8] = std::mem::transmute(self);
            let b: &[A;8] = std::mem::transmute(b);
            a == b
        }
    }
}

#[no_mangle]
pub fn is_eq0(a: T1<T, T, T, T, T, T, T, T>, 
            b: T1<T, T, T, T, T, T, T, T>)->bool{
    a==b
}
