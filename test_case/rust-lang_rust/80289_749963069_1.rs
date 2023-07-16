rs
fn find_gate<'a>(gates: &[&'a str], gate: &'a str) -> &'a str {
    println!("before regex");
    let output_to_gate = Regex::new(&format!(r"(?i).+\s->\s{}$", gate)).unwrap();
    println!("after regex");

    let output_to_gate = gates
        .iter()
        .find(|line| output_to_gate.is_match(line))
        .unwrap();
    println!("after output_to_gate");
    output_to_gate.substring(0, output_to_gate.len() - 5)
}
