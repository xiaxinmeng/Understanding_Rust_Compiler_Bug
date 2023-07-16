
fn triples() -> impl Iterator<Item=(u32, u32, u32)> {
    (1 ..).flat_map(|z| (1u32 ..= z)
                        .flat_map(move |x| (x ..= z)
                                           .filter(move |&y| x.pow(2) + y.pow(2) == z.pow(2))
                                           .map(move |y| (x, y, z))))
}

fn main() {
    let result: u32 = triples().take(3_000).map(|(x, y, z)| x + y + z).sum();
    println!("{}", result);
}
