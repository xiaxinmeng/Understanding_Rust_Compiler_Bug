 rust
#[feature(macro_rules)];

macro_rules! mymacro( ($x:ident) => (
    expand_string_to_expr!(concat!("fn foo_", stringify!($x),"() { }"))
))

mymacro!(bar)

fn main() {
    foo_bar();
}
