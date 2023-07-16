 rust
extern crate time;
extern crate test;

#[inline(never)]
pub fn f() {
    let mut v = [1i, 1, 1];
    for _ in range(0u, 10000000000) {
        for i in range(0u, 3) {
            while v[i] > 0 { v[i] -= 1; }
        }
    }
    test::black_box(v[0]);
    test::black_box(v[1]);
    test::black_box(v[2]);
}

#[inline(never)]
pub fn g() {
    let mut v = [1i, 1, 1];
    for _ in range(0u, 10000000000) {
        for i in range(0u, 3) {
            if v[i] > 0 { v[i] -= 1; }
        }
    }
    test::black_box(v[0]);
    test::black_box(v[1]);
    test::black_box(v[2]);
}

#[inline(never)]
pub fn h() {
    let mut v = [1i, 1, 1];
    for _ in range(0u, 10000000000) {
        for i in range(0u, 3) {
            if v[i] > 0 { v[i] = 0; }
        }
    }
    test::black_box(v[0]);
    test::black_box(v[1]);
    test::black_box(v[2]);
}

fn main() {
    for &f in [f, g, h].iter() {
        let start = time::precise_time_s();
        f();
        println!("{}", time::precise_time_s() - start);
    }
}

