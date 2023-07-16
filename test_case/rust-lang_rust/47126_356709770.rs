rust
// Yes please
[1, 2, 3].exact_chunks(2);

// No, thank you
let a = [1, 2, 3];
let x = a.len() % 2 * 2;
a[..x].exact_chunks(2);
