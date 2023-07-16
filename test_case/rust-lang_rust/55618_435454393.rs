rust
use std::sync::Arc;

mod foo {
    mod bar {
        // uses the `use` from parent module, because it is in the same file
        fn baz() -> Arc<u32> { ... }
    }
}
