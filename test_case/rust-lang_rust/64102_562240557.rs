rust
fn bench_black_box_rt<T>(x: T) -> T { asm!("....") }
const fn bench_black_box_ct<T>(x: T) -> T { x }
pub const fn bench_black_box<T>(x: T) { 
    const_eval_select(bench_black_box_ct, bench_black_box_rt)
}
