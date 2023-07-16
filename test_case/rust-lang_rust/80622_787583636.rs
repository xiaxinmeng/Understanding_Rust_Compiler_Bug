rust
#[derive(Clone, Copy)]
enum Bool {
        True,
        False
}

fn main() {
        const TEST: [[[Bool; 256]; 256]; 256] = [[[Bool::True; 256]; 256]; 256];
}
