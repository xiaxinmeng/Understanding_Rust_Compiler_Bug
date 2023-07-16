
extern crate serialize;
use self::serialize::json;

#[deriving(Decodable, Encodable)]
pub struct WindowOptions {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}
