sh
error[[E0423]](https://doc.rust-lang.org/stable/error-index.html#E0423): expected value, found struct `HashMap`
 --> src/main.rs:6:44
  |
6 | let mut builder = PassportBuilder{ fields: HashMap<String, String>{} };
  |                                            ^^^^^^^ help: use struct literal syntax instead: `HashMap { base: val }`
  