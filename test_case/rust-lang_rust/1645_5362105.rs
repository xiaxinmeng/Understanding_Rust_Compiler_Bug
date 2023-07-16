
enum g = option<u64>;
type r = {f: u8, g: g};
fn main() {
    let x = {f: 1_u8, g: g(some(22_u64))};
    let y = {f: 1_u8, g: g(some(22_u64))};
   assert x == y;
}
