
fn banana(f: fn()) -> ! { f(); loop {} }
fn diverging() -> ! { loop {} }
fn main() {
    banana(diverging)
}
