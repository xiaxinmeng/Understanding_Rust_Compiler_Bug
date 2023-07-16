
//#![feature(const_transmute)]

fn main() {
    let _: [_; unsafe { std::mem::transmute(|| {}) }];
}
