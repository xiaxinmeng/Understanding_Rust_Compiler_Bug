Rust
// issue #102317
// run-pass
// compile-flags: --edition 2021 -C opt-level=3 -Zvalidate-mir

struct SegmentJob {
    spec: String,
}

pub async fn run() -> Result<(), ()> {
    let jobs = Vec::<SegmentJob>::new();
    let Some(job) = jobs.into_iter().next() else {
        return Ok(())
    };

    Ok(())
}

fn main() {}
