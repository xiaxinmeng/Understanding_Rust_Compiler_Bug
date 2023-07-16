rust
use std::collections::HashMap;
use std::ops::Range;
use std::time::Instant;

use rayon::prelude::*;
use uuid;

fn main() {
    let mut map = HashMap::new();

    for i in 0..1000000 {
        map.insert(i, i);
    }

    let start = Instant::now();
    let mut sum = 0;
    for i in 0..1000000 {
        sum += *map.get(&i).unwrap_or(&0);
    }
    println!("single thread sum(={}) with map.get: {}ms", sum, start.elapsed().as_millis());

    let start = Instant::now();
    let thread_num = 10usize;
    let range: Vec<usize> = Range { start: 0usize, end: thread_num }.collect();
    let sum: usize = range.into_par_iter().map(|_| {
        let range: Vec<usize> = Range { start: 0, end: 1000000 / thread_num }.collect();
        let sum: usize = range.iter().map(|i| {
            *map.get(&i).unwrap_or(&0)
        }).sum();
        sum
    }).sum();

    println!("multi thread sum(={}) with map.get: {}ms", sum, start.elapsed().as_millis());
}
