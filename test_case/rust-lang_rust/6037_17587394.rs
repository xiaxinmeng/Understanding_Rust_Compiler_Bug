 rust
macro_rules! cond(
    ( $(if $pred:expr $body:block )+ else $default:block ) => (
        $(if $pred $body else)+
        $default
    )
)

fn main() {
    let x = 1;
    cond!(
        if x < 0 { io::println(~"woops");   }
        if true  { io::println(~"hullo");   }
        else     { io::println(~"bananas"); }
    )
}
