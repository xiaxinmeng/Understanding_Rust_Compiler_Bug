
/// use std::rand;
///
/// let x = rand::random();
/// println!("{}", 2u8 * x);
///
/// let y = rand::random::<f64>();
/// println!("{}", y);
///
/// if rand::random() { // generates a boolean
///     println!("Better lucky than good!");
/// }
/// 