rust
 fn main() {
    "\nhello\n\nworld"
        .to_string()
        .lines()
        .enumerate()
        .map(|(i, s)| {
            if i == 0 {
                println!("{:?}", s);
            };
        })
        .for_each(drop);
}
