 Rust
struct Counter(u64);
impl Counter { fn new()->Counter { Counter(0) } }
impl Callbale<(), u64> for Counter {
    type SelfType = &mut Counter;
    fn call(this: &mut Counter, _: ())->u64{
        this.0 = this.0 + 1;
        this.0
    }
}
