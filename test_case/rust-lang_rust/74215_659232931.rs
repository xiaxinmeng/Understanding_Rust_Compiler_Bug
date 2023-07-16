rust
enum SomeEnum { A, B = 255 }
enum OtherEnum { A, B = 256 }
enum Single { B = 256 }
fn main() {
    dbg!(std::mem::size_of::<SomeEnum>());
    dbg!(std::mem::size_of::<OtherEnum>());
    dbg!(std::mem::size_of::<Single>());
}
