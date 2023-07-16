rust
#[export_name = "SOME_KNOWN_NAME_TO_HAVE_THE_LINKER_PUT_THIS_IN_THE_RIGHT_PLACE"]
pub unsafe extern "C" fn interrupt_handler_that_has_some_random_name_unknown_to_the_user(
    token: InterruptsDisabled,
) {
    on_button_pressed(token);
}

static COUNT: InterruptLock<usize> = InterruptLock(UnsafeCell::new(
    0,
));
fn on_button_pressed(mut token: InterruptsDisabled) {
    // This mutable borrow of `token` prevents using it for
    // accessing any `InterruptLock`s, including `COUNT` itself.
    // The previous version was like `&token` from this one.
    let count = COUNT.get_mut(&mut token);
    *count += 1;
    println!("Button has been pressed {} times", *count);
}
