
let mut iter = (1..20).peekable();
for v in iter.while(|&x| x < 10) {
    println!("{}", v);
}
