
struct S: 8 bytes
- field `f: bool`: 1 bit used, 31 bits padding
- field `g: i32`: 4 bytes used, 0 bytes padding

enum E: 24 bytes
- discriminant: 8 bytes
- variant `A`: 16 bytes (largest variant)
  - field `0: i64`: 8 bytes used, 0 bytes padding
  - field `1: i32`: 4 bytes used, 4 bytes padding
- variant B: 8 bytes
  - field `0: S`: 8 bytes used, 8 bytes unused
