
#[repr(align = "16")]
struct Align16(i32);
enum Enum {
    A(i32),
    B(Align16)
}
let e = Enum::B(Align16(0));
match e {
  Enum::B(ref a) => (),
  _ => ()
}
