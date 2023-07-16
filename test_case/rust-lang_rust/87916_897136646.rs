
> failures:
> 
> ---- [ui] ui/sanitize/memory.rs stdout ----
> 
> error: error pattern ' in the stack frame of function 'random'' not found!
> command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/memory/a"
> stdout:
> ------------------------------------------
> 
> 
> ------------------------------------------
> stderr:
> ------------------------------------------
> ==10866==WARNING: MemorySanitizer: use-of-uninitialized-value
>     #0 0x5565d1c6fb44  (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/memory/a+0x67b44)
>     #1 0x7f93163a10b2  (/lib/x86_64-linux-gnu/libc.so.6+0x270b2)
>     #2 0x5565d1c111cd  (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/memory/a+0x91cd)
> 
>   Uninitialized value was created by an allocation of 'r.i' in the stack frame of function 'main'
>     #0 0x5565d1c6fa40  (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/memory/a+0x67a40)
> 
> SUMMARY: MemorySanitizer: use-of-uninitialized-value (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/memory/a+0x67b44) 
> 
> ------------------------------------------
> 