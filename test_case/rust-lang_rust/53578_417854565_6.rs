
Sep 01 02:58:21.346 INFO kablam! error: expected one of `!`, `(`, `)`, `,`, `::`, or `<`, found `.`
Sep 01 02:58:21.346 INFO kablam!   --> src/preprocess.rs:72:41
Sep 01 02:58:21.346 INFO kablam!    |
Sep 01 02:58:21.346 INFO kablam! 72 |         Ok(self.mtime < File::open(&self.path)?.metadata()?.modified()?)
Sep 01 02:58:21.346 INFO kablam!    |                                         ^ expected one of `!`, `(`, `)`, `,`, `::`, or `<` here
