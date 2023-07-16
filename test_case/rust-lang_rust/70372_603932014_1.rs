rust
match Some((3,42)) {
    Some((a, _)) | Some((_, a)) if a > 10 => {println!("{}", a)}
    _ => ()
}
match Some((3,42)) {
    Some((a, _) | (_, a)) if a > 10 => {println!("{}", a)}
    _ => ()
}
