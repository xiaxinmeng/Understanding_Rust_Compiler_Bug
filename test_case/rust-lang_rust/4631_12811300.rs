 rust
fn main() {
    // old version
    let b = ~[0xC3u8, 0xA5];
    io::println(str::from_bytes(b)); // prints 'å'

    // new version
    let mut s = ~"";
    str::push_char(&mut s,0xC3 as char);
    str::push_char(&mut s,0xA5 as char);
    io::println(s); // prints 'Ã¥'

}
