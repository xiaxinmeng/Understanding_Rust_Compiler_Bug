 rust
// we generate this function using macros
#[export_name = "SOME_KNOWN_NAME_TO_HAVE_THE_LINKER_PUT_THIS_IN_THE_RIGHT_PLACE"]
pub unsafe extern "C" fn interrupt_handler_that_has_some_random_name_unknown_to_the_user() {
    static mut STATE: usize = 0; // initial value provided by the user

    on_button_pressed(&mut STATE);
}

// the user provides this function along with the initial value for the state
fn on_button_pressed(count: &mut usize) {
    *count += 1;
    println!("Button has been pressed {} times", *count);
}
