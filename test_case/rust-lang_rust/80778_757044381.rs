rust
fn unwrap_left<L, R>(either: &Cell<Either<L, R>>) -> &Cell<L> { ... }

let cell = Cell::new(Either::Left(...));
let left = unwrap_left(&cell);
cell.set(Either::Right(...));
left.get()
