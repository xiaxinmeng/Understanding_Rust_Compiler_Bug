
Aug 31 13:20:54.029 INFO kablam! error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `.`
Aug 31 13:20:54.029 INFO kablam!   --> src/wire/arp.rs:93:40
Aug 31 13:20:54.029 INFO kablam!    |
Aug 31 13:20:54.029 INFO kablam! 93 |         } else if len < field::TPA(self.hardware_len(), self.protocol_len()).end {
Aug 31 13:20:54.029 INFO kablam!    |                                        ^ expected one of 7 possible tokens here
