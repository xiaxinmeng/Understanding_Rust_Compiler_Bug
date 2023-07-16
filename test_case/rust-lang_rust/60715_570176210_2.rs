rust
    fn is_some(cell: &'_ Cell<Option<String>>) -> bool
    {
        cell.read_with(Option::is_some)
    }
    