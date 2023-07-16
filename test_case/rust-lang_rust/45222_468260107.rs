rust
#![feature(test)]
extern crate test;

use test::Bencher;

fn triples_exclusive() -> impl Iterator<Item = (u32, u32, u32)> {
    (1u32..).flat_map(|z| {
        (1..z + 1).flat_map(move |x| {
            (x..z + 1)
                .filter(move |&y| x.pow(2) + y.pow(2) == z.pow(2))
                .map(move |y| (x, y, z))
        })
    })
}

fn triples_inclusive() -> impl Iterator<Item = (u32, u32, u32)> {
    (1u32..).flat_map(|z| {
        (1..=z).flat_map(move |x| {
            (x..=z)
                .filter(move |&y| x.pow(2) + y.pow(2) == z.pow(2))
                .map(move |y| (x, y, z))
        })
    })
}

#[bench]
fn range_exclusive(b: &mut Bencher) {
    b.iter(|| triples_exclusive().take(1_000).map(|(x, y, z)| x + y + z).sum::<u32>());
}

#[bench]
fn range_inclusive(b: &mut Bencher) {
    b.iter(|| triples_inclusive().take(1_000).map(|(x, y, z)| x + y + z).sum::<u32>());
}
