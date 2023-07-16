 rust
#[deriving(Show, PartialEq, Eq)]
enum MyError3Kind {
    EndOfFile,
    Error,
    _Error1, //(uint),
}

#[deriving(Show, PartialEq, Eq)]
struct MyError3 {
    kind: MyError3Kind,
    //desc: &'static str,
    //description: Option<String>,
}

impl Error for MyError3 {
    fn is_eof(&self) -> bool {
        self.kind == MyError3Kind::EndOfFile
    }
}

#[bench]
fn bench_foo11_enum_smaller_error(b: &mut test::Bencher) {
    let bytes = generate_bytes();
    b.bytes = bytes.len() as u64;

    b.iter(|| {
        let mut rdr = bytes.as_slice();
        let iter = Foo11::new(|buf| -> Result<(), MyError3> {
            match rdr.push(BUFFER_SIZE, buf) {
                Ok(_) => Ok(()),
                Err(io::IoError { kind: io::EndOfFile, .. }) => Ok(()),
                Err(_) => {
                    Err(MyError3 {
                        kind: MyError3Kind::Error,
                        //desc: "",
                        //description: Some("foo".to_string()),
                    })
                }
            }
        });

        for (idx, item) in iter.enumerate() {
            assert_eq!(idx as u8, item.unwrap());
        }
    })
}
