 rust
#[deriving(IterBytes)]
enum A { B, }
#[doc = "Automatically derived."]
pub impl ::core::to_bytes::IterBytes for A {
    pub fn iter_bytes(&self, __lsb0: bool, __f: core::to_bytes::Cb) {
        match *self { B => { 0u.iter_bytes(__lsb0, __f); } }; }
    }
