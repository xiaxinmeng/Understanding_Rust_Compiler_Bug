rust
fn main() {
    let mut n = 0;
    match 0 {
        | _ | _ | _ | _ | _ | _ | _ | _ | _ | _ if
        { println!("{}", n); n += 1; false } => {}
        _ => {}
    }
}
