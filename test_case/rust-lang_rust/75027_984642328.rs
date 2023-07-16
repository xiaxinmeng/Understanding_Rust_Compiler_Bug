rust
fn parse1(input: &str) -> usize {
    let lines: Vec<u16> = input.lines().map(|line| line.parse().unwrap()).collect();
    lines.array_windows().filter(|[n1, n2]| n2 > n1).count()
}
