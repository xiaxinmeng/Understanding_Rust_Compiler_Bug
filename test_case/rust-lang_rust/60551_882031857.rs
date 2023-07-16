rust
// n-dimensional point
struct Point<const NDims: usize>([f64; NDims]);

// configuration of various things, including number of dimensions
pub trait Config {
    const NDims: usize;
    // more config here...
}

// use Point in structs generic over C: Config
struct ColorSpace<C: Config> {
    data: Vec<(Point<C::NDims>, u8)>,
}
