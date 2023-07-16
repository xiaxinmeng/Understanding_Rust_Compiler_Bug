
fn main() {
    println!("{}", (0..1u32<<30)
                   .filter(|&i| i.count_ones() == 15)
                   .count()
    );
}
