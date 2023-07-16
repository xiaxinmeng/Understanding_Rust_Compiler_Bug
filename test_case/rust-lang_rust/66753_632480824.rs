rust
#![feature(const_forget)]
#![feature(const_fn)]
#![feature(const_if_match)]
#![feature(const_panic)]

const fn unwrap<T>(opt:Option<T>)->T{
    match opt {
        Some(x)=>x,
        None=>{
            std::mem::forget(opt);
            panic!("Trying to unwrap a None")
        },
    }
}
