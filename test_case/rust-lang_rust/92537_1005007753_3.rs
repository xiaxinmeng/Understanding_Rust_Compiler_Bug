rust
fn main() {
    let g: *mut dyn Fn(&'_ u32) -> VNode<'_> =
        unsafe {make_fat_ptr(&12)};
}
