
pub enum Infallable {}

pub enum E1 {
    V1 { f: bool },
    V2 { f: Infallable },
    V3,
    V4,
}

fn main() {
    if let E1::V2 { .. } = (E1::V1 { f: true }) {
        unsafe { std::hint::unreachable_unchecked(); }
    }
}
