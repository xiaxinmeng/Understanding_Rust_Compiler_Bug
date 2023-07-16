
use std::unstable::intrinsics::log10f32;
use std::cast::transmute;

fn main() {
        let x = unsafe { log10f32(100f32)};
        let y = unsafe { transmute::<f32,u32>(x)}.to_str_radix(2);
        println(y);
}
