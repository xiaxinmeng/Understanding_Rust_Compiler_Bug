
fn sum_and_show(data: &[u32]) -> String {
    data.iter().sum().to_string()
}
fn main() {
    let s: String = sum_and_show(&[1, 2, 3]);
}
