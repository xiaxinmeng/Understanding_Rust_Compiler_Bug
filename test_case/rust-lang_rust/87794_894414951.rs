rust

#[repr(usize)]
pub enum Size {
    One = 1,
    Two = 2,
    Three = 3,
}

pub fn handle(x: Option<Size>) -> usize {
    match x {
        None => {0}
        Some(size) => {size as usize}
    }
}
