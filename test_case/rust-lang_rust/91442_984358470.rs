rust
#![no_std]
#![no_main]

#![feature(bench_black_box)]

psp::module!("crash-test",1,0);


fn psp_main() {
    let dx = core::hint::black_box(0.0); 
    let tdx = do_calc(dx);
    psp::dprintln!("{}", tdx);
}

#[no_mangle]
#[inline(never)]
fn do_calc(dx: f32) -> f32 {
    if dx == 0.0 {
        core::f32::MAX
    } else {
        1.0 / dx
    }
}
