rust
// Current rustc and new version both reject this like so:
//
// error: `<` is interpreted as a start of generic arguments for `u32`, not a shift
//   --> f.rs:15:14
//    |
// 15 |     1 as u32 << 24
//    |              ^^ -- interpreted as generic arguments
//    |              |
//    |              not interpreted as shift
//    |
// help: try shifting the cast value
//    |
// 15 |     (1 as u32) << 24
//    |     +        +

fn sample2() -> u32 {
    1 as u32 << 24
}

// Current rustc accepts this, new version doesn't.
macro_rules! f {
    ($u_scalar:ty) => {
        fn sample1() -> u32 {
            1 as $u_scalar << 24
        }
    }
}

f! { u32 }

fn main() {
    let _ = sample1() + sample2();
}
