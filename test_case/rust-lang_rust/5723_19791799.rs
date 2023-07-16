
20:04:55 < bblum> rusti: use std::task; use std::io; fn f() -> @io::Reader { let mut y = None;
                  let x = ~[0xcdu8, ..64]; do io::with_bytes_reader(x) |r| { y = Some(r); };
                  y.unwrap() } let y = f(); do task::spawn {}; let mut z = [0, ..32];
                  y.read(z, 32); z
20:04:57 -rusti:#rust- [1, 0, 0, 0, 0, 0, 0, 0, 128, 207, 141, 113, 98, 127, 0, 0, 64, 27, 32, 104,
                        98, 127, 0, 0, 0, 9, 32, 104, 98, 127, 0, 0]
