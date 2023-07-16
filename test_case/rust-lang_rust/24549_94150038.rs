 rust
trait NotStatic for lifetime {}

impl NotStatic for .. {}
impl !NotStatic for 'static {}
