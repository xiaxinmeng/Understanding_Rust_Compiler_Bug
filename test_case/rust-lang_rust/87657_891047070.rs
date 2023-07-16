rust
// lazy_static::lazy_static! {
//     static ref LOADERS: Vec<&'static u8> = Vec::new();
// }

const LOADERS: &Vec<&'static u8> = &Vec::new();

pub fn break_code() -> Option<&'static u8> {
    for loader in *LOADERS {
        return Some(loader);
    }
    None
}
