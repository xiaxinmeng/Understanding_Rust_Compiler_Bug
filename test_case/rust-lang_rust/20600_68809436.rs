
fn type_of<T>(_: &T) -> &'static str { unsafe { (*std::intrinsics::get_tydesc::<T>()).name } }

fn main() {
    let x = !0u8;
    println!("{}", type_of(&x));
}
