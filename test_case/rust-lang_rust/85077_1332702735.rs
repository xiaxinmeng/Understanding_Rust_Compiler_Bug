 rust
#![feature(generic_arg_infer)]

trait Register<const N: usize>: Default {}

#[derive(Default)]
struct Test([u8;4]);

impl Register<4> for Test {}

struct Accessor();

impl Accessor {
    fn default<R: Register<{N}>, const N: usize>(&mut self) -> R {
        Default::default()
    }
}

fn main() {
    let mut regs = Accessor();
    let test1: Test = regs.default();
    let test2 = regs.default::<Test, _>();
    let test3 = regs.default::<Test>(); // :(
}
