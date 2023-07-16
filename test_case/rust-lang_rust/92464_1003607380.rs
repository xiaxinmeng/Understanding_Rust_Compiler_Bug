rust
#[repr(align(2))] // 2, 4, 8 crashes, 1 compiles
enum Aligned {
    Zero = 0,
    One = 1,
}

pub fn main() {
    println!("{}", tou8(Aligned::Zero))
}

fn tou8(al: Aligned) -> u8 {
    al as u8
}
