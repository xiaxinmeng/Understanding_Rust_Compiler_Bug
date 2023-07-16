 rust
#[deriving(IterBytes)]
enum A { B, }
#[doc = "Automatically derived."]
pub impl ::core::to_bytes::IterBytes for A {
    pub fn iter_bytes(&self, __arg_0: ::bool, __arg_1: ::core::to_bytes::Cb)
     -> ::bool {
        match *self { B => 0u.iter_bytes(__arg_0, __arg_1) }
    }
}
