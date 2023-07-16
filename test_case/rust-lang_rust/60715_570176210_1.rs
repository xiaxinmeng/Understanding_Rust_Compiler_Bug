rust
    fn is_some (cell: &'_ Cell<Option<String>>) -> bool
    {
        let prev = cell.replace(None);
        let ret = prev.is_some();
        cell.set(prev);
        ret
    }
    