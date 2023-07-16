
Sep 01 03:41:31.071 INFO kablam! error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `.`
Sep 01 03:41:31.071 INFO kablam!   --> /cargo-home/registry/src/github.com-1ecc6299db9ec823/gazetta-render-ext-0.2.1/src/content.rs:53:70
Sep 01 03:41:31.071 INFO kablam!    |
Sep 01 03:41:31.071 INFO kablam! 53 |             "mkd" | "md" | "markdown" => tmpl << ::Markdown::new(self.0.content.data, self.0.href),
Sep 01 03:41:31.071 INFO kablam!    |                                                                      ^ expected one of 7 possible tokens here
Sep 01 03:41:31.071 INFO kablam! 
Sep 01 03:41:31.071 INFO kablam! error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `.`
Sep 01 03:41:31.071 INFO kablam!   --> /cargo-home/registry/src/github.com-1ecc6299db9ec823/gazetta-render-ext-0.2.1/src/content.rs:54:39
Sep 01 03:41:31.071 INFO kablam!    |
Sep 01 03:41:31.071 INFO kablam! 54 |             "html" => tmpl << Raw(self.0.content.data),
Sep 01 03:41:31.071 INFO kablam!    |                                       ^ expected one of 7 possible tokens here
