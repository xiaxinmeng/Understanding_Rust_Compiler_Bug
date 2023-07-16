rust
fn parse1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .array_windows()
        .filter(|[n1, n2]| n2 > n1)
        .count()
}
