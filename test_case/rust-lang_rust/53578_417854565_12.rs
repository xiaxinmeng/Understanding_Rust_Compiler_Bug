
Aug 31 10:29:47.659 INFO kablam! error: expected one of `!`, `(`, `)`, `,`, `::`, or `<`, found `.`
Aug 31 10:29:47.659 INFO kablam!    --> /cargo-home/registry/src/github.com-1ecc6299db9ec823/tsk_lib-0.0.0/src/query/parser.rs:217:52
Aug 31 10:29:47.659 INFO kablam!     |
Aug 31 10:29:47.659 INFO kablam! 217 |             && (precedence < Precedence::from(&self.peek_token) || precedence == Precedence::STRING)
Aug 31 10:29:47.659 INFO kablam!     |                                                    ^ expected one of `!`, `(`, `)`, `,`, `::`, or `<` here
