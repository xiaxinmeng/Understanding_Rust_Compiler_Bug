rust
#![feature(rustc_attrs)]
#![feature(nll)]

#[rustc_regions]
fn main() {
    let mut x = 0;
    || {
        move || {
            let _y = &mut x;
        }
    };
}
