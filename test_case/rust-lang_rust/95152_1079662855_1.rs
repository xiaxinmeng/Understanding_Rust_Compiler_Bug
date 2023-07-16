rust
macro_rules! foo { ... }

fn main () {
    #[track_caller]
    foo!( ... )
}
