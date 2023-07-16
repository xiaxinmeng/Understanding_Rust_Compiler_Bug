rust
// bin.rs

#[link(name = "my_staticlib")]
extern "C" {
    #[used(linker)]
    fn some_symbol_from_my_staticlib_that_needs_to_be_kept();

   #[used(linker)]
   static SAME_BUT_STATIC: u8;
}
