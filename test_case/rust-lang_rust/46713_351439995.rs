rust
#![feature(test)]

extern crate test;
use test::Bencher;

#[bench]
fn find_byte(b: &mut Bencher) {
    let x = test::black_box("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.").as_bytes();
    b.iter(|| test::black_box(x.contains(&0xff)));
}

#[bench]
fn find_byte_old(b: &mut Bencher) {
    let x = test::black_box("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.").as_bytes();
    b.iter(|| test::black_box(x.iter().any(|x| *x == 0xff)));
}
