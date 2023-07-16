
const SOME_REF: &Option<Cell<i32>> = &{
    if /* some complicated logic */ {
        None
    } else {
        Some(Cell::new(7))
    }
};
