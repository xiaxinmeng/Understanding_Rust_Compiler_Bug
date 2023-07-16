rust
#[test]
fn foo_bench() {
    let mut durs = vec![Duration::ZERO; 10000];
    let mut start = Instant::now();
    for dur in &mut durs {
        black_box(foo());
        let next = Instant::now();
        *dur = next - start;
        start = next;
    }
    dbg!(durs.into_iter().sum::<Duration>() / 10000);
}
