rust
conditional {
    stage0 => {
        trait Foo { }
        impl trait Foo for .. { }
    }
    stageN => {
        auto trait Foo { }
    }
}
