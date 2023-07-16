rust
fn foo(_a: &str) {}
fn type_of<T>(_: T){
    println!("type -> {:#?}",std::any::type_name::<T>())
}
fn main() {
    let x = foo as fn(&'static str);
    type_of(foo);
    let _ = x == foo;
}
