rust
#![feature(test, binary_heap_into_iter_sorted)]
extern crate test;

use std::collections::BinaryHeap;

// tweak this to change the size of the heap
const N: usize = 1000;

fn interleaved() -> BinaryHeap<usize> {
    let mut heap = BinaryHeap::new();

    for (a, b) in (0..N / 2).zip((N / 2..N).into_iter().rev()) {
        heap.push(a);
        heap.push(b);
    }

    heap
}

fn already_sorted() -> BinaryHeap<usize> {
    let mut heap = BinaryHeap::new();

    for i in (0..N).into_iter().rev() {
        heap.push(i);
    }

    heap
}

#[bench]
fn interleaved_heap_into_sorted_vec(b: &mut test::Bencher) {
    let heap = interleaved();

    b.iter(|| {
        let heap = heap.clone();
        heap.into_sorted_vec()
    });
}

#[bench]
fn interleaved_heap_collect_sorted_vec(b: &mut test::Bencher) {
    let heap = interleaved();
    let mut sorted = Vec::new();

    sorted.extend(heap.clone().into_iter_sorted());

    b.iter(|| {
        let heap = heap.clone();
        sorted.clear();

        // Unlike the `into_vec` methods, this creates a new `Vec` each time
        // We amortize the cost a little be re-using it
        sorted.extend(heap.into_iter_sorted());

        test::black_box(&sorted);
    });
}

#[bench]
fn interleaved_heap_into_vec_sort_unstable(b: &mut test::Bencher) {
    let heap = interleaved();

    b.iter(|| {
        let mut heap = heap.clone().into_vec();
        heap.sort_unstable();
        heap
    });
}

#[bench]
fn already_sorted_heap_into_sorted_vec(b: &mut test::Bencher) {
    let heap = already_sorted();

    b.iter(|| {
        let heap = heap.clone();
        heap.into_sorted_vec()
    });
}

#[bench]
fn already_sorted_heap_collect_sorted_vec(b: &mut test::Bencher) {
    let heap = already_sorted();
    let mut sorted = Vec::new();

    sorted.extend(heap.clone().into_iter_sorted());

    b.iter(|| {
        let heap = heap.clone();
        sorted.clear();

        // Unlike the `into_vec` methods, this creates a new `Vec` each time
        // We amortize the cost a little be re-using it
        sorted.extend(heap.into_iter_sorted());

        test::black_box(&sorted);
    });
}

#[bench]
fn already_sorted_heap_into_vec_sort_unstable(b: &mut test::Bencher) {
    let heap = already_sorted();

    b.iter(|| {
        let mut heap = heap.clone().into_vec();
        heap.sort_unstable();
        heap
    });
}
