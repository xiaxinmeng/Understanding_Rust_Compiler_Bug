
pub fn main() {
    let mut i = 0;

    (|| i += 1 )();
    dbg!(i);  // i = 1

    (move || i += 1 )();
    dbg!(i);  // i still 1
}
