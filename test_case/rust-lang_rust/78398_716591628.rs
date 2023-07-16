`rust
fn main() {}

macro_rules! mac_extern {
    ($i:item) => {
        extern "C" { $i }
    }
}

mac_extern! {
    fn foo();
}
