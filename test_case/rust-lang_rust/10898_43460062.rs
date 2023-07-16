 rust
extern crate test;

#[main]
fn say_hi () {
   println!("Hello!!");
}

#[test]
fn addition_works () {
   assert! (2 + 2 == 4);
}

#[test]
fn test2() {
    assert!(1 + 1 == 2);
}

#[bench]
fn addition_benchmarked (b: &mut test::Bencher) {
   let mut sum = 0;
   b.iter(|| sum += 1)
}
