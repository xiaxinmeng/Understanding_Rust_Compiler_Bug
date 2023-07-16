rust
fn fn_ref<F: Fn()>(f: F) -> F { f }

fn two_closures_ref(x: i32) {
    fn_ref(|| {
        || //~ ERROR
         x = 1;} //~ ERROR
    );
    fn_ref(move || {
        ||  //~ ERROR
    x = 1;}); //~ ERROR
}
