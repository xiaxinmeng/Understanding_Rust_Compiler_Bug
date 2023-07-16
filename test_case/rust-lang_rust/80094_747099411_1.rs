rust
const RES: usize = WIDTH * HEIGHT;
fn compute_pixels_for_screen_on_embedded_thing(
    background: [ImgColor; RES],
    object: [ImgColor; RES]
) -> [ScreenColor; RES] {
    background.zip(object).map(magic_color_mix)
}
