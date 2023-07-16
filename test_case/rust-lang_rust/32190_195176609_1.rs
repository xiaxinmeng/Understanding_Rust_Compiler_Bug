
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<f64> = Some(5.0f64);
}
