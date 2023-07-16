rust
fn for_value<T>(t: &T) -> Layout
fn align_to(&self, align: usize) -> Result<Layout, LayoutError>
fn pad_to_align(&self) -> Layout
fn extend(&self, next: Layout) -> Result<(Layout, usize), LayoutError>
fn array<T>(n: usize) -> Result<Layout, LayoutError>
