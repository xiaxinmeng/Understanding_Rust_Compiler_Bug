rust
for e in err.borrow() as &(dyn Error) {
    eprintln!("{}", e);
}
