
error[E0308]: mismatched types
  --> src/main.rs:2:5
   |
1  |   fn main() {
   |             - expected `()` because of default return type
2  | /     tokio::runtime::Builder::new_multi_thread()
3  | |         .enable_all()
4  | |         .build()
5  | |         .unwrap()
...  |
9  | |             }
10 | |         })
   | |          ^- help: try adding a semicolon: `;`
   | |__________|
   |            expected `()`, found enum `std::result::Result`
   |
   = note: expected unit type `()`
                   found enum `std::result::Result<(), _>`
