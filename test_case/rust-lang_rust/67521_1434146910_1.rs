rust
unsafe fn for_value_raw<T>(t: *const T) -> Layout
fn padding_needed_for(&self, align: usize) -> usize
fn repeat(&self, n: usize) -> Result<(Layout, usize), LayoutError>
fn repeat_packed(&self, n: usize) -> Result<Layout, LayoutError>
fn extend_packed(&self, next: Layout) -> Result<Layout, LayoutError>
