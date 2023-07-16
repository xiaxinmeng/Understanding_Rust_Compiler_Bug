
fn main() {
    let a = vec!["".to_string()];
    a.iter().enumerate()
            .take_while(|(_, &t)| false)
            .collect::<Vec<_>>();
}
