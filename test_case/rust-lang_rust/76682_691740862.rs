rust
time_split_off_0.rs
use std::time::Instant;

fn main() {
    benchmark_suite(/*bigger_elems=*/1.0, /*buffer_more_before_flush=*/1.0, /*more_elems=*/1.0);
    for growth in 2..=5 {
        let factor = growth as f64;
        benchmark_suite(/*bigger_elems=*/1.0, /*buffer_more_before_flush=*/factor, /*more_elems=*/1.0);
        benchmark_suite(/*bigger_elems=*/1.0, /*buffer_more_before_flush=*/1.0, /*more_elems=*/factor);
        benchmark_suite(/*bigger_elems=*/factor, /*buffer_more_before_flush=*/1.0, /*more_elems=*/1.0);
    }
}

fn benchmark_suite(bigger_elems: f64, buffer_more_before_flush: f64, more_elems: f64, ) {
    let mut elem_size = (10.0 * bigger_elems) as usize;
    let mut max_buffer = (1000.0 * buffer_more_before_flush) as usize;
    let mut loops = max_buffer * (100.0 * more_elems) as usize;

    for _ in 1..=5 {
        println!("========================== BENCHMARK TEST ==========================");
        println!(
            "pushing {} elements of size {} bytes into a Vec buffer flushed at max {} elements",
            loops,
            elem_size,
            max_buffer,
        );
        println!("--------------------------------------------------------------------");
        benchmark(elem_size, loops, max_buffer, false, false);
        benchmark(elem_size, loops, max_buffer, /*replace_with_capacity=*/true, false);
        benchmark(elem_size, loops, max_buffer, false, /*replace_with_new=*/true);
        elem_size *= 2;
        loops *= 2;
        max_buffer *= 2;
    }
}

fn benchmark(elem_size: usize, loops: usize, max_buffer: usize, replace_with_capacity: bool, replace_with_new: bool) {
    let now = Instant::now();
    let mut buffer = Vec::new();
    let mut flushed = 0;
    for _ in 0..loops {
        if buffer.len() > max_buffer {
            if replace_with_capacity {
                let capacity = buffer.capacity();
                flush(std::mem::replace(&mut buffer, Vec::with_capacity(capacity)));
            } else if replace_with_new {
                flush(std::mem::replace(&mut buffer, Vec::new()));
            } else {
                flush(buffer.split_off(0));
            }
            flushed += 1;
        }
        buffer.push(vec![b'\0'; elem_size]);
    }
    if buffer.len() > 0 {
        flush(buffer);
        flushed += 1;
    }

    let elapsed_usec = now.elapsed().as_micros();
    println!(
        "{} microseconds {}, called flush {} times",
        elapsed_usec,
        if replace_with_capacity {
            "replaced with `with_capacity()`"
        } else if replace_with_new {
            "replaced with `new()`"
        } else {
            "WITH EXISTING split_off()"
        },
        flushed,
    );
}

fn flush(buffer: Vec<Vec<u8>>) {
    let _ = buffer;
    // drops and frees this `Vec`
}
