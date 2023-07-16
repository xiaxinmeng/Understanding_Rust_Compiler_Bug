rust
pub trait Deserializer<'a> { type Error; }

fn _deserialize<'de, D>(the_d: D) where D: Deserializer<'de>
{
    d(the_d);
}

pub fn d<'de, D>(_des: D) -> D::Error where D: Deserializer<'de>,
                                            D::Error:
{
    loop { }
}

fn main() { }
