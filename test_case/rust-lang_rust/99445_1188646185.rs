rust
#![feature(trace_macros)]

trace_macros!(true);

fn main() {
    let _ = unic_langid::langid!("en-us");
}
