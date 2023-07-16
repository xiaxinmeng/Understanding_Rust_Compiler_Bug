 rust
#![feature(test, rand)]
extern crate rand;
extern crate test;
use rand::{Rand, Rng, SeedableRng, XorShiftRng};

#[derive(PartialEq)]
enum Test {
    Variant1,
    Variant2,
    Variant3,
    Variant4,
    Variant5,
}
#[derive(PartialEq)]
enum Test2 {
    Variant1(i32, i32),
    Variant2(f64),
    Variant3,
    Variant4(Vec<Test2>),
    Variant5,
}

impl Rand for Test {
    fn rand<R: Rng> (r: &mut R) -> Test {
        match r.gen_range(0, 5) {
            0 => Test::Variant1,
            1 => Test::Variant2,
            2 => Test::Variant3,
            3 => Test::Variant4,
            4 => Test::Variant5,
            _ => panic!()
        }
    }
}

impl Rand for Test2 {
    fn rand<R: Rng> (r: &mut R) -> Test2 {
        match r.gen_range(0, 5) {
            0 => Test2::Variant1(i32::rand(r), i32::rand(r)),
            1 => Test2::Variant2(f64::rand(r)),
            2 => Test2::Variant3,
            3 => {
                let i = r.gen_range(0, 10);
                Test2::Variant4(r.gen_iter().take(i).collect())
            }
            4 => Test2::Variant5,
            _ => panic!()
        }
    }
}

#[bench]
fn test1(b: &mut ::test::Bencher) {
    let mut rng: XorShiftRng = SeedableRng::from_seed([1,2,3,4]);
    let values: Vec<Test> = rng.gen_iter().take(500).collect();
    b.iter(|| {
        for x in &values {
            for y in &values {
                ::test::black_box(x == y);
            }
        }
    })
}

#[bench]
fn test2(b: &mut ::test::Bencher) {
    let mut rng: XorShiftRng = SeedableRng::from_seed([1,2,3,4]);
    let values: Vec<Test2> = rng.gen_iter().take(500).collect();
    b.iter(|| {
        for x in &values {
            for y in &values {
                ::test::black_box(x == y);
            }
        }
    })
}
