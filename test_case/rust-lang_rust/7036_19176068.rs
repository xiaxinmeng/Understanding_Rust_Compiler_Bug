
fn main() {
    fn collatz (i: int, acc: int) -> int {
        if i == 1 { acc }
        else if i % 2 == 0 { collatz(i/2, acc + 1) }
        else { collatz(3*i + 1, acc + 1) }
    };
    println(fmt!("collatz(400) = %d", collatz(400, 0)));
}
