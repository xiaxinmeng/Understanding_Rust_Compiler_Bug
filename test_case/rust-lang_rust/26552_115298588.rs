 rust
println!("{:?}", unsafe { 
    ::std::mem::transmute::<*const u8, Option<&u8>>(::std::ptr::null()) 
});
