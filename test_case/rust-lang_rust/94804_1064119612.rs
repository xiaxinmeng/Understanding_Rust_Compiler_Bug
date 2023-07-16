rust
const fn f(_: Box<i32>) {}

pub const C: () = {
    unsafe { f(std::mem::transmute(&0)) };
};
