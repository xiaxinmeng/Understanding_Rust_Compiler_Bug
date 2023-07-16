 rust
#[cfg(test)]
mod tests {
    use AtomicCell;

    static CELL: AtomicCell<u32> = AtomicCell::new();

    #[test]
    fn it_works() {
        CELL.spin_lock(|x| print!("{}\n", x));
    }
}
