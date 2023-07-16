
fn main() {
    let blah;
    {
        let ss: &[~str] = ~[~"sup?"];
        blah = ~ss as ~ToStr:;
    }
    let oh_noes = ~[666];
    println!("{}", blah.to_str());
}
