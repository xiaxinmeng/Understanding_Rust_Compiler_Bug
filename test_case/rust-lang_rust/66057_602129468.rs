rust
fn make()-> impl  Extend<char> + Extend<&'static str>{
    String::new()
}
fn main(){
    make().extend(Some("string"));
    make().extend(Some('c'));
}
