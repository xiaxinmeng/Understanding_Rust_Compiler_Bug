
Aug 31 23:12:20.942 INFO kablam! error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `.`
Aug 31 23:12:20.942 INFO kablam!    --> src/torrent/v1/read.rs:131:60
Aug 31 23:12:20.942 INFO kablam!     |
Aug 31 23:12:20.942 INFO kablam! 131 |             if total_piece_length < util::i64_to_usize(self.length)? {
Aug 31 23:12:20.942 INFO kablam!     |                                                            ^ expected one of 7 possible tokens here
