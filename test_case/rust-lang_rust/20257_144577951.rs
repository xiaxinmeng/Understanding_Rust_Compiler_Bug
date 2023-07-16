 rust
let x = Arc::new(1);

let guard = thread::scoped(|| {
    let y = x.clone();
});

// illegal, since `x` is still borrowed by `guard`
drop(x);
