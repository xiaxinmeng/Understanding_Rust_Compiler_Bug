 rust
mod aaa {
    fn bbb() -> ddd::eee::fff {
        struct ccc { ... }    
    }
}

mod ddd {
    fn eee -> aaa::bbb::ccc {
        struct fff { ... }
        ...
    }
}
