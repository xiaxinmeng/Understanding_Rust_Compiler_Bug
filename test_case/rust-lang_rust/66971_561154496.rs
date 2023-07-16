
fn encode(this: ((), u8, u8)) {
    // this assert panics although this.2 is 0
    assert!(this.2 == 0);
}

fn main() {
    encode(((), 0, 0));
}
