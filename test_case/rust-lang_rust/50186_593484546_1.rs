rust
cell.update(|x| {
    cell.update(|x| x + 1); // Error: cell is exclusively borrowed above
    x + 1
});
