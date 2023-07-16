 rust
macro_rules! foo {
    ($($p: pat)|+) => {{
        $(let $p = 1i32;)+
    }}
}

foo!(x | y);
