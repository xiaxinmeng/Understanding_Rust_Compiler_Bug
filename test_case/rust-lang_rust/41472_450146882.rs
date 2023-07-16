rust
macro_rules! m {
    ( a $(@$meta:meta)* ) => {
        m! ( b $(@$meta)* );
    };
    ( b @ meta ) => { "ok" };
    ( b ) => { "ok" };
}

fn main() {
    println!("{}", m!(a @meta));
    println!("{}", m!(a));
}
