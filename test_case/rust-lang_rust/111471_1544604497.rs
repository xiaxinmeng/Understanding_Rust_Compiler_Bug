rust
> #![feature(asm_experimental_arch)]
> #![no_std]
> 
> #[panic_handler]
> fn panic_handler(_: &core::panic::PanicInfo) -> ! {
>     loop {}
> }
> 
> #[no_mangle]
> pub unsafe fn get_global() -> i64 {
>     let ret;
>     core::arch::asm!("global.get 0", "local.set {0}", out(local) ret);
>     ret
> }
> 