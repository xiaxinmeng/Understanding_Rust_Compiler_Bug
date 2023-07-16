
   #![crate_type="staticlib"]
   #![crate_id = "test#0.0.1"]
   #![no_std]
   #![feature(lang_items)]
   #![allow(dead_code)]
   
   extern crate core;
   extern crate rlibc;
   
   #[allow(dead_code)]
   #[no_mangle]
   pub fn main()
   {
       tst();
   }
   
   pub fn tst() {}
   
   #[lang = "begin_unwind"]
   extern fn begin_unwind(_args : &core::fmt::Arguments, _file : &str, _line : uint) -> !
   {
       loop {}
   }
   
   #[lang = "stack_exhausted"]
   extern fn stack_exhausted() {}
   
   #[lang = "eh_personality"]
   extern fn eh_personality() {}
   