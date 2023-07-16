
Mar 23 13:28:42.820 INFO kablam! error: unreachable expression
Mar 23 13:28:42.820 INFO kablam!    --> src/main.rs:106:10
Mar 23 13:28:42.820 INFO kablam!     |
Mar 23 13:28:42.820 INFO kablam! 106 | #[derive(StructOpt, Debug)]
Mar 23 13:28:42.820 INFO kablam!     |          ^^^^^^^^^
Mar 23 13:28:42.820 INFO kablam!     |
Mar 23 13:28:42.820 INFO kablam! note: lint level defined here
Mar 23 13:28:42.821 INFO kablam!    --> src/main.rs:20:9
Mar 23 13:28:42.821 INFO kablam!     |
Mar 23 13:28:42.821 INFO kablam! 20  | #![deny(warnings)]
Mar 23 13:28:42.821 INFO kablam!     |         ^^^^^^^^
Mar 23 13:28:42.821 INFO kablam!     = note: #[deny(unreachable_code)] implied by #[deny(warnings)]
