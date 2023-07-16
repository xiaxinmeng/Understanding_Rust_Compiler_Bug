
let color = match foo {
    0.0...0.1 => Color::Red,
    0.1...0.4 => Color::Yellow,
    0.4...0.8 => Color::Blue,
    _         => Color::Grey,
};
