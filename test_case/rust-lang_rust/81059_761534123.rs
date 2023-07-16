rust
fn takes_mut<T>(v: &mut T) -> &mut T {
    v
}

fn test(outer: &mut Option<i32>) {
    match (takes_mut(outer), 23) {
        (Some(inner), _) => {
            *inner = 17;
        },
        _ => {
            *outer = Some(2);
        },
    }
}
