rust
    fn create_box() {
        let _box = Box::new(1u8);
    }
    fn main() {
        let _box_one = Box::new(10u8);
        {
            let _box_two = Box::new(20u8);
        }
        for _ in 0u16..1_000 {
            create_box();
        }
    }
    