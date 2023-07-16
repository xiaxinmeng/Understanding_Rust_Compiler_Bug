rust
// rust. pass a mutable vector to multiple nested functions

fn main() {
    let mut v: Vec<String> = Vec::new();
    f1(&mut v); // OK
    println!("{}", v.join(" "));
}

fn f1(v: &mut Vec<String>) {
	v.push(String::from("well"));
	f2(&mut v); // ERROR should be f2(v)
}

fn f2(v: &mut Vec<String>) {
	v.push(String::from("hello"));
	f3(&mut v); // ERROR should be f3(v)
}

fn f3(v: &mut Vec<String>) {
	v.push(String::from("world"));
}
