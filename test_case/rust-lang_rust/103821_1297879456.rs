
fn get_mut_ref<'a>() -> &'a mut () {
    static mut _HACK: () = ();

    unsafe {&mut _HACK}
}
