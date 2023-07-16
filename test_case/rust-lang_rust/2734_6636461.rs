
iface hax { } 
impl <A> of hax for A { } 
fn perform_hax<T>(x: @T) -> hax {
    x as hax 
}
fn perform_hax_int(x: @int) -> hax {
    x as hax 
}
fn perform_hax_str(x: @str) -> hax {
    x as hax 
}
fn deadcode() {
    perform_hax_str(@"deadcode");
}
fn main() {
    let _ = perform_hax_int(@42);
}
