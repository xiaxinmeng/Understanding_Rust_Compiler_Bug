 rust
pub enum Foo { A1, A2, A3, }
#[automatically_derived]
impl ::std::cmp::PartialEq for Foo {
    #[inline]
    fn eq(&self, __arg_0: &Foo) -> ::bool {
        match (&*self, &*__arg_0) {
            (&A1, &A1) => true,
            (&A2, &A2) => true,
            (&A3, &A3) => true,
            _ => {
                let __self_vi =
                    match *self { A1(..) => 0u, A2(..) => 1u, A3(..) => 2u, };
                let __arg_1_vi =
                    match *__arg_0 {
                        A1(..) => 0u,
                        A2(..) => 1u,
                        A3(..) => 2u,
                    };
                false
            }
        }
    }
    #[inline]
    fn ne(&self, __arg_0: &Foo) -> ::bool {
        match (&*self, &*__arg_0) {
            (&A1, &A1) => false,
            (&A2, &A2) => false,
            (&A3, &A3) => false,
            _ => {
                let __self_vi =
                    match *self { A1(..) => 0u, A2(..) => 1u, A3(..) => 2u, };
                let __arg_1_vi =
                    match *__arg_0 {
                        A1(..) => 0u,
                        A2(..) => 1u,
                        A3(..) => 2u,
                    };
                true
            }
        }
    }
}
