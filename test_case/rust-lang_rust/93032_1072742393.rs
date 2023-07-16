rust
struct InhabittedLayout { size: usize, align: usize }
enum Layout {
   Uninhabitted,
   Inhabitted(InhabittedLayout),
}
