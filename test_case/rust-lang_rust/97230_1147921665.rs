plain
    Checking addr2line v0.16.0
error: implementation has missing stability attribute
    --> library/std/src/io/mod.rs:2643:1
     |
2643 | / impl<T: Seek> Seek for Take<T> {
2644 | |     fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
2645 | |         let stream_end = self.cursor + self.limit;
2646 | |         let position = match pos {
2681 | |     }
2682 | | }
     | |_^

