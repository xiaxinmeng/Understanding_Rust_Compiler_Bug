rust
#![feature(const_generics)]

struct Config(usize);

impl std::cmp::Eq for Config {}
impl std::cmp::PartialEq for Config {
    fn eq(&self, _: &Self) -> bool { false }
}

struct B<const CFG: Config> {
    arr: [u8; {CFG.0}]
}

fn main() {}
