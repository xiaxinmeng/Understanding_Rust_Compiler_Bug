rust
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const DISPLAY_SIZE: usize = WIDTH * HEIGHT;

pub fn draw(data: &[u8; DISPLAY_SIZE], x: u8, y: u8) -> bool {
    let x2 = usize::from(x) % WIDTH;
    let y2 = usize::from(y) % HEIGHT;
    let mut collision = false;
    for y3 in y2 .. HEIGHT {
        let coord = x2 + y3 * WIDTH;
        collision |= data[coord] != 0;
    }
    return collision;
}
