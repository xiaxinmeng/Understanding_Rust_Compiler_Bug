rust
enum TrivialDrop {
 A, B, C
}

impl !Sync for TrivialDrop {}
