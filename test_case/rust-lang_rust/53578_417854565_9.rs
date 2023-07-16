
Aug 31 17:15:44.146 INFO kablam! error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `.`
Aug 31 17:15:44.146 INFO kablam!   --> src/repeat_every.rs:78:41
Aug 31 17:15:44.146 INFO kablam!    |
Aug 31 17:15:44.146 INFO kablam! 78 |         if added.minutes < cmp::max(self.minutes, rhs.minutes) || added.days < cmp::max(self.days, rhs.days) || added.months < cmp::max(self.months, rhs.months) {
Aug 31 17:15:44.146 INFO kablam!    |                                         ^ expected one of 7 possible tokens here
