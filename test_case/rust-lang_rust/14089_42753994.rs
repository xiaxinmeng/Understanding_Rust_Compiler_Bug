
let mut vari = StrBuf::new();
vari.push_char('c');

match vari.as_slice() {
    "c" => println!("Got a `c`"),
    _ => println!("Not a `c`")
}
