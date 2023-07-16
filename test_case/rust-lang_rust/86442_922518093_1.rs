
match kind {
    IoErrorKind::Uncategorised(_) => { println!("expected case"); }
    _ => { panic!("unexpected error occured"); }
}
