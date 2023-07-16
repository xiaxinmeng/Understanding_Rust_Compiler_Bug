
match kind {
    IoErrorKind::Uncategorised(UnstableCategory::NotADirectory) => { println!("expected case"); }
    _ => { panic!("unexpected error occured"); }
}
