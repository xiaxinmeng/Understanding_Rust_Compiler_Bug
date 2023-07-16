rust
#![feature(const_type_id)]
#![feature(core_intrinsics)]

struct X([u8; std::intrinsics::type_id::<Self>() as usize]);

fn main() {
    let _ = std::mem::MaybeUninit::<X>::uninit();
}
