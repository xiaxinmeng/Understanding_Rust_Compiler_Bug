rust
#[export_name = "SOME_KNOWN_NAME_TO_HAVE_THE_LINKER_PUT_THIS_IN_THE_RIGHT_PLACE"]
pub unsafe extern "C" fn interrupt_handler_that_has_some_random_name_unknown_to_the_user(
    token: InterruptsDisabled,
) {
    on_button_pressed(token);
}

static COUNT: InterruptCell<usize> = InterruptLock(UnsafeCell::new(
    Cell::new(0),
));
fn on_button_pressed(token: InterruptsDisabled) {
    let count = COUNT.get(token);
    count.set(count.get() + 1);
    println!("Button has been pressed {} times", count.get());
}
