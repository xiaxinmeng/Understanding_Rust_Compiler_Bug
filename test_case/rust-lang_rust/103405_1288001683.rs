rust

fn should_ok() {
    if true && if let x = 1 { true } else { true } {}
}
