 rust
    #[doc =
          r" hello everyone"]
    struct Five {
        #[doc = r" doc for i32"]
        something: i32,
    }
    #[doc = "Generated struct builder"]
    struct FiveBuilder {
        #[doc = r" doc for i32"]
        something: Option<i32>,
    }
    impl FiveBuilder {
        /// Construct the builder
        pub fn new() -> FiveBuilder { FiveBuilder{something: Some(0),} }
        /// Build the struct
        pub fn build(self) -> Five {
            let something = self.something.unwrap();
            Five{something: something,}
        }
        #[allow(dead_code)]
        /// Specify a value for the $F_NAME field
        fn something(mut self, value: i32) -> Self {
            self.something = Some(value);
            self
        }
    }
