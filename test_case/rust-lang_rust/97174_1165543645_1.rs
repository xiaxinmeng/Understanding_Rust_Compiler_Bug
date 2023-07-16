rust
use cust::prelude::*;

use cust::stream::{
    Stream,
    StreamFlags
};

const NO_OPT_PTX: &str = include_str!("no_opt.ptx");
const OPT_PTX: &str = include_str!("opt.ptx");

#[repr(C)]
#[derive(Clone, Copy, Default, cust::DeviceCopy)]
pub struct ThreeU8 {
    a: u8,
    b: u8,
    c: u8,
}

fn main() {
    let ctx = cust::quick_init().unwrap();
    let module_no_opt = Module::from_ptx(NO_OPT_PTX, &[]).unwrap();
    let module_opt = Module::from_ptx(OPT_PTX, &[]).unwrap();
    let stream = Stream::new(StreamFlags::NON_BLOCKING, None).unwrap();

    let i = cust::memory::DeviceBox::new(&ThreeU8 {a: 4, b: 5, c: 6}).unwrap();
    let o = cust::memory::DeviceBox::new(&ThreeU8::default()).unwrap();

    let func_no_opt = module_no_opt.get_function("kernel_three_u8").unwrap();
    let func_opt = module_opt.get_function("kernel_three_u8").unwrap();

    // warm up
    let mut before_run = std::time::Instant::now();
    unsafe {
        launch!(
            // slices are passed as two parameters, the pointer and the length.
            func_no_opt<<<1, 1, 0, stream>>>(i.as_device_ptr(), o.as_device_ptr())
        ).unwrap();
    }
    stream.synchronize().unwrap();
    let no_opt_warmup = std::time::Instant::now() - before_run;

    before_run = std::time::Instant::now();
    unsafe {
        launch!(
            // slices are passed as two parameters, the pointer and the length.
            func_no_opt<<<1, 1, 0, stream>>>(i.as_device_ptr(), o.as_device_ptr())
        ).unwrap();
    }
    stream.synchronize().unwrap();
    let no_opt = std::time::Instant::now() - before_run;
    
    before_run = std::time::Instant::now();
    unsafe {
        launch!(
            // slices are passed as two parameters, the pointer and the length.
            func_opt<<<1, 1, 0, stream>>>(i.as_device_ptr(), o.as_device_ptr())
        ).unwrap();
    }
    stream.synchronize().unwrap();
    let opt_warmup = std::time::Instant::now() - before_run;
    
    before_run = std::time::Instant::now();
    unsafe {
        launch!(
            // slices are passed as two parameters, the pointer and the length.
            func_opt<<<1, 1, 0, stream>>>(i.as_device_ptr(), o.as_device_ptr())
        ).unwrap();
    }
    stream.synchronize().unwrap();
    let opt = std::time::Instant::now() - before_run;


    println!("No opt time - warmup: {:?}, normal: {:?}", no_opt_warmup, no_opt);
    println!("With opt time - warmup: {:?}, normal: {:?}", opt_warmup, opt);
}

