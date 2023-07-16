
enum Foo { A = 0, X = 0x8000_0000_0000_0000 }
fn main() {
    println!("{} {}", Foo::X as u64, unsafe { std::intrinsics::discriminant_value(&Foo::X) });
}
