 rust
extern mod extra;
use std::libc::sleep;
use extra::test::BenchHarness;

#[bench]
#[fixed_stack_segment] 
fn sleep_2_seconds(bh: &mut BenchHarness) {
    do bh.iter {
        unsafe { sleep(2); }
    }
}
