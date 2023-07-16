 rust
fn main(){
    let val = Foo;
    let x: Option<&Foo> = Some(&val);
    let tmp = match x {
        Some(val) => *val,
        None => panic!(),
    };
    let y = &tmp;
}
