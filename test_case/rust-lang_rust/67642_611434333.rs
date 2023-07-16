rust
struct Span { lo: u32, hi: u32 }

struct Interner {
  interned: HashSet<Span>,  // equality & hash is determined by `&self.storage[span]`
  storage: String,
}
