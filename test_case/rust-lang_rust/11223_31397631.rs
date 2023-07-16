 rust
let mut abstime = monotonic();
loop {
    abstime.add_second(1);
    abstime.sleep_until();
    println("foo")
}
