
/* mockups, just so that we can try the interface for real.
 * not the actual implementation, of course. */
macro_rules! print(
    ($($A:expr),*) => ( write!(std::io::stdout(), $($A),*))
)
macro_rules! println(
    ($($A:expr),*) => ( std::io::stdout().write_str(ifmt!($($A),*) + "\n"))
)
macro_rules! write(
    ($B:expr, $($A:expr),*) => ( ($B).write_str(ifmt!($($A),*)))
)

fn main() {
    print!("hi {}, ", "adridu59");
    println!("nice to meet you.");
    write!(std::io::stdout(), "hi times {}\n", 3);
}
