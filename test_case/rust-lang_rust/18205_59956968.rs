 Rust
pub trait Segger {
    fn val(& self) -> uint;
}

impl<'a> Segger for &'a uint {
    fn val(& self) -> uint{
        **self
    }
}

pub struct SegObscure<'a> {
    seg: Box<Segger + 'a>
}

// This function is sound
fn create<'a>(r: &'a uint) -> SegObscure<'a> {
        let seg: Box<&'a uint> = box r;
        SegObscure { seg: seg }
}

// WRONG!
fn create_dangling_pointer() -> SegObscure<'static> {
    let r = 1u;
    create(&r)
}

fn access(s: SegObscure) {
  println!("{}", s.seg.val());
}

fn main() {
    let s = create_dangling_pointer();
    access(s);
}
