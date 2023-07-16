
warning: unnecessary path disambiguator
 --> file2.rs:3:36
  |
3 |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>());
  |                                    ^^ try removing `::`

error: expected token: `,`
 --> file2.rs:3:58
  |
3 |     println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>());
  |                                                          ^
