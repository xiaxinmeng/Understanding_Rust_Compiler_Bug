Rust
let [r, g, b] = egui::color::rgb_from_hsv((f32::from(byte) / 288.0, 1.0, 1.0));
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Ranges are in 0-1, they will never be multiplied above 255"
)]
Color::rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
