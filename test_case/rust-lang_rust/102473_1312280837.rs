
2 |     let path = std::path::PathBuf::from("haha");
  |         ---- binding `path` is initialized here
3 |     std::fs::create_dir_all(path).unwrap();
  |                             ---- `Because type `PathBuf` does not implement the
                              `Copy` trait, binding `path` is invalidated when it is
                             moved into function `create_dir_all`
4 |     path
  |     ^^^^ invalid value later used here
