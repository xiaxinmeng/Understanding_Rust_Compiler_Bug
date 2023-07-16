rust
impl<'a> Deserializer<'a> {
    pub fn deserialize_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        match self.bytes.get(self.idx..self.idx + N) {
            Some(v) => {
                self.idx += N;
                Ok(v.try_into().unwrap()) // this shall not panic since slice method succeeded
            }
            None => {
                // ****** WE KNOW THIS ARM IS NEVER MATCHED BUT CHANGING IRRELEVANT HERE AFFECTS BENCHMARKS ******
                Err(self.deserialize_bytes_error())
            }
        }
    }

    #[cold]
    fn deserialize_bytes_error(&self) -> Error {
        let e = Error {
            message: "".to_string(),                            // (A)    test test_playground ... bench:         397 ns/iter (+/- 7)
            // message: format!("{}", "a".repeat(usize::MAX)),  // (B)    test test_playground ... bench:         784 ns/iter (+/- 26)
        };

        // uncommenting print                                   // (A=on) test test_playground ... bench:         780 ns/iter (+/- 6)
        // uncommenting print                                   // (B=on) test test_playground ... bench:         783 ns/iter (+/- 4)
        println!("Does not appear in terminal");

        e
    }
