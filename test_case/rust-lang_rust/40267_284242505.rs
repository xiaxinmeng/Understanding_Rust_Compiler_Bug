Rust
#![crate_type="rlib"]

/// Audio sample rate for the test set, used for realtime speed
/// calculation
const SAMPLE_RATE: f64 = 48000.0;
/// Total length of samples the filter benchmarks are ran on
const SAMPLE_COUNT: u64 = 524288;
/// Select how many IIR filters should be applied consecutively
/// on each buffer during the benchmark
const FILTER_COUNT: usize = 100;
const BUFFER_LEN: usize = 128;

/// 2nd order biquad filter
#[derive(Copy)]
struct Biquad {
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,

    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

impl Clone for Biquad {
    fn clone(&self) -> Biquad {
        *self
    }
}

impl Biquad {
    fn new() -> Biquad {
        Biquad {
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }
}

fn iir(buf: &mut [f64], bq: &mut Biquad) {
    for i in 0..buf.len() {
        let x = buf[i];
        buf[i] = (bq.b0 * x) + (bq.b1 * bq.x1) + (bq.b2 * bq.x2) - (bq.a1 * bq.y1) -
                 (bq.a2 * bq.y2);

        bq.x2 = bq.x1;
        bq.x1 = x;

        bq.y2 = bq.y1;
        bq.y1 = buf[i];
    }
}

#[cfg(slow)]
pub fn foo() {
    println!("Create an empty vector, resized then discarded");
    let mut vec_test: Vec<f64> = Vec::new();
    vec_test.resize(1234, 0.0);
}

#[inline(never)]
pub fn sample(buffer_len: usize) {
    let buffer_count = SAMPLE_COUNT / buffer_len as u64;

    for _ in 0..10 {
        let mut buf = vec![0.0; buffer_len];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
    }
}
