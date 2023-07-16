
use std::unstable::intrinsics::log10f64;
use std::cast::transmute;

fn main() {
        let x = unsafe { log10f64(100f64)};
        let z = unsafe { transmute::<f64,u64>(x)}.to_str_radix(2);
        println(z);
}
