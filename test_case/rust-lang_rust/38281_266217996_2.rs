 diff

 #[lang = "panic_fmt"]
-extern "C" fn panic_fmt() -> ! {
+pub extern "C" fn panic_fmt() -> ! {
     loop {}
 }
