
mod submod {
    // 'number' shouldn't be visible here (only with 'super::number')
    pub fn subfn(number: int) -> int {
        2 * number
    }
}

static number: int = 1;

fn main() {
    submod::subfn(3);
}
