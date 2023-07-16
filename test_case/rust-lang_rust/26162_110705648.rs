 rust
// scene.rs
use renderable::{Hit, Renderable};

...

if obj as *const Renderable == light as *const Renderable { true } else { false }
