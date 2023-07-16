rust
    fn consume(self) -> &'a mut () {
        let MyStruct { field, field2 } = self;
        field
    }
