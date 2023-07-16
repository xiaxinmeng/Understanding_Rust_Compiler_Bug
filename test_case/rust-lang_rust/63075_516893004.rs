
error: any use of this value will cause an error
     | 
    ::: <source>:13:1
     |
13   | / pub const OFFSET: usize = {
14   | |     let uninit = std::mem::MaybeUninit::<Struct>::uninit();
15   | |     let base_ptr: *const Struct = unsafe { TransmuteHack { from: &uninit }.to };
16   | |     let field_ptr = unsafe { &(*base_ptr).field as *const _ };
17   | |     let offset = unsafe { (field_ptr as usize).wrapping_sub(base_ptr as usize) };
18   | |     offset
19   | | };
     | |__-
     |
     = note: `#[deny(const_err)]` on by default
