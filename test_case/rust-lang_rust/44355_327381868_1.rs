Rust
#![feature(drain_filter)]

use std::time::{Duration, Instant};

fn main() {
    let mut total = Duration::new(0, 0);
    for _ in 0..100 {
        let mut v = Vec::new();
        for i in 0..9999999 {
            v.push(i);
        }
        let start = Instant::now();
        assert_eq!(v.drain_filter(|e| *e == 0).count(), 1);
        total += start.elapsed() / 1000;
    }
    println!("Average time for removing one at beginning: {:?}", total);
    let mut total = Duration::new(0, 0);
    for _ in 0..100 {
        let mut v = Vec::new();
        for i in 0..9999999 {
            v.push(i);
        }
        let start = Instant::now();
        assert_eq!(v.drain_filter(|e| *e == 9999998).count(), 1);
        total += start.elapsed() / 1000;
    }
    println!("Average time for removing one at ending:    {:?}", total);
    let mut total = Duration::new(0, 0);
    for _ in 0..100 {
        let mut v = Vec::new();
        for _ in 0..9999999 {
            v.push(0);
        }
        let start = Instant::now();
        assert_eq!(v.drain_filter(|e| *e == 0).count(), 9999999);
        total += start.elapsed() / 1000;
    }
    println!("Average time for removing all elements:     {:?}", total);
    let mut total = Duration::new(0, 0);
    for _ in 0..100 {
        let mut v = Vec::new();
        for _ in 0..9999999 {
            v.push(0);
        }
        let start = Instant::now();
        assert_eq!(v.drain_filter(|e| *e != 0).count(), 0);
        total += start.elapsed() / 1000;
    }
    println!("Average time for removing no elements:      {:?}", total);
}
