
#[deriving(IterBytes)]
enum Test<T> { Zero, One(T), Two(T), }
#[doc = "Automatically derived."]
pub impl <T: ::std::to_bytes::IterBytes> ::std::to_bytes::IterBytes for
         Test<T> {
    fn iter_bytes(&self, __arg_0: ::bool, __arg_1: ::std::to_bytes::Cb) ->
     ::bool {
        match *self {
            Zero => 0u.iter_bytes(__arg_0, |_buf| { __arg_1(_buf) }),
            One(ref __self_0) =>
            1u.iter_bytes(__arg_0, |_buf| { __arg_1(_buf) }) &&
                __self_0.iter_bytes(__arg_0, |_buf| { __arg_1(_buf) }),
            Two(ref __self_0) =>
            2u.iter_bytes(__arg_0, |_buf| { __arg_1(_buf) }) &&
                __self_0.iter_bytes(__arg_0, |_buf| { __arg_1(_buf) })
        }
    }
}
