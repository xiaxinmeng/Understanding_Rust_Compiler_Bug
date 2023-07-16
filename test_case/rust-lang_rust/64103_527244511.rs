`
warning: unreachable expression
 --> src/main.rs:2:6
  |
2 | /      std::process::Command::new("test")
3 | |      .output()
4 | |      .unwrap_or(
5 | |          panic!("oh no")
6 | |      );
  | |______^
  |
