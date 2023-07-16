
struct InnerBase {/* actual data here */}
struct Base([InnerBase; 1]);
#[repr(transparent)] // for the transmute
struct Ref([InnerBase]);

impl std::ops::Deref for Base {
    type Target = Ref;

    fn deref<'a>(&'a self) -> &'a Ref {
        // safety: the lifetimes and types are spelled out, and `&Ref` should be identical to `&[InnerBase]`
        unsafe { std::mem::transmute::<&[InnerBase], &Ref>(&*std::ptr::slice_from_raw_parts(self.0.as_ptr(), 1)) }
    }
}

impl std::ops::Deref for Ref {
    type Target = InnerBase;

    fn deref<'a>(&'a self) -> &'a InnerBase {
        &self.0[0]
    }
}

impl InnerBase {
    // will work as if it were implemented for `&Ref`
    pub fn hello_world(&self) {}
}

// no error
let r: &Ref = &{ Base([InnerBase {}; 1]) };
r.hello_world();
