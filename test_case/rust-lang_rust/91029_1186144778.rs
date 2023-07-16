rust
use std::num::NonZeroU8 as NZ;

struct HasNiche(NZ);

impl Drop for HasNiche {
    fn drop(&mut self) {
        println!("Niche drop")
    }
}

struct LoudDrop(u8);

impl Drop for LoudDrop {
    fn drop(&mut self) {
        println!("Extra field drop")
    }
}

#[inline(never)]
fn print_boop() {
    println!("boop");
}

fn real_main(e: Option<(HasNiche, LoudDrop)>) {
    match e {
        Some((_a, _)) => {}
        None => {}
    }
    print_boop();
}

fn main() {
    assert_eq!(std::mem::size_of::<Option<(HasNiche, LoudDrop)>>(), 2);
    let x = (HasNiche(NZ::new(5).unwrap()), LoudDrop(10));
    real_main(Some(x));
}
